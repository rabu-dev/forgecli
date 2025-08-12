# ForgeCLI - Generador de Proyectos Multi-Stack

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB" alt="React">
  <img src="https://img.shields.io/badge/Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white" alt="Next.js">
  <img src="https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript">
</p>

ForgeCLI es una herramienta de línea de comandos desarrollada en Rust que te permite crear proyectos listos para usar en múltiples tecnologías con una sola línea de comando.

## 🚀 Características

- **Multi-Stack**: Soporta Rust, React y Next.js
- **Configuración Automática**: Todos los proyectos vienen con configuraciones modernas preestablecidas
- **Instalación Inteligente**: Dependencias instaladas automáticamente
- **Estructura Profesional**: Sigue las mejores prácticas de cada framework
- **Listo para Desarrollar**: Proyectos completamente funcionales desde el primer momento

## 📦 Stacks Disponibles

### Rust
- Estructura estándar de Cargo
- `src/main.rs` con "Hello, world!" básico
- `Cargo.toml` configurado
- `.gitignore` optimizado

### React
- Vite como bundler
- TypeScript con configuración estricta
- Tailwind CSS preconfigurado
- Estructura moderna con `src/` y `public/`

### Next.js
- App Router de Next.js 14+
- TypeScript completo
- Tailwind CSS integrado
- Configuración de PostCSS y Autoprefixer

## 🛠️ Requisitos Previos

Antes de usar ForgeCLI, asegúrate de tener instalado:

- **Rust y Cargo** (para el stack Rust)
- **Node.js** v16+ (para stacks React/Next.js)
- **npm** o **yarn** (incluido con Node.js)
- **Git** (opcional pero recomendado)

## 📖 Uso

### Instalación

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/forgecli.git
cd forgecli

# Construir el proyecto
cargo build --release
