# Akowe Frontend

**The Documenter** — Professional accounting interface for LedgerForge.

Built with Leptos 0.6 (Rust + WebAssembly, CSR) for performance and type safety.

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
│   ├── main.rs                  # Entry point
│   ├── app.rs                   # App router (protected routes)
│   ├── api/                     # API clients
│   │   ├── client.rs            # HTTP client (JWT + refresh)
│   │   ├── accounts.rs, transactions.rs, contacts.rs,
│   │   │   invoices.rs, payments.rs, reporting.rs,
│   │   │   bills.rs, bill_payments.rs
│   ├── components/
│   │   ├── layout.rs            # Sidebar, header, breadcrumbs, dark toggle
│   │   ├── modal.rs             # Modal component
│   │   └── ui.rs                # Button, ButtonLink, Card, Table, Charts
│   ├── pages/                   # Pages by domain
│   │   ├── dashboard.rs
│   │   ├── accounts/{list,create,detail}.rs
│   │   ├── transactions/{list,create,detail}.rs
│   │   ├── invoices/{list,create,detail}.rs
│   │   ├── payments/{list,create,detail}.rs
│   │   ├── bills/{list,create,detail}.rs
│   │   ├── bill_payments/{create}.rs
│   │   └── reporting/{index,trial_balance,profit_loss,balance_sheet,ar_aging}.rs
│   ├── state/                   # Auth + toast
│   │   ├── auth.rs, notify.rs, mod.rs
│   ├── types/                   # Shared types
│   │   ├── accounts.rs, transactions.rs, invoices.rs,
│   │   │   payments.rs, reporting.rs, contacts.rs, bills.rs,
│   │   │   bill_payments.rs, user.rs, api.rs, mod.rs
│   └── utils/                   # Helpers (formatting, storage)
│       ├── format.rs, storage.rs, mod.rs
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

## ✅ Implemented Features

- Authentication: login, protected routes, JWT refresh
- Layout: sidebar with active state, sticky header, breadcrumbs, dark mode toggle
- UI Kit: Button, ButtonLink, Card, Modal, Table (sticky header + zebra)
- Charts: bar and line components with tooltips (Net Income line, Cash bars)
- Accounts: list, create, detail (hierarchy, balance, activate/deactivate)
- Transactions: list, create (balanced lines), detail (status transitions)
- Invoices (AR): list, create (per-line calc), detail (payments, status)
- Payments (AR): list, create, detail, apply to invoices, unapplied filter
- Bills (AP): list, create, detail
- Bill Payments (AP): create (apply to open vendor bills)
- Reports: Trial Balance, Profit & Loss, Balance Sheet, A/R Aging
  - CSV export + Print to PDF on all reports
- Formatting: currency formatting + masked numeric inputs
- Dark Mode: class strategy with global overrides (via Tailwind CDN)

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

### Components (high-level)
- Buttons/Links: primary, secondary, ghost variants
- Cards: neutral backgrounds (light/dark)
- Table: sticky header, zebra rows, scroll container
- Modal: centered overlay with action slot
- Charts: bar and line (SVG), tooltips

## 🔗 Backend Integration

Connects to LedgerForge backend at:
- Development: `http://localhost:3000/api/v1`
- Production: configure in `src/api/client.rs` (`api_base()`), or adapt to read from meta/env

Auth: JWT access tokens with automatic refresh on 401, stored in localStorage.

Routes (selected):
- `/dashboard`
- `/accounts`, `/accounts/new`, `/accounts/:id`
- `/transactions`, `/transactions/new`, `/transactions/:id`
- `/invoices`, `/invoices/new`, `/invoices/:id`
- `/payments`, `/payments/new`, `/payments/:id`
- `/bills`, `/bills/new`, `/bills/:id`
- `/bill-payments/new`
- `/reports`, `/reports/trial-balance`, `/reports/profit-loss`, `/reports/balance-sheet`, `/reports/ar-aging`

## 📝 License

Proprietary - All rights reserved
