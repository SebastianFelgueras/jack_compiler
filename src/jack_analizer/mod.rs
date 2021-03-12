mod tokenizer;
mod compilation_engine;
use crate::CompilationError;
use std::path::PathBuf;
pub fn tokenize(file:&str)->Result<(),CompilationError>{
    let mut path = PathBuf::from(file);
    let mut file_name = path.file_name().unwrap().to_os_string();
    file_name.push("T");
    path.with_extension("xml");
    path.set_file_name(&file_name);
    tokenizer::Tokenizer::new(file)?.to_xml(path.to_str().unwrap())
}