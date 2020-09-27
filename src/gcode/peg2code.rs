//! Here it's the peg grammar to parse the peg input  ;-P
//!
//! There is also a function to print the rust source code
//!
//! It's used in order to develop myself
//!
//! A parser for the parser (you know)
//!
//! To generate the code to parse the peg, you just have to run...
//!

pub(crate) fn text_peg2code() -> &'static str {
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
                                  '?'  symbol                   -> NAMED_OPT$(:endl)$(symbol)$(:endl)
                //  by name
                                /  symbol                       -> NAMED$(:endl)$(symbol)$(:endl)
                //  by pos
                                /   "."  pos:([0-9]+)           -> POS$(:endl)$(pos)$(:endl)
                //  by function
                                /   ":"  ->$(:none)
                                      fn:((!(")" / eol) .)+)    -> FUNCT$(:endl)$(fn)$(:endl)
                              )
                        ")"                                     -> $(:none)

    "#
}
