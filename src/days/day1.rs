pub fn day1() {
    println!("hello day1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        day1();
        assert_eq!(2, 2);
    }
}
