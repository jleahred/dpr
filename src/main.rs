extern crate dpr;

fn main() {
    dpr::print_rules2parse_peg2();
}

// extern crate dpr;

// fn main() -> Result<(), dpr::Error> {
//     let result = dpr::Peg::new(
//         r#"
//         main    =   expr

//         expr    =   term    (
//                             _  add_op   _  term     ->$(term)$(add_op)
//                             )*

//         term    =   factor  (
//                             _  mult_op  _  factor   ->$(factor)$(mult_op)
//                             )*

//         factor  =   pow     (
//                             _  pow_op   _  subexpr  ->$(subexpr)$(pow_op)
//                             )*

//         pow     =   subexpr (
//                             _  pow_op   _  pow  ->$(pow)$(pow_op)
//                             )*

//         subexpr =   '(' _ expr _                    ->$(expr)
//                                 (  ')'              ->$(:none)
//                                 /  error("parenthesis error")
//                         )
//                 /   number                        ->PUSH $(number)$(:endl)
//                 /   '-' _ subexpr                 ->PUSH 0$(:endl)$(subexpr)SUB$(:endl)

//         number  =   ([0-9]+  ('.' [0-9])?)

//         add_op  =   '+'     ->EXEC ADD$(:endl)
//                 /   '-'     ->EXEC SUB$(:endl)

//         mult_op =   '*'     ->EXEC MUL$(:endl)
//                 /   '/'     ->EXEC DIV$(:endl)

//         pow_op  =   '^'     ->EXEC POW$(:endl)

//         _       = ' '*
//         "#,
//     )
//     .gen_rules()?
//     .parse("1+2*3")?
//         //     .parse("2^3^4^5")?
//         //     .parse("2-3-4-5")?
//         //     .parse("-(-1+2* 3^5 ^(- 2 ) -7)+8")?
//         //     .parse("-(1))")?
//     .replace()?
//     //  ...
//     ;

//     //     println!("{:#?}", result);
//     println!("{}", result.str());
//     Ok(())
// }

// extern crate dpr;

// fn main() -> Result<(), dpr::Error> {
//     let result = dpr::Peg::new(
//         "
//         main    =   char+
//         char    =   'a'     -> $(:el)A
//                 /   'b'     -> $(:el)B
//                 /   ch:.    -> $(:el)$(ch)
//     ",
//     )
//     .gen_rules()?
//     .parse("aaacbbabdef")?
//     .replace(Some(&dpr::FnCallBack(custom_funtions)))?
//     //  ...
//     ;

//     println!("{:#?}", result);
//     println!("{}", result.str());
//     Ok(())
// }

// fn custom_funtions(fn_txt: &str) -> Option<String> {
//     match fn_txt {
//         "el" => Some("\n".to_string()),
//         _ => None,
//     }
// }
