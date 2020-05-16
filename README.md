# dpr

Evolution of... [dynparser](https://github.com/jleahred/dynparser)

- [repository](https://github.com/jleahred/dpr)
- [doc](https://docs.rs/dpr/)
- [rust-crate](https://crates.io/crates/dpr)

```txt
  Text -> Parsing -> Transform -> Text
```

More info about the `peg` syntax bellow.

## Usage

Add to `cargo.toml`

```toml
[dependencies]
# dpr = "0.1.0" soon
dpr = {git = "https://github.com/jleahred/dpr" }
```

Wach examples below

## Modifications

```txt
    0.1.0 First version
```

## TODO

* Add IR
* remove unnecessary code
* document  ->
* add trasf2  ?
* don't neede to be multiexpr
    pub struct Transf2Expr {
        pub mexpr: MultiExpr,
* remove and or multiexpr when only one option  (and/or)
* Document readme
  * add IR
* Adding external functions
* Upload to `crates`

## About

Giveng a `peg` grammar extended, it will verify the input and can generate an output based on transformation rules

But let's see by examples

### Simple example

Starting with this `peg`

Peg:
```text
        main    =   char+
        char    =   'a'     -> A
                /   'b'     -> B
                /   .
```

Given this `input`

Input:
```text
    aaacbbabdef
```

We got as result:

Output:
```text
    AAAcBBABdef
```

Addition example

Peg:
```text
        main    =   expr

        expr    =   num:num                -> PUSH $(num)$(:endl)
                    (op:op  expr:expr)?    -> $(expr)EXEC $(op)$(:endl)

        op      =   '+'     -> ADD
                /   '-'     -> SUB

        num     =   [0-9]+  ('.' [0-9])?
```

Input:
```text
    1+2-3
```

Output:
```text
    PUSH 1
    PUSH 2
    PUSH 3
    EXEC SUB
    EXEC ADD
```

### Execution flow

Basic text trasnformation flow.


```text

   DSL flow


 .--------.
 |  peg   |
 |  user  |
 '--------'
      |
      v
 .--------.
 |  GEN   |
 | rules  |
 '--------'
      |                .----------.
      |                |  input   |
      |                |   user   |
      |                '----------'
      |                      |
      |                      v
      |                .----------.
      |                |  parse   |
      '--------------->|          |
                       '----------'
                             |
                             v
                        .---------.
                        | replace |
                        |         |
                        '---------'
                             |
                             v
                        .--------.
                        | OUTPUT |
                        |        |
                        '--------'


```

The `rust` code for first example...

```rust
extern crate dpr;

fn main() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        "
        main    =   char+
        char    =   'a'     -> A
                /   'b'     -> B
                /   .
    ",
    )
    .gen_rules()?
    .parse("aaacbbabdef")?
    .replace()?
    //  ...
    ;

    println!("{:#?}", result);
    Ok(())
}
```

### Let's see step by step

Creating rules...

```rust
extern crate dpr;

fn main() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        "
        main    =   char+
        char    =   'a'     -> A
                /   'b'     -> B
                /   .
    ",
    )
    .gen_rules()?
    // .parse("aaacbbabdef")?
    // .replace()?
    //  ...
    ;

    println!("{:#?}", result);
    Ok(())
}
```

Produce a set of rules like...

```text
SetOfRules(
    {
        "main": And(
            MultiExpr(
                [
                    Repeat(
                        RepInfo {
                            expression: RuleName(
                                "char",
                            ),
                            min: NRep(
                                1,
                            ),
                            max: None,
                        },
                    ),
                ],
            ),
        ),
        "char": Or(
            MultiExpr(
                [
                    And(
                        MultiExpr(
                            [
                                MetaExpr(
                                    Transf2(
                                        Transf2Expr {
                                            mexpr: MultiExpr(
                                                [
                                                    Simple(
                                                        Literal(
                                                            "a",
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            transf2_rules: "A",
                                        },
                                    ),
                                ),
                            ],
                        ),
                    ),
                    And(
                        MultiExpr(
                            [
                                MetaExpr(
                                    Transf2(
                                        Transf2Expr {
                                            mexpr: MultiExpr(
                                                [
                                                    Simple(
                                                        Literal(
                                                            "b",
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            transf2_rules: "B",
                                        },
                                    ),
                                ),
                            ],
                        ),
                    ),
                    And(
                        MultiExpr(
                            [
                                Simple(
                                    Dot,
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ),
    },
)
```

This set of rules will let us to `parse` and generate the `AST` for any `input`

Next step, `parsing` the `input` with generated `rules`...

Creating rules...
(With a simplified input in order to reduce the `output` size)

```rust
extern crate dpr;

fn main() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        "
        main    =   char+
        char    =   'a'     -> A
                /   'b'     -> B
                /   .
    ",
    )
    .gen_rules()?
    .parse("acb")?
    // .replace()?
    //  ...
    ;

    println!("{:#?}", result);
    Ok(())
}
```

Now you can see de produced `AST`

```text
Rule(
    (
        "main",
        [
            Rule(
                (
                    "char",
                    [
                        Transf2(
                            (
                                "A",
                                [
                                    Val(
                                        "a",
                                    ),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
            Rule(
                (
                    "char",
                    [
                        Val(
                            "c",
                        ),
                    ],
                ),
            ),
            Rule(
                (
                    "char",
                    [
                        Transf2(
                            (
                                "B",
                                [
                                    Val(
                                        "b",
                                    ),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

And running the transformations...

```rust
extern crate dpr;

fn main() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        "
        main    =   char+
        char    =   'a'     -> A
                /   'b'     -> B
                /   .
    ",
    )
    .gen_rules()?
    .parse("acb")?
    .replace()?
    //  ...
    ;

    println!("{:#?}", result);
    Ok(())
}
```

```txt
"AcB"
```

## PEG rules grammar

You saw some examples, let see in detail

| token      | Description                                                           |
| ---------- | --------------------------------------------------------------------- |
| `=`        | On left, symbol, on right expresion defining symbol                   |
| `symbol`   | It's an string without quotes, no spaces, and ascii                   |
| `.`        | Any char                                                              |
| `"..."`    | Literal delimited by quotes                                           |
| `<space>`  | Separate tokens and Rule concatenation (`and` operation)              |
| `/`        | Or operation                                                          |
| `(...)`    | A expression composed of sub expresions                               |
| `?`        | One optional                                                          |
| `*`        | Repeat 0 or more                                                      |
| `+`        | Repeat 1 or more                                                      |
| `!`        | negate expression, continue if not followed without consume           |
| `[...]`    | Match chars. It's a list or ranges (or both)                          |
| `->`       | after the arrow, we have the transformation rule                      |
| `:`        | To give a name, in order to use later in transformation               |
| error(...) | This let's you to define an error message when this rule is satisfied |

Below there is the `grammar` witch define the valid `peg` inputs.
BTW, this `grammar` has been parsed to generate the code to parse itself ;-)

Let's see by example

### Rules by example

A simple literal string.

```peg
main = "Hello world"
```

Concatenation (and)

```peg
main = "Hello "  "world"
```

Referencing symbols

Symbol

```peg
main = hi
hi   = "Hello world"
```

Or conditions `/`

```peg
main = "hello" / "hi"
```

Or multiline

```peg
main
    = "hello"
    / "hi"
    / "hola"
```

Or multiline 2

```peg
main = "hello"
     / "hi"
     / "hola"
```

Or disorganized

```peg
main = "hello"
     / "hi" / "hola"
```

Parenthesis

```peg
main = ("hello" / "hi")  " world"
```

Just multiline

Multiline1

```peg
main
    = ("hello" / "hi")  " world"
```

Multiline2

```peg
main
    = ("hello" / "hi")
    " world"
```

Multiline3

```peg
main = ("hello" / "hi")
     " world"
```

It is recomended to use or operator `/` on each new line and `=` on first line, like

Multiline organized

```peg
main = ("hello" / "hi")  " world"
     / "bye"
```

One optional

```peg
main = ("hello" / "hi")  " world"?
```

Repetitions

```peg
main         = one_or_more_a / zero_or_many_b
one_or_more  = "a"+
zero_or_many = "b"*
```

Negation will not move current possition

Next example will consume all chars till get an "a"

Negation

```peg
main = (!"a" .)* "a"
```

Consume till

```peg
comment = "//" (!"\n" .)*
        / "/*" (!"*/" .)* "*/"
```

Match a set of chars.
Chars can be defined by range.

```peg
number  = digit+ ("." digit+)?
digit   = [0-9]
a_or_b  = [ab]
id      = [_a-zA-Z][_a-zA-Z0-9]*

a_or_b_or_digit  = [ab0-9]
```

Simple recursion

one or more "a" recursive

```peg
as  = "a" as
    / "a"

//  simplified with `+`
ak = "a"+
```

Recursion to match parentheses

Recursion match par

```peg
match_par = "(" match_par ")"
          / "(" ")"
```

In order to produce custom errors, you have to use `error(...)` constructor

In next example, the system will complain with parenthesis error if they are unbalanced
```peg
    parenth         =   '('  _  expr  _  (  ')'
                                         /  error("unbalanced parethesis: missing ')'")
                                         )
```

As you can see, if you can run the rule to close properly the parenthesis, everything is OK, in other case, custom error message will be produced

### Replacing

You can set the replace rules with `->`

```text
        op      =   '+'     -> ADD
                /   '-'     -> SUB
```

When `+` will be found and validated, it will be replaced by `ADD`

```text
        expr    =   num:num                -> PUSH $(num)$(:endl)
                    (op:op  expr:expr)?    -> $(expr)EXEC $(op)$(:endl)
```

To refer to parsed chunk, you can name it using `:`

When refering to a `symbol`, you don't need to give a name

Next examples, are equivalent

```text
        expr    =   num:num                -> PUSH $(num)$(:endl)
                    (op:op  expr:expr)?    -> $(expr)EXEC $(op)$(:endl)
```

```text
        expr    =   num            -> PUSH $(num)$(:endl)
                    (op  expr)?    -> $(expr)EXEC $(op)$(:endl)
```

After the arrow, you will have the transformation rule.

`Replacing tokens`:
Things inside `$(...)` will be replaced.
Text outside it, will be written as it

`Replacing tokens` can refer to parsed text by name or by position.

```text
           -> $(num)
```

This will look for a name called `num`defined on left side to write it on output

```text
           -> $(.1)
```

You can also refer to `functions` starting the `replacing token` with `:`

```text
        expr    =   num            -> $(:endl)
```

Predefined functions are...

(Watch on `replace.rs` to see full replace functions)
```rust
        "endl" => "\n",
        "spc" => " ",
        "_" => " ",
        "tab" => "\t",
        "(" => "\t",
        // "now" => "pending",
        _ => "?unknown_fn?",
```


Example

```text
        expr    =   num            -> PUSH $(num)$(:endl)
                    (op  expr)?    -> $(.2)EXEC $(.1)$(:endl)
```

## Full math expresion compiler example

What is a parser without an math expresion calculator?

Obiously, it's necessary to consider the operator priority, operator asociativity and parenthesis, and negative numbers and negative expresions

```rust
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
                            _  pow_op   _  pow  ->$(pow)$(pow_op)
                            )*

        subexpr =   '(' _ expr _ ')'              ->$(expr)
                /   number                        ->PUSH $(number)$(:endl)
                /   '-' _ subexpr                 ->PUSH 0$(:endl)$(subexpr)SUB$(:endl)
                /   '(' _ expr _      error("parenthesis unbalanced")
                /       _ expr _ ')'  error("parenthesis unbalanced")

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
    .parse("-(-1+2* 3^5 ^(- 2 ) -7)+8")?
    .replace()?
    //  ...
    ;

    println!("{:#?}", result);
    println!("{}", result.str());
    Ok(())
}
```

The output is a program for a stack machine, composed of a command with a parameter...

```text
PUSH 0
PUSH 0
PUSH 1
EXEC SUB
PUSH 2
PUSH 3
PUSH 5
PUSH 0
PUSH 2
EXEC SUB
EXEC POW
EXEC POW
EXEC MUL
EXEC ADD
PUSH 7
EXEC SUB
EXEC SUB
PUSH 8
EXEC ADD
```

## Full peg grammar doc spec

At the moment it's...

(for an updated reference, open peg2code.rs file :-)

```txt
fn text_peg2code() -> &'static str {
    r#"
    /*      A peg grammar to parse peg grammars
     *
     */

    main            =   grammar

    grammar         =   rule+

    symbol          =   [_a-zA-Z0-9] [_'"a-zA-Z0-9]*

    rule            =   _  rule_name  _  '='  _  expr  _eol _
    rule_name       =   '.'?  symbol  ('.' symbol)*

    expr            =   or

    or              =   and     ( _  '/'  _  or )?
    error           =   'error' _  '('  _  literal  _  ')'

    and             =   error
                    /   named? rep_or_neg  transf2?  ( _1 _  !(rule_name _ ('=' / '{'))  and )?
    _1              =   (' ' / eol)     //  this is the and separator

    rep_or_neg      =   atom_or_par ('*' / '+' / '?')?
                    /   '!' atom_or_par

    atom_or_par     =   (atom / parenth)

    parenth         =   '('  _  expr  _  (  ')'
                                         /  error("unbalanced parethesis: missing ')'")
                                         )

    atom            =   literal
                    /   match
                    /   rule_name
                    /   dot             //  as rule_name can start with a '.', dot has to be after rule_name

    literal         =  lit_noesc  /  lit_esc

    lit_noesc       =   _'   (  !_' .  )*   _'
    _'              =   "'"

    lit_esc         =   _"
                            (   esc_char
                            /   hex_char
                            /   !_" .
                            )*
                        _"
    _"              =   '"'

    esc_char        =   '\r'
                    /   '\n'
                    /   '\t'
                    /   '\\'
                    /   '\"'

    hex_char        =   '\0x' [0-9A-F] [0-9A-F]

    eol             =   ("\r\n"  /  "\n"  /  "\r")
    _eol            =   (' ' / comment)*  eol

    match           =   '['
                            (
                                (mchars  mbetween*)
                                / mbetween+
                            )
                        ']'

    mchars          =   (!']' !(. '-') .)+
    mbetween        =   (.  '-'  .)

    dot             =   '.'

    _               =   (  ' '
                        /   eol
                        /   comment
                        )*

    comment         =   line_comment
                    /   mline_comment

    line_comment    =   '//' (!eol .)*
    mline_comment   =   '/*' (!'*/' .)* '*/'

    named           =   symbol ":"

    transf2         =   _1 _  '->'  ' '*  transf_rule   eol
    transf_rule     =   ( tmpl_text  /  tmpl_rule )*
    tmpl_text       =   (!("$(" / eol) .)+
    tmpl_rule       =   _:"$("          //  _:  trick to avoid compactation
                            (
                                symbol                      //  template by name
                                /   "."  [0-9]+             //  by pos
                                /   ":"  (!(")" / eol) .)+  //  by function
                            )
                        _:")"
    "#
}
```

## Hacking the code

As you can see, the code to start parsing the `peg` input, is written in a text `peg` file

How is it possible?

At the moment, the `peg parser`code is...

```txt
pub(crate) fn parse_peg() -> parser::expression::SetOfRules {
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
```

Writting it by hand, it's dificult.

Then, we have a `peg` file defining the `peg` grammar accepted by this program. And we have a function to generate the `rust` code.

Isn't this program desineg to receive a text `peg` grammar and an text input and produce a text output?


### IR

`IR` is from Intermediate Representation

Why???

Once we parse the input, we have an `AST`.
We could process the `AST` but...

The `AST` is strongly coupled to the grammar. Most of the times we modify the grammar, we will need to modify the code to process the `AST`.

Some times the grammar modification will be a syntax modif, or adding some feature that requiere some syntax modification, therefore a different `AST` but all, or almost all of the concepts remain the same.

Imagine if we wont to add de function `sqrt` to the math expresion compiler. We will need to modify the rules generator in order to process the new `AST`

To decouple the `peg` grammar from parsing the `AST`, we will create the `IR` (Intermediate Representation)

The `IR` how to get the `IR` will be defined in the own `peg` grammar as transformation rules.

An interpreter of the `IR` will produce the rules in memory. Later, we can generate de `rust` code from the rules produced, or we could have a specific interpreter to generate them.

To develop this feature... we need a parser, and a code generator... Hey!!! I do it. `dpr` do that!!!

How to generate the `IR`

```txt
  peg_grammar()
    .parse(peg_grammar())
    .gen_rules()
    .replace()
```

The `peg_grammar` will have in `transformation rules` the intructions to generate the `IR`
