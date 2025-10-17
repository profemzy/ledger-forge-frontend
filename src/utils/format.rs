use rust_decimal::Decimal;

pub fn format_money(d: &Decimal) -> String {
    // Round to 2 decimal places and return string
    d.round_dp(2).to_string()
}
