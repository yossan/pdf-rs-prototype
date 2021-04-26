use super::xref::XRef;
use super::stream::Stream;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Debug};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Name(pub Vec<u8>);

impl Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Name").field(&String::from_utf8(self.0.clone()).unwrap()).finish()
    }
}

impl Borrow<[u8]> for Name {
    fn borrow(&self) -> &[u8] {
        &*self.0
    }
}

impl Name {
    pub fn name(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Dict {
    _map: HashMap<Name, Primitives>,
    obj_id: Option<String>,
}

impl Dict {
    pub fn new(hashmap: HashMap<Name, Primitives>) -> Self {
        Dict {
            _map: hashmap,
            obj_id: None,
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&Primitives> {
        // if let Some(Primitives::Ref(value)) = self._map.get(key) {
        //     return 
        // }

        None
    }
    
    pub fn has(&self, key: &[u8]) -> bool {
        self._map.contains_key(key)
    }

    pub fn obj_id(&self) -> Option<&String> {
        self.obj_id.as_ref()
    }

    pub fn set_obj_id(&mut self, obj_id: &str) {
        self.obj_id = Some(obj_id.to_string());
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Ref {
    num: i32,
    gen: i32,
}

impl Ref {
    pub fn new(num: i32, gen: i32) -> Self {
        Ref {
            num: num,
            gen: gen,
        }
    }
    pub fn num(&self) -> i32 {
        self.num
    }
    pub fn gen(&self) -> i32 {
        self.gen
    }
}

impl ToString for Ref {
    fn to_string(&self) -> String {
        if self.gen == 0 {
            format!("{}R", self.num)
        } else {
            format!("{}R{}", self.num, self.gen)
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Primitives {
    Null,
    Int(i64),
    Str(Vec<u8>),
    HexStr(Vec<u16>),
    Real(f64),
    Name(Name),
    Array(Vec<Primitives>),
    Dict(Dict),
    //Stream(Stream),
    Ref(Ref),
    Cmd(Vec<u8>),
    EOF,
}

impl Debug for Primitives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitives::Null => f.debug_tuple("Null").finish(),
            Primitives::Int(num) => f.debug_tuple("Int").field(num).finish(),
            Primitives::Str(str) => f.debug_tuple("Str").field(&String::from_utf8(str.clone()).unwrap()).finish(),
            Primitives::HexStr(str) => f.debug_tuple("HexStr").field(&String::from_utf16(str.as_slice())).finish(),
            Primitives::Real(num) => f.debug_tuple("Real").field(num).finish(),
            Primitives::Name(name) => f.debug_tuple("Name").field(name).finish(),
            Primitives::Array(objects) => f.debug_tuple("Array").field(objects).finish(),
            Primitives::Dict(dict) => f.debug_tuple("Dict").field(dict).finish(),
            Primitives::Ref(r) => f.debug_tuple("Ref").field(&r.num).field(&r.gen).finish(),
            Primitives::Cmd(cmd) => f.debug_tuple("Cmd").field(&String::from_utf8(cmd.clone()).unwrap()).finish(),
            Primitives::EOF => f.debug_tuple("EOF").finish(),
        }
    }
}


impl Eq for Primitives {}

impl PartialEq<Primitives> for Option<Primitives> {
    fn eq(&self, other: &Primitives) -> bool {
        self.as_ref().map_or(false, |me| *me == *other)
    }
}

impl Primitives {

    pub fn cmd(cmd: &str) -> Primitives {
        Primitives::Cmd(cmd.as_bytes().to_vec())
    }

    pub fn name(bytes: Vec<u8>) -> Primitives {
        Primitives::Name(Name(bytes))
    }

    pub fn is_cmd(&self, cmd: &str) -> bool {
        if let Primitives::Cmd(bytes) = self {
            if bytes == cmd.as_bytes() {
                return true;
            }
        }
        false
    }

    pub fn is_name(&self) -> bool {
        if let Primitives::Name(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_integer(&self) -> bool {
        if let Primitives::Int(num) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Primitives::Str(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_cmd(&self) -> Option<&Vec<u8>> {
        if let Primitives::Cmd(bytes) = self {
            return Some(&bytes);
        }
        None
    }

    pub fn get_integer(&self) -> Option<i64> {
        if let Primitives::Int(num) = self {
            return Some(*num as i64);
        }
        None
    }

    pub fn get_str(&self) -> Option<&Vec<u8>> {
        if let Primitives::Str(bytes) = self {
            return Some(&bytes);
        }
        None
    }

    pub fn get_hexstr(&self) -> Option<&Vec<u16>> {
        if let Primitives::HexStr(bytes) = self {
            return Some(&bytes);
        }
        None
    }
    
    pub fn get_dict(&self) -> Option<&Dict> {
        if let Primitives::Dict(dict) = self {
            return Some(&dict);
        }
        None
    }

    pub fn get_dict_mut(&mut self) -> Option<&mut Dict> {
        if let Primitives::Dict(ref mut dict) = self {
            return Some(dict);
        }
        None
    }

    pub fn unwrap_dict(self) -> Dict {
        if let Primitives::Dict(dict) = self {
            return dict;
        }
        panic!("unwrap_dict {:?}", self);
    }

    //pub fn get_stream(&self) -> Option<Stream> {
    //    if let Primitives::Stream(stream) = self {
    //    }
    //}
}
