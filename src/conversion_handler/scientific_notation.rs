/// Convert a number to scientific notation with specified significant figures
pub fn convert_to_scientific(input: f64, precision: usize) -> String {
    if input.is_nan() || input.is_infinite() {
        return input.to_string();
    }

    if input == 0.0 {
        return "0".to_string();
    }

    let abs_val = input.abs();
    let exponent = abs_val.log10().floor() as i32;
    let mantissa = input / 10_f64.powi(exponent);

    // Format with precision decimal places (which gives precision+1 sig figs for mantissa)
    // We want 'precision' total significant figures, so we need precision-1 decimal places
    let decimal_places = if precision > 0 { precision - 1 } else { 0 };

    format!("{:.prec$}e{:+}", mantissa, exponent, prec = decimal_places)
}

/// Check if a number should be auto-formatted in scientific notation based on digit count threshold
/// digit_threshold: number of digits/leading zeros before triggering sci notation
/// - For large numbers: triggers when number has digit_threshold or more digits (e.g., 6 means 100000+)
/// - For small numbers: triggers when number has digit_threshold or more leading zeros (e.g., 4 means 0.0001 or smaller)
pub fn should_use_scientific(input: f64, digit_threshold: usize) -> bool {
    if input == 0.0 || input.is_nan() || input.is_infinite() {
        return false;
    }

    let abs_val = input.abs();

    // For large numbers: count digits before decimal point
    if abs_val >= 1.0 {
        let digits = abs_val.log10().floor() as i32 + 1;
        return digits >= digit_threshold as i32;
    }

    // For small numbers: count leading zeros after decimal point
    if abs_val < 1.0 {
        let leading_zeros = (-abs_val.log10()).floor() as usize;
        return leading_zeros >= digit_threshold;
    }

    false
}

/// Format a number based on scientific notation settings
pub fn format_number(
    input: f64,
    sci_enabled: bool,
    precision: usize,
    digit_threshold: usize,
) -> String {
    if sci_enabled && should_use_scientific(input, digit_threshold) {
        convert_to_scientific(input, precision)
    } else {
        input.to_string()
    }
}
