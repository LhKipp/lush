# Design
This documents aims to provide a basic overview of some features contained in lush.

## Settled / Wanted features
### Loops
There should be magic indexing by column name

```lush
# ls returns a frame of:
# [[access_rights hardlinks group owner size creation_date name]]
# Where and first return the input table.
for name, size in {ls | where {$group != user}| first 10} { #Okay, var identifier group is table col
    ...
}
# An array [zero one two] is a shorthand for: [[elem]; [zero] [one] [two]]
# Therefore $elem is a column name
for elem in {[zero one two] | where {$elem != one}| first 10} { #Okay, array
    ...
}
```


### Error handling
Good error handling plays a major role! The script should run, even with unhandled errors, but the shell should be able to hint for unhandled errors.

Errors are handled by error handlers. Every error handler has a scope.
```lush
$on_err = fn(e:Error) {echo "ERROR: " $e; exit} #Error handler for file

def my_err_handler {
 echo $e; exit # Default behaviour
}

def func {
    $on_err = my_err_handler # Set error handler for function
    statement.1 
    statement.2
    let count = if expr1 {
        $on_err = fn(e:Error) {echo $e; 0 #only echo error. Return 0 and keep going (TODO: This requires value of block and value of error handler to be the same! Dynamically verifiable?!)}
        statement.3.1
    } else {
        # Okay. No custom err handling. If statement.3.2 fails, it will call the
        # error handler of the outer block.
        statement.3.2
    }
    # TODO should the following statement be possible and equivialent to the one below?
    # This would be better syntax?
    statement.4 | on_err {...} | statement.5
    {$on_err = fn (e:Error){...}; statement.4} | statement.5
}
```
This is like try and catch but

Pros:
- It is way less noisy. Almost every command can fail for some reason in Shell scripts. Wrapping all commands always around try{}catch() increases indent and reduces readability
Cons: 
- ?

TODO: What should be handled by error handlers, what shouldn't?
Make a split like Java (checked unchecked)? Should wrong command args, flags be errors? Or should that always be failure?
e.G. is a error on the call side (passing wrong flag) the same as an error in the command (e.G. FileNotFound?)

### Dynamic analysis (original ? :) )
Checking types statically with tables and indexing/selecting of columns is "hard". Instead every command (should) provide a "check" function (if necessary) to specify what will be returned by the command.

The input of this "check" function is either the input types or the input values. (A little meta programming here)
```lush
# ls returns:
    Table(access_rights, hard_links, ...) 
# where returns input table
# select takes a table and returns the columns specified by $args.
# rm takes an array of type FileName
ls | where {$size > 10kb} | select name | rm # okay works. Can be dynamically verified
ls | where {$size > 10kb} | select size, name | rm # Table has more than 1 column. It is ambigous for rm what to do here. This doesnt work.

# select -v (inverse) takes any table (so input is correct) and returns the columns not specified by $args.
ls | select nam size | $input.0..10.name # Oops no 'nam' column. Error
```
Sometimes only the type can be provided, but not the value
ls | select $x $y $z

### Blocks have an implicit return value (rust)
```lush
let x = if {3 == 4} { 1 } else { 3 }
let x = {
    vec b
    b[0] = 0
    b[1] = 1
    b
}
```

### Arrays
An array is the same as an table with 2 columns (idx and elem). `[zero one two]` is a shorthand for
```lush
[[idx elem]; [0 zero] [1 one] [2 two]]
```
### Associative array / map / dictionary
A map is the same as an table with 1 row and many columns. `[name => Peter, age => 10]` is a shorthand for
```lush
[[name age] [Peter 10]]
```

## Features which need design
### Structs (Low prio)
Structs are the same as tables/maps.
```lush
let Ip = [[];] # Ip is struct / class table
Ip.new = fn (part1: Int, part2: Int, part3: Int, part4: Int) -> Ip_t {
	# A new object is simply a table with a specific layout
	#! A table row is an object; an object is a table row
	let new_obj = [[p1 p2 p3 p4]; [part1 part2 part3 part4]]
	# But this new object holds a reference to his Struct (Lua metatable concept)
	new_obj.set_metatable Ip # Lua metatable concept
	return new_obj
}

Ip.ping = fn (addr: Ip_t) { ... }
```
Reminder of Lua metatable concept: Whenever an object is indexed and it does not hold a value for the indexed value, the value of the metatable (with the object as first parameter for functions) is returned instead.

This allows for calling functions on "objects" via '.' syntax and of calling the functions standalone.
```lush
Ip.ping 127.0.0.1 # okay valid
let home = Ip.new 127 0 0 1
$home.ping # calls Ip.ping with $home as first parameter
```
Class functions can now also be passed 
```lush
[$home $home $home] | each Ip.ping
```

#### TODO How to use member function on next object
```lush Proposal???
Dir.ls -> select name -> map File.rm
```

# Thrown away ideas
<!-- ## The return value != what one sees -->
<!-- DataFrame { -->
<!--     columnFormats: [[col: ColumnName format: Formatter];] -->
<!--     values: [name: FilePath ...] -->
<!-- } -->

<!-- Any to_string cmd takes an optional formatter. -->
<!-- That way, the rowFormatter can overwrite formatting rules ... -->

<!-- Additionaly every row holds a pointer to the RowFormatter. -->
This is not important. Instead commands are always asked (where applicable) to return
everything and user shall pick what he wants to see.

<!-- ## Enums --> # Could potentially be a feature
<!-- A variable is always typed. A variable can never be nil! That means we need special constructs to represent optional + variant states -->

<!-- There should be an rust like enum type -->
<!-- ```lush -->
<!-- enum ReturnValue{ -->
<!--     date: Date, -->
<!--     int: Int, -->

<!--     fn holds_date -> bool { holds_type == Date } --> 
<!-- } -->
<!-- ``` -->
<!-- Enums are implemented like structs. There only clue: They only hold one of the data members. -->
<!-- They are transient by default. That means: Any operation is: --> 
<!-- - if possible done on the enum itself -->
<!-- - otherwise done on the contained value. -->
<!--     (This must recurse !) -->
<!-- That means in the example above: -->
<!-- ```lush -->
<!-- let date = OptionalDate::Date() -->
<!-- $date + 1d # Operator + called from Date -->
<!-- $date.holds_date() # calls the enum holds_date func -->
<!-- let date = OptionalDate::Nil() -->
<!-- ``` -->
<!-- This will lead to any operation done on an enum, potentially returning an error. -->
The feature is removed because of the above reason
