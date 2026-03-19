use std::collections::HashMap;
use std::sync::LazyLock;


//TODO: More units and support magnitude shifting and standard units

pub static UNITS_MAGNITUDES: LazyLock<HashMap<&str, f64>> = LazyLock::new(|| {
    HashMap::from([
        ("k", 1000.0),
        ("d", 0.1),
        ("c", 0.01),
        ("m", 0.001),
    ])
});

pub static UNITS_KNOWN: [&str; 3] = [
    "m",
    "N",
    "t",
];


