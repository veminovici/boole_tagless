type Repr = i32;

pub fn lit(n: i32) -> Repr {
    n
}

pub fn neg(n: Repr) -> Repr {
    -n
}

pub fn add(n1: Repr, n2: Repr) -> Repr {
    n1 + n2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_simple_eval() {
        let e = add(lit(1), lit(2));
        assert_eq!(e, 3);
    }
}
