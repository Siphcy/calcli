use crate::definition_handler::function::Function;



pub const KNOWN_FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
    "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "sqrt", "exp", "ln", "log", "log10", "log2",
    "abs", "floor", "ceil", "round",
    "max", "min", "pow",
];


// CONSTANT VARIABLES
const PI: f64 = 3.1415926535897932385;
const EULER: f64 = 2.71828182845904523536;
const EP_0: f64 = 8.8541878176e-13;
const SPEED_OF_LIGHT: f64 = 299792458.0;

// CONSTANT FUNCTIONS
const SQUARE: Function = Function::define_new("f", "x", "x^2");
const DEGREE_TO_RADIAN: Function = Function::define_new("r", "x", "(xπ)/180");
const CUBE: Function = Function::define_new("g", "x", "x^3");

pub const CONSTANT_VAR: &[(&'static str, f64)] = &[
    ("π", PI),
    ("e", EULER),
    ("e_0", EP_0),
    ("c", SPEED_OF_LIGHT)
];

pub const CONSTANT_FUNC: &[(&'static str, Function)] = &[
    ("r", DEGREE_TO_RADIAN),
    ("f", SQUARE),
    ("g", CUBE),
];
