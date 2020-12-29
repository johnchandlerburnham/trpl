---
title: "Notes (TRPL 05/21): Using Structs to Struture Related Data"
---

# 5 Using Structs to Structure Related Data

see `src/main.rs`

## 5.1 Defining and Instantiating Structs

The text is right, field init shorthand is convenient.

### Tuple Structs without Named Fields to Create Different Types

Okay so tuple structs give us labeled product types, do we have sum types?

### Ownership of Struct Data

Very curious to see what all these references to *lifetimes* are about.

## 5.2 An Example Program Using Structs

see `src/main2.rs`

BTW, to easily use `cargo run` to build an run multiple binaries, add the
following section to `Cargo.toml`:

```toml
[[bin]]
name = "main"
path = "src/main.rs"

[[bin]]
name = "main2"
path = "src/main2.rs"
```

I wonder why the third area function uses a `&Rectangle` argument instead of
just a `Rectangle`.

I really like that we have separate traits for `Debug` and `Display`, in Haskell
I feel like it's easy to overload `Show`.

## 5.3 Method Syntax

Methods are just functions that live inside structs. Or in other words they're
functions associated with Struct S with the type signature `method :: S -> a` or
`fn method(&self) -> <T>`.

The distinction between a function from a struct versus a method of that struct
is that the method can be used with *method syntax*.

Okay, so my question as to why we're borrowing with `&Rectangle` is because of
ownership. If we did:

```rust
fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };

  let foo = take_ownership(rect1);

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );
}

fn take_ownership(rect: Rectangle) -> () {
  return ()
 }
```

We get a compiler error, because `rect1` gets moved into `take_ownership` and
thus it gets freed when `take_ownership` returns.

So I guess methods are broader than just functions with type `method :: S -> a`,
because they can also side effect on what values are in scope afterwards.

### Associated Functions

Okay, so the `::` operator is how we get some namespace management. I wonder
what this implies about the relationship between `struct`s and modules though.

