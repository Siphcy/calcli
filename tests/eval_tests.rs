use calcli::eval::evaluate_input;
use calcli::eval_context::EvalContext;
use std::f64::consts::{E, PI};

fn eval_test(input: &str) -> Result<f64, calcli::error::EvalError> {
    let mut ctx = EvalContext::new();
    evaluate_input(&mut ctx, input)
}

fn eval_with_ctx(ctx: &mut EvalContext, input: &str) -> Result<f64, calcli::error::EvalError> {
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
    assert_eq!(eval_with_ctx(&mut ctx, "let y_1 = 20").unwrap(), 20.0);
    assert_eq!(eval_with_ctx(&mut ctx, "y_1").unwrap(), 20.0);

    // Variable with expression
    assert_eq!(eval_with_ctx(&mut ctx, "let z = 2 + 3").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "z").unwrap(), 5.0);
}

#[test]
fn test_variable_iteration_format() {
    let mut ctx = EvalContext::new();
    assert_eq!(eval_with_ctx(&mut ctx, "let a_1 = 5").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "let b_2 = 10").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "let c_123 = 15").unwrap(), 15.0);

    assert_eq!(eval_with_ctx(&mut ctx, "a_1 + b_2").unwrap(), 15.0);
    assert_eq!(eval_with_ctx(&mut ctx, "c_123").unwrap(), 15.0);
}

#[test]
fn test_line_references() {
    let mut ctx = EvalContext::new();

    // First calculation
    eval_with_ctx(&mut ctx, "5 + 3").unwrap();

    // Second calculation
    eval_with_ctx(&mut ctx, "10 * 2").unwrap();

    // Reference previous lines
    assert_eq!(eval_with_ctx(&mut ctx, "lin_1").unwrap(), 8.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin_2").unwrap(), 20.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin_1 + lin_2").unwrap(), 28.0);
}

#[test]
fn test_line_references_with_implicit_mult() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "5").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "2lin_1").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin_1(3)").unwrap(), 15.0);
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

    // Line 2: 2x
    eval_with_ctx(&mut ctx, "2x").unwrap();

    // Line 3: lin_2 + x
    let result = eval_with_ctx(&mut ctx, "lin_2 + x").unwrap();
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
    }

    assert_eq!(eval_with_ctx(&mut ctx, "lin_1").unwrap(), 1.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin_10").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "lin_12").unwrap(), 12.0);

    // lin_1 should not match from lin_10
    assert_eq!(eval_with_ctx(&mut ctx, "lin_1 + lin_10").unwrap(), 11.0);
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
    // n1n (no underscore) should be [n] * 1 * [n] = 5 * 1 * 5 = 25
    assert_eq!(eval_with_ctx(&mut ctx, "n1n").unwrap(), 25.0);

    // Now define n_1
    eval_with_ctx(&mut ctx, "let n_1 = 10").unwrap();
    // n_1n should now be [n_1] * [n] = 10 * 5 = 50
    assert_eq!(eval_with_ctx(&mut ctx, "n_1n").unwrap(), 50.0);

    // n_1 alone should be 10
    assert_eq!(eval_with_ctx(&mut ctx, "n_1").unwrap(), 10.0);

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

// ========== User-Defined Function Tests ==========

#[test]
fn test_function_basic() {
    let mut ctx = EvalContext::new();

    // Define f(x) = x^2
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "f(2)").unwrap(), 4.0);
    assert_eq!(eval_with_ctx(&mut ctx, "f(5)").unwrap(), 25.0);
    assert_eq!(eval_with_ctx(&mut ctx, "f(10)").unwrap(), 100.0);
    assert_eq!(eval_with_ctx(&mut ctx, "f(0)").unwrap(), 0.0);
}

#[test]
fn test_function_linear() {
    let mut ctx = EvalContext::new();

    // Define g(x) = 2x + 3
    eval_with_ctx(&mut ctx, "let g(x) = 2x + 3").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "g(0)").unwrap(), 3.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(1)").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(5)").unwrap(), 13.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(-2)").unwrap(), -1.0);
}

#[test]
fn test_function_with_expressions() {
    let mut ctx = EvalContext::new();

    // Define h(x) = (x+1)(x-1)
    eval_with_ctx(&mut ctx, "let h(x) = (x+1)(x-1)").unwrap();

    // h(x) = x^2 - 1
    assert_eq!(eval_with_ctx(&mut ctx, "h(0)").unwrap(), -1.0);
    assert_eq!(eval_with_ctx(&mut ctx, "h(2)").unwrap(), 3.0);
    assert_eq!(eval_with_ctx(&mut ctx, "h(5)").unwrap(), 24.0);
}

#[test]
fn test_function_with_variable() {
    let mut ctx = EvalContext::new();

    // Define variable
    eval_with_ctx(&mut ctx, "let a = 10").unwrap();

    // Define function that uses external variable
    eval_with_ctx(&mut ctx, "let f(x) = x + a").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "f(5)").unwrap(), 15.0);
    assert_eq!(eval_with_ctx(&mut ctx, "f(0)").unwrap(), 10.0);
}

#[test]
fn test_multiple_functions() {
    let mut ctx = EvalContext::new();

    // Define multiple functions
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();
    eval_with_ctx(&mut ctx, "let g(y) = 2y").unwrap();
    eval_with_ctx(&mut ctx, "let h(z) = z + 1").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 9.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(3)").unwrap(), 6.0);
    assert_eq!(eval_with_ctx(&mut ctx, "h(3)").unwrap(), 4.0);

    // Combine function calls
    assert_eq!(eval_with_ctx(&mut ctx, "f(2) + g(3)").unwrap(), 10.0); // 4 + 6
}

#[test]
fn test_function_composition() {
    let mut ctx = EvalContext::new();

    // Define f(x) = x + 1
    eval_with_ctx(&mut ctx, "let f(x) = x + 1").unwrap();

    // Define g(x) = 2x
    eval_with_ctx(&mut ctx, "let g(x) = 2x").unwrap();

    // Test f(g(3)) which should be f(6) = 7
    assert_eq!(eval_with_ctx(&mut ctx, "f(g(3))").unwrap(), 7.0);

    // Test g(f(3)) which should be g(4) = 8
    assert_eq!(eval_with_ctx(&mut ctx, "g(f(3))").unwrap(), 8.0);
}

#[test]
fn test_function_with_complex_expressions() {
    let mut ctx = EvalContext::new();

    // Define f(x) = x^2 + 2x + 1
    eval_with_ctx(&mut ctx, "let f(x) = x^2 + 2x + 1").unwrap();

    // f(0) = 1
    assert_eq!(eval_with_ctx(&mut ctx, "f(0)").unwrap(), 1.0);

    // f(1) = 1 + 2 + 1 = 4
    assert_eq!(eval_with_ctx(&mut ctx, "f(1)").unwrap(), 4.0);

    // f(3) = 9 + 6 + 1 = 16
    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 16.0);
}

#[test]
fn test_function_with_implicit_multiplication() {
    let mut ctx = EvalContext::new();

    // Define function
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();

    // 2 * f(3) = 2 * 9 = 18 (explicit multiplication)
    assert_eq!(eval_with_ctx(&mut ctx, "2 * f(3)").unwrap(), 18.0);

    // f(3) + f(2) = 9 + 4 = 13
    assert_eq!(eval_with_ctx(&mut ctx, "f(3) + f(2)").unwrap(), 13.0);

    // (f(2))^2 = 4^2 = 16
    assert_eq!(eval_with_ctx(&mut ctx, "(f(2))^2").unwrap(), 16.0);
}

#[test]
fn test_function_with_builtin_functions() {
    use std::f64::consts::PI;
    let mut ctx = EvalContext::new();

    // Define f(x) = sin(x) + 1
    eval_with_ctx(&mut ctx, &format!("let f(x) = sin(x) + 1")).unwrap();

    // f(0) = sin(0) + 1 = 1
    let result = eval_with_ctx(&mut ctx, "f(0)").unwrap();
    assert!((result - 1.0).abs() < 1e-10);

    // f(π/2) = sin(π/2) + 1 = 2
    let result = eval_with_ctx(&mut ctx, &format!("f({})", PI / 2.0)).unwrap();
    assert!((result - 2.0).abs() < 1e-10);
}

#[test]
fn test_function_parameter_naming() {
    let mut ctx = EvalContext::new();

    // Functions with different parameter names
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();
    eval_with_ctx(&mut ctx, "let g(y) = y^2").unwrap();
    eval_with_ctx(&mut ctx, "let h(z) = z^2").unwrap();

    // All should work the same way
    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 9.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(3)").unwrap(), 9.0);
    assert_eq!(eval_with_ctx(&mut ctx, "h(3)").unwrap(), 9.0);
}

// ========== Batch/Array Assignment Tests ==========

#[test]
fn test_batch_assignment_single() {
    let mut ctx = EvalContext::new();

    // Single item in brackets should also work: [f(x)] = [x^2]
    eval_with_ctx(&mut ctx, "let [f(x)] = [x^2]").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "f(5)").unwrap(), 25.0);

    // Single variable: [x] = [10]
    eval_with_ctx(&mut ctx, "let [x] = [10]").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 10.0);
}

#[test]
fn test_batch_assignment_simple() {
    let mut ctx = EvalContext::new();

    // Assign multiple variables at once: [x, y, z] = [1, 2, 3]
    eval_with_ctx(&mut ctx, "let [x, y, z] = [1, 2, 3]").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 1.0);
    assert_eq!(eval_with_ctx(&mut ctx, "y").unwrap(), 2.0);
    assert_eq!(eval_with_ctx(&mut ctx, "z").unwrap(), 3.0);
}

#[test]
fn test_batch_assignment_expressions() {
    let mut ctx = EvalContext::new();

    // Assign with expressions: [a, b, c] = [2+3, 4*5, 10/2]
    eval_with_ctx(&mut ctx, "let [a, b, c] = [2+3, 4*5, 10/2]").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "a").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "b").unwrap(), 20.0);
    assert_eq!(eval_with_ctx(&mut ctx, "c").unwrap(), 5.0);
}

#[test]
fn test_batch_assignment_functions() {
    let mut ctx = EvalContext::new();

    // Assign functions: [f(x), g(y)] = [x^2, 2y]
    eval_with_ctx(&mut ctx, "let [f(x), g(y)] = [x^2, 2y]").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 9.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(5)").unwrap(), 10.0);
}

#[test]
fn test_batch_assignment_mixed() {
    let mut ctx = EvalContext::new();

    // Mix variables and functions: [f(x), a, g(y)] = [x^2, 10, y+1]
    eval_with_ctx(&mut ctx, "let [f(x), a, g(y)] = [x^2, 10, y+1]").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "f(5)").unwrap(), 25.0);
    assert_eq!(eval_with_ctx(&mut ctx, "a").unwrap(), 10.0);
    assert_eq!(eval_with_ctx(&mut ctx, "g(3)").unwrap(), 4.0);
}

#[test]
fn test_batch_assignment_using_variables() {
    let mut ctx = EvalContext::new();

    // Define a variable first
    eval_with_ctx(&mut ctx, "let n = 5").unwrap();

    // Use it in batch assignment
    eval_with_ctx(&mut ctx, "let [x, y] = [n, n*2]").unwrap();

    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 5.0);
    assert_eq!(eval_with_ctx(&mut ctx, "y").unwrap(), 10.0);
}

#[test]
fn test_batch_assignment_error_mismatch() {
    let mut ctx = EvalContext::new();

    // Too many variables, not enough values
    let result = eval_with_ctx(&mut ctx, "let [x, y, z] = [1, 2]");
    assert!(result.is_err());

    // Too many values, not enough variables
    let result = eval_with_ctx(&mut ctx, "let [x, y] = [1, 2, 3]");
    assert!(result.is_err());
}

#[test]
fn test_batch_assignment_with_spacing() {
    let mut ctx = EvalContext::new();

    // Test with various spacing
    eval_with_ctx(&mut ctx, "let [x,y,z] = [1,2,3]").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 1.0);

    eval_with_ctx(&mut ctx, "let [a, b, c] = [4, 5, 6]").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "a").unwrap(), 4.0);

    eval_with_ctx(&mut ctx, "let [ p , q ] = [ 7 , 8 ]").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "p").unwrap(), 7.0);
}

#[test]
fn test_batch_then_use_in_expressions() {
    let mut ctx = EvalContext::new();

    // Batch assign
    eval_with_ctx(&mut ctx, "let [x, y, z] = [2, 3, 4]").unwrap();

    // Use in expressions
    assert_eq!(eval_with_ctx(&mut ctx, "x + y + z").unwrap(), 9.0);
    assert_eq!(eval_with_ctx(&mut ctx, "xyz").unwrap(), 24.0);
    assert_eq!(eval_with_ctx(&mut ctx, "x^2 + y^2 + z^2").unwrap(), 29.0); // 4+9+16
}

// ========== Error Handling Tests ==========

#[test]
fn test_error_batch_mismatched_brackets() {
    let mut ctx = EvalContext::new();

    // Missing closing bracket
    let result = eval_with_ctx(&mut ctx, "let [x, y = [1, 2]");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("closing bracket"));

    // Missing opening bracket
    let result = eval_with_ctx(&mut ctx, "let x, y] = [1, 2]");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("opening bracket"));
}

#[test]
fn test_error_batch_one_side_brackets() {
    let mut ctx = EvalContext::new();

    // Left side has brackets, right doesn't
    let result = eval_with_ctx(&mut ctx, "let [x, y] = 1, 2");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("brackets"));

    // Right side has brackets, left doesn't
    let result = eval_with_ctx(&mut ctx, "let x, y = [1, 2]");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("brackets"));
}

#[test]
fn test_error_batch_empty_lists() {
    let mut ctx = EvalContext::new();

    // Empty variable list - this will fall through to regular definition parsing
    let result = eval_with_ctx(&mut ctx, "let [] = [1, 2]");
    assert!(result.is_err());
    // Just verify it errors, not specific message

    // Empty value list - this should catch the empty values error
    let result = eval_with_ctx(&mut ctx, "let [x, y] = []");
    assert!(result.is_err());
    // The regex won't match [] as a valid value list, so it falls through
}

#[test]
fn test_error_function_missing_parentheses() {
    let mut ctx = EvalContext::new();

    // Missing opening parenthesis
    let result = eval_with_ctx(&mut ctx, "let fx) = x^2");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("(") || err_msg.contains("Missing"));

    // Missing closing parenthesis
    let result = eval_with_ctx(&mut ctx, "let f(x = x^2");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains(")") || err_msg.contains("Missing"));
}

#[test]
fn test_error_function_empty_parameter() {
    let mut ctx = EvalContext::new();

    // Empty parentheses
    let result = eval_with_ctx(&mut ctx, "let f() = 5");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("no parameter"));
}

#[test]
fn test_error_function_multiple_parameters() {
    let mut ctx = EvalContext::new();

    // Multiple parameters not supported
    let result = eval_with_ctx(&mut ctx, "let f(x, y) = x + y");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("multiple parameters"));
}

#[test]
fn test_error_function_conflicts_with_variable() {
    let mut ctx = EvalContext::new();

    // Define a variable first
    eval_with_ctx(&mut ctx, "let p_53493249328920 = 5").unwrap();

    // Try to define a function with the same name
    let result = eval_with_ctx(&mut ctx, "let p_53493249328920(x) = x^2");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("conflicts"));
}

#[test]
fn test_error_variable_conflicts_with_function() {
    let mut ctx = EvalContext::new();

    // Define a function first
    eval_with_ctx(&mut ctx, "let g(x) = x^2").unwrap();

    // Try to define a variable with the same name
    let result = eval_with_ctx(&mut ctx, "let g = 5");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("conflicts"));
}

#[test]
fn test_error_empty_variable_name() {
    let mut ctx = EvalContext::new();

    // This should fail in parsing before reaching variable validation
    let result = eval_with_ctx(&mut ctx, "let  = 5");
    assert!(result.is_err());
}

#[test]
fn test_error_empty_variable_value() {
    let mut ctx = EvalContext::new();

    // Variable with no value
    let result = eval_with_ctx(&mut ctx, "let x = ");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("no value") || err_msg.contains("Empty"));
}

#[test]
fn test_error_invalid_function_name_format() {
    let mut ctx = EvalContext::new();

    // Function name with multiple letters followed by digits not allowed
    let result = eval_with_ctx(&mut ctx, "let func1(x) = x^2");
    assert!(result.is_err());
}

#[test]
fn test_function_redefinition_allowed() {
    let mut ctx = EvalContext::new();

    // Define a function
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 9.0);

    // Redefine it (should be allowed now - just overwrites)
    eval_with_ctx(&mut ctx, "let f(x) = 2x").unwrap();
    // After redefinition, f(3) should be 2*3 = 6
    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 6.0);
}

#[test]
fn test_variable_reassignment_order() {
    let mut ctx = EvalContext::new();

    // Define a variable
    eval_with_ctx(&mut ctx, "let x = 5").unwrap();

    // Define another variable
    eval_with_ctx(&mut ctx, "let y = 10").unwrap();

    // Reassign x - it should be moved to the end
    eval_with_ctx(&mut ctx, "let x = 15").unwrap();

    // The last defined variable should be x, not y
    assert_eq!(ctx.defined_vars.last().unwrap().0, "x");
    assert_eq!(*ctx.defined_vars.last().unwrap().1, 15.0);
}

// ========== Remove/Delete Tests ==========

#[test]
fn test_remove_variable() {
    let mut ctx = EvalContext::new();

    // Define a variable
    eval_with_ctx(&mut ctx, "let x = 5").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "x").unwrap(), 5.0);

    // Remove it
    eval_with_ctx(&mut ctx, "remove x").unwrap();

    // Now using x should error
    assert!(eval_with_ctx(&mut ctx, "x").is_err());
}

#[test]
fn test_remove_function() {
    let mut ctx = EvalContext::new();

    // Define a function
    eval_with_ctx(&mut ctx, "let f(x) = x^2").unwrap();
    assert_eq!(eval_with_ctx(&mut ctx, "f(3)").unwrap(), 9.0);

    // Remove it
    eval_with_ctx(&mut ctx, "remove f").unwrap();

    // Now using f should error (treated as undefined variable)
    assert!(eval_with_ctx(&mut ctx, "f(3)").is_err());
}

#[test]
fn test_delete_alias() {
    let mut ctx = EvalContext::new();

    eval_with_ctx(&mut ctx, "let x = 10").unwrap();

    // delete and rm should work as aliases
    eval_with_ctx(&mut ctx, "delete x").unwrap();
    assert!(eval_with_ctx(&mut ctx, "x").is_err());

    eval_with_ctx(&mut ctx, "let y = 20").unwrap();
    eval_with_ctx(&mut ctx, "rm y").unwrap();
    assert!(eval_with_ctx(&mut ctx, "y").is_err());
}

#[test]
fn test_remove_nonexistent() {
    let mut ctx = EvalContext::new();

    // Try to remove something that doesn't exist
    let result = eval_with_ctx(&mut ctx, "remove nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}



#[test]
fn test_multi_char_constant_recognition() {
    let mut ctx = EvalContext::new();

    // Test 1: xpi should work as x * pi
    eval_with_ctx(&mut ctx, "let x = 2").unwrap();
    let result = eval_with_ctx(&mut ctx, "xpi").unwrap();
    let expected = 2.0 * PI;
    assert!((result - expected).abs() < 0.0001, "xpi should equal x * pi, got {} expected {}", result, expected);
}

#[test]
fn test_multi_letter_with_digits_separator() {
    let mut ctx = EvalContext::new();

    // Test eee_0 with e_0 defined - should not return 0
    eval_with_ctx(&mut ctx, "let e_0 = 5").unwrap();
    let result = eval_with_ctx(&mut ctx, "eee_0").unwrap();
    let expected = E * E * 5.0; // [e][e][e_0]
    assert!((result - expected).abs() < 0.01, "eee_0 should equal e * e * e_0, got {} expected {}", result, expected);
}

#[test]
fn test_constant_function() {
    let mut ctx = EvalContext::new();

    // The constant function 'f' should be defined as f(x) = x^2
    let result = eval_with_ctx(&mut ctx, "f(5)").unwrap();
    assert_eq!(result, 25.0, "f(5) should equal 25");

    let result = eval_with_ctx(&mut ctx, "f(3)").unwrap();
    assert_eq!(result, 9.0, "f(3) should equal 9");
}
