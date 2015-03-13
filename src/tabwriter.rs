use std::cmp;
use std::io::{self, Write};
use std::iter::{AdditiveIterator, repeat};
use std::mem;
use std::str;

/// TabWriter wraps an arbitrary writer and aligns tabbed output.
///
/// Elastic tabstops work by aligning *contiguous* tabbed delimited fields
/// known as *column blocks*. When a line appears that breaks all contiguous
/// blocks, all buffered output will be flushed to the underlying writer.
/// Otherwise, output will stay buffered until `flush` is explicitly called.
pub struct TabWriter<W> {
    w: W,
    buf: io::Cursor<Vec<u8>>,
    lines: Vec<Vec<Cell>>,
    curcell: Cell,
    minwidth: usize,
    padding: usize,
}

#[derive(Debug)]
struct Cell {
    start: usize, // offset into TabWriter.buf
    width: usize, // in characters
    size: usize,  // in bytes
}

impl<W: io::Write> TabWriter<W> {
    /// Create a new `TabWriter` from an existing `Writer`.
    ///
    /// All output written to `Writer` is passed through `TabWriter`.
    /// Contiguous column blocks indicated by tabs are aligned.
    ///
    /// Note that `flush` must be called to guarantee that `TabWriter` will
    /// write to the given writer.
    pub fn new(w: W) -> TabWriter<W> {
        TabWriter {
            w: w,
            buf: io::Cursor::new(Vec::with_capacity(1024)),
            lines: vec!(vec!()),
            curcell: Cell::new(0),
            minwidth: 2,
            padding: 2,
        }
    }

    /// Set the minimum width of each column. That is, all columns will have
    /// *at least* the size given here. If a column is smaller than `minwidth`,
    /// then it is passed with spaces.
    ///
    /// The default minimum width is `2`.
    pub fn minwidth(mut self, minwidth: usize) -> TabWriter<W> {
        self.minwidth = minwidth;
        self
    }

    /// Set the padding between columns. All columns will be separated by
    /// *at least* the number of spaces indicated by `padding`. If `padding`
    /// is zero, then columns may run up against each other without any
    /// separation.
    ///
    /// The default padding is `2`.
    pub fn padding(mut self, padding: usize) -> TabWriter<W> {
        self.padding = padding;
        self
    }

    /// Returns the underlying writer. Note that `flush` must be called before
    /// unwrapping or else data will likely be lost.
    pub fn unwrap(self) -> W {
        self.w
    }

    /// Resets the state of the aligner. Once the aligner is reset, all future
    /// writes will start producing a new alignment.
    fn reset(&mut self) {
        self.buf = io::Cursor::new(Vec::with_capacity(1024));
        self.lines = vec!(vec!());
        self.curcell = Cell::new(0);
    }

    /// Adds the bytes received into the buffer and updates the size of
    /// the current cell.
    fn add_bytes(&mut self, bytes: &[u8]) {
        self.curcell.size += bytes.len();
        let _ = self.buf.write_all(bytes); // cannot fail
    }

    /// Ends the current cell, updates the UTF8 width of the cell and starts
    /// a fresh cell.
    fn term_curcell(&mut self) {
        let mut curcell = Cell::new(self.buf.position() as usize);
        mem::swap(&mut self.curcell, &mut curcell);

        curcell.update_width(&self.buf.get_ref());
        self.curline_mut().push(curcell);
    }

    /// Return a view of the current line of cells.
    fn curline(&mut self) -> &[Cell] {
        let i = self.lines.len() - 1;
        &*self.lines[i]
    }

    /// Return a mutable view of the current line of cells.
    fn curline_mut(&mut self) -> &mut Vec<Cell> {
        let i = self.lines.len() - 1;
        &mut self.lines[i]
    }
}

impl Cell {
    fn new(start: usize) -> Cell {
        Cell { start: start, width: 0, size: 0 }
    }

    fn update_width(&mut self, buf: &[u8]) {
        let end = self.start + self.size;
        self.width = display_columns(&buf[self.start..end]);
    }
}

impl<W: io::Write> io::Write for TabWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut lastterm = 0usize;
        for (i, &c) in buf.iter().enumerate() {
            match c {
                b'\t' | b'\n' => {
                    self.add_bytes(&buf[lastterm..i]);
                    self.term_curcell();
                    lastterm = i + 1;
                    if c == b'\n' {
                        let ncells = self.curline().len();
                        self.lines.push(vec!());
                        // Having a single cell means that *all* previous
                        // columns have been broken, so we should just flush.
                        if ncells == 1 {
                            try!(self.flush());
                        }
                    }
                }
                _ => {}
            }
        }
        self.add_bytes(&buf[lastterm..]);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.curcell.size > 0 {
            self.term_curcell();
        }
        let widths = cell_widths(&self.lines, self.minwidth);

        // This is a trick to avoid allocating padding for every cell.
        // Just allocate the most we'll ever need and borrow from it.
        let biggest_width = widths.iter()
                                  .map(|ws| ws.iter().map(|&w|w).max()
                                              .unwrap_or(0))
                                  .max().unwrap_or(0);
        let padding: String =
            repeat(' ').take(biggest_width + self.padding).collect();

        let mut first = true;
        for (line, widths) in self.lines.iter().zip(widths.iter()) {
            if !first { try!(self.w.write_all(b"\n")); } else { first = false }
            for (i, cell) in line.iter().enumerate() {
                let bytes = &self.buf.get_ref()[cell.start..cell.start + cell.size];
                try!(self.w.write_all(bytes));
                if i >= widths.len() {
                    assert_eq!(i, line.len()-1);
                } else {
                    assert!(widths[i] >= cell.width);
                    let padsize = self.padding + widths[i] - cell.width;
                    try!(write!(&mut self.w, "{}", &padding[0..padsize]));
                }
            }
        }

        self.reset();
        Ok(())
    }
}

fn cell_widths(lines: &Vec<Vec<Cell>>, minwidth: usize) -> Vec<Vec<usize>> {
    // Naively, this algorithm looks like it could be O(n^2m) where `n` is
    // the number of lines and `m` is the number of contiguous columns.
    //
    // However, I claim that it is actually O(nm). That is, the width for
    // every contiguous column is computed exactly once.
    let mut ws: Vec<_> = (0..lines.len()).map(|_| vec![]).collect();
    for (i, iline) in lines.iter().enumerate() {
        if iline.is_empty() {
            continue
        }
        for col in ws[i].len()..(iline.len()-1) {
            let mut width = minwidth;
            let mut contig_count = 0;
            for line in lines[i..].iter() {
                if col + 1 >= line.len() { // ignores last column
                    break
                }
                contig_count += 1;
                width = cmp::max(width, line[col].width);
            }
            assert!(contig_count >= 1);
            for j in i..(i+contig_count) {
                ws[j].push(width);
            }
        }
    }
    ws
}

fn display_columns(bytes: &[u8]) -> usize {
    // If we have a Unicode string, then attempt to guess the number of
    // *display* columns used.
    match str::from_utf8(bytes) {
        Err(_) => bytes.len(),
        Ok(s) => s.chars().map(|c| c.width(false).unwrap_or(0)).sum(),
    }
}
