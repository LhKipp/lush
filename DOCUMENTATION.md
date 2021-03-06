# Basics

## Comments
Everything behind a `#` until the end of a line, is considered a comment
```lush
# This is a comment
echo here is code # This is a comment again
```

## Variables
Variables can be declared via the `let` statement.
```lush
let unassigned  # Variable without initial value
let x = 1       # Variable with intial value
```
Variables are typed in lush. If the type is not declared, it will be inferred based on the usage (see below).
```
let var1: num = 1 # var1 with an explicit type (number)
let var2
var2 = 2 # var2 inferred to be a number here
```
Type coherence is statically verified. Meaning: there won't be type errors during runtime (except typecasts).

## Types
`lush` is a typed shell. The following types exists:

* any - The variable can be of any type
* nil - The empty void type
* bool - Boolean, either `true` or `false`
* num - A number, e.G. 1, 0.5, -5
* str - A string, e.G. "Hello World"
* path - A file path (potentially containing wildcards), e.G. /home/sweet/*
* [type] - An array of "type", e.G. [num]
* type? - An `Optional` of "type", e.G. num?
* Structs - See below
* Functions - See below

`lush` supports type inference. Types do not have to be spelled out each and every time - they are mostly inferred due to the usage of variables, constants and commands.

The type of a value can be visualized by leveraging the `type_of` command. A value of type `any` can be casted at runtime to a different type by using the `as` operator.

### Type: Optional
An `Optional` is either `None` or `Some{<value>}`. Examples:
```lush
let nothing = None
let something = Some{1}
```

### Type: path
A path is a specifier containing slashes ('/'). It may contain wildcards (aka Pattern's).
```lush
let pics_of_my_dog = Pictures/holidays/*/sweet_dog.png
```
Currently only '*' and '**' is supported as patterns.

Wildcards are not expanded by default. They are passed to internal commands unmodified, while they are being expanded if used as an argument to an external (!) command.
```lush
fn ls_in_home_dir(sub_path: path)
    let ls_path = combine_paths /home/user/ $sub_path # The function 'combine_paths' does not yet exists
    ls $ls_path # Wildcards expanded before passed to external cmd "ls"
end
ls_in_home_dir Music                 # passed as 1 argument
ls_in_home_dir Music/**/*bob_marley* # passed as 1 argument
```
(This choice has been taken for better compatability with external cmds, while making internal commands more convenient. A call like "git_branch_wrapper --list ma*" would not require you to qoute "ma*", if git_branch_wrapper's --list flag takes a `path` argument.)

## Control structures
### if - ifopt - elif - elifopt - else
`if`, `elif` and `else` behave as usual.
`ifopt` stands for "if optional", while `elifopt` stands for "else if optional". They can be used to check whether an optional is `Some` and get the inner value at the same time.
```lush
let lush = 1
let world = Some{"world"}
if $lush < 3    # The following equality operators are supported: < <= == != => >
    echo hello
elifopt world_val = $world
    # The above line reads as:
    # else if the optional $world is Some, 
    # assign the contained value of $world to world_val and run this block.
    # the "counterpart" of elifopt is ifopt
    echo $world_val
elif 42 == 1337
    # Do nothing. Never true
else
    echo "!"
end
```

### for
Iteration over strings and arrays is possible.
```lush
for character in "abcde"
    # character is of type string
    echo $character
end

for elem in [1 2 3]
    # elem is of the arrays inner type (number here)
    echo $elem
end
```

## Command calls
A command (or function) can be called by typing its name and the arguments.
```lush
command_or_func_name $arg
```
If the commands name is not found, lush will try to start a process by that name
```lush
echo $arg # starting the 'echo' process with $arg as its first argument
```

For convenience: when passing simple-words to arguments of type `str`, they do not have to be quoted.
```lush
echo hello world "!" # Better quote operators. 
                     # They are not promoted to strings automatically.
```
## Pipes
Commands do not only receive arguments via arguments and flags, but also by what is "piped" into them.
```lush
echo "This value gets passed to cat" | cat
```

## Structs
Lush has c-style structs. Please note, that struct-names have to (!) start with an upper case letter.
```lush
struct Ip{ # Declaration
    a: num
    b: num
    c: num
    d: num
}
let x = Ip { a: 192 b: 0 c: 0 d: 1 }
echo $x.a
```

## Tables
An array of structs is visualized as a table. For example the `ls` command from `std:fs` outputs a table as:
```lush
use std:fs
ls
# Name     | Type      | Size 
# dir_a    | Directory | 40 
# file.txt | File      | 1 
# dir_b    | Directory | 40 
```

## Functions
A function can be declared via the `fn` keyword
```lush
fn my_first_fn
    echo "Hello"
end
```

### Return
Functions can return a value via the `ret` keyword.
```lush
fn func1 (ret: int)
    ret 1
end
fn func2 (arg: int)
    ret $arg # The return type of func2 is inferred to be int
end
```
If the return type of a function is not declared, it will be inferred based on the values returned.
<!-- However all `ret` statements within a function have to be type consistent. -->
<!-- ```lush -->
<!-- fn fn_with_type_error(arg: int) -->
<!--     if $arg < 1 -->
<!--         ret "Less than 1" # Type 'str' here -->
<!--     end --> 
<!--         ret $arg          # Type 'int' here -->
<!--     end -->
<!-- end -->
<!-- ``` -->

### Input
Values can be "piped" into a function. Those values can be handled via the special `in` argument.
```lush
fn take_num_ret_num(in: num ret: num)
    ret $in
end
1 | take_num_ret_num
```
`in` does not have to be declared. It will be automatically received.
```lush
fn take_num_ret_num
    ret $in
end
1 | take_num_ret_num
```

### Arguments
#### Required arguments
Functions can accept arguments by declaring them within a `signature`. Simple arguments are always required to pass.
```lush
fn fn_with_args (arg1: num arg2:str)
    echo $arg1 $arg2
end
```
#### Variable amount of arguments
A variable amount of arguments can be taken by declaring a `vararg` argument. They are declared by prepending "..." to the arguments name. It is possible for users to pass zero arguments, leaving the `vararg` an empty array.
```lush
fn fn_with_args (arg1: num ...rest: num)
    echo $arg1 
    for val in $rest
        echo $val
    end
end
fn_with_args 1     2 3 4            # Okay
             ^arg1 ^----rest
fn_with_args 1                      # Okay
             ^arg1 (rest left empty)
fn_with_args                        # Error. Required argument not passed
```
#### Optional arguments
Optional arguments (arguments which can be passed, but don't have to be) can be declared by appending a questionmark ('?') to the end of an arguments name. Users can either pass a value or not. 
If the user passes a value, the argument is `Some{<passed_value>}`, otherwise its `None`.
```lush
fn fn_with_opt_arg (arg?)
    ifopt val = $arg
        echo passed $val
    else
        echo passed nothing
    end
end
fn_with_opt_arg "value provided" 
fn_with_opt_arg
```

#### Default arguments
If no signature is declared a command will have an implicit var_arg argument named `args` of type `[any]`. The `ret` value and the `in` value will be inferred as usual.
```lush
fn passthrough
    echo $args
end
passthrough 1 2 3
```

### Flags
#### Optional flags
Flags can be declared by prepeding "--" to their name.
```lush
fn fn_with_flag( --flag1: num )
    ifopt val = $flag1
        echo $val
    end
end
fn_with_flag --flag 1
```
A flag can also be given a shorter name (one character name), or only a shortname
```lush
fn fn_with_flag( --flag1 -f: num )
    ifopt val = $flag1
        echo $val
    end
end
fn fn_with_short_flag( -f: num )
    ifopt val = $f
        echo $val
    end
end
fn_with_flag --flag 1
fn_with_short_flag -f 1
```
Flags are by default optional to pass. If a value is passed, its forwarded as `Some{$passed_value}`, otherwise the value of the flag is left as `None`. Optional flags are (much like optional arguments) of the `Optional` type.

#### Required flags
Flags can be made required by adding the `req` keyword.
```lush
fn fn_with_req_flag( req --flag1: num )
    echo $flag1 # $flag1 is of type num (not Optional)
end
```

#### Switches
If the type of a flag is not declared, it defaults to bool. Boolean flags are like switches, passing them assigns `true` to them, `false` otherwise.
```lush
fn fn_with_switch( --switch1 # Type of switch is bool
                   --switch2: bool ) 
    echo $switch1 $switch2
end
fn_with_switch --switch1 # prints true false
```

### Function overloading
Functions can be overloaded by their required flags

```lush
fn file(arg: str
        req --delete )
    rm $arg
end

fn file(arg: str 
        req --list )
    ls $arg 
end

file --delete file1 # Deletes file1
file --list dir1    # Lists the contents of dir1
```

### Function purity
Functions can be marked `impure`. Lets refine the last example:
```lush
impure fn file(f_to_del: str
        req --delete )
    rm $f_to_del
end
```
Running an `impure` function or command (might) change the state of the machine. When running such a command during a debug session, the debugger will print a warning, asking whether the command shall be executed or skipped. (See the debug chapter reference)
External commands are by default considered to be impure, unless their name appears in a list of well known pure external commands (e.G. "cat", "awk" ... see lush/crates/lu_cmds/src/external_cmds_attr.rs for a complete list).
User defined functions are neither considered to be pure nor impure. The debugger will step into them, but will check any command call for its purity before execution.

### Generic functions
Functions can have generic arguments. For example the `push` command from the `std:array` module could be visualized in `lush` code as follows:
```lush
fn push(array: [T] ...to_push: T)
    # Impl here ...
end
```
Generics provide type safety. For example: the declaration of `push` guarantees that the type of "to_push" is the same as the inner type of `array`, making both values applicable to each other.

The name of the generic type cannot be freely choosen. Only T0, T1 ... T9 and U0, U1 ... U9 are valid generic type names.

Generic functions are currently not first class functions. They can be only called, but not assigned to variables, passed as arguments or returned from functions.

### Functions as types
Functions are first-class citizens in lush. They can be assigned to variables, passed as arguments or returned from functions.
The type of a function is its signature. Let us consider an example from the "std:iter" module.
```lush
use std:iter
# In std:iter
# "filter" takes a function "filter_fn", which must return a bool 
# and take an argument of type "T"
# fn filter (in: [T] ret: [T] filter_fn: fn(ret: bool arg: T))
#     ...
# end

fn is_bigger_3(ret: bool arg: num) # is_bigger_3 has such an signature
end

[1 2 3] | filter $is_bigger_3
```
As seen, writing a function-type is similar to declaring a function. Only the function name is left out.

### A word about return values

While in traditional shells functions and processes return their results (mostly) via stdout, the same approach has not been choosen for lush-functions. However to best work with external processes, the following is done:

- The return value of an unknown command / external command is assumed to be the content written to its stdout file descriptor. Therefore this content is captured and returned as a `str`. If the return value of the command is not further handled, its printed to stdout.
- The return value of an known command / lush function is its `ret` value. Therefore no content written to its stdout is captured.

This design decision leads to some situations to be aware of

```lush
fn print_and_ret_num
    echo this text is always printed to the terminal
    ret 1
end
# print_and_ret_num will write to stdout.
let x = print_and_ret_num
# The output of the external command "echo" is captured and assigned to x
let y = echo this text does not appear on the stdout
# The output of the external command "echo" is captured, but not further handled.
# Therefore it is then printed to stdout.
echo this text will appear on the stdout

# The output of the external command "echo" is later piped to another command.
# Therefore it is not printed to the stdout by the lush-engine
echo this text gets piped to cat | cat
# Only the return value of "print_and_ret_num" gets piped to cat.
# "print_and_ret_num" will still print to the stdout.
print_and_ret_num | cat
```

## Modules
Lush has a module system. A module is a file from which functions and struct declarations will be exported. Modules can be brought into scope via a `use` directive.
There are 3 different sources of modules

- Standard library modules. Those modules start with "std". (See below)
- All directories under '/home/user-name/.config/lush/plugins' are assumed to be a module.
- Files relative to the evaluated file.

Examples:
```lush
use std:array
push [] 1 2 3 # Use push from std:array
```
```lush
# Lets assume there is a file
# /home/user-name/.config/lush/plugins/my_plugin/file1.lu
# with the content:
# fn greet
#     echo "Hello from my_plugin/file1.lu"
# end
use my_plugin:file1.lu
greet         # Use greet from file1.lu
```
```lush
# In ./file.lu:
# fn greet
#   echo "Hi from file.lush"
# end
use ./file.lush
greet         # Use greet from ./file.lu 
```
Please note:

- Each evaluated file includes relative to its own path. "use ./file.lu" from "./start_file.lu" will include a different file than "use ./file.lu" from "./dir/other_file.lu".
- "use relative_file" is interpreted as a module include from "/home/user-name/.config/lush/plugins/". Prepend a "./" to the file name to make it a relative module include.
- The `use` directive, does not evaluate anything. Files imported via `use` are not run. e.G.
```lush
# In ./greet.lu:
# echo hello
use ./greet.lush # Won't execute "echo hello"
```

## Math-Expressions
Operators in lush behave like they do in many popular programming languages (e.G. python). Math can be done directly and does not need special function calls / expressions. 

### Math-Operators
The following math-operators are currently supported
* "+"     : Addition
* "-"     : Subtraction
* "*"     : Multiplication
* "//"    : Division
* "<"     : Less than
* ">"     : Greater than
* ">="    : Greater or equal than
* "<="    : Less or equal than
* "=="    : Equality
* "!="    : Inequality
* "and"   : Logical and
* "or"    : Logical or
* "as"    : Typecast for any

### Examples with a word of caution
This has some advantages, but also opens up for some subtle surprises.

All arguments are evaluated before passing them. And so are math-expressions
```lush
echo 1 + 1 # Echo will receive one argument: 2
```
Command calls can be part of an math expression
```lush
fn ret_num
    ret 41
end
ret_num + 1 == 42 # true
```
Passing the result of an command as an argument to the next is possible, but be sure to quote correctly
```lush
fn take_str(arg1: str)
    ret $arg1
end
fn ret_str
    ret "hi"
end
# Wrong example:
take_str ret_str    # take_str takes a bareword "ret_str" as $arg1.
# Correct example:
take_str (ret_str)  # Argument (ret_str) is a command invocation and the result will be passed as $arg1
```

The ">" operator does not redirect
```lush
cmd_which_prints > /dev/null # The "bigger than" operator does not redirect stdout
```

### The `as` operator
`as` can be used to cast a variable of type `any` to another type. If the typecast fails, an error is thrown at runtime. Examples:
```lush
fn cmd_taking_anything(arg: any)
    let ty = type_of $arg
    if $ty == "num"
        echo passed argument has type number
    else
        echo passed argument has type $ty
    end
end

cmd_taking_anything 1         # passed argument has type number
cmd_taking_anything "hello"   # passed argument has type str
```

### Math-Expressions as functions
A Math-Expression which is used where a function-argument is expected, gets "converted" to a function. This allows for a terse syntax.
```lush
fn apply_pred(pred: fn(ret: bool arg1 arg2))
    ret pred 1 2
end
apply_pred $arg1 < $arg2    # $arg1 and $arg2 are the declared function arguments

# A more realistic example (std:iter, std:fs and select are mentioned below)
use std:iter
use std:fs
# Returns all file names which are less than 100kb in size.
ls | filter $arg.type == "File" | filter $arg.size < 100 | select name
```

##  Debugging
`lush` offers the ability to run the code in an simple REPL debugger. Try `lush --debug <file>` to try it out.
The debugger will warn and ask for confirmation before executing any possible impure commands. Therefore it is safe to try out scripts in the development phase. 

The following commands are understood by the debugger:

* help, h - show help
* step, s - step to the next evaluated statement
* next, n - step over to the next statement (not recursing into function calls)
* skip, sk, [Value] - Skip the next statement and continue as if the statement returned [Value]. Providing no value will return nil
* print, p, [...var_names] - Print variables specified by `...var_names`
* scope, sc - Print the current scope

Hitting ctrl-c will quit the debug session.

## The standard library
The standard library currently only consists of:

- `std:array`
    - Exported functions
        - `push`: fn push(ret: [T], to_append: [T], ...elems_to_push: T) 
            - Returns a new array which is the concatenation of `to_append` with `...elems_to_push`
- `std:iter`
    - Exported functions
        - `map`: fn map (in: [T] ret: [U] map_fn: fn(ret: U arg: T))
            - Applies `map_fn` to every element of `in`, collects the results in an array and returns it.
        - `filter`: fn filter (in: [T] ret: [T] filter_fn: fn(ret: bool arg: T))
            - Applies `filter_fn` to every element in `in` and only returns those elements for which `filter_fn` returns true
- `std:fs`
    - Exported functions
        - `ls`: fn ls (ret: [LsEntry] ...files: path)
            - Lists content in current directory

## Builtins
Builtins are commands that are always available. Currently the following builtins are present:
- `is_set`: fn is_set(ret: bool, to_check: str)
    - Returns whether a var with name `to_check` exists
- `type_of`: fn type_of(ret: str, val: any)
    - Returns the type of `val` as a string
- `select`: fn select(in: [<struct_type>] ret: [<struct_type>] ...column_names: str)
    - Select all columns with names `column_names` from the incoming table `in` (must be an array of structs) and return them as a new table. All `column_names` have to be either a string expression or a bareword expression.
    ```lush
    use std:fs
    ls | select name "size"
    
    let x = size
    ls | select name $x  # Wrong
    # $x is an variable expression, expected a string or bareword expression
    ```
