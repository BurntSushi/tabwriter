//! This crate provides an implementation of
//! [elastic tabstops](http://nickgravgaard.com/elastictabstops/index.html).
//! It is a minimal port of Go's
//! [tabwriter](http://golang.org/pkg/text/tabwriter/) package.
//! Namely, its main mode of operation is to wrap a `Writer` and implement
//! elastic tabstops for the text written to the wrapped `Writer`.
//!
//! This package is also bundled with a program, `tabwriter`,
//! that exposes this functionality at the command line.
//!
//! Here's an example that shows basic alignment:
//!
//! ```rust
//! use std::io::Write;
//! use tabwriter::TabWriter;
//!
//! let mut tw = TabWriter::new(Vec::new());
//! write!(&mut tw, "
//! Bruce Springsteen\tBorn to Run
//! Bob Seger\tNight Moves
//! Metallica\tBlack
//! The Boss\tDarkness on the Edge of Town
//! ").unwrap();
//! tw.flush().unwrap();
//!
//! let written = String::from_utf8(tw.unwrap()).unwrap();
//! assert_eq!(&*written, "
//! Bruce Springsteen  Born to Run
//! Bob Seger          Night Moves
//! Metallica          Black
//! The Boss           Darkness on the Edge of Town
//! ");
//! ```
//!
//! Note that `flush` **must** be called or else `TabWriter` may never write
//! anything. This is because elastic tabstops requires knowing about future
//! lines in order to align output. More precisely, all text considered in a
//! single alignment must fit into memory.
//!
//! Here's another example that demonstrates how *only* contiguous columns
//! are aligned:
//!
//! ```rust
//! use std::io::Write;
//! use tabwriter::TabWriter;
//!
//! let mut tw = TabWriter::new(Vec::new()).padding(1);
//! write!(&mut tw, "
//!fn foobar() {{
//!    let mut x = 1+1;\t// addition
//!    x += 1;\t// increment in place
//!    let y = x * x * x * x;\t// multiply!
//!
//!    y += 1;\t// this is another group
//!    y += 2 * 2;\t// that is separately aligned
//!}}
//!").unwrap();
//! tw.flush().unwrap();
//!
//! let written = String::from_utf8(tw.unwrap()).unwrap();
//! assert_eq!(&*written, "
//!fn foobar() {
//!    let mut x = 1+1;       // addition
//!    x += 1;                // increment in place
//!    let y = x * x * x * x; // multiply!
//!
//!    y += 1;     // this is another group
//!    y += 2 * 2; // that is separately aligned
//!}
//!");
//! ```

pub use tabwriter::TabWriter;

mod tabwriter;
#[cfg(test)] mod test;
