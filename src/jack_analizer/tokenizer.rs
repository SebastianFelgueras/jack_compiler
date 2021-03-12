use std::{
    fs,
    fmt::Display,
};
use crate::CompilationError;
pub struct Tokenizer{
    tokens: Vec<TokenType>,
    current_token: usize,
}
const SIMBOLS: [char;20] = [
    '{',
    '}',
    '(',
    ')',
    '[',
    ']',
    '.',
    ',',
    ';',
    '+',
    '-',
    '*',
    '/',
    '&',
    '|',
    '<',
    '>',
    '=',
    '-',
    '~'
];
const KEYWORDS: [&str;21] = [ 
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return"
];
impl Tokenizer{
    pub fn new(file: &str)->Result<Self,CompilationError>{
        Ok(Tokenizer{
            tokens: TokenType::tokenize(file)?,
            current_token: 0,
        })
    }
    pub fn advance(&mut self){
        self.current_token += 1;
    }
    pub fn current_token(&self)->&TokenType{
        &self.tokens[self.current_token]
    }
    pub fn more_tokens_left(&self)->bool{
        self.tokens.len() >= self.current_token
    }
    pub fn to_xml(&self,file: &str)->Result<(),CompilationError>{
        let mut acum = String::from("<tokens>\n");
        for token in &self.tokens{
            acum.push_str(&token.to_xml());
            acum.push('\n');
        }
        acum.push_str("</tokens>\n");
        match fs::write(file, acum){
            Ok(_)=>return Ok(()),
            Err(_)=>return Err(CompilationError::FileAccessingError(file.to_string())),
        }
    }
}
enum TokenType{
    KeyWord(String),
    Symbol(char),
    Identifier(String),
    IntConst(String),
    StringConst(String),
}
impl TokenType{
    pub fn tokenize(file:&str)->Result<Vec<Self>,CompilationError>{
        if let Ok(valor) = fs::read_to_string(file) {
            let mut tokens = Vec::new();
            for line in valor.lines(){
                if let Some(mut value) = Self::parse_token(line) {
                    tokens.append(&mut value);
                }
            }
            Ok(tokens)
        }else{
            Err(CompilationError::FileAccessingError(file.to_string()))
        }
    }
    fn drain_accumulator(tokens: &mut Vec<TokenType>,acumulador:&mut String){
        if KEYWORDS.contains(&acumulador.as_str()){
            tokens.push(TokenType::KeyWord(acumulador.clone()));
            acumulador.clear();
        }else if acumulador.parse::<u16>().is_ok(){
            tokens.push(TokenType::IntConst(acumulador.clone()));
            acumulador.clear();
        }else if !acumulador.is_empty(){
            tokens.push(TokenType::Identifier(acumulador.clone()));
            acumulador.clear();
        }
    }
    pub fn parse_token(line:&str)->Option<Vec<TokenType>>{
        if let Some(valor) = Self::strip_token(line) {
            let mut tokens = Vec::new();
            let mut acumulador = String::new();
            let mut flag = false;
            for caracter in valor.chars(){
                if caracter == '"'{
                    if flag{
                        tokens.push(TokenType::StringConst(acumulador.clone()));
                        acumulador.clear();
                    }
                    flag = !flag;
                    continue;
                }
                if flag{
                    acumulador.push(caracter);
                    continue;
                }
                if SIMBOLS.contains(&caracter){
                    Self::drain_accumulator(&mut tokens,&mut acumulador);
                    tokens.push(TokenType::Symbol(caracter));
                }else if caracter.is_whitespace(){
                    Self::drain_accumulator(&mut tokens,&mut acumulador);
                }else{
                    acumulador.push(caracter);
                }
            }
            Some(tokens)
        }else{
            None
        }
    }
    fn strip_token(mut line:&str)->Option<&str>{
        line = line.trim();
        if line.starts_with("//") || line.starts_with("/**"){
            return None;
        }else{
            if let Some(valor) = line.find("//"){
                line = &line[0..valor];
                if line.is_empty(){
                    return None;
                }else{
                    return Some(line);
                }
            }else{
                if line.is_empty(){
                    return None;
                }else{
                    return Some(line);
                }
            }
        }
    }
    pub fn to_xml(&self)->String{
        match self{
            TokenType::Symbol(coso)=>Self::plantilla_xml("symbol",coso),
            TokenType::StringConst(coso)=>Self::plantilla_xml("stringConstant",coso),
            TokenType::KeyWord(coso)=>Self::plantilla_xml("keyword",coso),
            TokenType::IntConst(coso)=>Self::plantilla_xml("integerConstant",coso),
            TokenType::Identifier(coso)=>Self::plantilla_xml("identifier",coso),
        }
    }
    fn plantilla_xml<T: Display>(tipo:&str,valor:T)->String{
        format!("<{}>{}</{}>",tipo,valor,tipo)
    }
}
