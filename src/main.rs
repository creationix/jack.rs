use std::rc::Rc;

type Error = &'static str;

#[derive(Debug,Clone)]
enum Code {
    Integer(i64),
    Rational(i64, i64),
    String(String),
    Add(Rc<Code>, Rc<Code>),
    Sub(Rc<Code>, Rc<Code>),
    Div(Rc<Code>, Rc<Code>),
    Mul(Rc<Code>, Rc<Code>),
    Neg(Rc<Code>),
}

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

// This normalizes inputs and sometimes even converts to integers.
fn new_rational(mut n: i64, mut d: i64) -> Code {
    let g = gcd(n, d);
    n = n / g;
    d = d / g;
    if d < 0 {
      n = -n;
      d = -d;
    }
    if d == 1 {
        Code::Integer(n)
    }
    else {
        Code::Rational(n, d)
    }
}

fn rational_add(n1: i64, d1: i64, n2: i64, d2: i64) -> Code {
  if d1 == d2 {
    // Fast path for common divisors.
    new_rational(n1 + n2, d1)
  }
  else {
    let g = gcd(d1, d2);
    new_rational(n1 * (d2 / g) + n2 * (d1 / g), d1 / g * d2)
  }

}

fn add(left: &Code, right: &Code) -> Result<Code, Error> {
    match (try!(eval(left)), try!(eval(right))) {
        (Code::Integer(a), Code::Integer(b)) => Ok(Code::Integer(a + b)),
        (Code::Rational(n, d), Code::Integer(b)) => Ok(rational_add(n, d, b, 1)),
        (Code::Integer(a), Code::Rational(n, d)) => Ok(rational_add(a, 1, n, d)),
        (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(rational_add(n1, d1, n2, d2)),
        _ => Err("Add requires two numbers"),
    }
}

fn subtract(left: &Code, right: &Code) -> Result<Code, Error> {
    match (try!(eval(left)), try!(eval(right))) {
        (Code::Integer(a), Code::Integer(b)) => Ok(Code::Integer(a - b)),
        (Code::Rational(n, d), Code::Integer(b)) => Ok(rational_add(n, d, -b, 1)),
        (Code::Integer(a), Code::Rational(n, d)) => Ok(rational_add(a, 1, -n, d)),
        (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(rational_add(n1, d1, -n2, d2)),
        _ => Err("Subtract requires two numbers"),
    }
}

fn multiply(left: &Code, right: &Code) -> Result<Code, Error> {
    match (try!(eval(left)), try!(eval(right))) {
        (Code::Integer(a), Code::Integer(b)) => Ok(Code::Integer(a * b)),
        (Code::Rational(n, d), Code::Integer(b)) => Ok(new_rational(n * b, d)),
        (Code::Integer(a), Code::Rational(n, d)) => Ok(new_rational(a * n, d)),
        (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(new_rational(n1 * n2, d1 * d2)),
        _ => Err("Multiply requires two numbers"),
    }
}

fn divide(left: &Code, right: &Code) -> Result<Code, Error> {
    match (try!(eval(left)), try!(eval(right))) {
        (Code::Integer(a), Code::Integer(b)) => Ok(Code::Rational(a, b)),
        (Code::Rational(n, d), Code::Integer(b)) => Ok(new_rational(n, d * b)),
        (Code::Integer(a), Code::Rational(n, d)) => Ok(new_rational(a * d, n)),
        (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(new_rational(n1 * d2, d1 * n2)),
        _ => Err("Divide requires two numbers"),
    }
}

fn negate(num: &Code) -> Result<Code, Error> {
    match num {
        &Code::Integer(a) => Ok(Code::Integer(-a)),
        &Code::Rational(n, d) => Ok(Code::Rational(-n, d)),
        _ => Err("Negate requires a number"),
    }
}


fn eval(expr: &Code) -> Result<Code, Error> {
    match expr {
        &Code::Add(ref left, ref right) => add(left, right),
        &Code::Sub(ref left, ref right) => subtract(left, right),
        &Code::Mul(ref left, ref right) => multiply(left, right),
        &Code::Div(ref left, ref right) => divide(left, right),
        &Code::Neg(ref num) => negate(num),
        n @ &Code::Integer(_) => Ok(n.clone()),
        n @ &Code::Rational(_, _) => Ok(n.clone()),
        n @ &Code::String(_) => Ok(n.clone()),
    }
}

fn test(left: Code, right: Code) {
    let a = Rc::new(left);
    let b = Rc::new(right);
    let mut e = Rc::new(Code::Add(a.clone(), b.clone()));
    println!("{:?} = {:?}", e, eval(&e));
    e = Rc::new(Code::Sub(a.clone(), b.clone()));
    println!("{:?} = {:?}", e, eval(&e));
    e = Rc::new(Code::Mul(a.clone(), b.clone()));
    println!("{:?} = {:?}", e, eval(&e));
    e = Rc::new(Code::Div(a.clone(), b.clone()));
    println!("{:?} = {:?}", e, eval(&e));
    e = Rc::new(Code::Neg(a.clone()));
    println!("{:?} = {:?}", e, eval(&e));
}

fn main() {
    test(Code::Integer(44), Code::Integer(14));
    test(Code::Integer(14), Code::Integer(44));
    test(Code::Integer(44), Code::Integer(-14));
    test(Code::Integer(14), Code::Integer(-44));
    test(Code::Integer(-44), Code::Integer(14));
    test(Code::Integer(-14), Code::Integer(44));
    test(Code::Integer(-44), Code::Integer(-14));
    test(Code::Integer(-14), Code::Integer(-44));
    test(new_rational(1, 2), new_rational(2, 1));
    test(new_rational(2, 1), new_rational(1, 2));
    test(new_rational(1, 2), new_rational(1, 2));
    test(new_rational(2, 2), new_rational(2, 1));
    test(Code::String(str::to_string("Input string")),
         new_rational(2, 1));
    let e = Code::Div(
        Rc::new(Code::Add(
            Rc::new(Code::Rational(1, 2)),
            Rc::new(Code::Rational(1, 3)),
        )),
        Rc::new(Code::Sub(
            Rc::new(Code::Integer(2)),
            Rc::new(Code::Integer(5)),
        )),
    );
    println!("{:?} = {:?}", e, eval(&e));

}
