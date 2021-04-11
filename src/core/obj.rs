use super::xref::XRef;
use super::primitives::*;

pub struct Catalog<'a> {
    xref: &'a XRef<'a>,
}
impl<'a> Catalog<'a> {
    pub fn new(xref: &'a XRef<'a>) -> Self {
        Catalog {
            xref: xref,
        }
    }

    pub fn version(&self) -> Option<&str> {
        let dict = self.xref.get_catalog_obj();
        let version_obj = dict.get(b"Version");
        if let Some(Primitives::Name(version)) = version_obj {
            Some(version.name())
        } else {
            None
        }
    }
}
