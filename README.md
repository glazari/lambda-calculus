# Lambda Calculus

This repo is my attempt to implement the lambda calculus in Rust.
Trying to follow the book "Types and Programming Languages" by Benjamin C. Pierce.


# Untyped Arithmetic Expressions


## Grammar

```

term ::= true
       | false 
       | if term then term else term 
       | 0
       | succ term
       | pred term
       | iszero term

```

example REPL output:

```
Lambda REPL
> if iszero succ 0 then true else false
If(IsZero(Succ(Zero)), True, False)
consts: {"true", "false", "0"}
size: 6
depth: 4

> pred pred succ 0
Pred(Pred(Succ(Zero)))
consts: {"0"}
size: 4
depth: 4

>
```

# Untyped Lambda Calculus

## Grammar

```
term ::= x
       | λx. term
       | \x. term
       | term term
```
