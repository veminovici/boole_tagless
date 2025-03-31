use crate::tagless02::*;
use crate::tagless03::*;

pub trait HasExprSyn {
    type ES: ExprSym;
}

// i32 -> Eval
impl HasExprSyn for i32 {
    type ES = Eval;
}

// String -> View
impl HasExprSyn for String {
    type ES = View;
}

pub fn tagless_final<S: ExprSym<Repr = T>, T: HasExprSyn<ES = S>>() -> T {
    S::add(S::lit(1), S::lit(2))
}

pub fn eval() -> i32 {
    tagless_final()
}

pub fn view() -> String {
    tagless_final()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagless_final_eval() {
        let e = eval();
        assert_eq!(e, 3);
    }

    #[test]
    fn test_tagless_final_view() {
        let s = view();
        assert_eq!(s, "(1 + 2)");
    }
}
