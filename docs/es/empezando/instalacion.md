# Instalación y Configuración

## Requisitos Previos

- **Rust / Cargo**: Necesitas tener Rust instalado para compilar Axiom desde el código fuente. Si no lo tienes, instálalo desde [rustup.rs](https://rustup.rs/).

## Instalando Axiom

Puedes instalar Axiom directamente desde el repositorio de GitHub usando Cargo:

```bash
cargo install --git https://github.com/mpineda/axiom
```

Después de la instalación, ejecuta el comando de configuración para inicializar la configuración de Axiom, la base de datos local y los esquemas:

```bash
axiom install
```

## Verificando la Instalación

Verifica que Axiom se haya instalado correctamente:

```bash
axiom --version
```
Esto debería devolver la versión actual de Axiom.

Consulta tu configuración actual y el estado de la telemetría:
```bash
axiom status
```
