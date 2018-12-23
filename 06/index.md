---
title: "Notes (TRPL 06/21): Enums and Pattern Matching
---

# 6 Enums and Pattern Matching

Cool, so now we've got sum types.

## 6.1 Defining an Enum

see `enums/src/main.rs`

This section is showing us the pattern that:

```
(a || b) && c == a && c || b && c
```

Or in context:
```
(V4 || V6) && String == V4 && String || V6 && String
```

Or with numbers:

```
(1 + 1) * 1 = 1 * 1 + 1 * 1
```

Still, it's a very useful pattern!

### The Option Enum and Its Advantages Over Null Values

> This little piggy had roast beef,
> This little piggy had none,
>
> -*Mother Goose*

You know what my favorite thing about the `Option` type in Rust is? It's that
`Some` and `None` are both four letters long. This is also the case in OCaml and
it often makes the vertical alignment of code much prettier, than e.g. Haskell's
`Just` and `Nothing` type constructors for the `Maybe` type.

Here's a nice post describing the monadic structure of the `Option` type:
https://hoverbear.org/2014/08/12/option-monads-in-rust/

## 6.2 The match Control Flow Operator

see `enums/src/main2.rs`

Good old pattern matching. It's the best.


### Matches are Exhaustive

Compile-time totality checking is always helpful. I bet there's a compiler flag
somewhere to turn this off though.

### The `_` Placeholder

Ah `_`, my old friend. Man, for a systems language Rust really does use a lot of
recognizable functional idioms.

## 6.3 Concise Control Flow with `if let`

see `enums/src/main3.rs`

Interesting that `0u8` is `0 : u8`, that's a convenient shorthand syntax for
literals.




