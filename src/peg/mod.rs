#![warn(missing_docs)]
//! Module with functions to generate rules from PEG grammar
//!

use crate::ir::IR;
use std::{self, result};

#[cfg(test)]
mod test;

#[derive(Debug)]
/// Most of peg functions will return a result with this type
/// on Error side
pub enum Error {
    /// When error has been on `peg` side
    /// we will receive a description and
    /// optionally, a link to a stacked error
    /// Then, we can have a errors stack of ilimited size
    Peg((String, Option<Box<Error>>)),
    /// When error is on parser side
    Parser(crate::parser::Error),
    /// When error is on ast side
    Ast(crate::ast::Error),
}

impl From<crate::parser::Error> for Error {
    fn from(e: crate::parser::Error) -> Self {
        Error::Parser(e)
    }
}

impl From<crate::ast::Error> for Error {
    fn from(e: crate::ast::Error) -> Self {
        Error::Ast(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Peg((s, None)) => write!(f, "{}", s),
            Error::Peg((s, Some(b))) => write!(f, "{} > {}", s, b),
            Error::Parser(p) => write!(f, "Parser({:?})", p),
            Error::Ast(a) => write!(f, "AST({:?})", a),
        }
    }
}

/// Most of functions on peg module, will return a set of rules
/// or an error
pub type Result = result::Result<crate::parser::expression::SetOfRules, Error>;

// -------------------------------------------------------------------------------------
//  A P I

/// Given a ```peg``` set of rules on an string, it will generate
/// the set of rules to use in the parser
///
/// Next, is a full example showing the error messages, if so
/// ```
/// extern crate dpr;
/// use dpr::peg::rules_from_peg;
///
///     let rules = rules_from_peg(
///         r#"
///              main    =   'hello'   ' '   'world'  dot
///              dot     =   "\0x2E"
///          "#,
///     )
///     .map_err(|e| {
///         println!("{}", e);
///         panic!("FAIL");
///     })
///     .unwrap();
///     println!("{:#?}", rules);
///     let result = rules.parse("hello world.");
///     assert!(result.is_ok());
///     match result {
///         Ok(ast) => println!("{:#?}", ast),
///         Err(e) => println!("Error: {:?}", e),
///     };
/// ```
///
/// Next is an example with some ```and``` ```literals```
/// and comments on peg grammar
/// ```
/// extern crate dpr;
/// use dpr::peg::rules_from_peg;
///
///     let ast = rules_from_peg(
///         r#"
///          //  classic hello world
///          main    =   'hello'   ' '   'world'
///
///          /*  with a multiline comment
///          */
///         "#,
///     )
///     .unwrap()
///     .parse("hello world");
///
///     assert!(ast.is_ok());
/// ```
///
/// Next is an example with some  error info
///
/// ```
/// extern crate dpr;
/// use dpr::peg::rules_from_peg;
///
///     let rules = rules_from_peg(
///         r#"
///              main    =   '('  main  ( ')'  /  error("unbalanced parenthesys") )
///                      /   'hello'
///             "#,
///     )
///     .unwrap();
///
///     assert!(rules.parse("hello").is_ok());
///     println!("{:?}", rules.parse("(hello)"));
///     assert!(rules.parse("(hello)").is_ok());
///     assert!(rules.parse("((hello))").is_ok());
///     assert!(rules.parse("(((hello)))").is_ok());
///     match rules.parse("(hello") {
///         Err(dpr::Error::PaserErr(e)) => {
///             assert!(e.descr == "unbalanced parenthesys");
///         }
///         _ => panic!("testing"),
///     }
///     match rules.parse("((hello)") {
///         Err(dpr::Error::PaserErr(e)) => {
///             assert!(e.descr == "unbalanced parenthesys");
///         }
///         _ => panic!("testing"),
///     }
///
/// ```

pub fn rules_from_peg(peg: &str) -> Result {
    let irtxt = crate::gcode::rules::rules2parse_peg()
        .parse(peg)
        // .parse(txt)
        .unwrap()
        .replace()
        .unwrap();
    let ir = IR::new(&irtxt.str());

    Ok(ir.get_rules().unwrap())
}

//  A P I
// -------------------------------------------------------------------------------------
