# Especificação Técnica: Otimização de Binário e Linking Estático musl

- **Ticket**: #10
- **Escopo**: Configuração de LTO fat e strip em `[profile.release]` e flags de linking estático com musl libc no laboratório métrico `lirica`.

## 1. Contexto e Motivação
A configuração de release utilizava `lto = true` e omitia `strip = true`. Além disso, para suportar alvos estáticos e utilitários derivados de forma homogênea, `.cargo/config.toml` deve estipular `-C target-feature=+crt-static` para alvos musl.

## 2. Requisitos Técnicos
1. `Cargo.toml`:
   - `opt-level = "z"`.
   - `lto = "fat"`.
   - `codegen-units = 1`.
   - `panic = "abort"`.
   - `strip = true`.
2. `.cargo/config.toml`:
   - Configurar `rustflags = ["-C", "target-feature=+crt-static"]` para os targets `x86_64-unknown-linux-musl` e `aarch64-unknown-linux-musl` preservando as configurações de registro existentes.
