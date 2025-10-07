
# Akowe Frontend Architecture
## Professional Accounting Interface for LedgerForge

**Brand:** Akowe (Yoruba: "The Documenter")  
**Vision:** A beautiful, fast, professional accounting interface with exceptional UX  
**Platforms:** Web (Desktop & Mobile) + Native Mobile Apps  
**Last Updated:** October 7, 2025

---

## 🎯 Executive Summary

Akowe is the frontend interface for LedgerForge, designed to deliver a **professional, beautiful, and blazingly fast** user experience for accounting professionals. Built with Rust and WebAssembly, it combines native-like performance with cross-platform compatibility.

### Core Principles
1. **Professional First** - Enterprise-grade UI that accountants trust
2. **Speed Obsessed** - Sub-100ms interactions, instant feedback
3. **Beautiful Design** - Modern, clean, accessible interface
4. **Mobile Ready** - Responsive web + native mobile apps
5. **Type Safe** - Full-stack Rust for reliability

---

## 🏗️ Technology Stack

### Frontend Framework: **Leptos** (Recommended)

**Why Leptos:**
- ✅ **Fine-grained reactivity** - Only re-renders what changes
- ✅ **Server-Side Rendering (SSR)** - Fast initial loads, SEO-friendly
- ✅ **Islands Architecture** - Hydrate only interactive components
- ✅ **Full-stack Rust** - Share types between frontend/backend
- ✅ **Excellent DX** - Similar to React/Solid.js but faster
- ✅ **Small bundle size** - ~50KB gzipped WASM

**Alternative: Dioxus**
- Cross-platform (Web, Desktop, Mobile) from single codebase
- React-like API, familiar to many developers
- Good for unified mobile/desktop apps

### UI Component Library

**Option 1: Custom Components with Tailwind CSS** (Recommended)
```rust
// Tailwind + Custom Rust Components
// Full control, optimized for accounting workflows
```

**Option 2: Leptos UI Libraries**
- `leptos-use` - Utility hooks
- `leptos-icons` - Icon components
- Custom component library inspired by Shadcn/ui

### Styling Strategy

**Tailwind CSS + CSS Variables**
```css
/* Design tokens for theming */
:root {
  --color-primary: #2563eb;      /* Professional blue */
  --color-success: #059669;      /* Accounting green */
  --color-danger: #dc2626;       /* Alert red */
  --color-warning: #d97706;      /* Warning amber */
  --font-sans: 'Inter', system-ui;
  --font-mono: 'JetBrains Mono', monospace;
}
```

### State Management

**Leptos Signals + Context API**
```rust
// Global state with signals
#[derive(Clone)]
pub struct AppState {
    user: RwSignal<Option<User>>,
    theme: RwSignal<Theme>,
    notifications: RwSignal<Vec<Notification>>,
}

// Local component state
let (count, set_count) = create_signal(0);
```

### Data Fetching

**Server Functions + SWR Pattern**
```rust
#[server(GetInvoices, "/api")]
pub async fn get_invoices(
    status: Option<InvoiceStatus>
) -> Result<Vec<Invoice>, ServerFnError> {
    // Calls LedgerForge API
}

// Client usage with caching
let invoices = create_resource(
    || (),
    |_| get_invoices(Some(InvoiceStatus::Overdue))
);
```

### Mobile Strategy

**Phase 1: Progressive Web App (PWA)**
- Responsive design (mobile-first)
- Offline support with Service Workers
- Install prompt for home screen
- Push notifications

**Phase 2: Native Mobile Apps**
- **iOS/Android:** Dioxus or Tauri Mobile
- Shared Rust codebase with web
- Native performance and APIs
- Platform-specific optimizations

---

## 🎨 Design System

### Brand Identity: Akowe

**Logo Concept:**
- Stylized pen/quill (documenter symbol)
- Modern, geometric design
- Professional color palette

**Color Palette:**

```css
/* Primary - Professional Blue */
--blue-50: #eff6ff;
--blue-500: #3b82f6;  /* Primary */
--blue-600: #2563eb;  /* Primary Dark */
--blue-900: #1e3a8a;

/* Success - Accounting Green */
--green-50: #f0fdf4;
--green-500: #22c55e;
--green-600: #16a34a;

/* Danger - Alert Red */
--red-50: #fef2f2;
--red-500: #ef4444;
--red-600: #dc2626;

/* Neutral - Professional Gray */
--gray-50: #f9fafb;
--gray-100: #f3f4f6;
--gray-500: #6b7280;
--gray-900: #111827;
```

**Typography:**

```css
/* Headings */
font-family: 'Inter', -apple-system, sans-serif;
font-weight: 600-700;

/* Body */
font-family: 'Inter', system-ui, sans-serif;
font-weight: 400-500;

/* Numbers/Data */
font-family: 'JetBrains Mono', 'SF Mono', monospace;
font-variant-numeric: tabular-nums;
```

**Spacing System:**
- 4px base unit
- 8px, 12px, 16px, 24px, 32px, 48px, 64px

**Border Radius:**
- Small: 4px (buttons, inputs)
- Medium: 8px (cards)
- Large: 12px (modals)

### Component Library

**Core Components:**

1. **Navigation**
   - Sidebar (collapsible)
   - Top bar (breadcrumbs, search, user menu)
   - Mobile bottom nav

2. **Data Display**
   - Tables (sortable, filterable, paginated)
   - Cards (dashboard widgets)
   - Lists (transactions, contacts)
   - Charts (financial visualizations)

3. **Forms**
   - Text inputs (with validation)
   - Select dropdowns (searchable)
   - Date pickers (range support)
   - Number inputs (currency formatting)
   - File upload (CSV import)

4. **Feedback**
   - Toast notifications
   - Loading states (skeletons)
   - Empty states
   - Error boundaries

5. **Overlays**
   - Modals (forms, confirmations)
   - Drawers (details, filters)
   - Tooltips (help text)
   - Popovers (actions)

---

## 📱 Responsive Design

### Breakpoints

```css
/* Mobile First */
--screen-sm: 640px;   /* Tablet */
--screen-md: 768px;   /* Small laptop */
--screen-lg: 1024px;  /* Desktop */
--screen-xl: 1280px;  /* Large desktop */
--screen-2xl: 1536px; /* Ultra-wide */
```

### Layout Strategy

**Desktop (≥1024px):**
```
┌─────────────────────────────────────┐
│ Top Bar (breadcrumbs, search, user)│
├──────┬──────────────────────────────┤
│      │                              │
│ Side │     Main Content Area        │
│ Nav  │     (Dashboard/Forms/Tables) │
│      │                              │
│      │                              │
└──────┴──────────────────────────────┘
```

**Tablet (768px-1023px):**
```
┌─────────────────────────────────────┐
│ Top Bar (hamburger, search, user)   │
├─────────────────────────────────────┤
│                                     │
│     Main Content (full width)       │
│     Sidebar: Drawer overlay         │
│                                     │
└─────────────────────────────────────┘
```

**Mobile (<768px):**
```
┌─────────────────────────────────────┐
│ Top Bar (hamburger, logo, user)     │
├─────────────────────────────────────┤
│                                     │
│     Main Content (stacked)          │
│     Optimized for touch             │
│                                     │
├─────────────────────────────────────┤
│ Bottom Nav (Dashboard, +, Reports)  │
└─────────────────────────────────────┘
```

---

## 🚀 Performance Optimization

### Bundle Size Targets

- **Initial Load:** <100KB (gzipped)
- **WASM Module:** <50KB (gzipped)
- **Total JS:** <150KB (gzipped)
- **CSS:** <20KB (gzipped)

### Loading Strategy

**1. Critical Path Optimization**
```rust
// SSR for instant first paint
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <LoadingSkeleton /> }>
            <Dashboard />
        </Suspense>
    }
}
```

**2. Code Splitting**
```rust
// Lazy load heavy components
let ReportViewer = lazy(|| import("./components/ReportViewer"));
```

**3. Image Optimization**
- WebP format with fallbacks
- Lazy loading below fold
- Responsive images (srcset)
- CDN delivery

**4. Caching Strategy**
```rust
// SWR (Stale-While-Revalidate)
let data = create_resource(
    || (),
    |_| async {
        // Check cache first
        // Fetch in background
        // Update cache
    }
);
```

### Performance Metrics

**Target Metrics:**
- **First Contentful Paint (FCP):** <1s
- **Largest Contentful Paint (LCP):** <2.5s
- **Time to Interactive (TTI):** <3s
- **Cumulative Layout Shift (CLS):** <0.1
- **First Input Delay (FID):** <100ms

---

## 🎭 User Experience Design

### Key User Flows

**1. Dashboard (Home)**
```
┌─────────────────────────────────────────┐
│ Welcome back, John! 👋                  │
├─────────────────────────────────────────┤
│ Quick Stats (Cards)                     │
│ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐   │
│ │ AR   │ │ AP   │ │ Cash │ │ P&L  │   │
│ └──────┘ └──────┘ └──────┘ └──────┘   │
├─────────────────────────────────────────┤
│ Recent Activity (Timeline)              │
│ • Invoice #1234 paid - $1,500          │
│ • New bill from Vendor X - $850        │
│ • Payment received - $2,300            │
├─────────────────────────────────────────┤
│ Quick Actions (Buttons)                 │
│ [+ Invoice] [+ Bill] [+ Payment]       │
└─────────────────────────────────────────┘
```

**2. Invoice Creation**
```
Step 1: Customer Selection
  ↓
Step 2: Line Items (with live total)
  ↓
Step 3: Terms & Notes
  ↓
Step 4: Review & Send
  ↓
Success: Invoice created + PDF generated
```

**3. Financial Reports**
```
1. Select Report Type (P&L, Balance Sheet, etc.)
2. Choose Date Range (with presets)
3. Apply Filters (accounts, categories)
4. View Report (interactive table)
5. Export (PDF, CSV, Excel)
```

### Interaction Patterns

**Optimistic UI Updates**
```rust
// Update UI immediately, rollback on error
let create_invoice = create_action(|invoice: &Invoice| {
    let invoice = invoice.clone();
    async move {
        // Optimistically add to list
        invoices.update(|list| list.push(invoice.clone()));
        
        // Send to server
        match api::create_invoice(invoice).await {
            Ok(_) => { /* Success! */ },
            Err(e) => {
                // Rollback on error
                invoices.update(|list| list.pop());
                show_error(e);
            }
        }
    }
});
```

**Keyboard Shortcuts**
- `Ctrl/Cmd + K` - Command palette
- `Ctrl/Cmd + N` - New invoice
- `Ctrl/Cmd + S` - Save draft
- `Ctrl/Cmd + /` - Search
- `Esc` - Close modal/drawer

**Smart Defaults**
- Auto-fill customer info
- Remember last used accounts
- Suggest invoice numbers
- Pre-populate dates

---

## 📊 Data Visualization

### Chart Library: **Plotly.rs** or **Charts.rs**

**Dashboard Charts:**

1. **Revenue Trend (Line Chart)**
   - Monthly revenue over 12 months
   - Comparison with previous year
   - Interactive tooltips

2. **Expense Breakdown (Pie/Donut Chart)**
   - Top expense categories
   - Percentage of total
   - Drill-down capability

3. **Cash Flow (Bar Chart)**
   - Inflows vs Outflows
   - Monthly comparison
   - Color-coded (green/red)

4. **AR Aging (Stacked Bar)**
   - Current, 30, 60, 90+ days
   - By customer
   - Click to view details

**Example Implementation:**
```rust
#[component]
pub fn RevenueChart(data: Vec<MonthlyRevenue>) -> impl IntoView {
    let chart_data = create_memo(move |_| {
        // Transform data for Plotly
        PlotlyData {
            x: data.iter().map(|d| d.month).collect(),
            y: data.iter().map(|d| d.revenue).collect(),
            type_: "line",
            marker: Marker { color: "#3b82f6" },
        }
    });
    
    view! {
        <div class="chart-container">
            <PlotlyChart data=chart_data />
        </div>
    }
}
```

---

## 🔐 Security & Authentication

### Auth Flow

**1. Login Page**
```rust
#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    
    let login = create_action(|credentials: &Credentials| {
        let credentials = credentials.clone();
        async move {
            match api::login(credentials).await {
                Ok(token) => {
                    // Store token securely
                    store_token(token);
                    // Redirect to dashboard
                    navigate("/dashboard");
                },
                Err(e) => show_error(e),
            }
        }
    });
    
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            login.dispatch(Credentials { username, password });
        }>
            <Input 
                type_="text"
                placeholder="Username"
                value=username
                on_input=set_username
            />
            <Input 
                type_="password"
                placeholder="Password"
                value=password
                on_input=set_password
            />
            <Button type_="submit">Login</Button>
        </form>
    }
}
```

**2. Token Management**
- Store JWT in httpOnly cookie (SSR) or secure storage
- Auto-refresh before expiry
-
**Mobile Ready** - Responsive + native apps  
🔒 **Secure** - Type-safe, tested, reliable  
🌍 **Accessible** - WCAG 2.1 AA compliant

The combination of Rust's performance, Leptos's reactivity, and thoughtful UX design will deliver an accounting interface that users love and trust.

**Next Steps:**
1. Review and approve this architecture
2. Create detailed Figma designs
3. Setup Leptos project structure
4. Begin Phase 1 implementation

---

*"Akowe - The Documenter: Where precision meets beauty in financial management."*

**Document Version:** 1.0  
**Last Updated:** October 7, 2025  
**Status:** Ready for Implementation