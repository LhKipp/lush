whitespace = ' '

root =  statements
statements = statement % terminator
terminator = "\n"

statement = 
    variable_def
    | func_def
    | pipeline
    | math expr

variable_def = 'let' var_ident = value

func_def = 
    'def' (func_ident % whitespace) signature block

pipeline = command % ('|' | '>>')

command = (func_ident % whitespace) (arg % whitespace)

arg = expression

value = number 
    | math_expr 
    | cmd
    | typed_bare_word 
    | string 
    | value_path
    | table 
    | array 
    | block

math_expr = bool_math_expr | number_math_expr
number/bool_math_expr = cmd (if command returns bool/number)

typed_bare_word = type_shorthand ':' arg

bool = true | false

if = 
    'if' convertible_to_bool newline
        <ident> statements
    (elif | else if convertible_to_bool
        <ident> statements)*
    (else
        <ident> statements)*

for = 
    'for' var_ident in iterable
        <ident> statements

while =
    'while' convertible_to_bool newline
        statements



convertible_to_bool = bool_math_expr | value_path

-- Copied from nu

signature: $ => seq(
    '[',
    repeat(
        seq(choice(
            $.parameter,
            $.flag,
            $.rest
        ),
            optional(',')),
    ),
    ']',
),
parameter: $ => seq(
    $.identifier,
    optional(seq(':', $.type)),
    optional('?'),
),
flag: $ => seq(
    $.flag_name,
    optional(
        seq('(', $.flag_shorthand_name ,')')
    ),
    optional(seq(':', $.type)),
),
flag_name: $ => /--[a-zA-Z_]+[a-zA-Z_0-9]*/,
flag_shorthand_name: $ => /-[a-zA-Z0-9]/,
rest: $ => seq(
    '...rest',
    optional(seq(':', $.type)),
),
type: $ => choice(
    'int',
    'string',
    'path',
    'table',
    'unit',
    'number',
    'pattern',
    'range',
    'block',
    'any',
),

