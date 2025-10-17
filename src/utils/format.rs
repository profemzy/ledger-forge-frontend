use rust_decimal::Decimal;

/// Format a Decimal as money with thousands separators and 2 decimals.
/// Examples: 1234.5 -> "1,234.50", -42 -> "-42.00"
pub fn format_money(d: &Decimal) -> String {
    let negative = d.is_sign_negative();
    let abs = d.abs();
    let s = abs.round_dp(2).to_string();

    // Ensure exactly 2 decimal places
    let parts = s.split('.').collect::<Vec<_>>();
    let int_part = parts[0].to_string();
    let mut frac = if parts.len() > 1 { parts[1].to_string() } else { "00".to_string() };
    if frac.len() < 2 { frac.push_str(&"0".repeat(2 - frac.len())); }
    if frac.len() > 2 { frac = frac.chars().take(2).collect(); }

    // Insert thousands separators
    let mut out = String::new();
    let mut cnt = 0usize;
    for ch in int_part.chars().rev() {
        if cnt == 3 {
            out.push(',');
            cnt = 0;
        }
        out.push(ch);
        cnt += 1;
    }
    let int_formatted: String = out.chars().rev().collect();

    let mut result = String::new();
    if negative { result.push('-'); }
    result.push_str(&int_formatted);
    result.push('.');
    result.push_str(&frac);
    result
}

/// Format a Decimal for CSV (fixed 2 decimals, no thousands separators)
pub fn format_money_csv(d: &Decimal) -> String {
    let s = d.round_dp(2).to_string();
    // Ensure exactly 2 decimals without separators
    let parts = s.split('.').collect::<Vec<_>>();
    let int_part = parts[0];
    let mut frac = if parts.len() > 1 { parts[1].to_string() } else { "00".to_string() };
    if frac.len() < 2 { frac.push_str(&"0".repeat(2 - frac.len())); }
    if frac.len() > 2 { frac = frac.chars().take(2).collect(); }
    format!("{}.{}", int_part, frac)
}

/// Compact money formatting for tight UI (e.g., charts): 1.2k, 3.4M
pub fn format_money_compact(d: &Decimal) -> String {
    let negative = d.is_sign_negative();
    let abs = d.abs();
    let thousand = Decimal::new(1_000, 0);
    let million = Decimal::new(1_000_000, 0);
    let billion = Decimal::new(1_000_000_000, 0);

    let (val, suffix) = if abs >= billion {
        ((abs / billion).round_dp(1), "B")
    } else if abs >= million {
        ((abs / million).round_dp(1), "M")
    } else if abs >= thousand {
        ((abs / thousand).round_dp(1), "k")
    } else {
        (abs.round_dp(0), "")
    };

    let mut s = val.to_string();
    // trim trailing .0
    if s.ends_with(".0") { s.truncate(s.len() - 2); }
    if negative { format!("-{}{}", s, suffix) } else { format!("{}{}", s, suffix) }
}

/// Sanitize and format an input string as money while typing.
/// - Keeps only digits and at most one '.'
/// - Limits to 2 decimal places
/// - Adds thousands separators to integer part
pub fn mask_money_input(input: &str) -> String {
    // Keep digits and one dot
    let mut cleaned = String::new();
    let mut seen_dot = false;
    for ch in input.chars() {
        if ch.is_ascii_digit() {
            cleaned.push(ch);
        } else if ch == '.' && !seen_dot {
            cleaned.push('.');
            seen_dot = true;
        }
    }

    // Split
    let mut parts = cleaned.split('.');
    let int_raw = parts.next().unwrap_or("");
    let mut frac_raw = parts.next().unwrap_or("").to_string();
    if frac_raw.len() > 2 { frac_raw.truncate(2); }

    // Remove leading zeros unless just "0"
    let int_trimmed = int_raw.trim_start_matches('0');
    let int_trimmed = if int_trimmed.is_empty() { "0" } else { int_trimmed };

    // Add commas
    let mut out = String::new();
    let mut cnt = 0usize;
    for ch in int_trimmed.chars().rev() {
        if cnt == 3 { out.push(','); cnt = 0; }
        out.push(ch); cnt += 1;
    }
    let int_fmt: String = out.chars().rev().collect();

    if seen_dot {
        format!("{}.{}", int_fmt, frac_raw)
    } else {
        int_fmt
    }
}
