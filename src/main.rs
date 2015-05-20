
type Error = &'static str;


#[derive(Debug,Clone)]
enum Slot {
    Integer(i64),
    Rational(i64, i64),
    String(String),
}

#[derive(Debug,Clone)]
enum Op {
  Integer(usize, i64),
  Rational(usize, i64, i64),
  Add(usize, usize, usize),
  Sub(usize, usize, usize),
  Mul(usize, usize, usize),
  Div(usize, usize, usize),
}


// fn binopfmt(f: &mut fmt::Formatter, left: &Code, right: &Code, op: &'static str) -> fmt::Result {
//     try!(match left {
//         &Code::Op(_) => write!(f, "({})", left),
//         _ => write!(f, "{}", left)
//     });
//     try!(write!(f, " {} ", op));
//     match right {
//         &Code::Op(_) => write!(f, "({})", right),
//         _ => write!(f, "{}", right)
//     }
// }

// impl fmt::Display for Code {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             &Code::Integer(n) => write!(f, "{}", n),
//             &Code::Rational(n, d) => write!(f, "{}/{}", n, d),
//             &Code::String(ref s) => write!(f, "'{}'", s),
//             &Code::Op(ref op) => match op {
//                 &Op::Add(ref left, ref right) => binopfmt(f, left, right, "+"),
//                 &Op::Sub(ref left, ref right) => binopfmt(f, left, right, "-"),
//                 &Op::Div(ref left, ref right) => binopfmt(f, left, right, "/"),
//                 &Op::Mul(ref left, ref right) => binopfmt(f, left, right, "*"),
//                 &Op::Neg(ref num) => write!(f, "-({})", num),
//             }
//         }
//     }
// }



// fn subtract(left: &Code, right: &Code) -> Result<Code, Error> {
//     match (try!(eval(left)), try!(eval(right))) {
//         (Code::Integer(a), Code::Integer(b)) => Ok(Code::Integer(a - b)),
//         (Code::Rational(n, d), Code::Integer(b)) => Ok(rational_add(n, d, -b, 1)),
//         (Code::Integer(a), Code::Rational(n, d)) => Ok(rational_add(a, 1, -n, d)),
//         (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(rational_add(n1, d1, -n2, d2)),
//         _ => Err("Subtract requires two numbers"),
//     }
// }

// fn multiply(left: &Code, right: &Code) -> Result<Code, Error> {
//     match (try!(eval(left)), try!(eval(right))) {
//         (Code::Integer(a), Code::Integer(b)) => Ok(Code::Integer(a * b)),
//         (Code::Rational(n, d), Code::Integer(b)) => Ok(new_rational(n * b, d)),
//         (Code::Integer(a), Code::Rational(n, d)) => Ok(new_rational(a * n, d)),
//         (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(new_rational(n1 * n2, d1 * d2)),
//         _ => Err("Multiply requires two numbers"),
//     }
// }

// fn divide(left: &Code, right: &Code) -> Result<Code, Error> {
//     match (try!(eval(left)), try!(eval(right))) {
//         (Code::Integer(a), Code::Integer(b)) => Ok(new_rational(a, b)),
//         (Code::Rational(n, d), Code::Integer(b)) => Ok(new_rational(n, d * b)),
//         (Code::Integer(a), Code::Rational(n, d)) => Ok(new_rational(a * d, n)),
//         (Code::Rational(n1, d1), Code::Rational(n2, d2)) => Ok(new_rational(n1 * d2, d1 * n2)),
//         _ => Err("Divide requires two numbers"),
//     }
// }

// fn negate(num: &Code) -> Result<Code, Error> {
//     match num {
//         &Code::Integer(a) => Ok(Code::Integer(-a)),
//         &Code::Rational(n, d) => Ok(Code::Rational(-n, d)),
//         _ => Err("Negate requires a number"),
//     }
// }

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
fn new_rational(mut n: i64, mut d: i64) -> Slot {
    let g = gcd(n, d);
    n = n / g;
    d = d / g;
    if d < 0 {
      n = -n;
      d = -d;
    }
    if d == 1 {
        Slot::Integer(n)
    }
    else {
        Slot::Rational(n, d)
    }
}

fn rational_add(n1: i64, d1: i64, n2: i64, d2: i64) -> Slot {
  if d1 == d2 {
    // Fast path for common divisors.
    new_rational(n1 + n2, d1)
  }
  else {
    let g = gcd(d1, d2);
    new_rational(n1 * (d2 / g) + n2 * (d1 / g), d1 / g * d2)
  }

}

macro_rules! check {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => {
            return Err("Stack out of bounds")
        }
    })
}

fn set(stack: &mut Vec<Slot>, index: usize, value: Slot) -> Result<(), Error> {
    if stack.len() == index {
        stack.push(value);
    }
    else {
        let slot = check!(stack.get_mut(index));
        *slot = value;
    }
    Ok(())
}

fn add(stack: &mut Vec<Slot>, index: usize, left: usize, right: usize) -> Result<(), Error> {
    let value = match (check!(stack.get(left)), check!(stack.get(right))) {
        (&Slot::Integer(a), &Slot::Integer(b)) => Slot::Integer(a + b),
        (&Slot::Rational(n, d), &Slot::Integer(b)) => rational_add(n, d, b, 1),
        (&Slot::Integer(a), &Slot::Rational(n, d)) => rational_add(a, 1, n, d),
        (&Slot::Rational(n1, d1), &Slot::Rational(n2, d2)) => rational_add(n1, d1, n2, d2),
        _ => return Err("Add requires two numbers")
    };
    set(stack, index, value)
}

fn subtract(stack: &mut Vec<Slot>, index: usize, left: usize, right: usize) -> Result<(), Error> {
    let value = match (check!(stack.get(left)), check!(stack.get(right))) {
        (&Slot::Integer(a), &Slot::Integer(b)) => Slot::Integer(a - b),
        (&Slot::Rational(n, d), &Slot::Integer(b)) => rational_add(n, d, -b, 1),
        (&Slot::Integer(a), &Slot::Rational(n, d)) => rational_add(a, 1, -n, d),
        (&Slot::Rational(n1, d1), &Slot::Rational(n2, d2)) => rational_add(n1, d1, -n2, d2),
        _ => return Err("Subtract requires two numbers")
    };
    set(stack, index, value)
}

fn multiply(stack: &mut Vec<Slot>, index: usize, left: usize, right: usize) -> Result<(), Error> {
    let value = match (check!(stack.get(left)), check!(stack.get(right))) {
        (&Slot::Integer(a), &Slot::Integer(b)) => Slot::Integer(a * b),
        (&Slot::Rational(n, d), &Slot::Integer(b)) => new_rational(n * b, d),
        (&Slot::Integer(a), &Slot::Rational(n, d)) => new_rational(a * n, d),
        (&Slot::Rational(n1, d1), &Slot::Rational(n2, d2)) => new_rational(n1 * n2, d1 * d2),
        _ => return Err("Multiply requires two numbers")
    };
    set(stack, index, value)
}

fn divide(stack: &mut Vec<Slot>, index: usize, left: usize, right: usize) -> Result<(), Error> {
    let value = match (check!(stack.get(left)), check!(stack.get(right))) {
        (&Slot::Integer(a), &Slot::Integer(b)) => new_rational(a, b),
        (&Slot::Rational(n, d), &Slot::Integer(b)) => new_rational(n, b * d),
        (&Slot::Integer(a), &Slot::Rational(n, d)) => new_rational(a * d, n),
        (&Slot::Rational(n1, d1), &Slot::Rational(n2, d2)) => new_rational(n1 * d2, d1 * n2),
        _ => return Err("Multiply requires two numbers")
    };
    set(stack, index, value)
}

fn eval(stack: &mut Vec<Slot>, code: &Vec<Op>) -> Result<(), Error> {
    for op in code {
        match *op {
            Op::Integer(index, num) => try!(set(stack, index, Slot::Integer(num))),
            Op::Rational(index, num, dem) => try!(set(stack, index, Slot::Rational(num, dem))),
            Op::Add(index, left, right) => try!(add(stack, index, left, right)),
            Op::Sub(index, left, right) => try!(subtract(stack, index, left, right)),
            Op::Mul(index, left, right) => try!(multiply(stack, index, left, right)),
            Op::Div(index, left, right) => try!(divide(stack, index, left, right)),
        }
    }
    Ok(())
}

fn main() {
    let mut stack = Vec::new();

    let code = vec![
      Op::Integer(0, 44),
      Op::Rational(1, 1, 3),
      Op::Add(2, 0, 1),
      Op::Sub(3, 0, 1),
      Op::Mul(4, 1, 2),
      Op::Div(5, 1, 2),
    ];

    println!("code = {:?}", code);

    println!("{:?}", eval(&mut stack, &code));

    println!("stack = {:?}", stack);

}
