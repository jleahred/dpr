// extern crate dpr;

// fn main() {
//     dpr::print_rules2parse_peg2();
// }

extern crate dpr;

fn main() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        r#"
        main    =   expr

        expr    =   term    (
                            _  add_op   _  term     ->$(term)$(add_op)
                            )*

        term    =   factor  (
                            _  mult_op  _  factor   ->$(factor)$(mult_op)
                            )*

        factor  =   pow     (
                            _  pow_op   _  subexpr  ->$(subexpr)$(pow_op)
                            )*

        pow     =   subexpr (
                            _  pow_op   _  pow      ->$(pow)$(pow_op)
                            )*

        subexpr =   (
                        '(' _ expr _ 
                         ( ')'
                         /  error("parenthesis unbalanced")
                         )
                    )                   ->$(expr)
                /   number                        ->PUSH $(number)$(:endl)
                /   '-' _ subexpr                 ->PUSH 0$(:endl)$(subexpr)EXEC SUB$(:endl)

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
    .parse("1+2*3")?
        //     .parse("2^3^4^5")?
        //     .parse("2-3-4-5")?
        //     .parse("-(-1+2* 3^5 ^(- 2 ) -7)+8")?
        //     .parse("-(1))")?
    .replace()?
    //  ...
    ;

    println!("{:#?}", result);
    println!("{}", result.str());
    Ok(())
}
