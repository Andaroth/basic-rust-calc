fn add(a: f64, b: f64) -> f64 { a + b }
fn substract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> f64 { a / b }
fn remainder(a: f64, b: f64) -> f64 { a % b }

type Operation = (
    &'static str, // name
    fn(f64, f64) -> f64, // math operation
    char // display symbol
);

pub static OPERATIONS: [Operation; 5] = [
    // Catalog operations (name, function, symbol)
    ("Addition", add, '+'),
    ("Soustraction", substract, '-'),
    ("Multiplication", multiply, '*'),
    ("Division", divide, '/'),
    ("Reste Modulo", remainder, '%'),
];