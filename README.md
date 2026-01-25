# QuMa - Quadlet Manager

<div align="center">

**Gestor web para archivos Quadlet de Podman**

[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-19.2-blue.svg)](https://reactjs.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

</div>

## 📋 Descripción

**QuMa (Quadlet Manager)** es una herramienta web para gestionar archivos Quadlet de Podman. Proporciona una interfaz intuitiva para visualizar, editar y aplicar configuraciones de contenedores, redes, volúmenes y definiciones de Kubernetes gestionadas por systemd.

### Características principales

- 🔍 **Escaneo automático** de archivos Quadlet en `~/.config/containers/systemd/`
- 📝 **Editor integrado** para modificar configuraciones
- 🔄 **Recarga automática** de servicios systemd tras guardar cambios
- 🎨 **Interfaz dark mode** diseñada para administradores de sistemas
- 🐳 **Gestión de tipos**: Containers, Networks, Volumes y Kube
- 🔒 **Gestión de usuarios** mediante SQLite

## 🛠️ Tecnologías

### Backend
- **Rust 2024** con framework **Axum**
- **Tokio** como runtime asíncrono
- **SQLite** para gestión de usuarios y configuración
- **sqlx** para consultas type-safe
- **serde** para serialización JSON

### Frontend
- **React 19.2** con **TypeScript**
- **Vite 7.x** con SWC (fast refresh)
- **Ant Design** (tema oscuro)
- **pnpm** como gestor de paquetes

### Deployment
- **Docker** multi-stage build
- Imagen basada en **Alpine Linux 3.22**
- Usuario no-root (uid 1000)

## 📦 Requisitos Previos

- [Rust](https://rustup.rs/) (edición 2024 o superior)
- [Node.js](https://nodejs.org/) 22+
- [pnpm](https://pnpm.io/)
- [just](https://github.com/casey/just) - command runner
- [Vampus](https://github.com/atareao/vampus) - version management (opcional)
- [Podman](https://podman.io/) con servicios rootless configurados

## 🚀 Inicio Rápido

### Instalación

```bash
# Clonar el repositorio
git clone https://github.com/atareao/quma.git
cd quma

# Instalar dependencias del frontend
cd frontend && pnpm install && cd ..

# Compilar el backend
cd backend && cargo build && cd ..
```

### Comandos de Desarrollo

El proyecto utiliza [just](https://github.com/casey/just) para automatizar tareas:

```bash
# Desarrollo completo (build frontend + run backend)
just dev

# Solo frontend (Vite dev server en localhost:5173)
just frontend

# Solo backend (servidor en localhost:3000)
just backend

# Backend con auto-reload (cargo-watch)
just watch

# Construir imagen Docker
just build

# Subir imagen al registry
just push

# Actualizar versión y hacer release
just upgrade
```

### Usando Docker

```bash
# Construir la imagen
docker buildx build -t quma:latest .

# Ejecutar el contenedor
docker run -d \
  -p 3000:3000 \
  -v ~/.config/containers/systemd:/root/.config/containers/systemd \
  quma:latest
```

Acceder a `http://localhost:3000`

## 📁 Estructura del Proyecto

```
quma/
├── .github/
│   └── copilot-instructions.md    # Instrucciones para AI coding agents
├── backend/
│   ├── src/
│   │   └── main.rs               # Punto de entrada del servidor Axum
│   ├── migrations/               # Migraciones SQLite (sqlx)
│   ├── static/                   # Assets del frontend (generado)
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── App.tsx               # Componente principal React
│   │   └── main.tsx              # Punto de entrada
│   ├── vite.config.ts            # Configuración Vite + SWC
│   ├── eslint.config.js          # ESLint flat config
│   └── package.json
├── .vampus.yml                   # Gestión de versiones
├── .justfile                     # Comandos de desarrollo
└── Dockerfile                    # Multi-stage build
```

## 🔧 Desarrollo

### Backend (Rust + Axum)

El backend implementa dos endpoints principales:

- `GET /api/quadlets` - Lista todos los archivos Quadlet encontrados
- `POST /api/quadlets` - Guarda cambios y ejecuta `systemctl --user daemon-reload`

**⚠️ Importante**: El servidor siempre usa `systemctl --user` ya que gestiona servicios rootless.

### Frontend (React + Ant Design)

La UI utiliza:
- **Layout** con Sider (menú categorizado) y Content (editor)
- **Notificaciones** para feedback de operaciones
- **Tema oscuro** profesional

### Gestión de Versiones

Este proyecto usa [Vampus](https://github.com/atareao/vampus) para versioning semántico:

```bash
# Incrementar versión (patch/minor/major)
vampus upgrade --patch

# Ver versión actual
vampus show
```

**❌ NO editar manualmente**: 
- `backend/Cargo.toml` version
- `frontend/.env` VITE_VERSION

### CORS en Desarrollo

Durante el desarrollo, el frontend (`localhost:5173`) y backend (`localhost:3000`) requieren CORS configurado en Axum:

```rust
use tower_http::cors::CorsLayer;

let cors = CorsLayer::permissive();
let app = Router::new()
    .route("/api/quadlets", get(list_quadlets))
    .layer(cors);
```

## 🐳 Docker

### Multi-stage Build

1. **client-builder**: Build frontend con Node 22 + pnpm
2. **server-builder**: Compilación Rust en Alpine con OpenSSL estático
3. **final**: Imagen Alpine mínima con binario + assets + migraciones

### Registry Personalizado

El proyecto usa un registry privado:

```
registry.territoriolinux.es/atareao/quma
```

## 📝 Base de Datos

SQLite se utiliza para:
- **Usuarios**: Autenticación y autorización básica
- **Configuración**: Preferencias de la aplicación

Las migraciones se gestionan con `sqlx-cli` en `backend/migrations/`.

## 🤝 Contribuir

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

### Convenciones

- Usa los comandos `just` para desarrollo
- Ejecuta `just upgrade` para versionar correctamente
- Mantén el setup de SWC en el frontend (no Babel)
- Siempre usa `systemctl --user` para comandos systemd

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver el archivo `LICENSE` para más detalles.

## 👤 Autor

**atareao**

- GitHub: [@atareao](https://github.com/atareao)
- Registry: [registry.territoriolinux.es](https://registry.territoriolinux.es)

## 🔗 Enlaces Útiles

- [Documentación de Podman Quadlet](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html)
- [Axum Framework](https://github.com/tokio-rs/axum)
- [Ant Design](https://ant.design/)
- [Just Command Runner](https://github.com/casey/just)
- [Vampus Version Manager](https://github.com/atareao/vampus)

---

<div align="center">
Made with ❤️ for Podman users
</div>
