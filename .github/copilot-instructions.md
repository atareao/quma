# Copilot Instructions for Quma

## Project Purpose

**QuMa (Quadlet Manager)** is a web-based management tool for Podman Quadlet files. It allows users to view, edit, and manage `.container`, `.network`, `.volume`, and `.kube` files located in `~/.config/containers/systemd/`.

**Core Functionality:**
- Scan and categorize Quadlet files by type (Container, Network, Volume, Kube)
- Provide a web interface to edit Quadlet configurations
- Save changes to disk and reload systemd user services automatically
- Runs as the same user managing rootless Podman services (always use `systemctl --user`)

## Architecture Overview

This is a full-stack monorepo with:
- **Backend**: Rust + Axum web framework serving REST API
- **Frontend**: React + TypeScript + Vite + Ant Design (dark mode UI)
- **Deployment**: Multi-stage Docker build producing Alpine-based container

The backend is served from `/app/backend` and frontend static assets from `/app/static/` in production.

## Version Management

**Critical**: This project uses [Vampus](https://github.com/atareao/vampus) for semantic versioning across both frontend and backend.

- Version is centrally managed in [.vampus.yml](.vampus.yml)
- DO NOT manually edit version fields in [backend/Cargo.toml](backend/Cargo.toml) or frontend `.env` files
- Use `vampus upgrade --patch|--minor|--major` to bump versions
- The `just upgrade` command handles version bumping, git tagging, and Docker image cleanup

## Development Workflow

Use [justfile](.justfile) commands (requires [just](https://github.com/casey/just)):

```bash
just dev          # Build frontend → copy to backend/static → run backend
just frontend     # Run Vite dev server (hot reload)
just backend      # Run Rust backend with RUST_LOG=debug
just watch        # Run backend with cargo-watch (60s debounce)
just upgrade      # Bump version, update deps, tag, cleanup old images
just build        # Build Docker image with current version
just push         # Push to registry.territoriolinux.es
```

**Key Pattern**: `just dev` copies built frontend assets to `backend/static/` - this mimics production setup where backend serves frontend.

## Frontend Specifics (React + Vite + Ant Design)

- **Build Tool**: Vite 7.x with `@vitejs/plugin-react-swc` (NOT Babel)
- **React**: Version 19.2.0
- **UI Framework**: Ant Design (antd) with dark mode theme
- **Icons**: `@ant-design/icons` (ContainerOutlined, GlobalOutlined, DatabaseOutlined)
- **ESLint**: Flat config format using `eslint/config` (see [frontend/eslint.config.js](frontend/eslint.config.js))
- **TypeScript**: Dual configs for app ([tsconfig.app.json](frontend/tsconfig.app.json)) and build tooling ([tsconfig.node.json](frontend/tsconfig.node.json))
- **Package Manager**: pnpm (configured in Dockerfile, though local dev may vary)

### Core UI Requirements

1. **Layout Structure**:
   - Use Ant Design `Layout` with `Sider` and `Content`
   - `Sider`: Menu with categorized submenu by type (Containers, Networks, Volumes, Kube)
   - `Content`: Card displaying selected file with text editor

2. **Editor Component**:
   - Use `Input.TextArea` with monospace font or integrate `react-simple-code-editor`
   - Display file name and content for selected Quadlet

3. **User Actions**:
   - Primary button: "Guardar y Aplicar" (Save and Apply) → POST to `/api/quadlets`
   - Use `notification` from Antd for success/error feedback after save operations

4. **Design Requirements**:
   - Dark mode theme (professional sysadmin-oriented design)
   - Backend API calls via fetch to `http://localhost:3000` in development

The frontend preserves SWC setup for performance - avoid switching to Babel.

## Backend Specifics (Rust + Axum)

- **Framework**: Axum web framework
- **Edition**: Rust 2024 (latest edition as of this project)
- **Runtime**: Tokio async runtime
- **Serialization**: serde for JSON handling
- **Database**: SQLite for minimal user management and application configuration
- **Migrations**: Use `sqlx-cli` for database migrations (located in `backend/migrations/`)
- **Static Files**: Served from `/app/static/` (populated by frontend build)
- **Environment**: Use `RUST_LOG=debug` for verbose logging during development

### Core Backend Requirements

1. **Data Model**:
   ```rust
   enum QuadletType { Container, Network, Volume, Kube }
   struct Quadlet {
       name: String,
       kind: QuadletType,
       content: String,
       path: PathBuf,
   }
   ```

2. **API Endpoints**:
   - `GET /api/quadlets` - Scan `~/.config/containers/systemd/`, identify files by extension, return JSON array
   - `POST /api/quadlets` - Receive `{ name, content }`, save to disk, execute `systemctl --user daemon-reload`

3. **System Integration**:
   - Use `std::process::Command` for executing `systemctl --user daemon-reload`
   - **Critical**: Always use `--user` flag since the binary runs as the rootless Podman user
   - Robust error handling with `Result` types

4. **CORS Configuration**:
   - Enable CORS for `http://localhost:5173` (Vite dev server) during development
   - Use `tower-http` crate's `CorsLayer` for Axum

5. **Database Schema** (SQLite):
   - **Users table**: Minimal user management (authentication, authorization)
   - **Configuration table**: Application settings and preferences
   - Use `sqlx` with compile-time checked queries
   - Migrations managed via `sqlx-cli` in `backend/migrations/`

## Development Integration

### CORS Setup (Development)
During development, the frontend runs on `localhost:5173` (Vite) and backend on `localhost:3000`. Configure Axum CORS:

```rust
use tower_http::cors::CorsLayer;

let cors = CorsLayer::permissive(); // Or configure specific origins
let app = Router::new()
    .route("/api/quadlets", get(list_quadlets))
    .layer(cors);
```

### Critical Security Note
The binary executes `systemctl --user` commands, so it must run as the same user managing rootless Podman services. Never use `systemctl` without the `--user` flag in this project.

## Current Implementation Status

- ✅ Infrastructure: Docker, justfile, versioning with Vampus
- ⚠️ Backend: Add Axum framework, implement Quadlet management API
- ⚠️ Frontend: Add Ant Design, implement file browser and editor UI
- ⚠️ Database: SQLite for users and configuration (migrations structure ready)
- ❌ No test infrastructure yet

When implementing features, follow the architectural patterns described above and maintain consistency with existing infrastructure.

## Docker Build Process

Multi-stage build ([Dockerfile](Dockerfile)):
1. **client-builder**: Node 22 slim → pnpm install → build frontend
2. **server-builder**: Rust Alpine 3.22 → compile release binary with static OpenSSL linking
3. **final**: Alpine 3.22 → copy binary + static assets + migrations, run as non-root user (uid 1000)

The final image exposes port 3000 and runs `/app/backend` as the `app` user.

## Important Conventions

- **Directory Naming**: Git history shows migration from `back/`+`front/` to `backend/`+`frontend/` - always use full names
- **Environment Variables**: Frontend uses `VITE_VERSION` from `.env` files (managed by Vampus, gitignored)
- **Registry**: Custom Docker registry at `registry.territoriolinux.es/atareao/quma`
- **Uploads**: Production container expects `/app/static/uploads` directory
