use crate::parser;

pub(crate) fn rules2parse_peg_new() -> parser::expression::SetOfRules {
    rules!(
        //    r#"lit_noesc"# => and!(ref_rule!(r#"_'"#), rep!(and!(not!(ref_rule!(r#"_'"#)), dot!()), 0), ref_rule!(r#"_'"#))

        r#"_'"# => or!(and!(lit!("'")))
        , r#"rep_symbol"# => or!(and!(transf2!( and!( and!(lit!("*")) ) , t2rules!(t2_text!("REPEAT"), t2_funct!("endl"), t2_text!("0"), t2_funct!("endl"), t2_text!("inf"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!("+")) ) , t2rules!(t2_text!("REPEAT"), t2_funct!("endl"), t2_text!("1"), t2_funct!("endl"), t2_text!("inf"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!("?")) ) , t2rules!(t2_text!("REPEAT"), t2_funct!("endl"), t2_text!("0"), t2_funct!("endl"), t2_text!("1"), t2_funct!("endl"), ) )))
        , r#"_""# => or!(and!(lit!("\"")))
        , r#"atom"# => or!(and!(transf2!( and!( and!(named!("a", ref_rule!(r#"literal"#))) ) , t2rules!(t2_text!("ATOM"), t2_funct!("endl"), t2_text!("LIT"), t2_funct!("endl"), t2_byname!("a"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(named!("a", ref_rule!(r#"match"#))) ) , t2rules!(t2_text!("MATCH"), t2_funct!("endl"), t2_byname!("a"), ) )), and!(transf2!( and!( and!(named!("a", ref_rule!(r#"rule_name"#))) ) , t2rules!(t2_text!("ATOM"), t2_funct!("endl"), t2_text!("RULREF"), t2_funct!("endl"), t2_byname!("a"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(ref_rule!(r#"dot"#)) ) , t2rules!(t2_text!("ATOM"), t2_funct!("endl"), t2_text!("DOT"), t2_funct!("endl"), ) )))
        , r#"parenth"# => or!(and!(transf2!( and!( and!(lit!("("), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_"#)) ) , t2rules!(t2_byname!("expr"), ) ), or!(and!(transf2!( and!( and!(lit!(")")) ) , t2rules!(t2_funct!("none"), ) )))))
        , r#"error"# => or!(and!(transf2!( and!( and!(lit!("error"), ref_rule!(r#"_"#), lit!("("), ref_rule!(r#"_"#), ref_rule!(r#"literal"#), ref_rule!(r#"_"#), lit!(")")) ) , t2rules!(t2_text!("ERROR"), t2_funct!("endl"), t2_byname!("literal"), t2_funct!("endl"), ) )))
        , r#"dot"# => or!(and!(lit!(".")))
        , r#"esc_char"# => or!(and!(lit!("\r")), and!(lit!("\n")), and!(lit!("\t")), and!(lit!("\\")), and!(lit!("\\\"")))
        , r#"expr"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"or"#)) ) , t2rules!(t2_text!("OR"), t2_funct!("endl"), t2_byname!("or"), t2_text!("CLOSE_MEXPR"), t2_funct!("endl"), ) )))
        , r#"rep_or_neg"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"atom_or_par"#), ref_rule!(r#"rep_symbol"#)) ) , t2rules!(t2_byname!("rep_symbol"), t2_byname!("atom_or_par"), ) )), and!(transf2!( and!( and!(ref_rule!(r#"atom_or_par"#)) ) , t2rules!(t2_byname!("atom_or_par"), ) )), and!(transf2!( and!( and!(lit!("!"), ref_rule!(r#"atom_or_par"#)) ) , t2rules!(t2_text!("NEGATE"), t2_funct!("endl"), t2_byname!("atom_or_par"), ) )))
        , r#"or"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_"#), ref_rule!(r#"and"#)) ) , t2rules!(t2_text!("AND"), t2_funct!("endl"), t2_byname!("and"), t2_text!("CLOSE_MEXPR"), t2_funct!("endl"), ) ), transf2!( and!( and!(rep!(or!(and!(ref_rule!(r#"_"#), lit!("/"), ref_rule!(r#"_"#), ref_rule!(r#"or"#))), 0, 1)) ) , t2rules!(t2_byname!("or"), ) )))
        , r#"match"# => or!(and!(transf2!( and!( and!(lit!("[")) ) , t2rules!(t2_funct!("none"), ) ), or!(and!(transf2!( and!( and!(ref_rule!(r#"mchars"#), named!("b", or!(and!(rep!(ref_rule!(r#"mbetween"#), 0))))) ) , t2rules!(t2_text!("CHARS"), t2_funct!("endl"), t2_byname!("mchars"), t2_funct!("endl"), t2_text!("BETW"), t2_funct!("endl"), t2_byname!("b"), t2_text!("EOBETW"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(named!("b", or!(and!(rep!(ref_rule!(r#"mbetween"#), 1))))) ) , t2rules!(t2_text!("BETW"), t2_funct!("endl"), t2_byname!("b"), t2_text!("EOBETW"), t2_funct!("endl"), ) ))), transf2!( and!( and!(lit!("]")) ) , t2rules!(t2_funct!("none"), ) )))
        , r#"tmpl_text"# => or!(and!(transf2!( and!( and!(named!("t", or!(and!(rep!(or!(and!(not!(or!(and!(lit!("$(")), and!(ref_rule!(r#"eol"#)))), dot!())), 1))))) ) , t2rules!(t2_text!("TEXT"), t2_funct!("endl"), t2_byname!("t"), t2_funct!("endl"), ) )))
        , r#"symbol"# => or!(and!(ematch!(chlist r#"_"#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist r#"_'""#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0)))
        , r#"mchars"# => or!(and!(rep!(or!(and!(not!(lit!("]")), not!(or!(and!(dot!(), lit!("-")))), dot!())), 1)))
        , r#"hex_char"# => or!(and!(lit!("\0x"), ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' ), ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' )))
        , r#"_1"# => or!(and!(lit!(" ")), and!(transf2!( and!( and!(ref_rule!(r#"eol"#)) ) , t2rules!(t2_funct!("none"), ) )))
        , r#"main"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"grammar"#)) ) , t2rules!(t2_byname!("grammar"), t2_text!("EOP"), ) )))
        , r#"grammar"# => or!(and!(rep!(ref_rule!(r#"rule"#), 1)))
        , r#"mline_comment"# => or!(and!(lit!("/*"), rep!(or!(and!(not!(lit!("*/")), dot!())), 0), lit!("*/")))
        , r#"tmpl_rule"# => or!(and!(transf2!( and!( and!(lit!("$(")) ) , t2rules!(t2_funct!("none"), ) ), or!(and!(transf2!( and!( and!(lit!("?"), ref_rule!(r#"symbol"#)) ) , t2rules!(t2_text!("NAMED_OPT"), t2_funct!("endl"), t2_byname!("symbol"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(ref_rule!(r#"symbol"#)) ) , t2rules!(t2_text!("NAMED"), t2_funct!("endl"), t2_byname!("symbol"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!("."), named!("pos", or!(and!(rep!(ematch!(chlist r#""#  , from '0', to '9' ), 1))))) ) , t2rules!(t2_text!("POS"), t2_funct!("endl"), t2_byname!("symbol"), t2_byname!("pos"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!(":")) ) , t2rules!(t2_funct!("none"), ) ), transf2!( and!( and!(named!("fn", or!(and!(rep!(or!(and!(not!(or!(and!(lit!(")")), and!(ref_rule!(r#"eol"#)))), dot!())), 1))))) ) , t2rules!(t2_text!("FUNCT"), t2_funct!("endl"), t2_byname!("fn"), t2_funct!("endl"), ) ))), transf2!( and!( and!(lit!(")")) ) , t2rules!(t2_funct!("none"), ) )))
        , r#"line_comment"# => or!(and!(lit!("//"), rep!(or!(and!(not!(ref_rule!(r#"eol"#)), dot!())), 0)))
        , r#"andchunk"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"name"#), named!("e", ref_rule!(r#"rep_or_neg"#))) ) , t2rules!(t2_text!("NAMED"), t2_funct!("endl"), t2_byname!("name"), t2_funct!("endl"), t2_byname!("e"), ) )), and!(ref_rule!(r#"rep_or_neg"#)))
        , r#"eol"# => or!(and!(lit!("\r\n")), and!(lit!("\n")), and!(lit!("\r")))
        , r#"transf_rule"# => or!(and!(rep!(or!(and!(ref_rule!(r#"tmpl_text"#)), and!(ref_rule!(r#"tmpl_rule"#))), 1)))
        , r#"_eol"# => or!(and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"comment"#))), 0), ref_rule!(r#"eol"#)))
        , r#"mbetween"# => or!(and!(transf2!( and!( and!(named!("f", dot!()), lit!("-"), named!("s", dot!())) ) , t2rules!(t2_byname!("f"), t2_funct!("endl"), t2_byname!("s"), t2_funct!("endl"), ) )))
        , r#"lit_noesc"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_'"#), named!("l", rep!(or!(and!(not!(ref_rule!(r#"_'"#)), dot!())), 0)), ref_rule!(r#"_'"#)) ) , t2rules!(t2_byname!("l"), ) )))
        , r#"transf2"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_1"#), ref_rule!(r#"_"#), lit!("->"), rep!(lit!(" "), 0)) ) , t2rules!(t2_funct!("none"), ) ), transf2!( and!( and!(ref_rule!(r#"transf_rule"#)) ) , t2rules!(t2_byname!("transf_rule"), ) ), transf2!( and!( and!(ref_rule!(r#"eol"#)) ) , t2rules!(t2_funct!("none"), ) )))
        , r#"rule_name"# => or!(and!(rep!(lit!("."), 0, 1), ref_rule!(r#"symbol"#), rep!(or!(and!(lit!("."), ref_rule!(r#"symbol"#))), 0)))
        , r#"andline"# => or!(and!(ref_rule!(r#"andchunk"#), rep!(or!(and!(transf2!( and!( and!(rep!(lit!(" "), 1)) ) , t2rules!(t2_funct!("none"), ) ), ref_rule!(r#"andchunk"#))), 0)))
        , r#"_"# => or!(and!(transf2!( and!( and!(or!(and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"eol"#)), and!(ref_rule!(r#"comment"#))), 0)))) ) , t2rules!(t2_funct!("none"), ) )))
        , r#"literal"# => or!(and!(ref_rule!(r#"lit_noesc"#)), and!(ref_rule!(r#"lit_esc"#)))
        , r#"and"# => or!(and!(ref_rule!(r#"error"#)), and!(transf2!( and!( and!(or!(and!(ref_rule!(r#"andline"#), ref_rule!(r#"transf2"#), named!("and", rep!(or!(and!(transf2!( and!( and!(ref_rule!(r#"_"#)) ) , t2rules!(t2_funct!("none"), ) ), not!(or!(and!(ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), or!(and!(lit!("=")), and!(lit!("{")))))), ref_rule!(r#"and"#))), 0, 1))))) ) , t2rules!(t2_text!("TRANSF2"), t2_funct!("endl"), t2_byname!("transf2"), t2_text!("EOTRANSF2"), t2_funct!("endl"), t2_text!("AND"), t2_funct!("endl"), t2_byname!("andline"), t2_text!("CLOSE_MEXPR"), t2_funct!("endl"), t2_byname!("and"), ) )), and!(ref_rule!(r#"andline"#), rep!(or!(and!(transf2!( and!( and!(or!(and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"comment"#))), 0), rep!(ref_rule!(r#"eol"#), 1), ref_rule!(r#"_"#)))) ) , t2rules!(t2_funct!("none"), ) ), not!(or!(and!(ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), or!(and!(lit!("=")), and!(lit!("{")))))), ref_rule!(r#"and"#))), 0, 1)))
        , r#"lit_esc"# => or!(and!(transf2!( and!( and!(or!(and!(ref_rule!(r#"_""#), named!("l", rep!(or!(and!(ref_rule!(r#"esc_char"#)), and!(ref_rule!(r#"hex_char"#)), and!(not!(ref_rule!(r#"_""#)), dot!())), 0)), ref_rule!(r#"_""#)))) ) , t2rules!(t2_byname!("l"), ) )))
        , r#"rule"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_"#), ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), lit!("="), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_eol"#), ref_rule!(r#"_"#)) ) , t2rules!(t2_text!("RULE"), t2_funct!("endl"), t2_byname!("rule_name"), t2_funct!("endl"), t2_byname!("expr"), ) )))
        , r#"name"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"symbol"#), lit!(":")) ) , t2rules!(t2_byname!("symbol"), ) )))
        , r#"comment"# => or!(and!(transf2!( and!( and!(or!(and!(ref_rule!(r#"line_comment"#)), and!(ref_rule!(r#"mline_comment"#)))) ) , t2rules!(t2_funct!("none"), ) )))
        , r#"atom_or_par"# => or!(and!(ref_rule!(r#"atom"#)), and!(ref_rule!(r#"parenth"#)))
    )
}

pub(crate) fn rules2parse_peg() -> parser::expression::SetOfRules {
    rules!(
        r#"lit_noesc"# => and!(ref_rule!(r#"_'"#), rep!(and!(not!(ref_rule!(r#"_'"#)), dot!()), 0), ref_rule!(r#"_'"#))
        , r#"tmpl_text"# => and!(rep!(and!(not!(or!(and!(lit!("$(")), and!(ref_rule!(r#"eol"#)))), dot!()), 1))
        , r#"transf_rule"# => and!(rep!(or!(and!(ref_rule!(r#"tmpl_text"#)), and!(ref_rule!(r#"tmpl_rule"#))), 0))
        , r#"_eol"# => and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"comment"#))), 0), ref_rule!(r#"eol"#))
        , r#"mbetween"# => and!(and!(dot!(), lit!("-"), dot!()))
        , r#"dot"# => and!(lit!("."))
        , r#"rep_or_neg"# => or!(and!(ref_rule!(r#"atom_or_par"#), rep!(or!(and!(lit!("*")), and!(lit!("+")), and!(lit!("?"))), 0, 1)), and!(lit!("!"), ref_rule!(r#"atom_or_par"#)))
        , r#"eol"# => and!(or!(and!(lit!("\r\n")), and!(lit!("\n")), and!(lit!("\r"))))
        , r#"atom"# => or!(and!(ref_rule!(r#"literal"#)), and!(ref_rule!(r#"match"#)), and!(ref_rule!(r#"rule_name"#)), and!(ref_rule!(r#"dot"#)))
        , r#"line_comment"# => and!(lit!("//"), rep!(and!(not!(ref_rule!(r#"eol"#)), dot!()), 0))
        , r#"match"# => and!(lit!("["), or!(and!(and!(ref_rule!(r#"mchars"#), rep!(ref_rule!(r#"mbetween"#), 0))), and!(rep!(ref_rule!(r#"mbetween"#), 1))), lit!("]"))
        , r#"esc_char"# => or!(and!(lit!("\\r")), and!(lit!("\\n")), and!(lit!("\\t")), and!(lit!("\\\\")), and!(lit!("\\\"")))
        , r#"_""# => and!(lit!("\""))
        , r#"comment"# => or!(and!(ref_rule!(r#"line_comment"#)), and!(ref_rule!(r#"mline_comment"#)))
        , r#"literal"# => or!(and!(ref_rule!(r#"lit_noesc"#)), and!(ref_rule!(r#"lit_esc"#)))
        , r#"tmpl_rule"# => and!(named!("_", lit!("$(")), or!(and!(ref_rule!(r#"symbol"#)), and!(lit!("."), rep!(ematch!(chlist r#""#  , from '0', to '9' ), 1)), and!(lit!(":"), rep!(and!(not!(or!(and!(lit!(")")), and!(ref_rule!(r#"eol"#)))), dot!()), 1))), named!("_", lit!(")")))
        , r#"atom_or_par"# => and!(or!(and!(ref_rule!(r#"atom"#)), and!(ref_rule!(r#"parenth"#))))
        , r#"mchars"# => and!(rep!(and!(not!(lit!("]")), not!(and!(dot!(), lit!("-"))), dot!()), 1))
        , r#"hex_char"# => and!(lit!("\\0x"), ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' ), ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' ))
        , r#"or"# => and!(ref_rule!(r#"and"#), rep!(and!(ref_rule!(r#"_"#), lit!("/"), ref_rule!(r#"_"#), ref_rule!(r#"or"#)), 0, 1))
        , r#"rule_name"# => and!(rep!(lit!("."), 0, 1), ref_rule!(r#"symbol"#), rep!(and!(lit!("."), ref_rule!(r#"symbol"#)), 0))
        , r#"transf2"# => and!(ref_rule!(r#"_1"#), ref_rule!(r#"_"#), lit!("->"), rep!(lit!(" "), 0), ref_rule!(r#"transf_rule"#), ref_rule!(r#"eol"#))
        , r#"parenth"# => and!(lit!("("), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_"#), or!(and!(lit!(")")), and!(error!("unbalanced parethesis: missing ')'"))))
        , r#"main"# => and!(ref_rule!(r#"grammar"#))
        , r#"mline_comment"# => and!(lit!("/*"), rep!(and!(not!(lit!("*/")), dot!()), 0), lit!("*/"))
        , r#"_1"# => and!(or!(and!(lit!(" ")), and!(ref_rule!(r#"eol"#))))
        , r#"_"# => and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"eol"#)), and!(ref_rule!(r#"comment"#))), 0))
        , r#"lit_esc"# => and!(ref_rule!(r#"_""#), rep!(or!(and!(ref_rule!(r#"esc_char"#)), and!(ref_rule!(r#"hex_char"#)), and!(not!(ref_rule!(r#"_""#)), dot!())), 0), ref_rule!(r#"_""#))
        , r#"_'"# => and!(lit!("'"))
        , r#"named"# => and!(ref_rule!(r#"symbol"#), lit!(":"))
        , r#"grammar"# => and!(rep!(ref_rule!(r#"rule"#), 1))
        , r#"symbol"# => and!(ematch!(chlist r#"_"#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist r#"_'""#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0))
        , r#"rule"# => and!(ref_rule!(r#"_"#), ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), lit!("="), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_eol"#), ref_rule!(r#"_"#))
        , r#"error"# => and!(lit!("error"), ref_rule!(r#"_"#), lit!("("), ref_rule!(r#"_"#), ref_rule!(r#"literal"#), ref_rule!(r#"_"#), lit!(")"))
        , r#"and"# => or!(and!(ref_rule!(r#"error"#)), and!(rep!(ref_rule!(r#"named"#), 0, 1), ref_rule!(r#"rep_or_neg"#), rep!(ref_rule!(r#"transf2"#), 0, 1), rep!(and!(ref_rule!(r#"_1"#), ref_rule!(r#"_"#), not!(and!(ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), or!(and!(lit!("=")), and!(lit!("{"))))), ref_rule!(r#"and"#)), 0, 1)))
        , r#"expr"# => and!(ref_rule!(r#"or"#))
    )
}

//  ------------------------------------------------------------------------
//  ------------------------------------------------------------------------
//
//  this is the first version of code to parse the peg grammar
//  it was, obviously written by hand
// pub(crate) fn parse_peg_first() -> parser::expression::SetOfRules {
//     rules!(

//         "main"      =>       ref_rule!("grammar"),

//         "grammar"   =>       rep!(ref_rule!("rule"), 1),

//         "rule"      =>       and!(
//                                  ref_rule!("_"), ref_rule!("symbol"),
//                                  ref_rule!("_"), lit! ("="),
//                                  ref_rule!("_"), ref_rule!("expr"),
//                                 ref_rule!("_eol"),
//                                 ref_rule!("_")
//                              ),

//         "expr"      =>      ref_rule!("or"),

//         "or"        =>      and!(
//                                 ref_rule!("and"),
//                                 rep!(
//                                     and!(
//                                         ref_rule!("_"), lit!("/"),
//                                         ref_rule!("_"), ref_rule!("or")
//                                     ),
//                                     0
//                                 )
//                             ),

//         "and"       =>     and!(
//                                 ref_rule!("rep_or_neg"),
//                                 rep!(
//                                     and!(
//                                         ref_rule!("_1"), ref_rule!("_"),
//                                         not!(and!(
//                                                 ref_rule!("symbol"),
//                                                 ref_rule!("_"), lit! ("=")
//                                         )),
//                                         ref_rule!("and")
//                                     ),
//                                     0
//                                 )
//                             ),

//         "rep_or_neg" =>     or!(
//                                 and!(
//                                     ref_rule!("atom_or_par"),
//                                     rep!(
//                                         or!(
//                                             lit!("*"),
//                                             lit!("+"),
//                                             lit!("?")
//                                         )
//                                         , 0, 1
//                                     )
//                                 ),
//                                 and!(
//                                     lit!("!"),
//                                     ref_rule!("atom_or_par")
//                                 )
//                             ),

//         "atom_or_par" =>    or!(
//                                 ref_rule!("atom"),
//                                 ref_rule!("parenth")
//                             ),

//         "parenth"       =>  and!(
//                                 lit!("("),
//                                 ref_rule!("_"),
//                                 ref_rule!("expr"),
//                                 ref_rule!("_"),
//                                 lit!(")")
//                             ),

//         "atom"          =>  or!(
//                                 ref_rule!("literal"),
//                                 ref_rule!("match"),
//                                 ref_rule!("dot"),
//                                 ref_rule!("symbol")
//                             ),

//         "literal"       =>  and!(
//                                 ref_rule!(r#"_""#),
//                                 rep!(
//                                     and!(
//                                         not!(
//                                             ref_rule!(r#"_""#)
//                                         ),
//                                         dot!()
//                                     )
//                                 , 0
//                             ),
//                                 ref_rule!(r#"_""#)
//                             ),

//         r#"_""#         =>  lit!(r#"""#),

//         "match"         =>  and!(
//                                 lit!("["),
//                                 or!(
//                                     and!(
//                                         rep!(ref_rule!("mchars"), 1),
//                                         rep!(ref_rule!("mbetween"), 0)
//                                     ),
//                                     rep!(ref_rule!("mbetween"), 1)
//                                 ),
//                                 lit!("]")
//                             ),

//         "mchars"        =>  rep!(
//                                 and!(
//                                     not!(lit!("]")),
//                                     not!(and!(dot!(), lit!("-"))),
//                                     dot!())
//                                 ,1
//                             ),

//         "mbetween"      =>  and!(dot!(), lit!("-"), dot!()),

//         "dot"           =>  lit!("."),

//         "symbol"        =>  and!(
//                                 ematch!(    chlist "_'",
//                                         from 'a', to 'z',
//                                         from 'A', to 'Z',
//                                         from '0', to '9'
//                                 ),
//                                 rep!(
//                                     ematch!(    chlist "_'\"",
//                                             from 'a', to 'z',
//                                             from 'A', to 'Z',
//                                             from '0', to '9'
//                                     ),
//                                     0
//                                 )
//                             ),

//         "_"             =>  rep!(   or!(
//                                         lit!(" "),
//                                         ref_rule!("eol")
//                                         // ref_rule!("comment")
//                                     )
//                                     , 0
//                             ),

//         "_eol"          =>  and!(
//                                 rep!(   or!(
//                                         lit!(" ")
//                                     )
//                                     , 0
//                                 ),
//                                 ref_rule!("eol")
//                             ),

//         "_1"            =>  or!(
//                                         lit!(" "),
//                                         ref_rule!("eol")
//                                         // ref_rule!("comment")
//                                 ),

//         "spaces"        =>  rep!(lit!(" "), 0),

//         "eol"          =>   or!(
//                                     lit!("\r\n"),
//                                     lit!("\n"),
//                                     lit!("\r")
//                                 )

//         // "comment"       =>  or!(
//         //                         and!(
//         //                             lit!("//"),
//         //                             rep!(
//         //                                 and!(
//         //                                     not!(ref_rule!("eol")),
//         //                                     dot!()
//         //                                 )
//         //                                 , 0
//         //                             ),
//         //                             ref_rule!("eol")
//         //                         ),
//         //                         and!(
//         //                             lit!("/*"),
//         //                             rep!(
//         //                                 and!(
//         //                                     not!(lit!("*/")),
//         //                                     dot!()
//         //                                 )
//         //                                 , 0
//         //                             ),
//         //                             lit!("*/")
//         //                         )
//         //                 )
//     )
// }

//  And this is the first autogenerated code  :-)  working
//     "rep_or_neg" => or!(and!(ref_rule!("atom_or_par"), rep!(or!(lit!("*"), lit!("+"), lit!("?")), 0, 1)), and!(lit!("!"), ref_rule!("atom_or_par")))
//    , "literal" => and!(ref_rule!("_\""), rep!(or!(and!(lit!("\\"), dot!()), and!(not!(ref_rule!("_\"")), dot!())), 0), ref_rule!("_\""))
//    , "eol" => or!(lit!("\r\n"), lit!("\n"), lit!("\r"))
//    , "mchars" => rep!(and!(not!(lit!("]")), not!(and!(dot!(), lit!("-"))), dot!()), 1)
//    , "mbetween" => and!(dot!(), lit!("-"), dot!())
//    , "atom_or_par" => or!(ref_rule!("atom"), ref_rule!("parenth"))
//    , "dot" => lit!(".")
//    , "or" => and!(ref_rule!("and"), rep!(and!(ref_rule!("_"), lit!("/"), ref_rule!("_"), ref_rule!("or")), 0))
//    , "_eol" => and!(rep!(lit!(" "), 0), ref_rule!("eol"))
//    , "rule" => and!(ref_rule!("_"), ref_rule!("symbol"), ref_rule!("_"), lit!("="), ref_rule!("_"), ref_rule!("expr"), ref_rule!("_eol"), ref_rule!("_"))
//    , "symbol" => and!(ematch!(chlist "_'"  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist "_'\""  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0))
//    , "main" => ref_rule!("grammar")
//    , "match" => and!(lit!("["), or!(and!(rep!(ref_rule!("mchars"), 1), rep!(ref_rule!("mbetween"), 0)), rep!(ref_rule!("mbetween"), 1)), lit!("]"))
//    , "grammar" => rep!(ref_rule!("rule"), 1)
//    , "and" => and!(ref_rule!("rep_or_neg"), rep!(and!(ref_rule!("_1"), ref_rule!("_"), not!(and!(ref_rule!("symbol"), ref_rule!("_"), lit!("="))), ref_rule!("and")), 0))
//    , "_" => rep!(or!(lit!(" "), ref_rule!("eol")), 0)
//    , "parenth" => and!(lit!("("), ref_rule!("_"), ref_rule!("expr"), ref_rule!("_"), lit!(")"))
//    , "expr" => ref_rule!("or")
//    , "_\"" => lit!("\"")
//    , "atom" => or!(ref_rule!("literal"), ref_rule!("match"), ref_rule!("dot"), ref_rule!("symbol"))
//    , "_1" => or!(lit!(" "), ref_rule!("eol"))
