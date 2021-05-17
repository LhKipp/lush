## Lexing

```nu
echo $it.a # Result echo <content of $it.a>
vs
echo $it . a # Result echo <content of $it> . a
```
We can't simply lex "." as a token, and only look on the token stream. This would handle both cases the same. We need to lex each whitespace seperated word by itself, and parse each token stream of each word independently (nushell is ws sensitive)

```nu
echo Today is $day. # Result Error Expected value path, but trailing . ??? DISCUSS THIS
```
This could be resolved with brace expansion 
```nu
echo Today is ${day}. # Result Today is <content of day>.
```

The token '/' has to be handled differently:
    normal => path seperator
    math mode => divide by
could be solved by making divide by '//'


There should be structs It is crucial that structs can perform 'from string' and 'to string'


Lets consider 2 different 'ls' outputs: 

ls -l -h
drwxrw-rw- 2 leo autologin 20K  7. Apr 2019  '2008-06-19 001'
-rwxrw-rw- 1 leo autologin 65K 12. Aug 2017   map-ghana6.jpg

ls -l
drwxrw-rw- 2 leo autologin 20480  7. Apr 2019  '2008-06-19 001'
-rwxrw-rw- 1 leo autologin 66031 12. Aug 2017   map-ghana6.jpg

Both should result in the following table:
perms | hard_links | owner | group | size | timestamp | name

We can see that size and timestamp has to be parsed differently depending on the context
Same is true for timestamp

structs need to support
from string (option<format>)

script writers should be able to map the output of ls in the following way:
```
def ls []{
    let output = (ls -l -h)
    parse(Parser, String)
    parse()
    parse<Table<PermissionsP hard_links owner group size timestamp name]>>(output)
}
```
commands should be able to be added via shared lib loaded dynamically

syntax for let
let identifier = math-expr | string | array | table | explicit-path | path-with-slash | command

Todo tuples? 
tuple = <type...>

Ideas for explicit path:
explicit-path = p : path-identifier
explicit-path = "/" | "./" 
