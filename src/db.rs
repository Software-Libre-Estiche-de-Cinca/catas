use anyhow::Result;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier},
    Argon2,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use std::fmt;
use tracing::info;

pub type DbPool = Pool<SqliteConnectionManager>;

#[allow(dead_code)] // Evita advertencias por no usar el enum entero aún
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
enum Roles {
    AprendizDeMalta = 1,
    Catador = 2,
    CatadorMayor = 3,
    MaestreCervecero = 4,
    GranMaestreDeLaOrden = 5,
    PriorDelBarril = 6,
}

impl Roles {
    fn from_i32(value: i32) -> Option<Roles> {
        match value {
            1 => Some(Roles::AprendizDeMalta),
            2 => Some(Roles::Catador),
            3 => Some(Roles::CatadorMayor),
            4 => Some(Roles::MaestreCervecero),
            5 => Some(Roles::GranMaestreDeLaOrden),
            6 => Some(Roles::PriorDelBarril),
            _ => None,
        }
    }
}

pub struct User {
    id: i32,
    username: String,
    password_hash: String,
    role: Roles,
    created_at: String,
    updated_at: String,
}

impl User {
    pub fn fetch_from_db(conn: &Connection, username: &str) -> Result<Option<User>> {
        let mut stmt = conn.prepare(
            "SELECT id, username, password_hash, role, created_at, updated_at
             FROM users WHERE username = ?1",
        )?;

        let mut rows = stmt.query(params![username])?;

        if let Some(row) = rows.next()? {
            let role_value: i32 = row.get(3)?;
            let role = Roles::from_i32(role_value).ok_or_else(|| {
                anyhow::anyhow!("Rol inválido {} para el usuario {}", role_value, username)
            })?;
            Ok(Some(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn check_password(&self, password: &[u8]) -> Result<bool> {
        let parsed_hash = argon2::PasswordHash::new(&self.password_hash)?;
        let result = Argon2::default()
            .verify_password(password, &parsed_hash)
            .is_ok();
        Ok(result)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ id: {}, username: {}, role: {:?}, created_at: {}, updated_at: {} }}",
            self.id, self.username, self.role, self.created_at, self.updated_at
        )
    }
}

pub fn initialize_database_pool() -> Result<DbPool> {
    let manager = SqliteConnectionManager::file("catas.db");
    let pool = Pool::new(manager)?;

    let conn = pool.get()?;
    create_tables(&conn)?;
    create_default_admin_user(&conn)?;
    Ok(pool)
}

fn create_tables(conn: &Connection) -> Result<()> {
    // Inicializa el esquema base de la base de datos (idempotente)
    // https://dle.rae.es/idempotente
    info!("Inicializando tablas de la base de datos");

    // Tabla de usuarios
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role INTEGER NOT NULL
                CHECK (role IN (1, 2, 3, 4, 5, 6)),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Actualiza automáticamente `updated_at` en cada UPDATE
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_users_timestamp
         AFTER UPDATE ON users
         BEGIN
             UPDATE users SET updated_at = CURRENT_TIMESTAMP
             WHERE id = NEW.id;
         END",
        [],
    )?;

    Ok(())
}

fn generate_password(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789)(*&%$#@!";
    let mut rng = rand::rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rand::Rng::random_range(&mut rng, 0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

fn create_default_admin_user(conn: &Connection) -> Result<()> {
    // Crea un usuario administrador por defecto si no existe
    info!("Creando usuario administrador por defecto si no existe");

    let default_username = "admin";
    let default_password = generate_password(12);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(default_password.as_bytes())?
        .to_string();

    let rows = conn.execute(
        "INSERT OR IGNORE INTO users (username, password_hash, role) VALUES (?1, ?2, ?3)",
        params![
            default_username,
            &password_hash,
            Roles::PriorDelBarril as i32
        ],
    )?;

    if rows > 0 {
        info!("Usuario administrador creado con éxito");
        info!("La contraseña generada es: {}", default_password);
    } else {
        info!("El usuario administrador ya existe, no se creó uno nuevo");
    }

    Ok(())
}
