use calcli::conversion_handler::scientific_notation::{
    convert_to_scientific, should_use_scientific, format_number,
};
use calcli::eval::evaluate_input;
use calcli::eval_context::EvalContext;

// Unit tests for conversion functions

#[test]
fn test_convert_to_scientific_basic() {
    assert_eq!(convert_to_scientific(10000000.0, 6), "1.00000e+7");
    assert_eq!(convert_to_scientific(0.00001, 6), "1.00000e-5");
    assert_eq!(convert_to_scientific(123.456, 6), "1.23456e+2");
}

#[test]
fn test_convert_to_scientific_precision() {
    assert_eq!(convert_to_scientific(1234567.0, 3), "1.23e+6");
    assert_eq!(convert_to_scientific(1234567.0, 6), "1.23457e+6");
    assert_eq!(convert_to_scientific(1234567.0, 8), "1.2345670e+6");
}

#[test]
fn test_convert_to_scientific_negative() {
    assert_eq!(convert_to_scientific(-10000000.0, 6), "-1.00000e+7");
    assert_eq!(convert_to_scientific(-0.00001, 6), "-1.00000e-5");
    assert_eq!(convert_to_scientific(-123.456, 3), "-1.23e+2");
}

#[test]
fn test_convert_to_scientific_zero() {
    assert_eq!(convert_to_scientific(0.0, 6), "0");
    assert_eq!(convert_to_scientific(-0.0, 6), "0");
}

#[test]
fn test_convert_to_scientific_special_values() {
    let nan = f64::NAN;
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;

    assert_eq!(convert_to_scientific(nan, 6), "NaN");
    assert_eq!(convert_to_scientific(inf, 6), "inf");
    assert_eq!(convert_to_scientific(neg_inf, 6), "-inf");
}

#[test]
fn test_convert_to_scientific_edge_cases() {
    assert_eq!(convert_to_scientific(1.0, 6), "1.00000e+0");
    assert_eq!(convert_to_scientific(10.0, 6), "1.00000e+1");
    assert_eq!(convert_to_scientific(0.1, 6), "1.00000e-1");
    assert_eq!(convert_to_scientific(0.01, 6), "1.00000e-2");
}

#[test]
fn test_convert_to_scientific_precision_1() {
    assert_eq!(convert_to_scientific(1234567.0, 1), "1e+6");
    assert_eq!(convert_to_scientific(0.00789, 1), "8e-3");
}

#[test]
fn test_should_use_scientific_digit_threshold() {
    // Threshold of 6 means: 6+ digits or 6+ leading zeros
    // 1,000,000 has 7 digits -> triggers
    assert!(should_use_scientific(1_000_000.0, 6));
    // 10,000,000 has 8 digits -> triggers
    assert!(should_use_scientific(10_000_000.0, 6));
    // 100,000 has 6 digits -> triggers
    assert!(should_use_scientific(100_000.0, 6));
    // 99,999 has 5 digits -> doesn't trigger
    assert!(!should_use_scientific(99_999.0, 6));
}

#[test]
fn test_should_use_scientific_small_numbers() {
    // Threshold of 6 means 6+ leading zeros
    // 0.0000001 has 7 leading zeros -> triggers
    assert!(should_use_scientific(0.0000001, 6));
    // 0.00000001 has 8 leading zeros -> triggers
    assert!(should_use_scientific(0.00000001, 6));
    // 0.000001 has 6 leading zeros -> triggers
    assert!(should_use_scientific(0.000001, 6));
    // 0.00001 has 5 leading zeros -> doesn't trigger
    assert!(!should_use_scientific(0.00001, 6));
    // 0.001 has 3 leading zeros -> doesn't trigger
    assert!(!should_use_scientific(0.001, 6));
}

#[test]
fn test_should_use_scientific_normal_range() {
    assert!(!should_use_scientific(123.456, 6));
    assert!(!should_use_scientific(999.0, 6));
    assert!(!should_use_scientific(0.001, 6));
    assert!(!should_use_scientific(0.1, 6));
}

#[test]
fn test_should_use_scientific_negative() {
    assert!(should_use_scientific(-10_000_000.0, 6));
    assert!(should_use_scientific(-0.0000001, 6));
    assert!(!should_use_scientific(-123.456, 6));
}

#[test]
fn test_should_use_scientific_special_values() {
    assert!(!should_use_scientific(0.0, 6));
    assert!(!should_use_scientific(f64::NAN, 6));
    assert!(!should_use_scientific(f64::INFINITY, 6));
}

#[test]
fn test_format_number_enabled() {
    // Scientific notation enabled with digit threshold of 6
    assert_eq!(format_number(10_000_000.0, true, 6, 6), "1.00000e+7");
    assert_eq!(format_number(0.0000001, true, 6, 6), "1.00000e-7");
    assert_eq!(format_number(123.456, true, 6, 6), "123.456");
}

#[test]
fn test_format_number_disabled() {
    // Scientific notation disabled
    assert_eq!(format_number(10_000_000.0, false, 6, 6), "10000000");
    assert_eq!(format_number(0.0000001, false, 6, 6), "0.0000001");
    assert_eq!(format_number(123.456, false, 6, 6), "123.456");
}

// Integration tests for commands

#[test]
fn test_automatic_formatting_large_numbers() {
    let mut ctx = EvalContext::new();
    let result = evaluate_input(&mut ctx, "10000000", true).unwrap();
    assert_eq!(result, 10_000_000.0);
    assert_eq!(ctx.format_result(result), "1.00000e+7");
}

#[test]
fn test_automatic_formatting_small_numbers() {
    let mut ctx = EvalContext::new();
    let result = evaluate_input(&mut ctx, "0.0000001", true).unwrap();
    assert_eq!(result, 0.0000001);
    assert_eq!(ctx.format_result(result), "1.00000e-7");
}

#[test]
fn test_automatic_formatting_normal_numbers() {
    let mut ctx = EvalContext::new();
    let result = evaluate_input(&mut ctx, "123.456", true).unwrap();
    assert_eq!(result, 123.456);
    assert_eq!(ctx.format_result(result), "123.456");
}

#[test]
fn test_sci_toggle_command() {
    let mut ctx = EvalContext::new();

    // Should be enabled by default
    assert!(ctx.sci_notation_enabled);

    // Toggle off
    let result = evaluate_input(&mut ctx, "sci toggle", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: Scientific notation disabled");
    assert!(!ctx.sci_notation_enabled);

    // Toggle on
    let result = evaluate_input(&mut ctx, "sci toggle", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: Scientific notation enabled");
    assert!(ctx.sci_notation_enabled);
}

#[test]
fn test_sci_toggle_affects_formatting() {
    let mut ctx = EvalContext::new();
    let result = evaluate_input(&mut ctx, "10000000", true).unwrap();

    // Enabled: should use scientific notation
    assert_eq!(ctx.format_result(result), "1.00000e+7");

    // Toggle off
    evaluate_input(&mut ctx, "sci toggle", true).ok();
    assert_eq!(ctx.format_result(result), "10000000");
}

#[test]
fn test_precision_command() {
    let mut ctx = EvalContext::new();

    // Default precision is 6
    assert_eq!(ctx.precision, 6);

    // Set to 3
    let result = evaluate_input(&mut ctx, "precision 3", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: Precision set to 3 significant figures");
    assert_eq!(ctx.precision, 3);

    // Set to 10
    let result = evaluate_input(&mut ctx, "precision 10", true);
    assert!(result.is_err());
    assert_eq!(ctx.precision, 10);
}

#[test]
fn test_precision_affects_output() {
    let mut ctx = EvalContext::new();
    let result = evaluate_input(&mut ctx, "1234567", true).unwrap();

    // Default precision (6)
    assert_eq!(ctx.format_result(result), "1.23457e+6");

    // Change to 3
    evaluate_input(&mut ctx, "precision 3", true).ok();
    assert_eq!(ctx.format_result(result), "1.23e+6");

    // Change to 10
    evaluate_input(&mut ctx, "precision 10", true).ok();
    assert_eq!(ctx.format_result(result), "1.234567000e+6");
}

#[test]
fn test_precision_validation() {
    let mut ctx = EvalContext::new();

    // Too low
    let result = evaluate_input(&mut ctx, "precision 0", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: Precision must be between 1 and 15");

    // Too high
    let result = evaluate_input(&mut ctx, "precision 20", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: Precision must be between 1 and 15");

    // Invalid input
    let result = evaluate_input(&mut ctx, "precision abc", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: Usage: precision <number>  (e.g., precision 6)");
}

#[test]
fn test_sci_convert_number() {
    let mut ctx = EvalContext::new();

    let result = evaluate_input(&mut ctx, "sci 42", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: 4.20000e+1");
}

#[test]
fn test_sci_convert_expression() {
    let mut ctx = EvalContext::new();

    let result = evaluate_input(&mut ctx, "sci 2+3", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: 5.00000e+0");
}

#[test]
fn test_sci_convert_line_reference() {
    let mut ctx = EvalContext::new();

    // Create a result
    evaluate_input(&mut ctx, "123.456", true).unwrap();

    // Convert it
    let result = evaluate_input(&mut ctx, "sci lin_1", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: 1.23456e+2");
}

#[test]
fn test_sci_convert_with_custom_precision() {
    let mut ctx = EvalContext::new();

    // Set precision to 3
    evaluate_input(&mut ctx, "precision 3", true).ok();

    // Convert number
    let result = evaluate_input(&mut ctx, "sci 1234567", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Parse error: 1.23e+6");
}

#[test]
fn test_line_reference_with_sci_notation() {
    let mut ctx = EvalContext::new();

    // Large number stored in lin_1
    let result1 = evaluate_input(&mut ctx, "10000000", true).unwrap();
    assert_eq!(result1, 10_000_000.0);

    // Use lin_1 in calculation
    let result2 = evaluate_input(&mut ctx, "lin_1 * 2", true).unwrap();
    assert_eq!(result2, 20_000_000.0);
    assert_eq!(ctx.format_result(result2), "2.00000e+7");
}

#[test]
fn test_calculations_with_sci_numbers() {
    let mut ctx = EvalContext::new();

    // Calculate with large numbers
    let result = evaluate_input(&mut ctx, "1000000 + 2000000", true).unwrap();
    assert_eq!(result, 3_000_000.0);
    assert_eq!(ctx.format_result(result), "3.00000e+6");

    // Calculate with small numbers
    let result = evaluate_input(&mut ctx, "0.0000001 * 2", true).unwrap();
    assert_eq!(result, 0.0000002);
    assert_eq!(ctx.format_result(result), "2.00000e-7");
}

#[test]
fn test_default_settings() {
    let ctx = EvalContext::new();

    assert_eq!(ctx.sci_notation_enabled, true);
    assert_eq!(ctx.precision, 6);
    assert_eq!(ctx.digit_threshold, 6);
}

#[test]
fn test_negative_numbers_formatting() {
    let mut ctx = EvalContext::new();

    let result = evaluate_input(&mut ctx, "-10000000", true).unwrap();
    assert_eq!(ctx.format_result(result), "-1.00000e+7");

    let result = evaluate_input(&mut ctx, "-0.0000001", true).unwrap();
    assert_eq!(ctx.format_result(result), "-1.00000e-7");
}

#[test]
fn test_zero_formatting() {
    let mut ctx = EvalContext::new();

    let result = evaluate_input(&mut ctx, "0", true).unwrap();
    assert_eq!(ctx.format_result(result), "0");

    let result = evaluate_input(&mut ctx, "10 - 10", true).unwrap();
    assert_eq!(ctx.format_result(result), "0");
}

#[test]
fn test_boundary_values() {
    let mut ctx = EvalContext::new();

    // Threshold is 6 (triggers on 6+ digits)
    // 1,000,000 has 7 digits (should use sci notation)
    let result = evaluate_input(&mut ctx, "1000000", true).unwrap();
    assert_eq!(ctx.format_result(result), "1.00000e+6");

    // 100,000 has 6 digits (should use sci notation)
    let result = evaluate_input(&mut ctx, "100000", true).unwrap();
    assert_eq!(ctx.format_result(result), "1.00000e+5");

    // 99,999 has 5 digits (should not use sci notation)
    let result = evaluate_input(&mut ctx, "99999", true).unwrap();
    assert_eq!(ctx.format_result(result), "99999");

    // 0.0000001 has 7 leading zeros (should use sci notation)
    let result = evaluate_input(&mut ctx, "0.0000001", true).unwrap();
    assert_eq!(ctx.format_result(result), "1.00000e-7");

    // 0.000001 has 6 leading zeros (should use sci notation)
    let result = evaluate_input(&mut ctx, "0.000001", true).unwrap();
    assert_eq!(ctx.format_result(result), "1.00000e-6");

    // 0.00001 has 5 leading zeros (should not use sci notation)
    let result = evaluate_input(&mut ctx, "0.00001", true).unwrap();
    assert_eq!(ctx.format_result(result), "0.00001");
}
