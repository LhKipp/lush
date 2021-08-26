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
 
- Example editing of lu-program
```lu
fn filter[pred: fn]
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
