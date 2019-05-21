use crate::vec_read::VecRead;
use std::io::{Error, Write};
use std::ops::AddAssign;

pub struct Text {
    vec: String,
    start_of_line: bool,
    needs_indent: bool,
    single_indent: u8,
    indent: usize,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            vec: String::new(),
            start_of_line: true,
            needs_indent: true,
            single_indent: 2,
            indent: 0,
        }
    }
}

impl Text {
    /// Wraps new line if not already on a new line and adds indent.
    pub fn new_line(&mut self) {
        self.just_new_line();
    }

    /// Adds a space line.
    pub fn space(&mut self) {
        self.just_new_line();
        // now we're on a new line... now add add another line
        self.start_of_line = false;
        self.new_line()
    }

    pub fn inc_indent(&mut self) {
        self.indent += 1;
    }

    pub fn dec_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }

    pub fn add(&mut self, string: &str) {
        self.do_intent_if_required();
        self.just_write(string);
    }

    fn do_intent_if_required(&mut self) {
        if self.needs_indent {
            self.needs_indent = false;
            self.just_indent();
        }
    }

    fn just_write(&mut self, string: &str) {
        self.start_of_line = false;
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
