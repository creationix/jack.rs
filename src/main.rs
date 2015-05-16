
#[derive(Debug,Copy,Clone)]
enum Raw {
  // Most numbers will be the integer type in this language.
  Integer(i64),

  // Instead of floats, this language will have a rational type.
  Rational(i64, i64),
}

type Value = Result<Raw, &'static str>;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c;
    while a != 0 {
        c = a;
        a = b % a;
        b = c;
    }
    b
}

fn new_integer(n: i64) -> Value {
    Ok(Raw::Integer(n))
}

fn new_rational(mut n: i64, mut d: i64) -> Value {
    let g = gcd(n, d);
    n = n / g;
    d = d / g;
    if d < 0 {
      n = -n;
      d = -d;
    }
    if d == 1 {
        Ok(Raw::Integer(n))
    }
    else {
        Ok(Raw::Rational(n, d))
    }
}

fn divide(left: Value, right: Value) -> Value {
    match (try!(left), try!(right)) {

        // Implement int + int
        (Raw::Integer(a), Raw::Integer(b)) => new_rational(a, b),

        // The rest need implementing
        _ => Err("TODO: Implement more divide types"),
    }
}

fn main() {
    let a = new_integer(44);
    let b = new_integer(14);
    let c = divide(a, b);

    println!("{:?} / {:?} = {:?}", a, b, c);
    println!("20/5 = {:?}", new_rational(20, 5));
    println!("5/20 = {:?}", new_rational(5, 20));
    println!("20/-5 = {:?}", new_rational(20, -5));
    println!("5/-20 = {:?}", new_rational(5, -20));
    println!("5/-60 = {:?}", new_rational(5, -60));
    println!("120/-30- = {:?}", new_rational(120, -30));
    println!("-120/30- = {:?}", new_rational(-120, 30));
}
