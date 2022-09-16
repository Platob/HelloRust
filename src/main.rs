mod test;
mod variables;
mod r#if;
mod my;

fn main() {
    println!("Hello, Rust!");
    variables::run();
    r#if::run();
    my::run();
}
