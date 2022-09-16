mod my {
    use std::fmt;
    use std::fmt::{Display, Formatter};

    pub struct Noisy;

    impl Drop for Noisy {
        fn drop(&mut self) {
            println!("Dropping Noisy!");
        }
    }

    impl Display for Noisy {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "Noisy Display")
        }
    }
}

fn module() {
    println!("module");
    let s = my::Noisy;
    println!("{}", s);
    drop(s);
    println!("Dropped Noisy");
}

fn ifelse() {
    println!("if else");
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

fn variables() {
    println!("variables");
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    mutable = i32::from(true);

    // Variables can be overwritten with shadowing.
    let mutable = true;
}

fn main() {
    println!("Hello, Rust!");
    variables();
    ifelse();
    module();
}
