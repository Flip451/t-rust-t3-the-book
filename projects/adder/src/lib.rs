pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::{add_two, internal_adder};

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic] 
    fn another() {
        panic!()
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

}
