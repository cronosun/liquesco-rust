use crate::vec_read::VecRead;
use std::io::{Error, Write};
use std::ops::AddAssign;

pub struct Text {
    vec: String,
    start_of_line: bool,
    needs_indent: bool,
    single_indent: u8,
    indent: usize,
    space: bool,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            vec: String::new(),
            start_of_line: true,
            needs_indent: true,
            single_indent: 2,
            indent: 0,
            space: false,
        }
    }
}

impl Text {
    /// Wraps new line if not already on a new line and adds indent.
    pub fn new_line(&mut self) {
        self.just_new_line();
    }

    /// Adds a space line. When called multiple times, will add multiple space lines.
    pub fn space_multiple(&mut self) {
        self.just_new_line();
        // now we're on a new line... now add add another line
        self.start_of_line = false;
        self.new_line();
        self.space = true;
    }

    /// Adds a single space. When called multiple times, will only add a single space line.
    pub fn space(&mut self) {
        if !self.space {
            self.space_multiple();
            self.space = true;
        }
    }

    /// Increments indent.
    pub fn inc_indent(&mut self) {
        self.indent += 1;
    }

    /// Decrements indent.
    pub fn dec_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }

    /// Just adds text.
    pub fn add<'a, TStr>(&mut self, string: TStr)
    where
        TStr: AsRef<str>,
    {
        self.do_intent_if_required();
        self.just_write(string.as_ref());
    }

    /// First makes sure we're on a new line (does nothing if we're already on a new line,
    /// see `new_line`). Then writes a string (see `add`).
    pub fn line<'a, TStr>(&mut self, string: TStr)
    where
        TStr: AsRef<str>,
    {
        self.new_line();
        self.add(string);
    }

    fn do_intent_if_required(&mut self) {
        if self.needs_indent {
            self.needs_indent = false;
            self.just_indent();
        }
    }

    fn just_write(&mut self, string: &str) {
        self.start_of_line = false;
        self.space = false;
        self.vec.add_assign(string);
    }

    /// Just adds a new line (no indent). Only adds a new line if not
    /// already on new line.
    fn just_new_line(&mut self) {
        if !self.start_of_line {
            self.just_write("\n");
            self.start_of_line = true;
            self.needs_indent = true;
        }
    }

    /// Just adds a single indent.
    fn just_single_indent(&mut self) {
        for _ in 0..self.single_indent {
            self.just_write(" ");
        }
    }

    /// Adds indent.
    fn just_indent(&mut self) {
        for _ in 0..self.indent {
            self.just_single_indent();
        }
    }
}

impl Into<String> for Text {
    fn into(self) -> String {
        self.vec
    }
}

impl Into<VecRead> for Text {
    fn into(self) -> VecRead {
        let vec: Vec<u8> = self.vec.into_bytes();
        vec.into()
    }
}

impl Write for Text {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        unsafe { self.vec.as_mut_vec().write(buf) }
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
