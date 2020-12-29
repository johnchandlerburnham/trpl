---
title: "Notes (TRPL 07/21): Packages, Crates, and Modules"
---

# 7 Packages, Crates, Modules

- *Packages* contain crates wrapped up for `cargo` handling with a `Cargo.toml`
- *Crates* contain a module hierarchy (tree)
- Modules contain *path*s which are named expressions (structs, functions,
  modules)

## 7.1 Packages and Crates for Making Libraries and Executables

`cargo` looks for `src/main.rs` to build a binary by convention, if no other
source files are specified with e.g.:

```
[[bin]]
name = "main2"
path = "src/main2.rs"
```

## 7.2 The Module System to Control Scope and Privacy

Interesting, modules in rust do not have to correspond one-to-one with source
files. A file can contain multiple modules.

### Modules as the Privacy Boundary

Access upwards is implicitly allowed, access downwards has to be explicitly
granted.

Privacy rules apply to any named expression, i.e. functions and structs as well
as modules.

This feels very object-oriented.

### The use Keword to Bring Paths into a scope

The `a::b::c::d()` pattern is verbose, we can bring `d` into scope with

```rust
use crate::a::b::c::d
```

But if `d` is function then it might be preferable to import the containing
module `c`:

```rust
use crate::a::b::c
```

And call `d` with `c::d()`

### Idiomatic `use` Paths for Functions vs. Other Items

It's idiomatic to import functions with

```rust
use crate::module

module::function();
```

But to import structs or enums with

```rust
use crate::module::StructEnum

StructEnum::function();
```

Unless this would create a namespace conflict, in which you can disambiguate by
importing the containing modules, or by using the `as` keyword.

### Renaming Types Brought Into Scope with the `as` Keyword

With `as` you can assign a module to a name in your namespace:

```rust
use crate::module::StructEnum as SE

SE::function();
```

### Nested Paths for Cleaning Up Large `use` Lists

Some nice tools here for nicely managing import lists:

```rust
use std::io;
use std::io::Write;
use std::cmp::Ordering;
```

becomes

```rust
use  std::{
  cmp::Ordering,
  io::{self, Write}
};
```

which even gets nicely autoformatted by `rustfmt`!

And then of course the notorious glob operator:

```rust
use std::collections::*;
```

It's a good rule of thumb to always be very very wary of the glob. Much wailing
and gnashing of teeth has resulted from careless globbing, particularly when
used as a command line argument (cf. `rm -rf`). In this case, though you
probably won't do anything other than make your scope messy. Nevertheless,
¡Cuidado!

### Separating Modules into Different Files

If we have a file `src/separate_file.rs`:

```rust
pub fn do_something() {
  println!("doing something");
}

pub mod separate_file_module {
  pub fn do_another_thing() {
    println!("doing another thing");
  }
}
```

Then from `src/main.rs` we can do:

```rust
mod separate_file;

fn main() {

  separate_file::do_something();
  separate_file::separate_file_module::do_another_thing();
}
```

Which calls the functions as expected.
