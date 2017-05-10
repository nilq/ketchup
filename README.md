# smoke
a programming language

## syntax

```
# v constant here
imm a = r"raw string\n"

fun fib(a)
  if a < 3
    return a
  return fib(a - 1) + fib(a - 2)
  
fun apply(a, f) -> return f(a)

# v weak non-constant 
mut foo?! = 'c' # char bb

foo = apply(10, fun(x) -> return 2 * x)

fib(foo)
```

sexy af syntax

```
mut a =
  1 + 100 * (.123) / 0xDEDEDE

# ... should get hex at some point ^

imm function =
  fun
    return r"lol this is totally a thing"

# '~' for doing '()'
print(function~) # stdout => 'lol this is totally a thing'
```
