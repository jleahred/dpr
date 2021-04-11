extern crate dpr;

fn main() -> Result<(), dpr::Error> {
    //dpr::print_rules2parse_peg2();
    main2()
}

//  -----------------------------------------------------------------------------------
//  -----------------------------------------------------------------------------------

// extern crate dpr;

fn main2() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        r#"
        main    =   expr

        expr    =   (  term   /  unary_expr  )
                            (
                                _  add_op   _  term             ->$(term)$(add_op)
                            /   _  add_op   _                   error("invalid expression after operator")
                            )*

        unary_expr  =     _  '-'  _  parornum                    ->PUSH 0$(:endl)$(parornum)EXEC SUB$(:endl)
                    /     _  '+'  _  parornum                    ->PUSH 0$(:endl)$(parornum)EXEC SUB$(:endl)
                    /     _  ( '+' / '-' )  _                   error("waitting open parenth or number after unary operator")



        term    =   factor  (
                                _  mult_op  _  factor           ->$(factor)$(mult_op)
                            /   _  mult_op  _                   error("invalid expression after operator")
                            )*

        factor  =   pow     (
                                _  pow_op   _  parornum          ->$(parornum)$(pow_op)
                            /   _  pow_op   _                   error("waitting parenthesis or number")
                            )*

        pow     =   parornum (
                                _  pow_op   _  pow              ->$(pow)$(pow_op)
                            /   _  pow_op   _                   error("invalid expression after operator")
                            )*

        parornum =   '(' _ expr _                                ->$(expr)
                                (  ')'                          ->$(:none)
                                /  error("missing closing parenthesis")
                        )
                /   number                                      ->PUSH $(number)$(:endl)  

        number  =   ([0-9]+  ('.' [0-9])?)

        add_op  =   '+'     ->EXEC ADD$(:endl)
                /   '-'     ->EXEC SUB$(:endl)

        mult_op =   '*'     ->EXEC MUL$(:endl)
                /   '/'     ->EXEC DIV$(:endl)

        pow_op  =   '^'     ->EXEC POW$(:endl)

        _       = ' '*
        "#,
    )
    .gen_rules()?
    .parse("-1+2*3")?
        //     .parse("2^3^4^5")?
        //     .parse("2-3-4-5")?
        //     .parse("-(-1+2* 3^5 ^(- 2 ) -7)+8")?
        //     .parse("-(1))")?
    .replace(None)?
    //  ...
    ;

    //     println!("{:#?}", result);
    println!("{}", result.str());
    Ok(())
}

//  -----------------------------------------------------------------------------------
//  -----------------------------------------------------------------------------------

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
