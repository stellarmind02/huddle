fn main() {
    let greeting = greet();
    println!("{greeting}");
}

fn greet() -> &'static str {
    println!("Changed!");
    return "Hello, Test, Action";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!("Hello, Test, Action", greet());
    }
}
