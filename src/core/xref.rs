use super::lexer::Lexer;
use super::parser::{Parser, ParserError};
use super::stream::Stream;
use super::primitives::*;

use std::fmt::{self, Debug};
use std::collections::HashMap;

macro_rules! get_integer {
    ($obj:expr) => { $obj.get_integer().ok_or_else(|| ParserError(line!()))? };
}

pub struct XRef<'a> {
    stream: Stream<'a>,
    trailer_dict: Dict,
    entries: Vec<Entry>,
    _cache_map: HashMap<i32, Primitives>,
}

impl<'a> Debug for XRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XRef")
         .field("trailer_dict", &self.trailer_dict)
         .field("num of entries", &self.entries.len())
         .finish()
    }
}

impl<'a> XRef<'a> {

    pub fn parse(stream: Stream<'a>, startxref: u64) -> XRef {
        let mut entries: Vec<Entry> = Vec::new();
        let (trailer_dict, stream) = {
            let mut parser = XRefParser::new(stream, startxref);
            let dict = parser.read_xref(&mut entries);
            (dict, parser.stream.clone())
        };

        // let encrypt = trailer_dict.get(b"Encrypt");
        // if let Some(Primitives::Dict(dict)) = encrypt {
        //     if let Some(Primitives::Array(ids)) = trailer_dict.get(b"ID") {
        //         // The 'Encrypt' dictionary itself should not be encrypted, and by setting
        //         // `suppressEncryption` we can prevent on infinite loop inside of
        //         // `XRef_fetchUncompressed` if the dictionary contains indirect objects (fixes
        //         // issue7665.pdf).
        //         encrypt.suppressEncryption = true;
        //         self.encript = new CipherTransformFactory(
        //             encrypt,
        //             fileId,
        //             self.pdfManager.password
        //             );
        //     }
        // }


        XRef {
            stream: stream,
            trailer_dict: trailer_dict,
            entries: entries,
            _cache_map: HashMap::new(),
        }
    }

    pub fn root(&self) -> &Dict {
        // Get the root dictionary (catalog) object, and do some basic validation.
        let root_obj = self.trailer_dict.get(b"Root").expect("`XRef.parse - Invalid \"Root\" reference.");
        if let Primitives::Dict(root) = root_obj {
            if root.has(b"Pages") {
                return root;
            }
        }
        panic!("Invalid root reference");
    }

    pub fn get_catalog_obj(&self) -> &Dict {
        self.root()
    }


    pub fn fetch(&mut self, refer: Ref) -> Option<&Primitives> {
        let num = refer.num();

        // The XRef cache is populated with objects which are obtained through `Parser.getObj`,
        // and indierectly via `Lexer.getObj`.

        //{
            if let Some(ref mut cache_entry) = self._cache_map.get_mut(&num) {
                if let Some(dict) = cache_entry.get_dict_mut() {
                    dict.set_obj_id(&refer.to_string());
                }
            }
        //};

        if let Some(cache_entry) = self._cache_map.get(&num) {
            return Some(cache_entry);
        } else {
            let xref_entry = self.get_entry(num);
            if xref_entry.is_none() {
                //self._cache_map.insert(num, None);
                return None;
            }
            return None;
        }

        /*
        let xref_entry = xref_entry.unwrap();
        let mut fetched_obj;
        //if xref_entry.uncompressed {
            fetched_obj = self.fetch_uncompressed(&refer, &xref_entry/*, suppress_encryption*/);
        //} else {
        //    fetched_obj  =self.fetch_compressed(refer, xref_entry/*, suppress_encryption*/);
        //}

        if let Some(mut dict) = fetched_obj.get_dict() {
            dict.set_obj_id(&refer.to_string());
        //} else if let Some(stream) = fetched_obj.get_stream() {
        }
        
        return Some(&fetched_obj);
        */
    }

    fn get_entry(&self, i: i32) -> Option<&Entry> {
        if let Some(xref_entry) = self.entries.get(i as usize) {
            if !xref_entry.free && xref_entry.offset != 0 {
                return Some(xref_entry);
            }
        }
        None
    }

    fn fetch_uncompressed(&self, refer: &Ref, xref_entry: &Entry /*, suppress_encryption: bool*/) -> Primitives {
        let gen = refer.gen() as i64;
        let num = refer.num() as  i64;
        if xref_entry.gen != gen {
            panic!("Inconsistent generation in XRef: {:?}", refer);
        }
        let stream = self.stream.make_substream(xref_entry.offset as u64 + self.stream.start());
        let mut parser = Parser::new(Lexer::new(stream), true);
        let obj1 = parser.get_obj();
        let obj2 = parser.get_obj();
        let obj3 = parser.get_obj();

        dbg!(&obj1);
        dbg!(&obj2);
        dbg!(&obj3);

        obj3.unwrap()
    }

    //fn fetch_compressed(&self, entry: &Entry/*, suppress_encryption: bool*/) -> Primitives {
    //}
}

struct XRefParser<'a> {
    stream: Stream<'a>,
    startxref_queue: Vec<u64>,
}

impl<'a> XRefParser<'a> {
    pub fn new(stream: Stream<'a>, startxref: u64) -> XRefParser {
        XRefParser {
            stream: stream,
            startxref_queue: vec![startxref],
        }
    }

    fn read_xref(&mut self, entries: &mut Vec<Entry>) -> Dict {
        let mut startxref_parsed_cache = vec![0_u64; self.startxref_queue.len()];

        let mut top_dict: Option<Dict> = None;

        let mut stream = self.stream.clone();

        while self.startxref_queue.len() > 0 {
            let startxref = self.startxref_queue[0];
            if startxref_parsed_cache.contains(&startxref) {
                println!("read_xref - skipping XRef table since it was already parsed.");
                self.startxref_queue.remove(0);
                continue;
            }
            startxref_parsed_cache.push(startxref);

            stream.set_pos(startxref + stream.start());

            let lexer = Lexer::new(stream.clone());
            let mut parser = Parser::new(lexer, true);
            let xref_obj = parser.get_obj().unwrap();
            let mut dict: Option<&Dict> = None;

            //if xref_obj.is_cmd("xref") {
                let temp = self.process_xreftable(parser, entries).unwrap();
                if top_dict.is_none() {
                    top_dict = Some(temp);
                    dict = top_dict.as_ref();
                } else {
                    dict = Some(&temp);
                }

                // Recursively get other XRef's 'XRefStm', if any
                //obj = dict.get("XRefStm");
                //if obj.is_integer() {
                //    let pos = obj;
                //    // ignore previously loaded xre streams
                //    // (possible infinite recusion)
                //    if self.xrefstms.pos.is_none() {
                //        self.xrefstms.set_pos(1);
                //        self.startxref_queue.push(pos.get_integer().unwrap())
                //    }
                //}
            //} else if xref_obj.is_integer() {
                // Parse in-stream XRef
                // dict  =  self.processXRefStream(obj);
                //if top_dict.is_none() {
                //    top_dict = Some(dict);
                //}
            //} else {
            //    panic!("Invalid XRef stream header");
            //}

            // Recursively get previous dictionary, if any
            let prev_obj = dict.unwrap().get(b"Prev");
            if let Some(Primitives::Int(startxref)) = prev_obj {
                self.startxref_queue.push(startxref.clone() as u64);
            } else if let Some(Primitives::Ref(refer)) = prev_obj {
                // The spec says Prev must not be a reference, i.e. "/Prev NNN"
                // This is a fallback for non-compliant PDFs, i.e. "/Prev NNN 0 R"
                self.startxref_queue.push(refer.num() as u64);
            }
            self.startxref_queue.pop();
        }
        return top_dict.unwrap();
    }

    fn process_xreftable(&mut self, mut parser: Parser, entries: &mut Vec<Entry>) -> Result<Dict, ParserError> {
        //self.table_state = Some(TableState {
        //    entry_num: 0,
        //    stream_pos: parser.lexer().stream().pos(),
        //    parser_buf1: None,
        //    parser_buf2: None,
        //    first_entry_num: None,
        //    entry_count: None,
        //});
        let table_state = TableState {
            entry_num: 0,
            stream_pos: parser.lexer().stream().pos(),
            parser_buf1: None,
            parser_buf2: None,
            first_entry_num: None,
            entry_count: None,
        };

        let obj = self.read_xreftable(&mut parser, entries, table_state)?;
        dbg!(&obj);

        if !obj.is_cmd("trailer") {
            return Err(ParserError(line!()));
        }

        // Read trailer dictionary, e.g.
        // trailer
        //    << /Size 22
        //      /Root 20R
        //      /Info 10R
        //      /ID [ <81b14aafa313db63dbd6f981e49f94f4> ]
        //    >>
        // The parser goes through the entire stream << ... >> and provides
        // a getter interface for the key-value table
        let dict = parser.get_obj()?.unwrap_dict();
        return Ok(dict);
    }

    fn read_xreftable(&mut self, parser: &mut Parser, entries: &mut Vec<Entry>, table_state: TableState) -> Result<Primitives, ParserError> {
        // Example of cross-reference table:
        // xref
        // 0 1                    <-- subsection header (first obj #, obj count)
        // 0000000000 65535 f     <-- actual object (offset, generation #, f/n)
        // 23 2                   <-- subsection header ... and so on ...
        // 0000025518 00002 n
        // 0000025635 00000 n
        // trailer
        // ...

         let mut table_state = table_state;

        // Outer loop is over subsection headers.
        let mut obj;

        loop {
            obj = parser.get_obj()?;
            if table_state.first_entry_num.is_none() && table_state.entry_count.is_none() {
                if obj.is_cmd("trailer") {
                    break;
                }
                table_state.set_first_entry_num(get_integer!(obj));
                let next = parser.get_obj()?;
                table_state.set_entry_count(get_integer!(next));
            }

            let mut first = table_state.first_entry_num.unwrap();
            let count = table_state.entry_count.unwrap();

            // Inner loop is over objects themselves
            for i in table_state.entry_num .. count {
                table_state.stream_pos = parser.lexer().stream().pos();
                table_state.entry_num = i;
                table_state.parser_buf1 = parser.buf1();
                table_state.parser_buf2 = parser.buf2();

                let offset = get_integer!(parser.get_obj()?);
                let gen = get_integer!(parser.get_obj()?);
                let ty = parser.get_obj()?;

                let (free, uncompressed) = {
                    if ty.is_cmd("f") {
                        (true, false)
                    } else if ty.is_cmd("n") {
                        (false, true)
                    } else {
                        (false, false)
                    }
                };

                let entry = Entry {
                    offset: offset,
                    gen: gen,
                    ty: ty,
                    free: free,
                    uncompressed: uncompressed,
                };

                // The first xref table entry, i.e. obj 0, should be free. Attempting
                // to adjust an incorrect first obj # (fixes issue 3248 and 7229).
                if i == 0 && entry.free && first == 1 {
                    first = 0;
                }

                if entries.len() > i as usize {
                    entries[(i + first) as usize] = entry;
                } else {
                    entries.push(entry);
                }
            }

            table_state.entry_num = 0;
            table_state.stream_pos = parser.lexer().stream().pos();
            table_state.parser_buf1 = parser.buf1();
            table_state.parser_buf2 = parser.buf2();
            table_state.first_entry_num = None;
            table_state.entry_count = None;
        }

        // Sanity check: as per spec, first ojbect must be free
        if entries.len() > 1 && !entries[0].free {
            return Err(ParserError(line!()));
        }

        return Ok(obj); // Cmd("trailer")
    }
}


struct TableState {
    entry_num: i64,
    stream_pos: u64,
    parser_buf1: Option<Primitives>,
    parser_buf2: Option<Primitives>,
    first_entry_num: Option<i64>,
    entry_count: Option<i64>,
}

impl TableState {
    fn from_parser(parser: &Parser) -> Self {
        TableState {
            entry_num: 0,
            stream_pos: parser.lexer().stream().pos(),
            parser_buf1: parser.buf1(),
            parser_buf2: parser.buf2(),
            first_entry_num: None,
            entry_count: None,
        }
    }
    fn set_first_entry_num(&mut self, num: i64) {
        self.first_entry_num = Some(num);
    }
    fn set_entry_count(&mut self, count: i64) {
        self.entry_count = Some(count);
    }
}

#[derive(Debug)]
struct Entry { 
    offset: i64, 
    gen: i64, 
    ty: Primitives, 
    free: bool, 
    uncompressed: bool 
}

