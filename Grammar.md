whitespace = ' ' | '\n'
root =  statements
statements = statement % terminator
terminator = NEWLINE 

statement = module_import | variable_def | func_def | pipeline

module_import = import <file_path?>
variable_def = 'let' var_ident = value
func_def = func_modifier 'fn' (func_ident % whitespace) signature block
func_modifier = public? pure?
block = '{' statements '}'

signature = '(' ((param_name | flag) type_ident) % ',' ')' 
flag = flag_name '/' flag_shorthand_name
flag_name = --[a-zA-Z_]+[a-zA-Z_0-9]
flag_shorthand_name = /-[a-zA-Z0-9]
type_ident = ':' type_name '?'?
type_name= string | bool | path | table | number | range | function | any

pipeline = command % '|'
command = ((value_expr | if | for | while) % whitespace)

value_expr = number 
    | boolean
    | value_path
    | string 
    | table 
    | array
    | function 
    | bare_word 
    | math_expr 

if = 
    'if' value_expr_convertible_to_bool '{'
        statements
    '}' (elif | else if convertible_to_bool '{'
        statements)*
    '}'
    (else '{' 
        statements)?
    '}'

for = 
    'for' var_ident in <value_expr which is iterable (table, array, string?)> '{'
        <ident> statements
    '}'

while =
    'while' value_expr_convertible_to_bool newline '{'
        statements
    '}'
