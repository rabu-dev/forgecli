use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;


pub fn crear_proyecto(project_name: &str, stack_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creando proyecto '{}' con el stack '{}'...", project_name, stack_type);
    let project_path = Path::new(project_name);

    if project_path.exists() {
        println!("El directorio '{}' ya existe.", project_name);
        return Ok(());
    }
    fs::create_dir_all(project_path)?;

    if stack_type == "rust" {
        crear_stack_rust(project_path, project_name)?;
    } else if stack_type == "react" {
        crear_stack_react(project_path, project_name)?;
    } else if stack_type == "next" {
        crear_stack_next(project_path, project_name)?;
    } else {
        println!("Stack '{}' no reconocido. Solo se creó el directorio principal.", stack_type);
    }
    
    // Inicializar git
    Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .status()?;
    
    Ok(())
}

fn crear_stack_rust(project_path: &Path, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = project_path.join("src");
    fs::create_dir(src_path.clone())?;

    let main_rs_path = src_path.join("main.rs");
    let mut main_rs_file = File::create(main_rs_path)?;
    main_rs_file.write_all(b"fn main() {\n    println!(\"Hello, world!\");\n}\n")?;

    let cargo_toml_path = project_path.join("Cargo.toml");
    let mut cargo_toml_file = File::create(cargo_toml_path)?;
    let cargo_content = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
        project_name
    );
    cargo_toml_file.write_all(cargo_content.as_bytes())?;

    let gitignore_path = project_path.join(".gitignore");
    let mut gitignore_file = File::create(gitignore_path)?;
    gitignore_file.write_all(b"/target\nCargo.lock\n")?;

    println!("Estructura del proyecto Rust creada con éxito en '{}'.", project_name);
    Ok(())
}

fn crear_stack_react(project_path: &Path, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // --- Crear estructura de carpetas ---
    fs::create_dir(project_path.join("public"))?;
    fs::create_dir(project_path.join("src"))?;

    // --- Contenido de los archivos ---
    let package_json_content = format!(r#"{{
  "name": "{}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  }},
  "dependencies": {{
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  }},
  "devDependencies": {{
    "@types/react": "^18.2.48",
    "@types/react-dom": "^18.2.18",
    "@vitejs/plugin-react": "^4.2.1",
    "typescript": "^5.3.3",
    "vite": "^5.0.12",
    "tailwindcss": "^3.4.1",
    "postcss": "^8.4.33",
    "autoprefixer": "^10.4.17"
  }}
}}"#, project_name);

    let index_html_content = r###"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Vite + React + TS</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>"###;

    let main_tsx_content = r###"import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);"###;

    let app_tsx_content = r###"import React from 'react';

function App() {
  return (
    <div className="min-h-screen bg-gray-900 flex items-center justify-center text-white">
      <h1 className="text-4xl">Hello, React with TypeScript and Tailwind!</h1>
    </div>
  );
}

export default App;"###;

    let index_css_content = r###"@tailwind base;
@tailwind components;
@tailwind utilities;"###;

    let tailwind_config_content = r###"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}"###;

    let postcss_config_content = r###"module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}"###;

    let gitignore_content = r###"/node_modules
/build
/dist
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

npm-debug.log*
yarn-debug.log*
yarn-error.log*"###;

    let tsconfig_content = r###"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}"###;

    let tsconfig_node_content = r###"{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler"
  },
  "include": ["vite.config.ts"]
}"###;

    let vite_config_content = r###"import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
});
"###;

    // --- Crear y escribir en los archivos ---
    let mut package_json_file = File::create(project_path.join("package.json"))?;
    package_json_file.write_all(package_json_content.as_bytes())?;

    let mut index_html_file = File::create(project_path.join("index.html"))?;
    index_html_file.write_all(index_html_content.as_bytes())?;

    let mut main_tsx_file = File::create(project_path.join("src/main.tsx"))?;
    main_tsx_file.write_all(main_tsx_content.as_bytes())?;

    let mut app_tsx_file = File::create(project_path.join("src/App.tsx"))?;
    app_tsx_file.write_all(app_tsx_content.as_bytes())?;

    let mut index_css_file = File::create(project_path.join("src/index.css"))?;
    index_css_file.write_all(index_css_content.as_bytes())?;

    let mut tailwind_config_file = File::create(project_path.join("tailwind.config.js"))?;
    tailwind_config_file.write_all(tailwind_config_content.as_bytes())?;

    let mut postcss_config_file = File::create(project_path.join("postcss.config.js"))?;
    postcss_config_file.write_all(postcss_config_content.as_bytes())?;

    let mut gitignore_file = File::create(project_path.join(".gitignore"))?;
    gitignore_file.write_all(gitignore_content.as_bytes())?;

    let mut tsconfig_file = File::create(project_path.join("tsconfig.json"))?;
    tsconfig_file.write_all(tsconfig_content.as_bytes())?;

    let mut tsconfig_node_file = File::create(project_path.join("tsconfig.node.json"))?;
    tsconfig_node_file.write_all(tsconfig_node_content.as_bytes())?;

    let mut vite_config_file = File::create(project_path.join("vite.config.ts"))?;
    vite_config_file.write_all(vite_config_content.as_bytes())?;

    // Copiar un favicon SVG básico a public
    fs::create_dir_all(project_path.join("public"))?;
    let mut vite_svg_file = File::create(project_path.join("public/vite.svg"))?;
    vite_svg_file.write_all(br###"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
</svg>"###)?;

    // Instalar dependencias
    println!("Instalando dependencias...");
    Command::new("npm")
        .arg("install")
        .current_dir(project_path)
        .status()?;

    println!("Estructura del proyecto React creada con éxito en '{}'.", project_name);
    println!("Para ejecutar el proyecto:");
    println!("  cd {}", project_name);
    println!("  npm run dev");
    Ok(())
}

fn crear_stack_next(project_path: &Path, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // --- Crear estructura de carpetas ---
    fs::create_dir_all(project_path.join("app"))?;
    fs::create_dir_all(project_path.join("public"))?;

    // --- Contenido de los archivos ---
    let package_json_content = format!(r#"{{
      "name": "{}",
      "version": "0.1.0",
      "private": true,
      "scripts": {{
        "dev": "next dev",
        "build": "next build",
        "start": "next start",
        "lint": "next lint"
      }},
      "dependencies": {{
        "react": "^18",
        "react-dom": "^18",
        "next": "14.2.3"
      }},
      "devDependencies": {{
        "typescript": "^5",
        "@types/node": "^20",
        "@types/react": "^18",
        "@types/react-dom": "^18",
        "postcss": "^8",
        "tailwindcss": "^3.4.1",
        "autoprefixer": "^10.4.17",
        "eslint": "^8",
        "eslint-config-next": "14.2.3"
      }}
    }}"#, project_name);

    let next_config_content = r###"/** @type {import('next').NextConfig} */
const nextConfig = {};

module.exports = nextConfig;"###;

    let layout_tsx_content = r###"import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}"###;

    let page_tsx_content = r###"export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Get started by editing&nbsp;
          <code className="font-mono font-bold">app/page.tsx</code>
        </p>
      </div>
    </main>
  );
}"###;

    let globals_css_content = r###"@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  --foreground-rgb: 0, 0, 0;
  --background-start-rgb: 214, 219, 220;
  --background-end-rgb: 255, 255, 255;
}

@media (prefers-color-scheme: dark) {
  :root {
    --foreground-rgb: 255, 255, 255;
    --background-start-rgb: 0, 0, 0;
    --background-end-rgb: 0, 0, 0;
  }
}

body {
  color: rgb(var(--foreground-rgb));
  background: linear-gradient(
      to bottom,
      transparent,
      rgb(var(--background-end-rgb))
    )
    rgb(var(--background-start-rgb));
}

@layer utilities {
  .text-balance {
    text-wrap: balance;
  }
}"###;

    let tailwind_config_content = r###"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
    },
  },
  plugins: [],
}"###;

    let postcss_config_content = r###"module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}"###;

    let gitignore_content = r###"# See https://help.github.com/articles/ignoring-files/   for more about ignoring files.

# dependencies
/node_modules
/.pnp
.pnp.js
.yarn/install-state.gz

# testing
/coverage

# next.js
/.next/
/out/

# production
/build

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*

# local env files
.env*.local

# vercel
.vercel

# typescript
*.tsbuildinfo
next-env.d.ts
"###;

    let tsconfig_content = r###"{
  "compilerOptions": {
    "target": "es5",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}"###;

    let next_env_content = r###"/// <reference types="next" />
/// <reference types="next/image-types/global" />

// NOTE: This file should not be edited
// see https://nextjs.org/docs/basic-features/typescript for more information.
"###;

    // --- Crear y escribir en los archivos ---
    let mut package_json_file = File::create(project_path.join("package.json"))?;
    package_json_file.write_all(package_json_content.as_bytes())?;

    let mut next_config_file = File::create(project_path.join("next.config.js"))?;
    next_config_file.write_all(next_config_content.as_bytes())?;

    let mut layout_tsx_file = File::create(project_path.join("app/layout.tsx"))?;
    layout_tsx_file.write_all(layout_tsx_content.as_bytes())?;

    let mut page_tsx_file = File::create(project_path.join("app/page.tsx"))?;
    page_tsx_file.write_all(page_tsx_content.as_bytes())?;

    let mut globals_css_file = File::create(project_path.join("app/globals.css"))?;
    globals_css_file.write_all(globals_css_content.as_bytes())?;

    let mut tailwind_config_file = File::create(project_path.join("tailwind.config.js"))?;
    tailwind_config_file.write_all(tailwind_config_content.as_bytes())?;

    let mut postcss_config_file = File::create(project_path.join("postcss.config.js"))?;
    postcss_config_file.write_all(postcss_config_content.as_bytes())?;

    let mut gitignore_file = File::create(project_path.join(".gitignore"))?;
    gitignore_file.write_all(gitignore_content.as_bytes())?;

    let mut tsconfig_file = File::create(project_path.join("tsconfig.json"))?;
    tsconfig_file.write_all(tsconfig_content.as_bytes())?;

    let mut next_env_file = File::create(project_path.join("next-env.d.ts"))?;
    next_env_file.write_all(next_env_content.as_bytes())?;

    // Instalar dependencias
    println!("Instalando dependencias...");
    Command::new("npm")
        .arg("install")
        .current_dir(project_path)
        .status()?;

    println!("Estructura del proyecto Next.js creada con éxito en '{}'.", project_name);
    println!("Para ejecutar el proyecto:");
    println!("  cd {}", project_name);
    println!("  npm run dev");
    Ok(())
}