---
title: "Notes (TRPL 01/21): Getting Started
---

# 1 Getting Started

## 1.1 Installing Rust (on NixOS)

I run NixOS, so I'm going to have to modify the installation instructions in the
book in order to take advantage of all the awesome package management work that
Nix already does for me.

First, I'm not going to use `rustup`. Instead, the way I'm going to manage which
version of Rust is on my system is by using [Mozilla's Rust overlay for
`nixpkgs`](https://github.com/mozilla/nixpkgs-mozilla)

A `nixpkgs` overlay is a way to customize the packages that are published in the
official `nixpkgs` repository. We want to have easy access to all the
different Rust versions, rather than whatever version happens to be in
`nixpkgs` in the `nixos` or `unstable` channels. Fortunately for us, Mozilla
maintains an overlay that lets us use nix to [mimic `rustup`'s
behavior.](https://www.mail-archive.com/nix-dev@lists.science.uu.nl/msg33296.html)

This method (instructions in either of the above links) automatically installs
the right `rustc` and `cargo`, so make sure you haven't already installed
`rustc` and `cargo` with `nix-env` or globally in `environment.systemPackages`,
or you'll get a package name collision.

Also, make sure you have `gcc` installed, since Rust needs a linker.

```
$ nix-env -iA nixos.gcc
```

Actually probably not a bad idea to put `gcc` in your `configuration.nix`
system packages.

## 1.2 Hello, World!

[see `hello_world/main.rs`]

Okay, so apparently Rust has opinions about how I should indent my code. While I
respect their desire for consistency of style between libraries, I also have a
desire for consistency of my style across languages. My personal preference is
for 2-space indents with an 80 character line length. That's what I like, but
when collaborating with other people I also always try to defer to the
preference of the repository owner or primary author. I think that strikes a
reasonable balance between staying true to my aesthetics and also being
friendly.

(Some languages like Python enforces 4-space indentation as a matter of syntax.
My policy is to not push-back against language-level syntax enforcement, except
maybe by writing my code in a less draconian language, if reasonable)

I think what I'm gonna do here is install the `rust-lang/rust.vim` plugin and
set:

```

let g:rust_recommended_style=0
let g:rustfmt_options='--config-path ~/.config/rustfmt/rustfmt.toml'
let g:rustfmt_autosave=1
```

in my `.vimrc`, which turns off the soft style recommendations, and runs
`rustfmt` every time I write a buffer in vim, with a global format configuration
file.

This is probably going to be annoying when I write Rust code that someone else
touches, but I think as long as my global `rustfmt.toml` is only accessed by
vim, I should be okay, since running `rustfmt` and `cargo fmt` from the command
line will still format according to any local `rustfmt.toml`

## 1.3 Hello, Cargo!

Okay, so we're also going to have to do a little funkiness to get `nix` and
`cargo` to play nice together. We're going to use a program called `carnix` that
turns the `Cargo.lock` file that `cargo` generates into a nix expression that we
can build with `nix-build`. The reason we want to do this is that when we start
to manage dependencies, `nix` is a much more powerful than `cargo`. `nix` is a
general purpose build-system and package manager that allows us to have
reproducible, atomic builds. `cargo`, for all its virtues, only works with Rust
packages (crates). So if we run into a situation where we have non-rust
dependencies, `nix` is going to help us a lot.

Furthermore, since I'm running NixOS, and therefore already managing all my
packages with nix, there's some advantage to maintaining consistency. `nix`
expressions compose (you can import one expression into another), so if I ever
want to, e.g. combine Rust and PureScript into a single application, `nix` will
be useful for that (since I've already figured out how to have `nix` manage
Purescript's build environment by integrating with `psc-package` and `npm`)

~~Instructions for how to use `carnix` are
[here](https://nixos.org/nixpkgs/manual/#compiling-rust-crates-using-nix-instead-of-cargo):~~

These instructions are outdated, `carnix`'s UI has improved as of version `0.7`;

```
$ cargo new hello
$ cd hello
$ carnix build
...
$ ./result/bin/hello
Hello, world!
$ cargo run
...
Hello, world!
```

Seems like there's no wrapping for `cargo check` though.

