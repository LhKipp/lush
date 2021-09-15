# The Lu-Interpreter

Evaluation of lu-code happens in 4 steps:

* Step 1: Parsing
* Step 2: Resolution
* Step 3: Typechecking
* Step 4: Evaluation

In the last 3 steps correct handling of the scope is crucial.

## Scope
The Scope in lu-code has the following structure

```
GlobalFrame (Includes internal cmds, Environment variables from lu-startup)
|
Possible other globaly applying frames
|
|- SourceFileFrame 1: Includes Functions and global variables declared in this source file
|- ...
|- SourceFileFrame N
```

##  Step 1: Parsing
(This step involves lexing, but it is currently all handled by the lexer himself...)
Parsing takes in source code and produces a CST, on which an AST is layered. It is always possible to retrive the CST from the AST.

## Step 2: Resolution
Resolution takes in an AST source_file and a scope.
