extern crate pdf;
use pdf::core::stream::{ Stream };
use pdf::core::document::{ PdfDocument };

use std::env;
use std::fs::File;
use std::path::Path;
use std::fs::{self, DirEntry};
use std::io::prelude::*;

fn main() {
    if let Ok(entries) = fs::read_dir("./samples") {
        for entry in entries {

            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        println!("{:?}", entry.path());
                        if let Some(extension) = entry.path().extension() {
                            if extension == "pdf" {
                                read(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }
}

fn read(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let bytes = file.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>();
    let _document = PdfDocument::loadData(bytes, None);
}
