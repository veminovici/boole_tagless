pub trait ExprSym {
    type Repr;

    fn lit(n: i32) -> Self::Repr;
    fn neg(e: Self::Repr) -> Self::Repr;
    fn add(e1: Self::Repr, e2: Self::Repr) -> Self::Repr;
}

// Compared with the intial style,
// we have no intermediate representation,
// and the eval function is now a method on the ExprSym trait.

pub struct Eval;

impl ExprSym for Eval {
    type Repr = i32;

    fn lit(n: i32) -> Self::Repr {
        n
    }

    fn neg(e: Self::Repr) -> Self::Repr {
        -e
    }

    fn add(e1: Self::Repr, e2: Self::Repr) -> Self::Repr {
        e1 + e2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tagless_final<S: ExprSym>() -> S::Repr {
        S::add(S::lit(1), S::lit(2))
    }

    #[test]
    fn test_tagless_final_eval() {
        let e = tagless_final::<Eval>();
        assert_eq!(e, 3);
    }
}
