# crotchet

A LISP dialect with less `Shift`.

I wanted to play around with LISP diaclects, but didn't like having to hold 
`Shift` to type `(` and `)` all the time. So, this is a minimalist LISP subset 
based on [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)) 
that uses `[` and `]` instead.

The name *crotchet* comes from an old printing term for a square bracket.

This was originally written as my final project for a Rust programming course.

The more recent versions of the codebase are based somewhat on [lisp-rs](https://github.com/vishpat/lisp-rs),
with the earliest prototypes based on [Risp](https://stopa.io/post/222).

## Features

- 64-bit Floating Point Arithmetic (with help from `round`)
- Variables and Constants (`let` and `set`)
- Lambda Functions and Closures (`fn`)
- Lists and list methods (`list`, `first`, `rest`, and `len`)
- Input/Output (`print` and `input`)
- Random Number Generation (`rand`)
- Loops (`while`)

Feel free to check out `example.crl` or `guessing_game.crl` for more examples.

## Installation

To build crotchet, you'll need [Rust](https://www.rust-lang.org/) installed. 

Clone the repository with `git clone` and then build the interpreter:

```
cd crotchet
cargo build --release
```

Within the repository directory the executable can be found at `/target/release/crotchet`.

From here you can add it to your `$PATH` however you prefer; personally I use a
symlink.

*Note:* If you use the [micro](https://github.com/zyedidia/micro) text editor like 
I do, enjoy the syntax highlighting file in `crotchet.yaml`.

## Usage

To get the current version, use the `--version` flag.

To print the help/usage info, use the `--help` flag.

### REPL

To start an interactive session, run:

```
./target/release/crotchet
```

or simply `crotchet`.

### Scripts

To execute a crotchet script, provide the file as an argument:

```
./target/release/crotchet example.crl
```

or simply `crotchet example.crl`.

## Contributing

Contributions are welcome- feel free to fork and submit pull requests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
