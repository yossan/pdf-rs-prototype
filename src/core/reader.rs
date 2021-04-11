use super::document::PdfDocument;
use super::stream::Stream;
use super::utils::is_whitespace;
use super::xref::XRef;
use super::obj::Catalog;


impl PdfDocument {
    pub fn loadData(buffer: Vec<u8>, password: Option<String>) {
        let reader = Reader::new(&buffer);
        reader.parse();
    }
}

struct Reader<'a> {
    buffer: &'a Vec<u8>,
}

impl<'a> Reader<'a> {
    fn new(buffer: &'a Vec<u8>) -> Self {
        Reader {
            buffer: buffer,
        }
    }

    fn parse(&self) {
        let len = self.buffer.len();
        let mut stream = Stream::new(self.buffer, len);

        // 1. header
        stream.reset();
        // 2. startxref
        let startxref = Self::parse_startxref(&mut stream);
        dbg!(startxref);

        let mut xref = XRef::parse(stream.clone(), startxref);
        dbg!(&xref);
        let catalog = Catalog::new(&xref);
        if let Some(version) = catalog.version() {
            dbg!(version);
        }

    }

    fn parse_startxref(mut stream: &mut Stream) -> u64 {
        let mut start_xref = 0;
        /*
        if self.linearization {
        } else
        */

        // Find `startxref`.
        let step: i64 = 1024;
        let start_xref_length = "startxref".len() as i64;
        let mut found = false;
        let mut pos = stream.end() as i64;

        while !found && pos > 0 {
            pos -= step - start_xref_length;
            if pos < 0 {
                pos = 0;
            }
            stream.set_pos(pos as u64);
            found = Self::find(&mut stream, "startxref".as_bytes(), step as u64);
        }

        if found {
            stream.skip("startxref".len() as i64);
        }

        let mut ch;
        loop {
            ch = stream.get_byte().unwrap();
            if !is_whitespace(ch) {
                break;
            }
        }

        let mut str = String::new();
        while ch >= /* Space */ 0x20 && ch <= /* '9' = */ 0x39 {
            str.push(ch as char);
            ch = stream.get_byte().unwrap();
        }
        start_xref = str.parse::<u64>().unwrap_or(0);
        start_xref
    }

    fn find(stream: &mut Stream, signature: &[u8], limit: u64) -> bool {
        let signature_length = signature.len();
        let scan_bytes = stream.peek_bytes(limit as usize).unwrap();
        let scan_length = scan_bytes.len() - signature_length;
        if scan_length <= 0 {
            return false;
        }
        let mut pos: usize = 0;
        while pos <= scan_length {
            let mut j: usize = 0;
            while j < signature_length && scan_bytes[pos + j] == signature[j] {
                j += 1;
            }
            if j >= signature_length {
                // `signature` found.
                stream.seek_pos(pos as i64);
                return true;
            }
            pos += 1;
        }
        return false;
    }
}
