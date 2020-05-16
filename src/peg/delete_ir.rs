// use crate::parser::expression::{
//     Expression, MetaExpr, MultiExpr, NamedExpr, ReplItem, ReplTemplate, SetOfRules, Transf2Expr,
// };
// use idata::cont::IVec;

// #[derive(Debug)]
// /// IR error information
// pub struct Error(pub(crate) String);

// #[derive(Debug)]
// /// IR error information
// pub struct IR {
//     pos: usize,
//     commands: Vec<Command>,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub(crate) struct Command(String);

// impl IR {
//     pub(crate) fn new(txt: &str) -> Self {
//         Self {
//             pos: 0,
//             commands: txt
//                 .lines()
//                 .map(|l| Command(l.to_string()))
//                 .collect::<Vec<_>>(),
//         }
//     }

//     fn get(mut self) -> Result<(IR, Command), Error> {
//         dbg!(self.commands[self.pos].clone());
//         if self.pos >= self.commands.len() {
//             Err(Error("next over finished program".to_string()))
//         } else {
//             let cmd = self.commands[self.pos].clone();
//             self.pos += 1;
//             Ok((self, cmd))
//         }
//     }

//     fn peek(&self) -> Option<Command> {
//         self.commands.get(self.pos).map(|c| c.clone())
//     }

//     fn consume(self, val: &str) -> Result<IR, Error> {
//         let (ir, cmd) = self.get()?;
//         if cmd.0 == val {
//             Ok(ir)
//         } else {
//             Err(Error(format!("expected {}, received {}", val, cmd.0)))
//         }
//     }

//     /// get rules from an IR code
//     pub fn get_rules(self) -> Result<SetOfRules, Error> {
//         let (_ir, rules) = get_rule_rec(self, SetOfRules::empty())?;
//         Ok(rules)
//     }
// }

// fn get_rule_rec(ir: IR, rules: SetOfRules) -> Result<(IR, SetOfRules), Error> {
//     if ir.peek() == Some(Command("EOP".to_string())) {
//         Ok((ir, rules))
//     } else {
//         let (ir, rule) = get_rule(ir)?;
//         let rules = rules.merge(rule);
//         get_rule_rec(ir, rules)
//     }
// }

// fn get_expr(ir: IR) -> Result<(IR, Expression), Error> {
//     //  ATOM
//     //  LIT
//     //  literal
//     let (ir, cmd) = ir.get()?;
//     match cmd.0.as_ref() {
//         "ATOM" => get_atom(ir),
//         "AND" => {
//             let (ir, mexpr) = get_mexpr(ir)?;
//             Ok((ir, Expression::And(mexpr)))
//         }
//         "OR" => {
//             let (ir, mexpr) = get_mexpr(ir)?;
//             Ok((ir, Expression::Or(mexpr)))
//         }
//         "REPEAT" => get_repeat(ir),
//         "MATCH" => get_match(ir),
//         "NAMED" => get_named(ir),
//         "NEGATE" => get_negate(ir),
//         "ERROR" => get_error(ir),
//         "TRANSF2" => get_transf2(ir),
//         other => Err(Error(format!("unknown cmd reading expression <{}>", other))),
//     }
// }

// fn get_transf2(ir: IR) -> Result<(IR, Expression), Error> {
//     let (ir, repl_templ) = get_transf2_items_rec(ir, ReplTemplate::empty())?;
//     let (ir, expr) = get_expr(ir)?;
//     Ok((
//         ir,
//         Expression::MetaExpr(MetaExpr::Transf2(Transf2Expr {
//             mexpr: MultiExpr(vec![expr]),
//             transf2_rules: repl_templ,
//         })),
//     ))
// }

// fn get_transf2_item(ir: IR) -> Result<(IR, ReplItem), Error> {
//     //  TEXT
//     //  txt
//     //
//     if let Some(c) = ir.peek() {
//         match c.0.as_ref() {
//             "TEXT" => {
//                 let (ir, txt) = get_transf2_text(ir)?;
//                 Ok((ir, ReplItem::Text(txt)))
//             }
//             "NAMED" => {
//                 let (ir, txt) = get_transf2_named(ir)?;
//                 Ok((ir, ReplItem::ByName(txt)))
//             }
//             "FUNCT" => {
//                 let (ir, txt) = get_transf2_named(ir)?;
//                 Ok((ir, ReplItem::Function(txt)))
//             }
//             other => Err(Error(format!(
//                 "unxecpected command reading get_transf2 <{}>",
//                 other
//             ))),
//         }
//     } else {
//         Err(Error(format!("Missing transf2 item")))
//     }
// }

// fn get_transf2_items_rec(ir: IR, repl_templ: ReplTemplate) -> Result<(IR, ReplTemplate), Error> {
//     if ir.peek() == Some(Command("EOTRANSF2".to_string())) {
//         let (ir, _) = ir.get()?;
//         Ok((ir, repl_templ))
//     } else {
//         let (ir, item) = get_transf2_item(ir)?;
//         get_transf2_items_rec(ir, repl_templ.ipush(item))
//     }
// }

// fn get_transf2_named(ir: IR) -> Result<(IR, String), Error> {
//     let (ir, _) = ir.get()?;
//     let (ir, named) = ir.get()?;
//     Ok((ir, named.0))
// }

// fn get_transf2_text(ir: IR) -> Result<(IR, String), Error> {
//     let (ir, _) = ir.get()?;
//     let (ir, txt) = ir.get()?;
//     Ok((ir, txt.0))
// }

// fn get_error(ir: IR) -> Result<(IR, Expression), Error> {
//     //  <err message>

//     let (ir, msg) = ir.get()?;
//     let expr = error!(msg.0);
//     Ok((ir, expr))
// }

// fn get_negate(ir: IR) -> Result<(IR, Expression), Error> {
//     //  <expr>

//     let (ir, expr) = get_expr(ir)?;
//     let expr = not!(expr);
//     Ok((ir, expr))
// }

// fn get_named(ir: IR) -> Result<(IR, Expression), Error> {
//     //  name
//     //  <expr>

//     let (ir, n) = ir.get()?;
//     let (ir, expr) = get_expr(ir)?;
//     let expr = crate::parser::expression::Expression::MetaExpr(MetaExpr::Named(NamedExpr {
//         name: n.0,
//         expr: Box::new(expr),
//     }));
//     Ok((ir, expr))
// }

// fn get_match(ir: IR) -> Result<(IR, Expression), Error> {
//     //  CHARS
//     //  ASDFASDF
//     //  BETWEEN
//     //  a
//     //  b
//     //  0
//     //  9

//     let (ir, chars) = get_match_chars(ir)?;
//     let (ir, between) = get_match_between(ir)?;
//     let amatch =
//         crate::parser::atom::Atom::Match(crate::parser::atom::MatchRules::init(&chars, between));
//     let expr = crate::parser::expression::Expression::Simple(amatch);
//     Ok((ir, expr))
// }

// fn get_match_chars(ir: IR) -> Result<(IR, String), Error> {
//     if Some(Command("CHARS".to_string())) == ir.peek() {
//         let (ir, _) = ir.get()?;
//         let (ir, c) = ir.get()?;
//         Ok((ir, c.0))
//     } else {
//         Ok((ir, "".to_string()))
//     }
// }

// fn get_match_between(ir: IR) -> Result<(IR, Vec<(char, char)>), Error> {
//     if Some(Command("BETW".to_string())) == ir.peek() {
//         let (ir, _) = ir.get()?;
//         let (ir, v) = get_between_rec(ir, vec![])?;
//         Ok((ir, v))
//     } else {
//         Ok((ir, vec![]))
//     }
// }

// fn get_between_rec(ir: IR, v: Vec<(char, char)>) -> Result<(IR, Vec<(char, char)>), Error> {
//     if ir.peek() == Some(Command("EOBETW".to_string())) {
//         let (ir, _) = ir.get()?;
//         Ok((ir, v))
//     } else {
//         let (ir, ch1) = ir.get()?;
//         let (ir, ch2) = ir.get()?;
//         let fc = |s: String| {
//             s.chars()
//                 .nth(0)
//                 .ok_or_else(|| Error(format!("expected char received <{}>", s)))
//         };

//         let ch1 = fc(ch1.0)?;
//         let ch2 = fc(ch2.0)?;

//         let v = v.ipush((ch1, ch2));
//         get_between_rec(ir, v)
//     }
// }

// fn get_repeat(ir: IR) -> Result<(IR, Expression), Error> {
//     //  1
//     //  inf
//     //  expr
//     let (ir, min) = ir.get()?;
//     let (ir, max) = ir.get()?;
//     let (ir, expr) = get_expr(ir)?;
//     Ok((ir, expr))
// }

// fn get_mexpr(ir: IR) -> Result<(IR, MultiExpr), Error> {
//     get_mexpr_rec(ir, MultiExpr::new(vec![]))
// }

// fn get_mexpr_rec(ir: IR, me: MultiExpr) -> Result<(IR, MultiExpr), Error> {
//     if ir.peek() == Some(Command("CLOSE_MEXPR".to_string())) {
//         let (ir, _) = ir.get()?;
//         Ok((ir, me))
//     } else {
//         let (ir, e) = get_expr(ir)?;
//         let me = me.ipush(e);
//         get_mexpr_rec(ir, me)
//     }
// }

// fn get_atom(ir: IR) -> Result<(IR, Expression), Error> {
//     //  LIT
//     //  literal
//     let (ir, cmd) = ir.get()?;
//     match cmd.0.as_ref() {
//         "LIT" => get_lit(ir),
//         "RULREF" => get_rulref(ir),
//         "DOT" => Ok((ir, dot!())),
//         other => Err(Error(format!("unknown cmd reading atom <{}>", other))),
//     }
// }

// fn get_rulref(ir: IR) -> Result<(IR, Expression), Error> {
//     //  name
//     let (ir, cmd) = ir.get()?;
//     Ok((ir, ref_rule!(cmd.0)))
// }

// fn get_lit(ir: IR) -> Result<(IR, Expression), Error> {
//     //  literal
//     let (ir, cmd) = ir.get()?;
//     Ok((ir, lit!(cmd.0)))
// }

// fn get_rule(ir: IR) -> Result<(IR, SetOfRules), Error> {
//     //  RULE
//     //  name
//     //  ATOM
//     //  LIT
//     //  literal
//     let ir = ir.consume("RULE")?;

//     let (ir, name) = ir.get()?;

//     let (ir, expr) = get_expr(ir)?;

//     Ok((ir, rules! { &name.0 => expr }))
// }

// #[test]
// fn test_get_rules_simple() {
//     let rules = IR::new(
//         "RULE
// name
// ATOM
// LIT
// literal
// EOF",
//     )
//     .get_rules()
//     .unwrap();

//     assert_eq!(rules, rules! { "name" => lit!("literal")})
// }
// // #![warn(missing_docs)]
// // //! Process de IR and generate the parser rules

// // // mod rules;

// // use idata::{self, cont::IVec};
// // use std::{self, result};

// // #[derive(Debug)]
// // /// IR error information
// // pub struct Error(pub(crate) String);

// // #[derive(Debug)]
// // pub struct IR {
// //     current: i32,
// //     commands: Vec<Command>,
// // }

// // impl IR {
// //     pub(crate) fn new(source: &str) -> Self {
// //         Self {
// //             current: -1,
// //             commands: source.lines().map(|l| Command::new(l)).collect(),
// //         }
// //     }

// //     pub(crate) fn next(mut self) -> (Self, Option<Command>) {
// //         self.current += 1;
// //         dbg!(self.current);
// //         let curr = self.commands.get(self.current as usize).map(|c| c.clone());
// //         (self, curr)
// //     }

// //     pub(crate) fn peek(&self) -> Option<Command> {
// //         let curr = self
// //             .commands
// //             .get((self.current + 1) as usize)
// //             .map(|c| c.clone());
// //         curr
// //     }

// //     pub(crate) fn next_command_is(&self, cmd: &str) -> bool {
// //         match self.peek() {
// //             None => false,
// //             Some(c) => c.cmd == cmd,
// //         }
// //     }
// // }

// // // #[derive(Debug)]
// // // pub struct IR<'a> {
// // //     // pub(crate) source: &'a str,
// // //     it: std::str::Lines<'a>,
// // //     next: Option<Command<'a>>,
// // // }

// // // fn __next_cmd(mut it: std::str::Lines) -> (std::str::Lines, Option<Command>) {
// // //     match it.next() {
// // //         None => (it, None),
// // //         Some(l) => (it, Some(Command::new(l))),
// // //     }
// // // }

// // // impl<'a> IR<'a> {
// // //     pub(crate) fn new2(source: String) -> Self {
// // //         let (it, next) = __next_cmd(source.lines());
// // //         Self {
// // //             // source: &source,
// // //             it,
// // //             next,
// // //         }
// // //     }
// // //     // pub(crate) fn new(source: &'a str) -> Self {
// // //     //     let (it, next) = __next_cmd(source.lines());
// // //     //     Self { source, it, next }
// // //     // }

// // //     pub(crate) fn next(mut self) -> (Self, Option<Command<'a>>) {
// // //         match self.next {
// // //             None => (self, None),
// // //             Some(command) => {
// // //                 let curr_cmd = command;
// // //                 self.next = match self.it.next() {
// // //                     None => None,
// // //                     Some(l) => Some(Command::new(l)),
// // //                 };
// // //                 (self, Some(curr_cmd))
// // //             }
// // //         }
// // //     }

// // //     pub(crate) fn peek(&self) -> Option<Command> {
// // //         self.next.clone()
// // //     }
// // // }

// // #[test]
// // fn test_ir() {
// //     let ir = IR::new(
// //         "DO1 PAR1 PAR2
// // DO2 PAR3
// // DO3",
// //     );
// //     let cmd1 = Command {
// //         cmd: "DO1".to_string(),
// //         params: vec!["PAR1".to_string(), "PAR2".to_string()],
// //     };
// //     let cmd2 = Command {
// //         cmd: "DO2".to_string(),
// //         params: vec!["PAR3".to_string()],
// //     };
// //     let cmd3 = Command {
// //         cmd: "DO3".to_string(),
// //         params: vec![],
// //     };

// //     assert_eq!(ir.peek().unwrap(), cmd1);
// //     let (ir, cc) = ir.next();
// //     assert_eq!(cc.unwrap(), cmd1);
// //     assert_eq!(ir.peek().unwrap(), cmd2);
// //     let (ir, cc) = ir.next();
// //     assert_eq!(cc.unwrap(), cmd2);
// //     assert_eq!(ir.peek().unwrap(), cmd3);
// //     let (ir, cc) = ir.next();
// //     assert_eq!(cc.unwrap(), cmd3);

// //     let (_ir, cc) = ir.next();
// //     assert_eq!(cc, None);
// // }

// // #[derive(Debug, PartialEq, Clone)]
// // pub(crate) struct Command {
// //     // src: &'a str,
// //     pub(crate) cmd: String,
// //     pub(crate) params: Vec<String>,
// // }

// // impl Command {
// //     fn new(line: &str) -> Self {
// //         let mut components = line.split(" ").map(|l| l.to_string()).collect::<Vec<_>>();
// //         let params = components.split_off(1);
// //         Self {
// //             // src: line,
// //             cmd: components[0].clone(),
// //             params,
// //         }
// //     }
// // }

// // #[test]
// // fn test_new_command() {
// //     {
// //         let cmd = Command::new("DO PAR1 PAR2");
// //         let cmd2 = Command {
// //             // src: "DO PAR1 PAR2",
// //             cmd: "DO".to_string(),
// //             params: vec!["PAR1".to_string(), "PAR2".to_string()],
// //         };
// //         assert_eq!(cmd, cmd2)
// //     }
// //     {
// //         let cmd = Command::new("DO PAR1");
// //         let cmd2 = Command {
// //             // src: "DO PAR1",
// //             cmd: "DO".to_string(),
// //             params: vec!["PAR1".to_string()],
// //         };
// //         assert_eq!(cmd, cmd2)
// //     }
// //     {
// //         let cmd = Command::new("DO");
// //         let cmd2 = Command {
// //             // src: "DO",
// //             cmd: "DO".to_string(),
// //             params: vec![],
// //         };
// //         assert_eq!(cmd, cmd2)
// //     }
// // }
