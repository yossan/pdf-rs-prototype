use std::io::prelude::*;

struct Stream<'a> {
    source: std::io::Cursor<&'a Vec<u8>>,
}

struct Lexer<'a> {
    stream: &'a mut Stream<'a>,
}

impl<'a> Lexer<'a> {
    fn get_byte(&mut self) -> u8 {
        let mut bytes = [0_u8; 1];
        let _ = self.stream.source.read(&mut bytes).unwrap();
        bytes[0]
    }
}


fn main() {
    let source = vec![1,2,3,4,5];
    let mut stream = Stream { source: std::io::Cursor::new(&source) };
    let mut lexer = Lexer { stream: &mut stream };
    let u = lexer.get_byte();
    dbg!(u);

}
