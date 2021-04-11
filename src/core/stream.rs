use std::io::{ Read, Cursor, Seek, SeekFrom };
use super::primitives::Dict;

#[derive(Debug, PartialEq, Clone)]
pub struct Stream<'a> {
    //pub (super) source: Rc<dyn ReadSeek>,
    source: Cursor<&'a Vec<u8>>,
    start: u64,
    pos: u64,
    end: u64,
}

impl<'a> Stream<'a> {
    pub fn new(source: &'a Vec<u8>, length: usize) -> Self {
        Stream {
            source: Cursor::new(source),
            start: 0,
            pos: 0,
            end: length as u64,
        }
    }
    pub fn new_sub(&self, start: u64) -> Self {
        Stream {
            source: Cursor::new(self.source.get_ref()),
            start: start,
            pos: start,
            end: self.len(),
        }
    }

    pub fn len(&self) -> u64 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_byte(&mut self) -> Option<u8> {
        let byte = self.peek_byte()?;
        self.seek_pos(1);
        Some(byte)
    }

    pub fn get_uint16(&mut self) -> Option<u16> {
        let b0 = self.get_byte()? as u16;
        let b1 = self.get_byte()? as u16;
        Some((b0 << 8) | b1)
    }

    pub fn get_int32(&mut self) -> Option<i32> {
        let b0 = self.get_byte()? as i32;
        let b1 = self.get_byte()? as i32;
        let b2 = self.get_byte()? as i32;
        let b3 = self.get_byte()? as i32;
        Some((b0 << 24) | (b1 << 16) | (b2 << 8) | b3)
    }

    pub fn get_bytes(&mut self, len: usize) -> Option<Vec<u8>>{
        let bytes = self.peek_bytes(len)?;
        self.seek_pos(bytes.len() as i64);
        Some(bytes)
    }

    pub fn peek_byte(&mut self) -> Option<u8> {
        if self.pos >= self.end {
            return None;
        }
        let mut byte = [0_u8; 1];
        if let Ok(size) = self.source.read(&mut byte) {
            self.rollback_pos();
            if size > 0 {
                return Some(byte[0]);
            }
        }
        None
    }

    pub fn peek_bytes(&mut self, len: usize) -> Option<Vec<u8>> {
        let pos = self.pos;
        let str_end = self.end;
        let mut end = pos + len as u64;
        if end > str_end {
            end = str_end;
        }
        let length = end - pos;
        let mut bytes = vec![0_u8; length as usize];
        if let Ok(size) = self.source.read(&mut bytes) {
            self.rollback_pos();
            if size > 0 {
                return Some(bytes);
            }
        }
        None
    }

    pub fn get_byte_range(&mut self, begin: u64, end: u64) -> Option<Vec<u8>> {
        let (begin, end) = {
            let mut b = begin;
            if begin < 0 {
                b = 0;
            }
            let mut e = end;
            if end > self.end {
                e = self.end;
            }
            (b, e)
        };
    
        let length = end - begin;
        let mut bytes = vec![0_u8; length as usize];
        if let Ok(size) = self.source.read(&mut bytes) {
            if size > 0 {
                return Some(bytes);
            }
        }
        None
    }

    pub fn rollback_pos(&mut self) {
        self.set_pos(self.pos);
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn pos(&self) -> u64 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: u64) {
        self.pos = pos;
        let _ = self.source.seek(SeekFrom::Start(pos));
    }

    pub fn seek_pos(&mut self, n: i64) {
        let pos = self.pos as i64 + n;
        self.pos = pos as u64;
        let _ = self.source.seek(SeekFrom::Current(n));
    }

    pub fn skip(&mut self, n: i64) {
        let pos = self.pos as i64 + n;
        self.pos = pos as u64;
        let _ = self.source.seek(SeekFrom::Current(n));
    }

    pub fn reset(&mut self) {
        self.pos = self.start;
        let _ = self.source.seek(SeekFrom::Start(self.start as u64));
    }

    pub fn move_start(&mut self) {
        self.start = self.pos;
    }

    pub fn make_substream(&self, start: u64 /*, length: u64, dict: Primitives::Dict*/) -> Stream {
        self.new_sub(start)
    }
}


/*
struct StringStream<'a> {
    parent: Stream<'a>,
}

impl<'a> StringStream<'a> {
    pub fn new(str: &'a str) -> StringStream {
        let len = str.len();
        StringStream {
            parent: Stream::new(&str.as_bytes().to_vec(), len),
        }
    }
}
*/


