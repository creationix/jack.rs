
#[derive(Debug)]
enum Value {
  // Numbers in this language can be simple integers or rational.
  // They are the same to the user and automatically switch to the best
  // type on every calculation.
  Integer(i64),
  Rational(i64, i64),
  String(String)
}
type Error = &'static str;

// Calculate the greatest common divisor using Euclid's algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c;
    while a != 0 {
        c = a;
        a = b % a;
        b = c;
    }
    b
}

#[inline(always)]
fn new_integer(n: i64) -> Value {
    Value::Integer(n)
}

// This normalizes inputs and sometimes even converts to integers.
fn new_rational(mut n: i64, mut d: i64) -> Value {
    let g = gcd(n, d);
    n = n / g;
    d = d / g;
    if d < 0 {
      n = -n;
      d = -d;
    }
    if d == 1 {
        Value::Integer(n)
    }
    else {
        Value::Rational(n, d)
    }
}

// We could just let new_rational handle the gcd here, but this
// avoids overflows by dividing the gcd in first.
fn rational_add(n1: i64, d1: i64, n2: i64, d2: i64) -> Value {
  if d1 == d2 {
    // Fast path for common divisors.
    new_rational(n1 + n2, d1)
  }
  else {
    let g = gcd(d1, d2);
    new_rational(n1 * (d2 / g) + n2 * (d1 / g), d1 / g * d2)
  }
}

fn add(left: &Value, right: &Value) -> Result<Value, Error> {
    match (left, right) {
        (&Value::Integer(a), &Value::Integer(b)) => Ok(new_integer(a + b)),
        (&Value::Rational(n, d), &Value::Integer(b)) => Ok(rational_add(n, d, b, 1)),
        (&Value::Integer(a), &Value::Rational(n, d)) => Ok(rational_add(a, 1, n, d)),
        (&Value::Rational(n1, d1), &Value::Rational(n2, d2)) => Ok(rational_add(n1, d1, n2, d2)),
        _ => Err("Add requires two numbers"),
    }
}

fn subtract(left: &Value, right: &Value) -> Result<Value, Error> {
    match (left, right) {
        (&Value::Integer(a), &Value::Integer(b)) => Ok(new_integer(a - b)),
        (&Value::Rational(n, d), &Value::Integer(b)) => Ok(rational_add(n, d, -b, 1)),
        (&Value::Integer(a), &Value::Rational(n, d)) => Ok(rational_add(a, 1, -n, d)),
        (&Value::Rational(n1, d1), &Value::Rational(n2, d2)) => Ok(rational_add(n1, d1, -n2, d2)),
        _ => Err("Subtract requires two numbers"),
    }
}

fn divide(left: &Value, right: &Value) -> Result<Value, Error> {
    match (left, right) {
        (&Value::Integer(a), &Value::Integer(b)) => Ok(new_rational(a, b)),
        (&Value::Rational(n, d), &Value::Integer(b)) => Ok(new_rational(n, d * b)),
        (&Value::Integer(a), &Value::Rational(n, d)) => Ok(new_rational(a * d, n)),
        (&Value::Rational(n1, d1), &Value::Rational(n2, d2)) => Ok(new_rational(n1 * d2, d1 * n2)),
        _ => Err("Divide requires two numbers"),
    }
}

fn multiply(left: &Value, right: &Value) -> Result<Value, Error> {
    match (left, right) {
        (&Value::Integer(a), &Value::Integer(b)) => Ok(new_integer(a * b)),
        (&Value::Rational(n, d), &Value::Integer(b)) => Ok(new_rational(n * b, d)),
        (&Value::Integer(a), &Value::Rational(n, d)) => Ok(new_rational(a * n, d)),
        (&Value::Rational(n1, d1), &Value::Rational(n2, d2)) => Ok(new_rational(n1 * n2, d1 * d2)),
        _ => Err("Multiply requires two numbers"),
    }
}

fn test(a: Value, b: Value) {
    println!("{:?} / {:?} = {:?}", a, b, divide(&a, &b));
    println!("{:?} * {:?} = {:?}", a, b, multiply(&a, &b));
    println!("{:?} + {:?} = {:?}", a, b, add(&a, &b));
    println!("{:?} - {:?} = {:?}", a, b, subtract(&a, &b));
}

fn main() {
    test(new_integer(44), new_integer(14));
    test(new_integer(14), new_integer(44));
    test(new_integer(44), new_integer(-14));
    test(new_integer(14), new_integer(-44));
    test(new_integer(-44), new_integer(14));
    test(new_integer(-14), new_integer(44));
    test(new_integer(-44), new_integer(-14));
    test(new_integer(-14), new_integer(-44));
    test(new_rational(1, 2), new_rational(2, 1));
    test(new_rational(2, 1), new_rational(1, 2));
    test(new_rational(1, 2), new_rational(1, 2));
    test(new_rational(2, 2), new_rational(2, 1));
    test(Value::String(str::to_string("Input string")),
         new_rational(2, 1));
}
