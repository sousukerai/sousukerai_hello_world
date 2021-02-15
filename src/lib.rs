//! A Simple Hello World Crate

/// This function returns the greeting; `Hello, world!`
pub fn hello() -> String {
    ("Hello, world!").to_string()
}

#[cfg(test)]
mod tests {

    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
/*
 * the following lines are auto gerated when you use `cargo new --lib` to create this crate
 * #[cfg(test)]
 * mod tests {
 *     #[test]
 *     fn it_works() {
 *         assert_eq!(2 + 2, 4);
 *     }
 * }
 */
