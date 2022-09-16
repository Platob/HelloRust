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

pub fn run() {
    println!("module");
    let s = Noisy;
    println!("{}", s);
    drop(s);
    println!("Dropped Noisy");
}

fn private() {
    println!("module");
    let s = Noisy;
    println!("{}", s);
    drop(s);
    println!("Dropped Noisy");
}