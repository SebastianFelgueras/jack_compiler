use std::{
    env,
    process::exit,
    path::PathBuf,
    fs,
};
//el tokenizer no lo testee al 100% pero deberia estar bien
mod jack_analizer;
fn main() {
    let argumento = env::args().skip(1).collect::<Vec<String>>();
    if argumento.len() != 1{
        println!("Expected one argument containing the name of the file/directory to compile!");
        exit(-9);
    }
    let path = PathBuf::from(&argumento[0]);
    if path.is_file(){
        jack_analizer::tokenize(&argumento[0]).unwrap();
    }else{
        for file in fs::read_dir(path).unwrap(){
            let file = file.unwrap().path();
            if file.is_file() && file.extension() == Some(std::ffi::OsStr::new("jack")){
                jack_analizer::tokenize(file.to_str().unwrap()).unwrap();
            }
        }
    }
}
#[derive(Debug)]
pub enum CompilationError{
    FileAccessingError(String),
}