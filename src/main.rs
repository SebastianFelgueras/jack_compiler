use std::{
    env,
    process::exit,
    path::PathBuf,
    fs,
};
//NO HACE NINGUN CHEQUEO SOBRE EL CODIGO, PUEDE QUE COMPILE COSAS SIN SENTIDO
const COMPEXT:&str = "xmlC"; //la C es para evitar que sobreescriba los archivos de testeo
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
        fs::write(path.with_extension(COMPEXT), jack_analizer::compile(&argumento[0]).unwrap()).unwrap();
    }else{
        for file in fs::read_dir(path).unwrap(){
            let file = file.unwrap().path();
            if file.is_file() && file.extension() == Some(std::ffi::OsStr::new("jack")){
                fs::write(file.with_extension(COMPEXT), jack_analizer::compile(file.to_str().unwrap()).unwrap()).unwrap();
            }
        }
    }
}
#[derive(Debug)]
pub enum CompilationError{
    FileAccessingError(String),
    SintaxError,
}