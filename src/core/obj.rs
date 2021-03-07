use supe::xref::XRef;

struct Catalog {
    xref: XRef,
}
impl Catalog {
    pub fn new(xref: XRef) -> Self {
        Catalog {
            xref: xref,
        }
    }

    pub fn version(&self) -> &'str {
        self.cat_dict
    }

}
