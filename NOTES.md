## DataTypes
Typical ones: Nil, Bool, Number, String, BareWord, Array, Function,

Object: like lua table.
{
    name = "hello"
    origin = "world"
}

Table: Array of Object

## Typing
let x : <Type> = val
<Type> can be ommited and will be infered in that case

let x : [str] = ["hello" "world"]
fn <modifiers> name '<' generics '>' <InType> -> Signature -> <RetType>

fn decl: fn'<' <InType> -> Signature -> <RetType> '>'

end


fn can_fail (line: num) -> str | nil
    let file_content = read /home/user/.my_config.txt | split "\n"
    if $file_content.len < $line
        return nil
    else
        return file_content[$line]
end

1 + $size ==> 
fn (arg) -> num
    1 + $arg.size


Select is to havy to type with generics. Instead we will fallback to a custom type func
fn select<T> [{$args : any, ..}] -> (...args: string) -> [{$args}]


fn git branch (arg: string
    --list: [string] -> [string] # list will list all branches
    --delete: string -> nil # deletes the branch with name $arg
on --list

signature git_branch_generic_opts = (
    --verbose
    --help
    --track -t
)

fn git branch --list -l (
    ...patterns: string
) -> [string]

fn git branch --delete -D (
    ...branches: string
    $git_branch_generic_opts
) -> nil

end



## Pipeline
lexer | parser | func_call_resolution | math_expr_to_fn_transformation | evaluation
## Closures
When returning a function (closure), the closure captures its environment by value (!) on the return statement!

This is similiar to how lua and python handle closures (and therefore best we can do)
```lua
function a()
    local x = 1
    function b()
        local y = 2
        function c()
            return y + x
        end
        y = 12
        return c -- y is now captured
    end
    x = 11
    return b -- x is now captured
end
-- Prints 13
print(a()()()) 
```

## Stdout and return
Programs are asked to return values, instead of printing them to stdout. The last value in a pipeline gets printed
```lu
get_x # The interpreter will print the return value of `get_x`
get_table | map $col_a + 1 # The interpreter will print the return value of map
```

Programs however are free to still print to stdout. Therefore it still has to be possible to redirect the output
```lu
get_x >$cmd_out 2>log.txt
```

This should work fine in praxis, as most functions will be either user facing functions or internal non printing funcs.

IDEA: Maybe have a tag `verbose` for functions who use stdout? So that they could be silenced alltogether?


## Dry run
- Functions error out when they try to do an unpure operation in an pure context
    - This is necessary as e.G. `git branch --list` is pure, but `git branch <arg>` is not. Whether a function is pure or not depends on the args/flags
- Marking a function as pure, means it can always be run in a pure context

- A dry run does not print out all commands being run (that would be a "verbose" run)
- Whether a function is pure or not depends on all args + flags and on the combination
- Dry run is a stupid idea

```lu
pure fn foo()
end

git branch 
```
 
## HOF Funcs
```lu
fn filter[pred: fn] # type of fn is deduced
    assert ($in | type) == "table"
    let result: table
    for row in $in
        if pred $row # must return something convertible to bool and take an argument of $arg
            result = result + row
        end
    end
    return result
end
```
While this syntax should in principle work, it would be simpler to have the bounds being declared in the signature


```lu
fn filter[pred: fn<($in._elem_type) -> bool>]
    # fn<in_type -> (args, flags) -> bool>
    assert ($in | type) == "table"
    let result: table
    for row in $in
        if pred $row
            result = result + row
        end
    end
    return result
end
```
```lu
fn filter[pred: fn<($in._elem_type) -> bool>]
    assert ($in | type) == "table"
    let result: table
    for row in $in
        if pred $row
            result = result + row
        end
    end
    return result
end
```

echo [[name, size]; [file1, 1kb]] | filter $size > 1kb

transformed into:

echo [[name, size]; [file1, 1kb]] | filter fn[arg: {name, size}] $arg.size > 1kb end



## Evaluation of partial programs


## Cmd overloading
fn git_branch( --list -l
            branchname: str
            start_point?: str )
end

fn git_branch ( --delete -D
                ...to_delete: str)
end

<!-- fn generic_opts git_branch (--verbose -v) -->
