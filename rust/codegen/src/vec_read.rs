use std::cmp::min;
use std::io::Read;

pub struct VecRead {
    offset: usize,
    vec: Vec<u8>,
}

impl From<Vec<u8>> for VecRead {
    fn from(value: Vec<u8>) -> Self {
        Self {
            offset: 0,
            vec: value,
        }
    }
}

impl Read for VecRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        let vec_len = self.vec.len();
        let readable = vec_len - self.offset;
        if readable == 0 {
            return Ok(0);
        }
        let will_read = min(readable, buf_len);
        let target = &mut buf[0..will_read];
        target.copy_from_slice(&self.vec.as_slice()[self.offset..self.offset + will_read]);
        self.offset = self.offset + will_read;
        Ok(will_read)
    }
}
