use calcli::eval::evaluate_input;
use calcli::eval_context::EvalContext;

fn eval_test(input: &str) -> Result<f64, calcli::eval::EvalError> {
    let mut ctx = EvalContext::new();
    evaluate_input(&mut ctx, input)
}

fn eval_with_ctx(ctx: &mut EvalContext, input: &str) -> Result<f64, calcli::eval::EvalError> {
    evaluate_input(ctx, input)
}

#[test]
fn test_basic_arithmetic() {
    assert_eq!(eval_test("2 + 2").unwrap(), 4.0);
    assert_eq!(eval_test("10 - 3").unwrap(), 7.0);
    assert_eq!(eval_test("4 * 5").unwrap(), 20.0);
    assert_eq!(eval_test("20 / 4").unwrap(), 5.0);
    assert_eq!(eval_test("2 ^ 3").unwrap(), 8.0);
}

#[test]
fn test_implicit_multiplication_numbers() {
    assert_eq!(eval_test("2(3)").unwrap(), 6.0);
    assert_eq!(eval_test("3(4+5)").unwrap(), 27.0);
    assert_eq!(eval_test("(2)(3)").unwrap(), 6.0);
    assert_eq!(eval_test("2(3)(4)").unwrap(), 24.0);
}

#[test]
fn test_implicit_multiplication_variables() {
    let mut ctx = EvalContext::new();
    eval_with_ctx(&mut ctx, "let x = 5").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "2x").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "x2").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "3x").unwrap(), 15.0);
    assert_eq!(eval_with_ctx(&mut ctx, "x(2+3)").unwrap(), 25.0);
}

#[test]
fn test_variable_assignment() {
    let mut ctx = EvalContext::new();

    // Simple variable
    assert_eq!(eval_with_ctx(&mut ctx, "let x = 10").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 10.0);

    // Variable with number
    assert_eq!(eval_with_ctx(&mut ctx, "let y1 = 20").unwrap(), 20.0);
    assert_eq!(eval_with_ctx(&mut ctx, "y1").unwrap(), 20.0);

    // Variable with expression
    assert_eq!(eval_with_ctx(&mut ctx, "let z = 2 + 3").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "z").unwrap(), 5.0);
}

#[test]
fn test_variable_iteration_format() {
    let mut ctx = EvalContext::new();
    assert_eq!(eval_with_ctx(&mut ctx, "let a1 = 5").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "let b2 = 10").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "let c123 = 15").unwrap(), 15.0);

    assert_eq!(eval_with_ctx(&mut ctx, "a1 + b2").unwrap(), 15.0);
    assert_eq!(eval_with_ctx(&mut ctx, "c123").unwrap(), 15.0);
}

#[test]
fn test_line_references() {
    let mut ctx = EvalContext::new();

    // First calculation
    eval_with_ctx(&mut ctx, "5 + 3").unwrap();
    ctx.counter += 1;

    // Second calculation
    eval_with_ctx(&mut ctx, "10 * 2").unwrap();
    ctx.counter += 1;

    // Reference previous lines
    assert_eq!(eval_with_ctx(&mut ctx, "lin1").unwrap(), 8.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin2").unwrap(), 20.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin1 + lin2").unwrap(), 28.0);
}

#[test]
fn test_line_references_with_implicit_mult() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "5").unwrap();
    ctx.counter += 1;

    assert_eq!(eval_with_ctx(&mut ctx, "2lin1").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin1(3)").unwrap(), 15.0);
}

#[test]
fn test_functions_sin_cos() {
    use std::f64::consts::PI;

    let result = eval_test("sin(0)").unwrap();
    assert!((result - 0.0).abs() < 1e-10);

    let result = eval_test(&format!("sin({})", PI / 2.0)).unwrap();
    assert!((result - 1.0).abs() < 1e-10);

    let result = eval_test("cos(0)").unwrap();
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_functions_ln_log() {
    use std::f64::consts::E;

    let result = eval_test("ln(1)").unwrap();
    assert!((result - 0.0).abs() < 1e-10);

    let result = eval_test(&format!("ln({})", E)).unwrap();
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_implicit_mult_with_functions() {
    use std::f64::consts::PI;

    // 2*sin(0)
    let result = eval_test("2sin(0)").unwrap();
    assert!((result - 0.0).abs() < 1e-10);

    // sin(π/2) * 3
    let result = eval_test(&format!("sin({})3", PI / 2.0)).unwrap();
    assert!((result - 3.0).abs() < 1e-10);
}

#[test]
fn test_decimal_implicit_zero() {
    assert_eq!(eval_test(".5").unwrap(), 0.5);
    assert_eq!(eval_test(".25").unwrap(), 0.25);
    assert_eq!(eval_test("1 + .5").unwrap(), 1.5);
    assert_eq!(eval_test("2.").unwrap(), 2.0);
}

#[test]
fn test_brackets() {
    assert_eq!(eval_test("[2 + 3]").unwrap(), 5.0);
    assert_eq!(eval_test("[2][3]").unwrap(), 6.0);
    assert_eq!(eval_test("2[3+4]").unwrap(), 14.0);
}

#[test]
fn test_complex_expressions() {
    let mut ctx = EvalContext::new();

    // let x = 5
    eval_with_ctx(&mut ctx, "let x = 5").unwrap();

    // 2x + 3(x - 1)
    let result = eval_with_ctx(&mut ctx, "2x + 3(x - 1)").unwrap();
    assert_eq!(result, 22.0); // 2*5 + 3*(5-1) = 10 + 12 = 22

    // x^2 + 2x + 1
    let result = eval_with_ctx(&mut ctx, "x^2 + 2x + 1").unwrap();
    assert_eq!(result, 36.0); // 25 + 10 + 1 = 36
}

#[test]
fn test_combined_variables_and_lines() {
    let mut ctx = EvalContext::new();

    // Line 1: x = 3
    eval_with_ctx(&mut ctx, "let x = 3").unwrap();
    ctx.counter += 1;

    // Line 2: 2x
    eval_with_ctx(&mut ctx, "2x").unwrap();
    ctx.counter += 1;

    // Line 3: lin2 + x
    let result = eval_with_ctx(&mut ctx, "lin2 + x").unwrap();
    assert_eq!(result, 9.0); // 6 + 3 = 9
}

#[test]
fn test_multiple_variables() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "let x = 2").unwrap();
    eval_with_ctx(&mut ctx, "let y = 3").unwrap();
    eval_with_ctx(&mut ctx, "let z = 4").unwrap();

    // xyz should be x*y*z
    let result = eval_with_ctx(&mut ctx, "xyz").unwrap();
    assert_eq!(result, 24.0); // 2 * 3 * 4 = 24

    // x + y + z
    let result = eval_with_ctx(&mut ctx, "x + y + z").unwrap();
    assert_eq!(result, 9.0);
}

#[test]
fn test_nested_expressions() {
    assert_eq!(eval_test("((2 + 3) * 4)").unwrap(), 20.0);
    assert_eq!(eval_test("2((3 + 4)(5))").unwrap(), 70.0); // 2 * ((7) * 5) = 70
}

#[test]
fn test_error_empty_input() {
    assert!(eval_test("").is_err());
}

#[test]
fn test_error_invalid_variable_name() {
    let mut ctx = EvalContext::new();
    assert!(eval_with_ctx(&mut ctx, "let 1x = 5").is_err());
}

#[test]
fn test_complex_with_functions_and_variables() {
    use std::f64::consts::PI;
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, &format!("let x = {}", PI)).unwrap();

    // sin(x) should be sin(π) ≈ 0
    let result = eval_with_ctx(&mut ctx, "sin(x)").unwrap();
    assert!(result.abs() < 1e-10);

    // 2sin(x/2)
    let result = eval_with_ctx(&mut ctx, "2sin(x/2)").unwrap();
    assert!((result - 2.0).abs() < 1e-10); // sin(π/2) = 1, so 2*1 = 2
}

#[test]
fn test_line_reference_multiple_digits() {
    let mut ctx = EvalContext::new();

    // Create 12 line results
    for i in 1..=12 {
        eval_with_ctx(&mut ctx, &format!("{}", i)).unwrap();
        ctx.counter += 1;
    }

    assert_eq!(eval_with_ctx(&mut ctx, "lin1").unwrap(), 1.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin10").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin12").unwrap(), 12.0);

    // lin1 should not match from lin10
    assert_eq!(eval_with_ctx(&mut ctx, "lin1 + lin10").unwrap(), 11.0);
}

#[test]
fn test_ln_advanced() {
    use std::f64::consts::E;

    // ln(e^2) = 2
    let result = eval_test(&format!("ln({})", E.powi(2))).unwrap();
    assert!((result - 2.0).abs() < 1e-10);

    // ln(e^3) = 3
    let result = eval_test(&format!("ln({})", E.powi(3))).unwrap();
    assert!((result - 3.0).abs() < 1e-10);
}

#[test]
fn test_sin_with_variables() {
    use std::f64::consts::PI;
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, &format!("let a = {}", PI / 6.0)).unwrap();

    // sin(π/6) = 0.5
    let result = eval_with_ctx(&mut ctx, "sin(a)").unwrap();
    assert!((result - 0.5).abs() < 1e-10);

    // 2sin(a) = 1
    let result = eval_with_ctx(&mut ctx, "2sin(a)").unwrap();
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_implicit_multiplication_edge_cases() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "let x = 3").unwrap();
    eval_with_ctx(&mut ctx, "let y = 4").unwrap();

    // xy(2) = 3 * 4 * 2 = 24
    assert_eq!(eval_with_ctx(&mut ctx, "xy(2)").unwrap(), 24.0);

    // (x)(y) = 3 * 4 = 12
    assert_eq!(eval_with_ctx(&mut ctx, "(x)(y)").unwrap(), 12.0);

    // 2xy = 2 * 3 * 4 = 24
    assert_eq!(eval_with_ctx(&mut ctx, "2xy").unwrap(), 24.0);
}

#[test]
fn test_variable_assignment_with_parentheses() {
    let mut ctx = EvalContext::new();

    // Variable assignment with parentheses - should NOT be treated as function
    assert_eq!(eval_with_ctx(&mut ctx, "let n = (5)(67)").unwrap(), 335.0);
    assert_eq!(eval_with_ctx(&mut ctx, "n").unwrap(), 335.0);

    // Another test case
    assert_eq!(eval_with_ctx(&mut ctx, "let m = (2+3)(4)").unwrap(), 20.0);
    assert_eq!(eval_with_ctx(&mut ctx, "m").unwrap(), 20.0);

    // More complex expression
    assert_eq!(eval_with_ctx(&mut ctx, "let p = (10)(2) + (3)(4)").unwrap(), 32.0);
    assert_eq!(eval_with_ctx(&mut ctx, "p").unwrap(), 32.0);
}

#[test]
fn test_function_definition_and_call() {
    let mut ctx = EvalContext::new();

    // Define a function f(x) = x^2
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();

    // Call the function
    assert_eq!(eval_with_ctx(&mut ctx, "f(5)").unwrap(), 25.0);
    assert_eq!(eval_with_ctx(&mut ctx, "f(10)").unwrap(), 100.0);

    // Define another function g(y) = 2y + 1
    eval_with_ctx(&mut ctx, "let g(y) = 2y + 1").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "g(3)").unwrap(), 7.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(0)").unwrap(), 1.0);
}

#[test]
fn test_function_vs_variable_disambiguation() {
    let mut ctx = EvalContext::new();

    // Function definition - has parentheses on LEFT side of =
    eval_with_ctx(&mut ctx, "let f(x) = x + 1").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "f(5)").unwrap(), 6.0);

    // Variable assignment with parentheses on RIGHT side of =
    assert_eq!(eval_with_ctx(&mut ctx, "let a = (2)(3)").unwrap(), 6.0);
    assert_eq!(eval_with_ctx(&mut ctx, "a").unwrap(), 6.0);

    // Variable assignment with complex expression
    assert_eq!(eval_with_ctx(&mut ctx, "let b = (4+1)(2)").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "b").unwrap(), 10.0);

    // Use the function in a variable assignment
    assert_eq!(eval_with_ctx(&mut ctx, "let c = f(10)").unwrap(), 11.0);
    assert_eq!(eval_with_ctx(&mut ctx, "c").unwrap(), 11.0);
}

#[test]
fn test_function_names_not_affected_by_variables() {
    let mut ctx = EvalContext::new();

    // Define variable n
    eval_with_ctx(&mut ctx, "let n = 5").unwrap();

    // sin(30) should still work, not get converted to si[n](30)
    let result = eval_with_ctx(&mut ctx, "sin(30)").unwrap();
    assert!((result - (-0.9880316240928618)).abs() < 1e-10);

    // cos should work
    let result = eval_with_ctx(&mut ctx, "cos(0)").unwrap();
    assert!((result - 1.0).abs() < 1e-10);

    // ln should work
    let result = eval_with_ctx(&mut ctx, "ln(1)").unwrap();
    assert!((result - 0.0).abs() < 1e-10);

    // Variable n should still work
    assert_eq!(eval_with_ctx(&mut ctx, "n").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "2n").unwrap(), 10.0);
}

#[test]
fn test_variable_with_digit_sequences() {
    let mut ctx = EvalContext::new();

    // Just n defined
    eval_with_ctx(&mut ctx, "let n = 5").unwrap();
    // n1n should be [n] * 1 * [n] = 5 * 1 * 5 = 25
    assert_eq!(eval_with_ctx(&mut ctx, "n1n").unwrap(), 25.0);

    // Now define n1
    eval_with_ctx(&mut ctx, "let n1 = 10").unwrap();
    // n1n should now be [n1] * [n] = 10 * 5 = 50
    assert_eq!(eval_with_ctx(&mut ctx, "n1n").unwrap(), 50.0);

    // n1 alone should be 10
    assert_eq!(eval_with_ctx(&mut ctx, "n1").unwrap(), 10.0);

    // n2 where n2 is not defined but n is should be [n] * 2
    assert_eq!(eval_with_ctx(&mut ctx, "n2").unwrap(), 10.0);
}

#[test]
fn test_multi_letter_variable_sequences() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "let x = 2").unwrap();
    eval_with_ctx(&mut ctx, "let y = 3").unwrap();
    eval_with_ctx(&mut ctx, "let z = 4").unwrap();

    // xyz should be [x] * [y] * [z] = 2 * 3 * 4 = 24
    assert_eq!(eval_with_ctx(&mut ctx, "xyz").unwrap(), 24.0);

    // xy should be [x] * [y] = 2 * 3 = 6
    assert_eq!(eval_with_ctx(&mut ctx, "xy").unwrap(), 6.0);

    // xz should be [x] * [z] = 2 * 4 = 8
    assert_eq!(eval_with_ctx(&mut ctx, "xz").unwrap(), 8.0);

    // xyy should be [x] * [y] * [y] = 2 * 3 * 3 = 18
    assert_eq!(eval_with_ctx(&mut ctx, "xyy").unwrap(), 18.0);
}

#[test]
fn test_mixed_defined_undefined_letters() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "let x = 2").unwrap();
    // xa where a is undefined should be xa (not converted)
    assert!(eval_with_ctx(&mut ctx, "xa").is_err());

    // But x alone should work
    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 2.0);

    // And x with numbers should work: x2 = [x] * 2
    assert_eq!(eval_with_ctx(&mut ctx, "x2").unwrap(), 4.0);
}

#[test]
fn test_parentheses_multiplication_with_variables() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "let n = 5").unwrap();

    // (n)(n) should be [n] * [n] = 5 * 5 = 25
    assert_eq!(eval_with_ctx(&mut ctx, "(n)(n)").unwrap(), 25.0);

    // (2)(n) should be 2 * [n] = 2 * 5 = 10
    assert_eq!(eval_with_ctx(&mut ctx, "(2)(n)").unwrap(), 10.0);

    // (n+1)(n-1) should be (5+1) * (5-1) = 6 * 4 = 24
    assert_eq!(eval_with_ctx(&mut ctx, "(n+1)(n-1)").unwrap(), 24.0);
}
