---
title: "Notes (TRPL 04/21): Ownership"
---

# 4 Ownership

## What is Ownership?

> 1. Every value in Rust has a variable that's called its owner.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be dropped.

### Variable Scope

Variable scope is managed with `{}` braces.

### Memory and Allocation

Oh interesting! Rust prevents double frees by ensuring that pointers are one to
one from references to heap data.

### Stack-Only Data

I wonder how the constraint that tuples composed of `Copy`-trait types have the
`Copy`-trait is implemented. Like is it done manually up to a certain arity like
Haskell with tuples and Functor? Or recursively somehow like Purescript?

### Return Values and Scope

There's a neat purity to how Rust handlies pointers and scope here.

## References and Borrowing

### Mutable References

Interested in better understanding what lifetimes do.

### The Rules of References

> - At any given time, you can have either (but not both of) one mutable
>   reference or any number of immutable references.
> - References must always be valid.

## The Slice Type

What if we have two overlapping slices? I guess that's why slices have to be
immutable?

It's neat that string literals are slices of the binary.

 

