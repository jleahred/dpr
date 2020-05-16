// #![warn(missing_docs)]
// //! Process de IR and generate the parser rules

// // mod rules;

// use crate::parser::atom::Atom;
// use crate::parser::expression;
// use crate::parser::expression::{Expression, MultiExpr, SetOfRules};
// use crate::peg::ir::{Command, Error, IR};
// use idata::{self, cont::IVec};
// use std::{self, result};

// // fn get_expr(ir: IR) -> Result<(IR, Expression), Error> {
// //     // ATOM RUL grammar

// //     let next_cmd = ir.peek();
// //     match next_cmd {
// //         Some(Command {
// //             cmd: "ATOM",
// //             params: params,
// //         }) => get_atom(params),
// //         None => Err(Error(
// //             "expected expression on end of IR programm".to_string(),
// //         )),
// //     }
// //     // let next_command = lines.peek().and_then(|s| s.split(" ").nth(0));

// //     // match next_command {
// //     //     Some("ATOM") => get_atom(lines),
// //     //     Some("BEGIN_MULTEXPR") => get_multi_expr(lines),
// //     //     other => Err(Error(format!(
// //     //         "unknown command processing expr {:?}",
// //     //         other
// //     //     ))),
// //     // }
// // }

// // fn get_atom<'a>(ir: IR) -> Result<(IR, Expression), Error> {
// //     let (ir, command) = ir.next();

// //     match command {
// //         None => Err(Error("empty IR processing atom".to_string())),
// //         Some(c) => {
// //             match c.cmd {
// //                 "ATOM" => Ok((ir, process_atom_params(&c.params)?)),
// //                 other => Err(Error(format!("invalid command processing ATOM")))
// //             }
// //         }

// //         ,
// //     }
// // }

// fn process_atom_params(params: &[String]) -> Result<Expression, Error> {
//     match params
//         .iter()
//         .map(|p| p.as_ref())
//         .collect::<Vec<_>>()
//         .as_slice()
//     {
//         &["LIT", lit] => Ok(lit!(lit)),
//         other => Err(Error(format!("unknow atom params: {:?}", other))),
//     }
// }

// // #[test]
// // fn test_get_atom() {
// //     let ir = IR::new("ATOM LIT literal");
// //     let (_, e) = get_atom(ir).unwrap();
// //     assert_eq!(e, Expression::Simple(Atom::Literal("literal".to_string())))
// // }

// fn get_expr(ir: IR) -> Result<(IR, Expression), Error> {
//     match ir.peek() {
//         None => Err(Error("looking for expression finished program".to_string())),
//         Some(c) => match c.cmd.as_ref() {
//             "ATOM" => Ok((ir, process_atom_params(&c.params)?)),
//             other => Err(Error(format!(
//                 "unknown command processing expression {:?}",
//                 other
//             ))),
//         },
//     }
// }

// #[test]
// fn test_get_expr() {
//     let ir = IR::new("ATOM LIT literal");

//     let (_, e) = get_expr(ir).unwrap();
//     assert_eq!(e, Expression::Simple(Atom::Literal("literal".to_string())))
// }

// // fn get_multi_expr(lines: Lines) -> Result<(Lines, Expression), Error> {
// //     // BEGIN_MULTEXPR
// //     // ATOM RUL grammar
// //     // ATOM RUL grammar2
// //     // CLOSE_MULTEXPR
// //     fn get_rec_mexpr(lines: Lines) -> Result<(Lines, MultiExpr), Error> {
// //         // if next_is_multiexpr(&lines) {
// //         //     let (lines, params) = process_command(lines, "BEGIN".to_string())?;
// //         //     match params.as_slice() {
// //         //         ["OR"] => get_rec_mexpr(lines, or!()),
// //         //         ["AND"] => get_rec_mexpr(lines, and!()),
// //         //         [p] => Err(Error(format!("Expected AND/OR multiexp, rec: {}", p))),
// //         //     }
// //         // // let (lines, me) = get_rec_mexpr(lines)?;
// //         // // let (lines, e) = close_and_or(lines, me)?;
// //         // // Ok((lines, MultiExpr::new(vec![e])))
// //         // // Ok((lines, me))
// //         // } else {
// //         //     // let (lines, me) =
// //         //     Ok(get_rec_expr(lines, MultiExpr::new(vec![])))
// //         //     // close_and_or(lines, me)
// //         // }

// //         if next_is_multiexpr(&lines) {
// //             let lines = proces_value("BEGIN_MULTEXPR".to_string(), lines)?;
// //             let (lines, me) = get_rec_mexpr(lines)?;
// //             let (lines, e) = close_and_or(lines, me)?;
// //             Ok((lines, MultiExpr::new(vec![e])))
// //         // Ok((lines, me))
// //         } else {
// //             // let (lines, me) =
// //             get_rec_expr(lines, MultiExpr::new(vec![]))
// //             // close_and_or(lines, me)
// //         }
// //     };

// // let empty_rules = Ok(expression::SetOfRules::new(HashMap::<
// //     String,
// //     expression::Expression,
// // >::new()));
// // ir.0.split('\n').fold(empty_rules, |acc, c| match c {
// //     "BEGIN_MULTEXPR" => Err(Error(format!("unexpected command {}", c))),
// //     _ => Err(Error(format!("unexpected command {}", c))),
// // })

// // #[derive(Debug)]
// // Most of peg functions will return a result with this type
// // on Error side
// // pub struct Error(pub(crate) String);

// // A set of rules, or a ir2rules::Error
// // pub(crate) type Result = result::Result<crate::parser::expression::SetOfRules, Error>;

// // #[derive(Debug)]
// // pub struct IR(pub(crate) String);

// // struct Lines<'a> {
// //     lines: std::str::Lines<'a>,
// //     next: Option<&'a str>,
// // }

// // impl IR {
// //     // pub fn get_rules(&self) -> Result<SetOfRules, Error> {
// //     //     let (_, rules) = get_rules_rec(lines, SetOfRules::empty())?;
// //     //     Ok(rules)
// //     // }

// //     // pub fn str(&self) -> &str {
// //     //     self.source
// //     // }
// // }
// // fn get_rules_rec(ir: IR, rules: SetOfRules) -> Result<(IR, SetOfRules), Error> {
// //     let (ir, next) = ir.next();
// //     if next == None {
// //         Ok((ir, rules))
// //     } else {
// //         // let (lines, rule) = get_rule(lines)?;
// //         // Ok((lines, rules.merge(rule)))
// //         get_rules_rec(ir, rules.merge(rule))
// //     }
// // }

// // fn get_rules_rec(lines: Lines, rules: SetOfRules) -> Result<(Lines, SetOfRules), Error> {
// //     if lines.next == None {
// //         Ok((lines, rules))
// //     } else {
// //         let (lines, rule) = get_rule(lines)?;
// //         // Ok((lines, rules.merge(rule)))
// //         get_rules_rec(lines, rules.merge(rule))
// //     }
// // }

// // pub(crate) fn getrules_form_ir(ir: &IR) -> Result<SetOfRules, Error> {
// //     let lines = Lines::new(&ir.0);

// //     let (_, rules) = get_rules_rec(lines, SetOfRules::empty())?;
// //     Ok(rules)
// // }

// // fn get_rules_rec(lines: Lines, rules: SetOfRules) -> Result<(Lines, SetOfRules), Error> {
// //     if lines.next == None {
// //         Ok((lines, rules))
// //     } else {
// //         let (lines, rule) = get_rule(lines)?;
// //         // Ok((lines, rules.merge(rule)))
// //         get_rules_rec(lines, rules.merge(rule))
// //     }
// // }

// // impl<'a> Lines<'a> {
// //     // fn new2(lines: std::str::Lines<'a>) -> Self {
// //     //     Self(lines)
// //     // }

// //     fn new(lines: &'a str) -> Self {
// //         let mut lines = lines.lines();
// //         let next = lines.next();
// //         Self { lines, next }
// //     }

// //     fn next(mut self) -> (Option<&'a str>, Self) {
// //         dbg!(self.next);
// //         let result = self.next;
// //         self.next = self.lines.next();
// //         (result, self)
// //     }

// //     fn peek(&self) -> Option<&str> {
// //         dbg!(self.next);
// //         self.next
// //     }

// //     fn next_or(self, fn_error: &dyn Fn() -> Error) -> Result<(&'a str, Self), Error> {
// //         match self.next {
// //             Some(n) => {
// //                 let (_, s) = self.next();
// //                 Ok((n, s))
// //             }
// //             None => Err(fn_error()),
// //         }
// //     }
// // }

// // fn process_rule(ir: &IR) -> Result<(String, Expression), Error> {
// //     // BEGIN_MULTEXPR
// //     // ATOM RUL grammar
// //     // CLOSE AND
// //     // CLOSE OR
// //     // CLOSE_RULE main
// //     Err(Error("".to_string()))
// // }

// // fn get_expr(lines: Lines) -> Result<(Lines, Expression), Error> {
// //     // ATOM RUL grammar

// //     let next_command = lines.peek().and_then(|s| s.split(" ").nth(0));

// //     match next_command {
// //         Some("ATOM") => get_atom(lines),
// //         Some("BEGIN_MULTEXPR") => get_multi_expr(lines),
// //         other => Err(Error(format!(
// //             "unknown command processing expr {:?}",
// //             other
// //         ))),
// //     }
// // }

// // fn get_atom(lines: Lines) -> Result<(Lines, Expression), Error> {
// //     // ATOM RUL grammar

// //     let (lines, params) = process_command(lines, "ATOM".to_string())?;
// //     match params.as_slice() {
// //         ["RUL", name] => Ok((lines, Expression::RuleName(name.to_string()))),
// //         c => Err(Error(format!("unknown pattern processing atom {:?}", c))),
// //     }

// //     // let (l, lines) = lines.next();
// //     // let l = l.ok_or_else(|| Error("found end on looking for expression".to_string()))?;

// //     // let tokens = l.split(" ").collect::<Vec<_>>();
// //     // match tokens.as_slice() {
// //     //     ["ATOM", "RUL", name] => Ok((Expression::RuleName(name.to_string()), lines)),
// //     //     ["ATOM", _, _] => Err(Error(format!("unknown expr type {}", l))),
// //     //     _ => Err(Error(format!("unknown pattern processing expr {}", l))),
// //     // }
// // }

// // #[test]
// // fn test_get_atom() {
// //     let r = get_atom(Lines::new("ATOM RUL grammar")).unwrap();

// //     let e = ref_rule! {"grammar"};

// //     assert_eq!(r.1, e)
// // }

// // fn next_is_atom(lines: &Lines) -> bool {
// //     match lines.peek() {
// //         None => false,
// //         Some(l) => l.starts_with("ATOM "),
// //     }
// // }

// // fn next_is_multiexpr(lines: &Lines) -> bool {
// //     match lines.peek() {
// //         None => false,
// //         Some(l) => l.starts_with("BEGIN_MULTEXPR"),
// //     }
// // }

// // fn get_multi_expr(lines: Lines) -> Result<(Lines, Expression), Error> {
// //     // BEGIN_MULTEXPR
// //     // ATOM RUL grammar
// //     // ATOM RUL grammar2
// //     // CLOSE_MULTEXPR
// //     fn get_rec_mexpr(lines: Lines) -> Result<(Lines, MultiExpr), Error> {
// //         // if next_is_multiexpr(&lines) {
// //         //     let (lines, params) = process_command(lines, "BEGIN".to_string())?;
// //         //     match params.as_slice() {
// //         //         ["OR"] => get_rec_mexpr(lines, or!()),
// //         //         ["AND"] => get_rec_mexpr(lines, and!()),
// //         //         [p] => Err(Error(format!("Expected AND/OR multiexp, rec: {}", p))),
// //         //     }
// //         // // let (lines, me) = get_rec_mexpr(lines)?;
// //         // // let (lines, e) = close_and_or(lines, me)?;
// //         // // Ok((lines, MultiExpr::new(vec![e])))
// //         // // Ok((lines, me))
// //         // } else {
// //         //     // let (lines, me) =
// //         //     Ok(get_rec_expr(lines, MultiExpr::new(vec![])))
// //         //     // close_and_or(lines, me)
// //         // }

// //         if next_is_multiexpr(&lines) {
// //             let lines = proces_value("BEGIN_MULTEXPR".to_string(), lines)?;
// //             let (lines, me) = get_rec_mexpr(lines)?;
// //             let (lines, e) = close_and_or(lines, me)?;
// //             Ok((lines, MultiExpr::new(vec![e])))
// //         // Ok((lines, me))
// //         } else {
// //             // let (lines, me) =
// //             get_rec_expr(lines, MultiExpr::new(vec![]))
// //             // close_and_or(lines, me)
// //         }
// //     };

// //     fn get_rec_expr(lines: Lines, me: MultiExpr) -> Result<(Lines, MultiExpr), Error> {
// //         // match lines.peek_command() {
// //         //     Some("CLOSE") => Ok((lines, me)),
// //         //     None => Err(Error("processing expression, not found close {:?}", lines.peek()))
// //         //     _ => {
// //         //         let (lines, e) = get_expr(lines).unwrap();
// //         //         let me = me.ipush(e);
// //         //         get_rec_expr(lines, me)
// //         //     }
// //         // }
// //         // if lines.peek() == Some("CLOSE_MULTEXPR") {
// //         //     (lines, me)
// //         // } else {
// //         //     let (lines, e) = get_expr(lines).unwrap();
// //         //     let me = me.ipush(e);
// //         //     get_rec_expr(lines, me)
// //         // }
// //         // if next_is_expression(&lines) {
// //         //     let (e, lines) = get_expr(lines).unwrap();
// //         //     let me = me.ipush(e);
// //         //     get_rec_expr(lines, me)
// //         // } else {
// //         //     (lines, me)
// //         // }
// //         unimplemented!()
// //     };

// //     let (lines, me) = get_rec_mexpr(lines)?;
// //     let (oe, _) = me.0.ipop();
// //     match oe {
// //         None => Err(Error("empty multy expression".to_string())),
// //         Some(e) => Ok((lines, e)),
// //     }
// // }

// // fn close_and_or(lines: Lines, me: MultiExpr) -> Result<(Lines, Expression), Error> {
// //     // CLOSE AND

// //     let (l, lines) =
// //         lines.next_or(&|| Error("found end on looking for close and/or".to_string()))?;

// //     let close = l.split(" ").collect::<Vec<_>>();
// //     match close.as_slice() {
// //         ["CLOSE", "AND"] => Ok((lines, Expression::And(me))),
// //         ["CLOSE", "OR"] => Ok((lines, Expression::Or(me))),
// //         _ => Err(Error(format!("expected close, received: {}", l))),
// //     }
// // }

// // #[test]
// // fn test_get_multi_expr() {
// //     {
// //         let (_, me) = get_multi_expr(Lines::new(
// //             "BEGIN_MULTEXPR
// // ATOM RUL grammar
// // ATOM RUL grammar2
// // ATOM RUL grammar3
// // CLOSE OR",
// //         ))
// //         .unwrap();

// //         let e = or!(
// //             ref_rule!(r#"grammar"#),
// //             ref_rule!(r#"grammar2"#),
// //             ref_rule!(r#"grammar3"#)
// //         );

// //         assert_eq!(me, e)
// //     }
// //     {
// //         let (_, me) = get_multi_expr(Lines::new(
// //             "BEGIN_MULTEXPR
// // BEGIN_MULTEXPR
// // ATOM RUL grammar
// // ATOM RUL grammar2
// // ATOM RUL grammar3
// // CLOSE AND
// // CLOSE OR",
// //         ))
// //         .unwrap();

// //         let e = or!(and!(
// //             ref_rule!(r#"grammar"#),
// //             ref_rule!(r#"grammar2"#),
// //             ref_rule!(r#"grammar3"#)
// //         ));

// //         assert_eq!(me, e)
// //     }
// // }

// // fn get_rule(lines: Lines) -> Result<(Lines, SetOfRules), Error> {
// //     // BEGIN_RULE
// //     // BEGIN_MULTEXPR
// //     // ATOM RUL grammar
// //     // CLOSE AND
// //     // CLOSE_RULE main

// //     let lines = proces_value("BEGIN_RULE".to_string(), lines)?;
// //     let (lines, me) = get_expr(lines)?;
// //     let (lines, params) = process_command(lines, "CLOSE_RULE".to_string())?;
// //     match params.as_slice() {
// //         [name] => Ok((lines, rules! { name   =>  me})),
// //         p => Err(Error(format!(
// //             "expected close rule <name>, received: {:?}",
// //             p
// //         ))),
// //     }
// // }

// // #[test]
// // fn test_get_rule() {
// //     {
// //         let (_, rule) = get_rule(Lines::new(
// //             "BEGIN_RULE
// // BEGIN_MULTEXPR
// // ATOM RUL grammar
// // CLOSE AND
// // CLOSE_RULE main
// // ",
// //         ))
// //         .unwrap();

// //         let r = rules! {
// //             "main" => and!(ref_rule!("grammar"))
// //         };

// //         assert_eq!(rule, r)
// //     }
// // }

// // fn process_command(lines: Lines, command: String) -> Result<(Lines, Vec<&str>), Error> {
// //     let (l, lines) = lines.next();
// //     let l = l.ok_or_else(|| Error(format!("Expected command {} on end program", command)))?;

// //     Ok((lines, l.split(" ").skip(1).collect::<Vec<_>>()))
// // }

// // fn proces_value(line: String, lines: Lines) -> Result<Lines, Error> {
// //     let (l, lines) = lines.next();
// //     let l = l.ok_or_else(|| Error(format!("Expected {} on end program", line)))?;

// //     if l != line {
// //         Err(Error(format!("Expected {} received {}", line, l)))
// //     } else {
// //         Ok(lines)
// //     }
// // }
