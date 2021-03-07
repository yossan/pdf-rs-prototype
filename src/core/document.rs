use super::stream::Stream;
//use super::xref::XRef;

pub struct PdfDocument {
    pub (super) stream: Stream<'static>,
    pub (super) version: String,
 //   pub (super) xref: XRef,
}


impl PdfDocument {}

