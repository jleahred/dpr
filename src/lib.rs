#![warn(missing_docs)]
// #![feature(external_doc)]
// #![doc(include = "../README.md")]

//! For an introduction and context view, read...
//!
//! [README.md](https://github.com/jleahred/dpr)
//!
//! A very basic example...
//! ```rust
//!```
//!
//!
//!
//! Please, read [README.md](https://github.com/jleahred/dpr) for
//! more context information
//!

extern crate idata;
extern crate im;

use std::result;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod ast;
pub(crate) mod gcode;
pub(crate) mod ir;
pub(crate) mod parser;
pub(crate) mod rules_for_peg;

// -------------------------------------------------------------------------------------
//  T Y P E S

//  T Y P E S
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
//  A P I

/// Peg type for fluent API
pub struct Peg<'a>(&'a str);

/// Errors for fluent API
#[derive(Debug)]
pub enum Error {
    /// error on parsing
    PaserErr(crate::parser::Error),
    /// error on replace
    ReplaceErr(String),
    /// error processing IR
    IRErr(crate::ir::Error),
}

impl<'a> Peg<'a> {
    /// create an instance of Peg
    pub fn new(txt: &'a str) -> Self {
        Self(txt)
    }

    /// generate rules from peg grammar (fluent API)
    pub fn gen_rules(&self) -> result::Result<crate::parser::expression::SetOfRules, Error> {
        use crate::ir::IR;

        let irtxt = crate::rules_for_peg::rules().parse(self.0)?.replace()?;
        let ir = IR::new(&irtxt.str());

        Ok(ir.get_rules().unwrap())
    }
}

impl crate::parser::expression::SetOfRules {
    /// parse from a set of rules (fluent API)
    pub fn parse(&self, text: &str) -> Result<ast::Node, Error> {
        crate::parse(text, self).map_err(|e| Error::PaserErr(e))
    }

    /// parse with debug info
    pub fn parse_debug(&self, text: &str) -> Result<ast::Node, Error> {
        crate::parse_debug(text, self).map_err(|e| Error::PaserErr(e))
    }
}

/// A parser for the parser.
///
/// It will take the peg grammar to parse peg grammars
/// and will generate the rust code as a set of rules
pub fn print_rules2parse_peg2() {
    use crate::ir::IR;

    let irtxt = crate::rules_for_peg::rules()
        .parse(gcode::peg2code::text_peg2code())
        .unwrap()
        .replace()
        .unwrap();
    let ir = IR::new(&irtxt.str());

    let rules = ir.get_rules().unwrap();

    let r = crate::gcode::rust_from_rules(&rules);

    let r = r;
    println!("{}", r);
}

impl ast::Node {
    /// run the tree replacing acording the rules
    pub fn replace(&self) -> Result<crate::ast::replace::Replaced, Error> {
        ast::replace::replace(&self).map_err(|e| Error::ReplaceErr(e))
    }
}

//  A P I
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------

fn parse(s: &str, rules: &parser::expression::SetOfRules) -> Result<ast::Node, parser::Error> {
    parse_with_debug(s, rules, false)
}

fn parse_debug(
    s: &str,
    rules: &parser::expression::SetOfRules,
) -> Result<ast::Node, parser::Error> {
    parse_with_debug(s, rules, true)
}

fn parse_with_debug(
    s: &str,
    rules: &parser::expression::SetOfRules,
    debug: bool,
) -> Result<ast::Node, parser::Error> {
    let (st, ast) = if debug {
        parser::expression::parse(parser::Status::init_debug(s, &rules, debug))?
    } else {
        parser::expression::parse(parser::Status::init(s, &rules))?
    };
    match (st.pos.n == s.len(), st.potential_error.clone()) {
        (true, _) => Ok(ast),
        (false, Some(e)) => Err(e),
        (false, None) => Err(parser::Error::from_status_normal(
            &st,
            "not consumed full input",
        )),
    }
}

// -------------------------------------------------------------------------------------
