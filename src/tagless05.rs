use crate::tagless02::*;
use crate::tagless03::*;

pub trait MulExprSym: ExprSym {
    fn mul(e1: Self::Repr, e2: Self::Repr) -> Self::Repr;
}

impl MulExprSym for Eval {
    fn mul(e1: Self::Repr, e2: Self::Repr) -> Self::Repr {
        e1 * e2
    }
}

impl MulExprSym for View {
    fn mul(e1: Self::Repr, e2: Self::Repr) -> Self::Repr {
        format!("({} * {})", e1, e2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn tagless_final<S: MulExprSym>() -> S::Repr {
        S::mul(S::lit(1), S::lit(2))
    }

    #[test]
    fn test_tagless_final_eval() {
        let e = tagless_final::<Eval>();
        assert_eq!(e, 2);
    }

    #[test]
    fn test_tagless_final_view() {
        let s = tagless_final::<View>();
        assert_eq!(s, "(1 * 2)");
    }

}







