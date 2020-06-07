# dpr

Evolution of... [dynparser](https://github.com/jleahred/dynparser)

- [repository](https://github.com/jleahred/dpr)
- [doc](https://docs.rs/dpr/)
- [rust-crate](https://crates.io/crates/dpr)


Basic execution flow

```txt
  Text -> Parsing -> Transform -> Text
```

More info about the `peg` syntax bellow.

## Usage

Add to `cargo.toml`

```toml
[dependencies]
dpr = "0.1.0"
# dpr = {git = "https://github.com/jleahred/dpr" }
```

Wach examples below

## Modifications

```txt
    0.1.0 First version
```

## TODO

* Adding external functions
* don't needed to be multiexpr
    pub(crate)struct Transf2Expr {
        pub(crate)mexpr: MultiExpr,
* remove and or multiexpr when only one option  (and/or)

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

Addition calculator example

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
| `&`        | verify it follows..., but not consuming                               |
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

The arrow will work with current line. If you need to use trasnsformations
over some lines, you will have to use `(...)`

There is a grammar to parse the peg grammars that could be an example on file `gcode/peg2code.rs`

After the arrow, you will have the transformation rule.

`Replacing tokens`:
Things inside `$(...)` will be replaced.
Text outside it, will be written as it

`Replacing tokens` can refer to parsed text by name or by position.

```text
           -> $(num)
```

This will look for a name called `num` defined on left side to write it on output

Next line will also look for names, but on `rep_symbol` will not complain it it doesn't exists

```txt
    rep_or_unary    =   atom_or_par  rep_symbol?    -> $(?rep_symbol)$(atom_or_par)
```

You can also refer an element by position

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

You can define your own `functions` (aka `external functions`)

In next example we created the replacement token `el`

```cpp
fn main() -> Result<(), dpr::Error> {
    let result = dpr::Peg::new(
        "
        main    =   char+
        char    =   'a'     -> $(:el)A
                /   'b'     -> $(:el)B
                /   ch:.    -> $(:el)$(ch)
    ",
    )
    .gen_rules()?
    .parse("aaacbbabdef")?
    .replace(Some(&dpr::FnCallBack(custom_funtions)))?
    //  ...
    ;

    println!("{:#?}", result);
    println!("{}", result.str());
    Ok(())
}

fn custom_funtions(fn_txt: &str) -> Option<String> {
    match fn_txt {
        "el" => Some("\n".to_string()),
        _ => None,
    }
}
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

        subexpr =   '(' _ expr _                    ->$(expr)
                                (  ')'              ->$(:none)
                                /  error("parenthesis error")
                        )
                /   number                        ->PUSH $(number)$(:endl)
                /   '-' _ subexpr                 ->PUSH 0$(:endl)$(subexpr)SUB$(:endl)

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

    main            =   grammar                                     -> $(grammar)EOP
    grammar         =   rule+

    symbol          =   [_a-zA-Z0-9] [_'"a-zA-Z0-9]*

    rule            =   _  rule_name  _  '='  _  expr  _eol _       -> RULE$(:endl)$(rule_name)$(:endl)$(expr)

    rule_name       =   symbol

    expr            =   or                              -> OR$(:endl)$(or)CLOSE_MEXPR$(:endl)

    or              =   _  and                          -> AND$(:endl)$(and)CLOSE_MEXPR$(:endl)
                        ( _  '/'  _  or )?              -> $(or)

    and             =   error
                    /   (andline  transf2   and:(
                            _                ->$(:none)
                            !(rule_name   _  ('=' / '{'))   and )?)             -> TRANSF2$(:endl)$(transf2)EOTRANSF2$(:endl)AND$(:endl)$(andline)CLOSE_MEXPR$(:endl)$(and)
                    /   andline     (  
                                       ( ' ' / comment )*   eol+   _            -> $(:none)
                                       !( rule_name _   ('=' / '{') )   and 
                                    )?

    error           =   'error' _  '('  _  literal  _  ')'      -> ERROR$(:endl)$(literal)$(:endl)


    andline         =   andchunk  (     
                                        ' '+  ->$(:none)
                                        ( error / andchunk )
                                  )*

    andchunk        =   name   e:rep_or_unary                 -> NAMED$(:endl)$(name)$(:endl)$(e)
                    /            rep_or_unary
                        

    //  this is the and separator
    _1              =   ' ' / eol                   -> $(:none)

    //  repetitions or unary operator
    rep_or_unary    =   atom_or_par  rep_symbol?    -> $(?rep_symbol)$(atom_or_par)
                    //   atom_or_par                -> $(atom_or_par)
                    /   '!' atom_or_par             -> NEGATE$(:endl)$(atom_or_par)
                    /   '&' atom_or_par             -> PEEK$(:endl)$(atom_or_par)

    rep_symbol      =   '*'     -> REPEAT$(:endl)0$(:endl)inf$(:endl)
                    /   '+'     -> REPEAT$(:endl)1$(:endl)inf$(:endl)
                    /   '?'     -> REPEAT$(:endl)0$(:endl)1$(:endl)

    atom_or_par     =   atom / parenth

    parenth         =   '('  _  expr  _                 -> $(expr)
                                         (  ')'         -> $(:none)
                                         /  error("unbalanced parethesis: missing ')'")
                                         )

    atom            =   a:literal             -> ATOM$(:endl)LIT$(:endl)$(a)$(:endl)
                    /   a:match               -> MATCH$(:endl)$(a)
                    /   a:rule_name           -> ATOM$(:endl)RULREF$(:endl)$(a)$(:endl)
                    /     dot                 -> ATOM$(:endl)DOT$(:endl)
                                    //  as rule_name can start with a '.', dot has to be after rule_name

    literal         =  lit_noesc  /  lit_esc

    lit_noesc       =  _'   l:(  !_' .  )*   _'        -> $(l)

    _'              =   "'"

    lit_esc         =   (_"
                            l:(   esc_char
                              /   hex_char
                              /   !_" .
                              )*
                        _")                             -> $(l)

    _"              =   '"'

    esc_char        =   '\r'
                    /   '\n'
                    /   '\t'
                    /   '\\'
                    /   '\\"'

    hex_char        =   '\0x' [0-9A-F] [0-9A-F]

    eol             =   "\r\n"  /  "\n"  /  "\r"
    _eol            =   (' ' / comment)*  eol

    match           =   '['     -> $(:none)
                            (
                                mchars  b:(mbetween*)       -> CHARS$(:endl)$(mchars)$(:endl)BETW$(:endl)$(b)EOBETW$(:endl)
                                / b:(mbetween+)             -> BETW$(:endl)$(b)EOBETW$(:endl)
                            )
                        ']'                -> $(:none)

    mchars          =   (!']' !(. '-') .)+

    mbetween        =   f:.  '-'  s:.                 -> $(f)$(:endl)$(s)$(:endl)

    dot             =   '.'

    _               =   (
                            (  ' '
                            /   eol
                            /   comment
                            )*
                        )                                  -> $(:none)

    comment         =   (   line_comment
                        /   mline_comment
                        )                                  -> $(:none)

    line_comment    =   '//' (!eol .)*

    mline_comment   =   '/*' (!'*/' .)* '*/'

    name            =   symbol ":"                         -> $(symbol)

    transf2         =   _1 _  '->'  ' '*    -> $(:none)
                        transf_rule         -> $(transf_rule)
                        &eol

    transf_rule     =   ( tmpl_text  /  tmpl_rule )+

    tmpl_text       =   t:( (!("$(" / eol) .)+ )                -> TEXT$(:endl)$(t)$(:endl)

    tmpl_rule       =   "$("          -> $(:none)
                            (
                                    //  by name optional
                                  '?'  symbol                   ->NAMED_OPT$(:endl)$(symbol)$(:endl)
                                    //  by name
                                /  symbol                       ->NAMED$(:endl)$(symbol)$(:endl)
                                    //  by pos
                                /   "."  pos:([0-9]+)           ->POS$(:endl)$(symbol)$(pos)$(:endl)
                                    //  by function
                                /   ":"  ->$(:none)
                                      fn:((!(")" / eol) .)+)    ->FUNCT$(:endl)$(fn)$(:endl)
                              )
                        ")"                                     ->$(:none)
    "#
}
```

## Hacking the code


As you can see, the code to start parsing the `peg` input, is written in a text `peg` file

How is it possible?

At the moment, the `rules_for_peg`code is...

```rust
    r#"symbol"# => or!(and!(ematch!(chlist r#"_"#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), rep!(ematch!(chlist r#"_'""#  , from 'a', to 'z' , from 'A', to 'Z' , from '0', to '9' ), 0)))
    , r#"transf_rule"# => or!(and!(rep!(or!(and!(ref_rule!(r#"tmpl_text"#)), and!(ref_rule!(r#"tmpl_rule"#))), 1)))
    , r#"mbetween"# => or!(and!(transf2!( and!( and!(named!("f", dot!()), lit!("-"), named!("s", dot!())) ) , t2rules!(t2_byname!("f"), t2_funct!("endl"), t2_byname!("s"), t2_funct!("endl"), ) )))
    , r#"line_comment"# => or!(and!(lit!("//"), rep!(or!(and!(not!(ref_rule!(r#"eol"#)), dot!())), 0)))
    , r#"lit_noesc"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_'"#), named!("l", rep!(or!(and!(not!(ref_rule!(r#"_'"#)), dot!())), 0)), ref_rule!(r#"_'"#)) ) , t2rules!(t2_byname!("l"), ) )))
    , r#"error"# => or!(and!(transf2!( and!( and!(lit!("error"), ref_rule!(r#"_"#), lit!("("), ref_rule!(r#"_"#), ref_rule!(r#"literal"#), ref_rule!(r#"_"#), lit!(")")) ) , t2rules!(t2_text!("ERROR"), t2_funct!("endl"), t2_byname!("literal"), t2_funct!("endl"), ) )))
    , r#"atom_or_par"# => or!(and!(ref_rule!(r#"atom"#)), and!(ref_rule!(r#"parenth"#)))
    , r#"tmpl_text"# => or!(and!(transf2!( and!( and!(named!("t", or!(and!(rep!(or!(and!(not!(or!(and!(lit!("$(")), and!(ref_rule!(r#"eol"#)))), dot!())), 1))))) ) , t2rules!(t2_text!("TEXT"), t2_funct!("endl"), t2_byname!("t"), t2_funct!("endl"), ) )))
    , r#"atom"# => or!(and!(transf2!( and!( and!(named!("a", ref_rule!(r#"literal"#))) ) , t2rules!(t2_text!("ATOM"), t2_funct!("endl"), t2_text!("LIT"), t2_funct!("endl"), t2_byname!("a"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(named!("a", ref_rule!(r#"match"#))) ) , t2rules!(t2_text!("MATCH"), t2_funct!("endl"), t2_byname!("a"), ) )), and!(transf2!( and!( and!(named!("a", ref_rule!(r#"rule_name"#))) ) , t2rules!(t2_text!("ATOM"), t2_funct!("endl"), t2_text!("RULREF"), t2_funct!("endl"), t2_byname!("a"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(ref_rule!(r#"dot"#)) ) , t2rules!(t2_text!("ATOM"), t2_funct!("endl"), t2_text!("DOT"), t2_funct!("endl"), ) )))
    , r#"grammar"# => or!(and!(rep!(ref_rule!(r#"rule"#), 1)))
    , r#"dot"# => or!(and!(lit!(".")))
    , r#"eol"# => or!(and!(lit!("\r\n")), and!(lit!("\n")), and!(lit!("\r")))
    , r#"expr"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"or"#)) ) , t2rules!(t2_text!("OR"), t2_funct!("endl"), t2_byname!("or"), t2_text!("CLOSE_MEXPR"), t2_funct!("endl"), ) )))
    , r#"name"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"symbol"#), lit!(":")) ) , t2rules!(t2_byname!("symbol"), ) )))
    , r#"literal"# => or!(and!(ref_rule!(r#"lit_noesc"#)), and!(ref_rule!(r#"lit_esc"#)))
    , r#"rule"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_"#), ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), lit!("="), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_eol"#), ref_rule!(r#"_"#)) ) , t2rules!(t2_text!("RULE"), t2_funct!("endl"), t2_byname!("rule_name"), t2_funct!("endl"), t2_byname!("expr"), ) )))
    , r#"andchunk"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"name"#), named!("e", ref_rule!(r#"rep_or_unary"#))) ) , t2rules!(t2_text!("NAMED"), t2_funct!("endl"), t2_byname!("name"), t2_funct!("endl"), t2_byname!("e"), ) )), and!(ref_rule!(r#"rep_or_unary"#)))
    , r#"_""# => or!(and!(lit!("\"")))
    , r#"mline_comment"# => or!(and!(lit!("/*"), rep!(or!(and!(not!(lit!("*/")), dot!())), 0), lit!("*/")))
    , r#"andline"# => or!(and!(ref_rule!(r#"andchunk"#), rep!(or!(and!(transf2!( and!( and!(rep!(lit!(" "), 1)) ) , t2rules!(t2_funct!("none"), ) ), or!(and!(ref_rule!(r#"error"#)), and!(ref_rule!(r#"andchunk"#))))), 0)))
    , r#"hex_char"# => or!(and!(lit!("\0x"), ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' ), ematch!(chlist r#""#  , from '0', to '9' , from 'A', to 'F' )))
    , r#"mchars"# => or!(and!(rep!(or!(and!(not!(lit!("]")), not!(or!(and!(dot!(), lit!("-")))), dot!())), 1)))
    , r#"transf2"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_1"#), ref_rule!(r#"_"#), lit!("->"), rep!(lit!(" "), 0)) ) , t2rules!(t2_funct!("none"), ) ), transf2!( and!( and!(ref_rule!(r#"transf_rule"#)) ) , t2rules!(t2_byname!("transf_rule"), ) ), peek!(ref_rule!(r#"eol"#))))
    , r#"_'"# => or!(and!(lit!("'")))
    , r#"and"# => or!(and!(ref_rule!(r#"error"#)), and!(transf2!( and!( and!(or!(and!(ref_rule!(r#"andline"#), ref_rule!(r#"transf2"#), named!("and", rep!(or!(and!(transf2!( and!( and!(ref_rule!(r#"_"#)) ) , t2rules!(t2_funct!("none"), ) ), not!(or!(and!(ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), or!(and!(lit!("=")), and!(lit!("{")))))), ref_rule!(r#"and"#))), 0, 1))))) ) , t2rules!(t2_text!("TRANSF2"), t2_funct!("endl"), t2_byname!("transf2"), t2_text!("EOTRANSF2"), t2_funct!("endl"), t2_text!("AND"), t2_funct!("endl"), t2_byname!("andline"), t2_text!("CLOSE_MEXPR"), t2_funct!("endl"), t2_byname!("and"), ) )), and!(ref_rule!(r#"andline"#), rep!(or!(and!(transf2!( and!( and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"comment"#))), 0), rep!(ref_rule!(r#"eol"#), 1), ref_rule!(r#"_"#)) ) , t2rules!(t2_funct!("none"), ) ), not!(or!(and!(ref_rule!(r#"rule_name"#), ref_rule!(r#"_"#), or!(and!(lit!("=")), and!(lit!("{")))))), ref_rule!(r#"and"#))), 0, 1)))
    , r#"_"# => or!(and!(transf2!( and!( and!(or!(and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"eol"#)), and!(ref_rule!(r#"comment"#))), 0)))) ) , t2rules!(t2_funct!("none"), ) )))
    , r#"or"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"_"#), ref_rule!(r#"and"#)) ) , t2rules!(t2_text!("AND"), t2_funct!("endl"), t2_byname!("and"), t2_text!("CLOSE_MEXPR"), t2_funct!("endl"), ) ), transf2!( and!( and!(rep!(or!(and!(ref_rule!(r#"_"#), lit!("/"), ref_rule!(r#"_"#), ref_rule!(r#"or"#))), 0, 1)) ) , t2rules!(t2_byname!("or"), ) )))
    , r#"_1"# => or!(and!(lit!(" ")), and!(transf2!( and!( and!(ref_rule!(r#"eol"#)) ) , t2rules!(t2_funct!("none"), ) )))
    , r#"lit_esc"# => or!(and!(transf2!( and!( and!(or!(and!(ref_rule!(r#"_""#), named!("l", rep!(or!(and!(ref_rule!(r#"esc_char"#)), and!(ref_rule!(r#"hex_char"#)), and!(not!(ref_rule!(r#"_""#)), dot!())), 0)), ref_rule!(r#"_""#)))) ) , t2rules!(t2_byname!("l"), ) )))
    , r#"rep_symbol"# => or!(and!(transf2!( and!( and!(lit!("*")) ) , t2rules!(t2_text!("REPEAT"), t2_funct!("endl"), t2_text!("0"), t2_funct!("endl"), t2_text!("inf"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!("+")) ) , t2rules!(t2_text!("REPEAT"), t2_funct!("endl"), t2_text!("1"), t2_funct!("endl"), t2_text!("inf"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!("?")) ) , t2rules!(t2_text!("REPEAT"), t2_funct!("endl"), t2_text!("0"), t2_funct!("endl"), t2_text!("1"), t2_funct!("endl"), ) )))
    , r#"parenth"# => or!(and!(transf2!( and!( and!(lit!("("), ref_rule!(r#"_"#), ref_rule!(r#"expr"#), ref_rule!(r#"_"#)) ) , t2rules!(t2_byname!("expr"), ) ), or!(and!(transf2!( and!( and!(lit!(")")) ) , t2rules!(t2_funct!("none"), ) )), and!(error!("unbalanced parethesis: missing ')'")))))
    , r#"comment"# => or!(and!(transf2!( and!( and!(or!(and!(ref_rule!(r#"line_comment"#)), and!(ref_rule!(r#"mline_comment"#)))) ) , t2rules!(t2_funct!("none"), ) )))
    , r#"_eol"# => or!(and!(rep!(or!(and!(lit!(" ")), and!(ref_rule!(r#"comment"#))), 0), ref_rule!(r#"eol"#)))
    , r#"esc_char"# => or!(and!(lit!("\r")), and!(lit!("\n")), and!(lit!("\t")), and!(lit!("\\")), and!(lit!("\\\"")))
    , r#"match"# => or!(and!(transf2!( and!( and!(lit!("[")) ) , t2rules!(t2_funct!("none"), ) ), or!(and!(transf2!( and!( and!(ref_rule!(r#"mchars"#), named!("b", or!(and!(rep!(ref_rule!(r#"mbetween"#), 0))))) ) , t2rules!(t2_text!("CHARS"), t2_funct!("endl"), t2_byname!("mchars"), t2_funct!("endl"), t2_text!("BETW"), t2_funct!("endl"), t2_byname!("b"), t2_text!("EOBETW"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(named!("b", or!(and!(rep!(ref_rule!(r#"mbetween"#), 1))))) ) , t2rules!(t2_text!("BETW"), t2_funct!("endl"), t2_byname!("b"), t2_text!("EOBETW"), t2_funct!("endl"), ) ))), transf2!( and!( and!(lit!("]")) ) , t2rules!(t2_funct!("none"), ) )))
    , r#"rep_or_unary"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"atom_or_par"#), rep!(ref_rule!(r#"rep_symbol"#), 0, 1)) ) , t2rules!(t2_byname_opt!("rep_symbol"), t2_byname!("atom_or_par"), ) )), and!(transf2!( and!( and!(lit!("!"), ref_rule!(r#"atom_or_par"#)) ) , t2rules!(t2_text!("NEGATE"), t2_funct!("endl"), t2_byname!("atom_or_par"), ) )), and!(transf2!( and!( and!(lit!("&"), ref_rule!(r#"atom_or_par"#)) ) , t2rules!(t2_text!("PEEK"), t2_funct!("endl"), t2_byname!("atom_or_par"), ) )))
    , r#"tmpl_rule"# => or!(and!(transf2!( and!( and!(lit!("$(")) ) , t2rules!(t2_funct!("none"), ) ), or!(and!(transf2!( and!( and!(lit!("?"), ref_rule!(r#"symbol"#)) ) , t2rules!(t2_text!("NAMED_OPT"), t2_funct!("endl"), t2_byname!("symbol"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(ref_rule!(r#"symbol"#)) ) , t2rules!(t2_text!("NAMED"), t2_funct!("endl"), t2_byname!("symbol"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!("."), named!("pos", or!(and!(rep!(ematch!(chlist r#""#  , from '0', to '9' ), 1))))) ) , t2rules!(t2_text!("POS"), t2_funct!("endl"), t2_byname!("symbol"), t2_byname!("pos"), t2_funct!("endl"), ) )), and!(transf2!( and!( and!(lit!(":")) ) , t2rules!(t2_funct!("none"), ) ), transf2!( and!( and!(named!("fn", or!(and!(rep!(or!(and!(not!(or!(and!(lit!(")")), and!(ref_rule!(r#"eol"#)))), dot!())), 1))))) ) , t2rules!(t2_text!("FUNCT"), t2_funct!("endl"), t2_byname!("fn"), t2_funct!("endl"), ) ))), transf2!( and!( and!(lit!(")")) ) , t2rules!(t2_funct!("none"), ) )))
    , r#"main"# => or!(and!(transf2!( and!( and!(ref_rule!(r#"grammar"#)) ) , t2rules!(t2_byname!("grammar"), t2_text!("EOP"), ) )))
    , r#"rule_name"# => or!(and!(ref_rule!(r#"symbol"#)))
    )
}
```

Writting it by hand, it's dificult.

Isn't this program desineg to receive a text `peg` grammar and an text input and produce a text output?


### IR

`IR` is from Intermediate Representation

Why???

Once we parse the input, we have an `AST`.
We could process the `AST` but...

The `AST` is strongly coupled to the grammar. Most of the times we modify the grammar, we will need to modify the code to process the `AST`.

Some times the grammar modification will be a syntax modif, or adding some feature that requiere some syntax modification, therefore a different `AST` but all, or almost all of the concepts remain the same.

Imagine if we wanted to add de function `sqrt` to the math expresion compiler. We will need to modify the rules generator in order to process the new `AST`

To decouple the `peg` grammar from parsing the `AST`, we will create the `IR` (Intermediate Representation)

How to get the `IR` will be defined in the own `peg` grammar as transformation rules.

An interpreter of the `IR` will produce the rules in memory. Later, we can generate de `rust` code from the rules produced, or we could have a specific interpreter to generate them, but it's nice to get it from rust data structures

To develop this feature... we need a parser, and a code generator... Hey!!! I do it. `dpr` does that!!!

How to generate the `IR`

```rust
  peg_grammar()
    .parse(peg_grammar())
    .gen_rules()
    .replace()
```

The `peg_grammar` will have in `transformation rules` the intructions to generate the `IR`

Thanks to the `IR` it's easy to modify this program, and we don't need to deal with the `AST` coupled to the `peg-grammar`


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
