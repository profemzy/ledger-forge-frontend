# Akowe Frontend

**The Documenter** - Professional accounting interface for LedgerForge

Built with Leptos (Rust + WebAssembly) for blazing performance and type safety.

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk (build tool)
cargo install trunk

# Install Tailwind CSS
npm install -D tailwindcss
npx tailwindcss init
```

### Development

```bash
# Run development server
trunk serve

# Open browser at http://localhost:8080
```

### Build for Production

```bash
# Build optimized bundle
trunk build --release

# Output in ./dist/
```

## 📁 Project Structure

```
frontend/
├── src/
│   ├── main.rs              # Entry point ✅
│   ├── app.rs               # Main app with routing ✅
│   ├── api/                 # API client (TODO)
│   ├── components/          # Reusable components (TODO)
│   │   ├── auth/           # Login, Register
│   │   ├── dashboard/      # Stats, Charts, Activity
│   │   ├── layout/         # Sidebar, Topbar
│   │   └── ui/             # Button, Input, Card
│   ├── pages/              # Page components (TODO)
│   ├── state/              # State management ✅
│   ├── types/              # Type definitions (TODO)
│   └── utils/              # Helpers (TODO)
├── docs/                   # Documentation ✅
│   ├── FRONTEND_ARCHITECTURE.md
│   ├── FRONTEND_COMPARISON.md
│   ├── FRONTEND_IMPLEMENTATION.md
│   └── FRONTEND_MOCKUPS.md
├── Cargo.toml              # Dependencies ✅
├── Trunk.toml              # Build config ✅
├── index.html              # HTML template ✅
├── input.css               # Tailwind styles ✅
└── tailwind.config.js      # Tailwind config ✅
```

## ✅ Completed

- [x] Project structure
- [x] Configuration files (Cargo.toml, Trunk.toml, Tailwind)
- [x] Main entry point (main.rs)
- [x] App component with routing (app.rs)
- [x] Auth state management (state/auth.rs)
- [x] Comprehensive documentation

## 🚧 Next Steps

### 1. Create Module Files
```bash
# Create all mod.rs files
touch src/api/mod.rs
touch src/components/mod.rs
touch src/pages/mod.rs
touch src/types/mod.rs
touch src/utils/mod.rs
```

### 2. Implement Utils
- [ ] src/utils/storage.rs - LocalStorage helpers
- [ ] src/utils/mod.rs - Export utilities

### 3. Implement API Client
- [ ] src/api/client.rs - HTTP client
- [ ] src/api/auth.rs - Auth API calls
- [ ] src/api/mod.rs - Export API functions

### 4. Implement Types
- [ ] src/types/user.rs - User types
- [ ] src/types/api.rs - API response types
- [ ] src/types/mod.rs - Export types

### 5. Implement UI Components
- [ ] src/components/ui/button.rs
- [ ] src/components/ui/input.rs
- [ ] src/components/ui/card.rs
- [ ] src/components/ui/mod.rs

### 6. Implement Pages
- [ ] src/pages/login.rs - Login page
- [ ] src/pages/dashboard.rs - Dashboard page
- [ ] src/pages/not_found.rs - 404 page
- [ ] src/pages/mod.rs

### 7. Implement Dashboard Components
- [ ] src/components/dashboard/stat_card.rs
- [ ] src/components/dashboard/activity.rs
- [ ] src/components/dashboard/mod.rs

### 8. Implement Layout
- [ ] src/components/layout/sidebar.rs
- [ ] src/components/layout/topbar.rs
- [ ] src/components/layout/layout.rs
- [ ] src/components/layout/mod.rs

## 📚 Documentation

See [`docs/`](./docs/) for complete documentation:

- **FRONTEND_ARCHITECTURE.md** - Technical architecture
- **FRONTEND_COMPARISON.md** - Leptos vs React analysis
- **FRONTEND_IMPLEMENTATION.md** - Implementation guide
- **FRONTEND_MOCKUPS.md** - UI/UX designs

## 🎨 Design System

### Colors
- Primary: `#2563eb` (Akowe Blue)
- Success: `#22c55e` (Accounting Green)
- Danger: `#ef4444` (Alert Red)

### Typography
- Sans: Inter
- Mono: JetBrains Mono

### Components
- Buttons: `.btn-primary`, `.btn-secondary`
- Cards: `.card`
- Inputs: `.input-field`

## 🔗 Backend Integration

Connects to LedgerForge backend at:
- Development: `http://localhost:3000/api/v1`
- Production: Configure in environment

## 📝 License

Proprietary - All rights reserved