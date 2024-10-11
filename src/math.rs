fn add(a: f64, b: f64) -> f64 { a + b }
fn substract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> f64 { a / b }
fn remainder(a: f64, b: f64) -> f64 { a % b }

pub static OPERATIONS: [(&'static str, fn(f64, f64) -> f64, char); 5] = [
    // Catalog operations (name, function, symbol)
    ("Addition", add, '+'),
    ("Soustraction", substract, '-'),
    ("Multiplication", multiply, '*'),
    ("Division", divide, '/'),
    ("Reste Modulo", remainder, '%'),
];