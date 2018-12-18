---
title: "Notes (TRPL 03/21): Common Programming Concepts"
---

# 3 Common Programming Concepts

### Keywords

I'm sorry, but the following just popped into my head as I was reading this:
["When the text hits your eye shaded green by vi, that's a
keyword"](https://www.youtube.com/watch?v=OnFlx2Lnr9Q).

Che schifo.

### Identifiers

Lol, `__` is a valid identifier:

```rust
fn __(x : u32) -> u32 { return x }
```

It's nice that we can break through the identifier restriction with `r#` though.

## Variables and Mutability

Very good to have to explicitly note where one wants a variable to be mutable.

## Differences Between Variables and Constants

The way I like to think about constants is that a constant is data that you
happen to be including in a source file, and is only ever going to be an input
value in your program. It's a strong commitment that a given name will always
have a certain value (within a specific scope). `const` values also cannot be
shadowed;  the compiler will complain if you do:

```rust

fn main() {
   const x: u32 = 3
   println!("The value of x is: {}", x);
   let x = 6;
   println!("The value of x is: {}", x);
}

```

But the compiler's tot totally fine if you do:

```rust

fn main() {
  let x = 5;
  println!("The value of x is: {}", x);
  let x = 6;
  println!("The value of x is: {}", x);
}
```

### Shadowing

I really like the restriction on  only being able to mutate values and not
types. That's very much like how `IORef` works in Haskell (I know, I know,
another Haskell comparison, but at least it's another positive one!)

## Data Types

Does this mean that the `.parse` is polymorphic in its return values? I wonder
if that's implemented with traits/typeclasses.

### Integer Overflow

I wonder why overflow is allowed in release builds at all if it's considered bad
when debugging. Furthermore, why should debug mode differ from release mode?
I guess that might be okay if debug mode is more strict than release mode, so
you have a guarantee that if it runs fine in debug it'll run fine in release.

### Numeric Operations

From Appendix B, it looks like a lot of these reserved operators are
overloadedable via typeclasses!


## Compound Types

These are just product types, yes? I guess arrays are actually dependently typed
vectors. Very neat that Rust has dependent types, even in this very minor way.

Actually looks like the compiler does error if you do e.g. `a[6]` on an `[T;
5]`, but not if you hide the index behind a layer of indirection. This is a
little odd, I wonder why that is? Is there a macro that'll automatically
concretize an index referred to by a value so I can get compile-time bounds
checking?

This naive approach didn't work:

```rust
macro_rules! id {
  ($e:ident) => ($e);
}

println!("id(index) is: {}", id!(index));
```

I guess the macro doesn't force the variable to resolve.

## Functions

Statements are like expressions of type `IO ()` in Haskell, they might do
something, but they never return anything other than unit.

Actually I think this is literally true in rust, since the following typechecks:

```rust
let x:() = {let y = 6;};
```

So the semicolon in Rust is a little like saying `return ()` in Haskell.

## Control Flow

`if` expressions are really just like functions of `IO Bool -> IO a -> IO a -> IO a`

like all these are valid `if` expressions:

```rust
let x = if {let x = false; x} { () } else { () };
let x = if {let x = true; x} { () };
let x = if {let x = true; x} { 3 } else { 4 };
let x = if {let x = false; x} {;} else {;};
```

### Repetition with Loops

`loop` is like `IO a -> IO a`

`while` is like `IO Bool -> IO a -> IO a`

I think the thing `for` loops are most like is list comprehensions, where
`.iter` is just doing a `toList` conversion.

## Exercises

1. Temperature conversion:

```rust
fn farenheit_to_celsius (x : f32) -> f32 {
    (x - 32.0) * 100.0/180.0
}
fn celsius_to_farenheit (x : f32) -> f32 {
    x * 180.0/100.0 + 32.0
```

2. Fibonacii:

```rust
fn fib (x : i64) -> i64 {
  match x {
    0 | 1 => 1,
    n if n < 0 => 0,
    _ => fib(x - 1) + fib (x - 2)
  }
}
```


