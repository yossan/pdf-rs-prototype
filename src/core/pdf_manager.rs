use super::stream::Stream;

pub struct PdfManager<'a> {
    stream: Stream<'a>,
}

impl<'a> PdfManager<'a> {
    pub fn new(stream: Stream<'a>) -> Self {
        PdfManager {
            stream: stream,
        }
    }
}
