# No se hacer Readmes
Se lo pides a una IA que para eso están.
Y ya está.

Lo basico para ejecutar:

```bash
cargo run
```

Luego lo tipico de Rust para saber si has hecho algo o todo mal:

```bash
cargo check --all
cargo clippy --all
cargo build --all --release
```

Y si, tienes que instalar Rust y Cargo, obvio.

## Mas cosas

La aplicación incluye SwaggerUI para documentar los endpoints de la API.

Una vez iniciada la aplicación con `cargo run`, puedes acceder a:

- **SwaggerUI**: http://127.0.0.1:3000/swagger-ui/
- **OpenAPI Spec**: http://127.0.0.1:3000/api-docs/openapi.json