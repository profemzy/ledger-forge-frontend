
# Akowe UI/UX Mockups & Design Specifications

**Brand:** Akowe - The Documenter  
**Design System:** Professional Accounting Interface  
**Last Updated:** October 7, 2025

---

## 🎨 Visual Design Language

### Color System

```css
/* Primary Palette - Professional Blue */
--akowe-blue-50:  #eff6ff;
--akowe-blue-100: #dbeafe;
--akowe-blue-500: #3b82f6;  /* Primary Brand */
--akowe-blue-600: #2563eb;  /* Primary Dark */
--akowe-blue-700: #1d4ed8;

/* Success - Accounting Green */
--akowe-green-50:  #f0fdf4;
--akowe-green-500: #22c55e;
--akowe-green-600: #16a34a;

/* Warning - Amber */
--akowe-amber-50:  #fffbeb;
--akowe-amber-500: #f59e0b;
--akowe-amber-600: #d97706;

/* Danger - Red */
--akowe-red-50:  #fef2f2;
--akowe-red-500: #ef4444;
--akowe-red-600: #dc2626;

/* Neutral - Professional Gray */
--akowe-gray-50:  #f9fafb;
--akowe-gray-100: #f3f4f6;
--akowe-gray-200: #e5e7eb;
--akowe-gray-300: #d1d5db;
--akowe-gray-400: #9ca3af;
--akowe-gray-500: #6b7280;
--akowe-gray-600: #4b5563;
--akowe-gray-700: #374151;
--akowe-gray-800: #1f2937;
--akowe-gray-900: #111827;
```

### Typography Scale

```css
/* Font Families */
--font-display: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
--font-body: 'Inter', system-ui, sans-serif;
--font-mono: 'JetBrains Mono', 'SF Mono', Consolas, monospace;

/* Font Sizes */
--text-xs:   0.75rem;   /* 12px */
--text-sm:   0.875rem;  /* 14px */
--text-base: 1rem;      /* 16px */
--text-lg:   1.125rem;  /* 18px */
--text-xl:   1.25rem;   /* 20px */
--text-2xl:  1.5rem;    /* 24px */
--text-3xl:  1.875rem;  /* 30px */
--text-4xl:  2.25rem;   /* 36px */

/* Font Weights */
--font-normal:   400;
--font-medium:   500;
--font-semibold: 600;
--font-bold:     700;

/* Line Heights */
--leading-tight:  1.25;
--leading-normal: 1.5;
--leading-relaxed: 1.75;
```

---

## 📱 Screen Mockups

### 1. Login Screen

```
┌─────────────────────────────────────────────────┐
│                                                 │
│                                                 │
│              ✍️  Akowe                          │
│           The Documenter                        │
│                                                 │
│     ┌─────────────────────────────────────┐   │
│     │  Email or Username                  │   │
│     │  ________________________________   │   │
│     └─────────────────────────────────────┘   │
│                                                 │
│     ┌─────────────────────────────────────┐   │
│     │  Password                           │   │
│     │  ________________________________   │   │
│     └─────────────────────────────────────┘   │
│                                                 │
│     [ ] Remember me    Forgot password?        │
│                                                 │
│     ┌─────────────────────────────────────┐   │
│     │         Sign In                     │   │
│     └─────────────────────────────────────┘   │
│                                                 │
│     Don't have an account? Sign up             │
│                                                 │
└─────────────────────────────────────────────────┘
```

**Design Notes:**
- Centered layout with max-width 400px
- Soft shadow on card
- Blue gradient background
- Smooth focus states on inputs
- Loading spinner on submit

---

### 2. Dashboard (Desktop)

```
┌──────────────────────────────────────────────────────────────────┐
│ ☰ Akowe    [🔍 Search...]              [🔔] [👤 John Doe ▾]    │
├────────┬─────────────────────────────────────────────────────────┤
│        │  Dashboard > Overview                                   │
│        │                                                          │
│  📊    │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  Dash  │  │ AR       │ │ AP       │ │ Cash     │ │ Revenue  │  │
│        │  │ $45,230  │ │ $12,450  │ │ $89,340  │ │ $125,600 │  │
│  📄    │  │ ↑ 12%    │ │ ↓ 5%     │ │ ↑ 8%     │ │ ↑ 15%    │  │
│  Inv   │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
│        │                                                          │
│  📋    │  Revenue Trend (Last 12 Months)                         │
│  Bills │  ┌────────────────────────────────────────────────┐    │
│        │  │     📈 Line Chart                              │    │
│  💰    │  │                                                │    │
│  Pay   │  │                                                │    │
│        │  └────────────────────────────────────────────────┘    │
│  👥    │                                                          │
│  Cont  │  Recent Activity                                        │
│        │  ┌────────────────────────────────────────────────┐    │
│  📊    │  │ • Invoice #1234 paid - $1,500    2 hours ago   │    │
│  Rep   │  │ • New bill from Acme Corp - $850  4 hours ago  │    │
│        │  │ • Payment received - $2,300       Yesterday    │    │
│  ⚙️    │  └────────────────────────────────────────────────┘    │
│  Set   │                                                          │
│        │  Quick Actions                                           │
│        │  [+ New Invoice] [+ New Bill] [+ Record Payment]        │
└────────┴─────────────────────────────────────────────────────────┘
```

**Design Notes:**
- Collapsible sidebar (240px → 64px)
- Stat cards with trend indicators
- Interactive charts with tooltips
- Activity feed with timestamps
- Prominent CTAs for common actions

---

### 3. Invoice List

```
┌──────────────────────────────────────────────────────────────────┐
│ Dashboard > Invoices                                             │
│                                                                  │
│ [+ New Invoice]  [📥 Import]  [⚙️ Filters ▾]  [🔍 Search...]   │
│                                                                  │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │ # │ Customer    │ Amount    │ Due Date   │ Status    │ ⋮  │ │
│ ├───┼─────────────┼───────────┼────────────┼───────────┼────┤ │
│ │ 1 │ Acme Corp   │ $1,500.00 │ Oct 15     │ 🟢 Paid   │ ⋮  │ │
│ │ 2 │ TechStart   │ $3,200.00 │ Oct 20     │ 🟡 Sent   │ ⋮  │ │
│ │ 3 │ Global Inc  │ $850.00   │ Oct 10     │ 🔴 Overdue│ ⋮  │ │
│ │ 4 │ StartupXYZ  │ $2,100.00 │ Oct 25     │ ⚪ Draft  │ ⋮  │ │
│ └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ Showing 1-10 of 45    [← Previous] [1] [2] [3] [Next →]        │
└──────────────────────────────────────────────────────────────────┘
```

**Design Notes:**
- Sortable columns (click header)
- Status badges with colors
- Row hover effects
- Action menu (⋮) for quick actions
- Bulk selection checkboxes
- Advanced filters in dropdown

---

### 4. Invoice Creation (Multi-Step)

```
Step 1: Customer & Details
┌──────────────────────────────────────────────────────────────────┐
│ Create Invoice                                          [✕ Close] │
│                                                                  │
│ ● Customer ─── ○ Line Items ─── ○ Review                        │
│                                                                  │
│ Customer *                                                       │
│ [Acme Corporation ▾]                                            │
│                                                                  │
│ Invoice Number *        Invoice Date *      Due Date *          │
│ [INV-2024-001]         [Oct 7, 2024 📅]    [Nov 6, 2024 📅]    │
│                                                                  │
│ Payment Terms                                                    │
│ [Net 30 ▾]                                                      │
│                                                                  │
│ Notes                                                            │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │ Thank you for your business...                             │ │
│ └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│                                    [Cancel] [Next: Line Items →]│
└──────────────────────────────────────────────────────────────────┘

Step 2: Line Items
┌──────────────────────────────────────────────────────────────────┐
│ Create Invoice                                          [✕ Close] │
│                                                                  │
│ ○ Customer ─── ● Line Items ─── ○ Review                        │
│                                                                  │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │ Description      │ Qty │ Rate      │ Account    │ Amount   │ │
│ ├──────────────────┼─────┼───────────┼────────────┼──────────┤ │
│ │ Consulting       │ 10  │ $150.00   │ Revenue ▾  │ $1,500   │ │
│ │ Design Work      │ 5   │ $200.00   │ Revenue ▾  │ $1,000   │ │
│ └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ [+ Add Line Item]                                               │
│                                                                  │
│                                          Subtotal:    $2,500.00 │
│                                          Tax (10%):     $250.00 │
│                                          ─────────────────────── │
│                                          Total:       $2,750.00 │
│                                                                  │
│                                    [← Back] [Next: Review →]    │
└──────────────────────────────────────────────────────────────────┘
```

**Design Notes:**
- Progress indicator at top
- Auto-calculate totals
- Inline validation
- Keyboard navigation (Tab, Enter)
- Save draft functionality
- Smooth transitions between steps

---

### 5. Financial Reports

```
┌──────────────────────────────────────────────────────────────────┐
│ Dashboard > Reports > Profit & Loss                              │
│                                                                  │
│ Report Type: [Profit & Loss ▾]                                  │
│                                                                  │
│ Date Range: [Jan 1, 2024 📅] to [Dec 31, 2024 📅]              │
│                                                                  │
│ [🔄 Refresh] [📊 Chart View] [📥 Export ▾]                      │
│                                                                  │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │ Profit & Loss Statement                                    │ │
│ │ January 1 - December 31, 2024                              │ │
│ │                                                            │ │
│ │ REVENUE                                                    │ │
│ │   Service Revenue              $125,600.00                 │ │
│ │   Product Sales                 $45,300.00                 │ │
│ │   ─────────────────────────────────────                   │ │
│ │   Total Revenue                $170,900.00                 │ │
│ │                                                            │ │
│ │ EXPENSES                                                   │ │
│ │   Cost of Goods Sold            $42,150.00                 │ │
│ │   Operating Expenses            $35,200.00                 │ │
│ │   Rent                          $24,000.00                 │ │
│ │   Utilities                      $3,600.00                 │ │
│ │   ─────────────────────────────────────                   │ │
│ │   Total Expenses               $104,950.00                 │ │
│ │                                                            │ │
│ │ ═══════════════════════════════════════                   │ │
│ │ NET INCOME                      $65,950.00  ✅             │ │
│ │ ═══════════════════════════════════════                   │ │
│ └────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
```

**Design Notes:**
- Clean, readable table format
- Hierarchical indentation
- Bold totals and subtotals
- Export options (PDF, CSV, Excel)
- Comparison with previous period
- Drill-down to transactions

---

### 6. Mobile Dashboard

```
┌─────────────────────────────┐
│ ☰  Akowe          🔔  👤   │
├─────────────────────────────┤
│                             │
│ Welcome back, John! 👋      │
│                             │
│ ┌─────────┐ ┌─────────┐    │
│ │ AR      │ │ AP      │    │
│ │ $45,230 │ │ $12,450 │    │
│