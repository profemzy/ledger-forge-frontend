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
