# ketchup
a programming language without even the slightest implementation of any form of memory management.

## guide

comments
```
# this is a comment
# this is also a comment
# this is the only type of comment there is
# xd
```

literals
```
# string literals
r"raw string .. escapes are ignored"
"regular string"

# char literal
'a'
'\n'

# number stuff
1234
.123
-0.123
-321

# bool
yes # is hipster version of true
nah
```

data management
```
var foo = r"yes hello\n"
foo = 123 # dynamic!?

# nice identifiers
# regular assignments should literally be same as 'var' idk
hey?  = 'n'
hey!? = 'i'
hey!  = 'c'
_hey  = 'e'
_h3y? = "!?!1"
```

functions
```
# warning: recursion WILL cause overflow
fun fib(a)
  if a < 3
    return a
  
  # forcing copied memory?
  var a1 = a - 1
  var a2 = a - 2

  return fib(a1) + fib(a2)

putsln(fib(5)) # => 5
```

'~' and '()' is the same .. except for `return~` ...
```
fun test
putsln(test~) # => nil
putsln(test()) # => nil

fun test2 return~
putsln(test2~) # => nil

fun test3(a, b)
putsln(test3(1, 2)) # => nil

functional
```

functional
```
fun add(a, b)
  return a + b

fun sub(a, b) return a - b

fun mul(a, b)
  c = a * b
  return c

# higher order stuff
fun apply(f, a, b) return f(a, b)

putsln(apply(add, 1, 2)) # => 3
putsln(apply(sub, 1, 2)) # => -1
putsln(apply(mul, 1, 2)) # => 2
```

conditionals

```
if yes
  putsln("hey")
else
  angry("this will never happen") # angry *panics*

# will print
if "strings are truthy" puts r"so this will print" else angry("this won't happen")
```
