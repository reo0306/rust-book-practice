pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

pub fn divide(left: usize, right: usize) -> Option<usize> {
    if right == 0 {
        return None;
    }

    let r = left / right;

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_subtract() {
        let result = subtract(5, 2);
        assert_eq!(result, 3);
    }
    #[test]
    fn it_multiply() {
        let result = multiply(4, 6);
        assert_eq!(result, 24);
    }
    #[test]
    fn it_divide() {
        let result = divide(10, 2);
        assert_eq!(result, Some(5));

        let result = divide(10, 0);
        assert_eq!(result, None);
    }
}
