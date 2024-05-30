// Rewrite the factorial function using a `while` loop.
pub fn factorial(mut n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    let mut c = n;
    while c > 1 {
        c = c - 1;
        n = n * c;
    }
    n
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
