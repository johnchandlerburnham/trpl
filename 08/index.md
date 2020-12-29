---
title: "Notes (TRPL 08/21): Common Collections"
---

# 8 Common Collections

## 8.1 Vectors

see `src/main.rs`

Defining,

```rust
let mut v = Vec::new();

v.push(5);
```

looks like it causes the compiler to infer that `v : Vec<{integer}>`. I think
this is the case because if I force a type error with

```rust
let mut v = Vec::new();

v.push(5);
v.push("a");
```

The compiler says:

```rust
9 |   v3.push("a"); // type inference!
  |           ^^^ expected integral variable, found reference
  |
  = note: expected type `{integer}`
             found type `&'static str`
```

I wonder how the compiler decides to pick a concrete type like `i32` from
`{integer}` though.

### Reading Elements of Vectors

If you try to access an element out of bounds:

```rust
let v = vec![1, 2, 3, 4, 5];
let sixth: &i32 = &v[6];
println!("The sixth element is {}", sixth);
```

The thread will panic at runtime. Using `.get` we can get elements safely.

I really like that having an immutable reference to an element of a vector
prevents creating mutable references and vice-versa:

This has two immutable references to `v[0]` so no problems:

```rust
let mut v = vec![1, 2, 3, 4, 5];
println!("v is {:?}", v);

let first = &v[0];

println!("first is {}", first);
//v[0] = 3;
//println!("first is {}", first);
println!("v is {:?}", v);
```

Hmmm, looks like we can create an immutable reference to `v[0]` and then mutate
under it, as long as our immutable reference is never actually used.

```rust
let first = &v[0];

//println!("first is {}", first);
v[0] = 3;
//println!("first is {}", first);
/println!("v is {:?}", v);
```

I guess that makes sense because the scope we're in already mutably owns `v`
because we declared it here. But this still seems like something that could
cause us some confusion later on if we forget exactly how the borrow checker
works.

We can actually make the above code work by just pushing another value onto the
stack with `first`'s value:

```rust
let first = &v[0]; // ref A
let first_ = first;
println!("first_ is {}", first_);

v[0] = 3; // allowed because first is never borrowed by anything
          // we could have done
          // let first = v[0].clone()
          // or simply
          // let first = v[0];

let first = &v[0]; // this &v[0] is potentially a different reference than ref A
println!("first is {}", first);
/println!("v is {:?}", v);
```

Okay, so that explains why we're allowed to even have an immutable reference
that never gets borrowed in the same scope as a mutable reference. We can use
the value, but only to copy it's value into new memory, whether onto the stack
with a `let` or onto the heap with a `.clone()`.

### Iterating over the Values in a Vector

`*` is the dereference operator. `*ref` means the value at memory location
`ref`. It's important to distinguish `let *x = y` from `let y = *x` though. The
former is a write operation that sets the value at `x` to equal the value of the
variable `y` on the stack. The latter is a read operation that copies the value
at `x` and allocates a new variable `y` on the stack with that value.

It's a little confusing that `=` does different things to the left-hand and
right-hand side. Keep in mind that in C family languages (of which Rust is one),
the `=` assignment operator should properly be `:=`, which corresponds to the
definition notation from mathematics. The only reason it isn't is that in 1969
Dennis Ritchie and Ken Thompson were trying to fit their new B language
efficiently into the 9.2 KB memory of the PDP-7, so they cut the `:` out of `:=` [to
save space](https://www.hillelwayne.com/post/equals-as-assignment/).

## 8.2 String

see `src/main2.rs`

### Storing UTF-8 Encoded Text with Strings

Wondering how `+` is overloaded to be concatenation on Strings and addition on
numbers. Oh, I see, there's just an `Add` trait:
https://doc.rust-lang.org/std/ops/trait.Add.html

Strings are UTF-8 encoded so since UTF-8 is variable width, slices
and indexes of Strings don't always map cleanly to characters. The
Rust compiler flat out prevents you from accessing a single index of a string.
String slices are allowed but can cause a thread to panic if the slice bounds
don't correspond to UTF-8 char boundaries.

tldr; Strings are complicated, use methods and libraries.

## 8.3 Hash Maps

see `src/main3.rs`

A `HashMap<K, V>` is a map from keys of type `K` to values of type `V`.

The line:

```rust
teams.iter().zip(initial_scores.iter()).collect();
```

feels kind-of functional, but I want to get a beter understanding of what
`.iter()` does concretely.

### Hash Maps and Ownership

Once they get passed into `.insert()`, the `field_name` and `field_value`
references have been moved into the `map`.

How exactly does the `for ... in` in

```rust
for (key, value) in &scores
```

iterate over all the pairs in the HashMap? Is there a trait? I bet there is,
there's always a trait when I ask these kinds of questions...

Indeed there is! [It's the Iterator trait](http://xion.io/post/code/rust-for-loop.html).

## Exercises

### Mean, Median, Mode

```rust
fn median(ns: &Vec<i32>) -> f32 {
  let mut ms = ns.clone();
  ms.sort();
  let l = ns.len();
  if l % 2 == 1 {
    ms[l / 2] as f32
  } else {
    (ms[l / 2] as f32 + ms[l / 2 - 1] as f32) / 2.0
  }
}

fn mean(ns: &Vec<i32>) -> f32 {
  let total: i32 = ns.iter().sum();
  let size = ns.len() as f32;
  total as f32 / size
}

fn mode(ns: &Vec<i32>) -> Vec<i32> {
  let mut map: HashMap<i32, i32> = HashMap::new();
  for n in ns {
    let count = map.entry(*n).or_insert(0);
    *count += 1;
  }
  let max: i32 = *map.iter().max_by_key(|(_, &v)| v).unwrap().1;
  map
    .iter()
    .filter(|(_, &v)| v == max)
    .map(|(&k, _)| k)
    .collect()
}
```

The mode function was particularly fun. One question I have is how Rust
distinguishes between a reference to a struct vs a struct of references. In some
of the tuples above they're interchangeable, but not in all of them.

### Pig Latin

For pig-latin, we'll use the `unicode-segmentation` [external
crate](https://unicode-rs.github.io/unicode-segmentation/unicode_segmentation/index.html)

I don't know what the best way to test if graphemes are consonants or vowel
though... Properly this would require an `is_vowel` function that was total over
every `UTF-8` codepoint, but I don't know if there's a Rust library for that...

### Text Interface for a department records system

see `src/main3.rs`




