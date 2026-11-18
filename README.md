# The PJ Programming Language

> **Status:** an unfinished language-design experiment in Rust, paused in January 2024. The lexer and the expression
> parser exist, with tests; code generation was never started, and the last commits stop midway through a parser
> refactor, so the project doesn't build. The rest of this README is the language design as planned.
The compiler for the PJ Programming Language.

PJ Programming Language will be high-level general-purpose strongly typed programming language with very simple syntax.

## Basic Ideas

### Simple types:
- Int
- Float
- Bool

```
x1 = 42
x2 = Int(42)
x3 = x4 = 666

y1 = 0.69
y2 = Float(0.69)
y3 = 0.1, y4 = 0.2

z = True
```

### Type Aliases:
Type alias is a new type that behaves exactly like the aliased type.
However, these two types cannot be implicitly converted.
```
def Name = String

name = Name("PJ")

x: String = name         # Error, cannot implicitly convert Name to String.
x: String = String(name) # Ok.
```

### Sum Types:
Sum type is a type, such that its values are assigned to one of the type variants.
```
def Id(Int | String)

id1 = Id(420)   # id1 == Id(Int(420))
id2 = Id("abc") # id2 == Id(String("abc"))
```
Sum type can also use type aliases or even define them.
If the type has multiple parameters of the same type, it's necessary.
```
def Name(First = String | Last = String)

first_name = Name(First("Stephen"))
```

### Product Types:
Product type is the type commonly known as struct or class in programming languages.

```
def FullName(First = String, Last = String)

full_name = FullName(First("Ladislav"), Last("Vagner"))
```

### Functions:
Nothing too crazy here.
```
def f(n = Int) -> Int:
    return 2 * n
```
However, there are multiple ways to handle parameters.

```
def f(a <- Int, b <-> Int, c -> Int, d = Int, e := Int):
    ...
```
| Type  | Meaning                                              |
|-------|------------------------------------------------------|
| `<-`  | Moves the owned variable into the function.          |
| `<->` | References the variable.                             |
| `->`  | Initializes inside function and moves parameter out. |
| `=`   | Copies the parameter.                                |
| `:=`  | Copies the parameter as constant.                    |
> These are all very much a subject to change as I learn what works the best!

### Future plans:
- Pattern matching
- Some small standard library.