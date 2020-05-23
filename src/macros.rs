// -------------------------------------------------------------------------------------
//  M A C R O S

#[cfg_attr(feature = "cargo-clippy", allow(clippy::let_and_return))]
macro_rules! rules {
    ($($n:expr => $e:expr),*) => {{
        use $crate::parser::expression;
        use std::collections::HashMap;

        let rules = expression::SetOfRules::new(HashMap::<String, expression::Expression>::new());
        $(let rules = rules.add($n, $e);)*
        rules
    }};
}

macro_rules! lit {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::Literal(
            $e.to_string(),
        ))
    }};
}

macro_rules! error {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::Error(
            $e.to_string(),
        ))
    }};
}

macro_rules! dot {
    () => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::Dot)
    }};
}

macro_rules! eof {
    () => {{
        $crate::parser::expression::Expression::Simple($crate::parser::atom::Atom::EOF)
    }};
}

macro_rules! ematch {
    (chlist $chars:expr, $(from $from:expr,  to $to:expr),*) => {{
        //use idata::cont::IVec;  //  pending macros by example 2.0
        use $crate::parser;
        let mut v = Vec::<(char, char)>::new();

        //$(let v = v.ipush(($from, $to));)+  //  pending macros by example 2.0
        $(v.push(($from, $to));)+
        let amatch = parser::atom::Atom::Match(parser::atom::MatchRules::init($chars, v));
        parser::expression::Expression::Simple(amatch)
    }};

    (chlist $chars:expr, from2 $vfrom2:expr) => {{
        use $crate::parser;

        let amatch = parser::atom::Atom::Match(parser::atom::MatchRules::init($chars, $vfrom2));
        parser::expression::Expression::Simple(amatch)
    }};
}

macro_rules! and {
    ($($e:expr),*) => {{
        use $crate::parser::expression::{Expression, MultiExpr};

        Expression::And(MultiExpr::new(vec![$($e ,)*]))
    }};
}

macro_rules! or {
    ($($e:expr),*) => {{
        use $crate::parser::expression::{Expression, MultiExpr};

        Expression::Or(MultiExpr::new(vec![$($e ,)*]))
    }};
}

macro_rules! not {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Not(Box::new($e))
    }};
}

macro_rules! peek {
    ($e:expr) => {{
        $crate::parser::expression::Expression::Peek(Box::new($e))
    }};
}

macro_rules! rep {
    ($e:expr, $min:expr) => {{
        use $crate::parser::expression;

        expression::Expression::Repeat(expression::RepInfo::new(Box::new($e), $min, None))
    }};

    ($e:expr, $min:expr, $max:expr) => {{
        use $crate::parser::expression;

        expression::Expression::Repeat(expression::RepInfo::new(Box::new($e), $min, Some($max)))
    }};
}

macro_rules! ref_rule {
    ($e:expr) => {{
        $crate::parser::expression::Expression::RuleName($e.to_owned())
    }};
}

macro_rules! named {
    ($name:expr, $mexpr:expr) => {{
        use $crate::parser::expression::*;
        Expression::MetaExpr(MetaExpr::Named(NamedExpr {
            name: $name.to_string(),
            expr: Box::new($mexpr),
        }))
    }};
}

macro_rules! transf2 {
    ($expr:expr, $t2rules:expr) => {{
        use $crate::parser::expression::*;
        Expression::MetaExpr(MetaExpr::Transf2(Transf2Expr {
            mexpr: MultiExpr::new(vec![$expr]),
            transf2_rules: $t2rules,
        }))
    }};
}

macro_rules! t2rules {
    ($($rule:expr),* $(,)*) => {{
        use $crate::parser::expression::*;
        let v = vec![$($rule ,)*];
        ReplTemplate(v)
    }};
}

macro_rules! t2_funct {
    ($e:expr) => {{
        use $crate::parser::expression::*;
        ReplItem::Function($e.to_string())
    }};
}

macro_rules! t2_byname {
    ($e:expr) => {{
        use $crate::parser::expression::*;
        ReplItem::ByName($e.to_string())
    }};
}

macro_rules! t2_byname_opt {
    ($e:expr) => {{
        use $crate::parser::expression::*;
        ReplItem::ByNameOpt($e.to_string())
    }};
}

// macro_rules! t2_bypos {
//     ($e:expr) => {{
//         use $crate::parser::expression::*;
//         ReplItem::ByPos($e)
//     }};
// }

macro_rules! t2_text {
    ($e:expr) => {{
        use $crate::parser::expression::*;
        ReplItem::Text($e.to_string())
    }};
}

//  M A C R O S
// -------------------------------------------------------------------------------------
