# QUIPS Wallet

## Descripción

QUIPS Wallet es una aplicación de cartera de criptomonedas desarrollada en Rust, diseñada para la red IOTA. Esta wallet permite a los usuarios generar una semilla (seed), crear direcciones, consultar balances y enviar tokens QUIPS.

## Características

- Generación de semillas únicas para cada usuario.
- Creación de direcciones IOTA.
- Consulta de balances.
- Envío de tokens QUIPS a otras direcciones.
- Almacenamiento y carga de datos de usuario desde archivos JSON.

## Requisitos

- Rust (https://www.rust-lang.org/)
- Cargo (incluido con Rust)
- Visual Studio Code o cualquier editor de código de tu preferencia

## Dependencias

El proyecto utiliza las siguientes dependencias de Rust:

```toml
[dependencies]
iota-client = "1.4.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
