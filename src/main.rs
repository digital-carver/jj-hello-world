/// A Hello, world! program

// The main function runs when our program starts
fn main() {
    print("Hello, world!");
    bye();
}

// save three chars, one of which needs a shift!
fn print(m: &str) {
    println!("{m}")
}

fn bye() {
    print("Goodbye, world!");
}

/// And, we're done! sike, BOO!!!!
