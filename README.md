tabwriter is a crate that implements
[elastic tabstops](http://nickgravgaard.com/elastictabstops/index.html). It
provides both a library for wrapping Rust `Writer`s and a small program that
exposes the same functionality at the command line.

[![Build status](https://api.travis-ci.org/BurntSushi/tabwriter.png)](https://travis-ci.org/BurntSushi/tabwriter)
[![](http://meritbadge.herokuapp.com/tabwriter)](https://crates.io/crates/tabwriter)

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).


### Simple example of library

```rust
use std::io::MemWriter;
use tabwriter::TabWriter;

let mut tw = TabWriter::new(MemWriter::new());
tw.write_str("
Bruce Springsteen\tBorn to Run
Bob Seger\tNight Moves
Metallica\tBlack
The Boss\tDarkness on the Edge of Town
").unwrap();
tw.flush().unwrap();

let written = String::from_utf8(tw.unwrap().unwrap()).unwrap();

assert_eq!(written.as_slice(), "
Bruce Springsteen  Born to Run
Bob Seger          Night Moves
Metallica          Black
The Boss           Darkness on the Edge of Town
");
```

You can see an example of *real* use in my
[CSV toolkit](https://github.com/BurntSushi/xsv/blob/66a688c2df8f4579c7ce8f322eedea141dd79e8f/src/cmd/table.rs#L57-L59).


### Simple example of command line utility

```bash
[andrew@Liger tabwriter] cat sample | sed 's/   /\\t/g'
a\tb\tc
abc\tmnopqrstuv\txyz
abcmnoxyz\tmore text

a\tb\tc
[andrew@Liger tabwriter] ./target/tabwriter < sample
a          b           c
abc        mnopqrstuv  xyz
abcmnoxyz  more text

a   b   c
```

Notice that once a column block is broken, alignment starts over again.


### Documentation

The API is fully documented with some examples:
[http://burntsushi.net/rustdoc/tabwriter/](http://burntsushi.net/rustdoc/tabwriter/).


### Installation

This crate works with Cargo. Assuming you have Rust and
[Cargo](http://crates.io/) installed, simply check out the source and run
tests:

```bash
git checkout git://github.com/BurntSushi/tabwriter
cd tabwriter
cargo test
```

You can also add `tabwriter` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
tabwriter = "0.1"
```


### Dealing with ANSI escape codes

If you want `tabwriter` to be aware of ANSI escape codes, then compile it with
the `ansi_formatting` feature enabled.
