---
title: "Notes (TRPL 02/21): Programming a Guessing Game"
---

# 2 Programming a Guessing Game


## Setting up a New Project

Despite having gone through all the work to figure out `carnix` in the last
chapter, I think I'm just going to use `cargo` without `nix` for the moment, so
that I can learn the usual non-`nix` way first. ~~I kinda wish cargo
automatically integrated with `nix` like `stack` does with haskell, so I don't
have to throught the extra `cargo build`, `carnix` and `nix-build` steps.~~
(despite the fact that `carnix` is more ergonomic now, it's still bleeding-edge
undocumented code, so i don't want to run into issues that might complicate my
rust learning.)

## Handling Potential Failure with the `Result` Type

How can we make `read_line` fail though? The docs say that it errors on any
[bytes that aren't valid
UTF-8](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line),
but I tried typing in some various malformed UTF-8 sequences (with `<C-U>`) from
this [UTF-8 test
page](https://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt), and couldn't
get it to error.

I will keep investigating.

## Generating a Random Number

The `cargo doc --open` command is awesome.

## Comparing the Guess to the Secret Number

Okay, great we have pattern matching, excellent.

I'm not sure how I feel about the `.function` notation for doing function
"piping". It's very terse and legible, but I guess I can't help but compare it
to Haskell  does make me appreciate how `>>=` is just a regular function and not
special language syntax.

I'm going to try to dampen that "But in Haskell..." reflex though, this is a
different language with its own philosophy of programming, particularly on how
best to safely handle mutable state.

## Allowing Multiple Guesses with Looping

To expand on the previous thought, I think there's actually great value in
the way Rust provides concise conceptual tools for dealing with control flow and
state mutation. Take the `continue` keyword in this example, for instance. If we
were trying to accomplish the same thing with pure code, we'd probably assign
a name to the loop and use recursion:

```rust
fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", secret_number);
  loop_(secret_number);

}

fn loop_(secret : u32) -> u32 {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => loop_(secret),
      };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret) {
      Ordering::Less => {println!("Too small!"); loop_(secret) }
      Ordering::Greater => { println!("Too big!"); loop_(secret) }
      Ordering::Equal => { println!("You win!"); return guess; }
    }
  }
```

But this actually doesn't work! If the parsing fails, the pattern matching
continuation doubles up on itself. So we actually have to do:

```rust
fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", secret_number);
  loop_(secret_number);

}

fn loop_(secret : u32) -> u32 {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => continue_(secret, num),
        Err(_) => loop_(secret),
      };
    return guess;
  }

fn continue_(secret : u32, guess : u32) -> u32 {
    println!("You guessed: {}", guess);

    match guess.cmp(&secret) {
      Ordering::Less => {println!("Too small!"); loop_(secret) }
      Ordering::Greater => { println!("Too big!"); loop_(secret) }
      Ordering::Equal => { println!("You win!"); return guess; }
    }
 }
```

This is pure, but ugly. As opposed to the example from the text, which is impure
yet lovely (["O daughters of Jerusalem"](https://biblehub.com/songs/1-5.htm)).

The pure code has a lot of plumbing, and okay a language designed for
that plumbing is going to be little cleaner, but only a little, and for
differet reasons.

I mean, I know I said was going to tone down the Haskell comparisons, but this
is actually a point in Rust's favo, so look at what a Haskell verison of the
guessing game might look like:

```haskell
module Main where

import           System.Random
import           Text.Read

main :: IO ()
main = do
  secret <- getStdRandom (randomR (1,101))
  putStrLn "secret is: "
  print secret
  let loop = do
        putStrLn "Please input your guess"
        guess <- getLine
        case (readMaybe guess) :: Maybe Int of
          Nothing -> loop
          Just x  -> case (compare guess secret) of
            GT -> (putStrLn "Too high" >> loop)
            EQ -> (putStrLn "You win!" >> return ())
            LT -> (putStrLn "Too low" >> loop)
  loop
```

Is this more concise? Definitely more consise than the pure Rust guessing game,
but it's actually about comparable to the idiomatic `main.rs` in the book (23
lines for Haskell vs 25 or 33 for Rust depending on if count the empty lines in
the function body).

But the thing is that the Rust code exposes a lot of information that the
Haskell code abstracts over! For example, the `guess` in my Haskell
code should actually be using `IORef` and `Text` to better match the fact that
the Rust code creates an empty mutable string variable. And that's not even
getting into the borrowing and reference management that Rust is doing.

In this particular example it doesn't matter, but I can imagine a lot of cases
where the fact that Rust exposes a lot of granular low-level detail might
come in really helpful.


