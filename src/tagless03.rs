use crate::tagless02::*;

pub struct View;

impl ExprSym for View {
    type Repr = String;

    fn lit(n: i32) -> Self::Repr {
        n.to_string()
    }

    fn neg(e: Self::Repr) -> Self::Repr {
        format!("-({})", e)
    }

    fn add(e1: Self::Repr, e2: Self::Repr) -> Self::Repr {
        format!("({} + {})", e1, e2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tagless_final<S: ExprSym>() -> S::Repr {
        S::add(S::lit(1), S::lit(2))
    }

    #[test]
    fn test_tagless_final_view() {
        let e = tagless_final::<View>();
        assert_eq!(e, "(1 + 2)");
    }
}
