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
```
Crear un Proyecto 
```bash
# Crear proyecto Rust
cargo run -- crear mi-proyecto-rust rust

# Crear proyecto React
cargo run -- crear mi-app-react react

# Crear proyecto Next.js
cargo run -- crear mi-app-next next
```
#📁 Estructura Generada
## Proyecto React
```bash
mi-app-react/
├── public/
│   └── vite.svg
├── src/
│   ├── App.tsx
│   ├── index.css
│   └── main.tsx
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
└── .gitignore
```
##Proyecto Next.js
```bash
mi-app-next/
├── app/
│   ├── layout.tsx
│   └── page.tsx
├── public/
├── package.json
├── next.config.js
├── tsconfig.json
├── tailwind.config.js
├── postcss.config.js
└── .gitignore
```
## Proyecto Rust

```bash
mi-proyecto-rust/
├── src/
│   └── main.rs
├── Cargo.toml
└── .gitignore
```
#⚙️ Configuraciones Incluidas
##React
- Vite: Bundler rápido con Hot Module Replacement
- TypeScript: Tipado estático para mejor desarrollo
- Tailwind CSS: Framework CSS utility-first
- ESLint: Linting para mantener el código limpio
##Next.js
- App Router: Arquitectura moderna de Next.js
- TypeScript: Configuración completa
- Tailwind CSS: Integración nativa
- Optimizaciones: Configuradas por defecto
##Rust
- Edition 2021: Última edición estable
- Estructura limpia: Siguiendo las convenciones de Cargo
#🤝 Contribuciones
Las contribuciones son bienvenidas. Por favor, sigue estos pasos:

Haz un fork del repositorio
Crea una rama para tu feature (git checkout -b feature/NuevaFeature)
Haz commit de tus cambios (git commit -m 'Añadir nueva feature')
Haz push a la rama (git push origin feature/NuevaFeature)
Abre un Pull Request
#📄 Licencia
Este proyecto está licenciado bajo la Licencia MIT - ver el archivo LICENSE para más detalles.

# 📞 Contacto
 - Autor: Rabudev
 - Email: rabudevelopers@gmail.com
 - GitHub: @rabu-dev

# 🙏 Agradecimientos
Gracias a la comunidad de Rust por crear un lenguaje increíble
A los equipos de React, Next.js y Vite por sus excelentes herramientas
A todos los contribuidores de las bibliotecas utilizadas en este proyecto
#📈 Roadmap
Próximas versiones:
* Soporte para Vue.js
* Soporte para Svelte/SvelteKit
