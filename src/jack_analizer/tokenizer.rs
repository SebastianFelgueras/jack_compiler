use std::{
    fs,
    fmt::Display,
};
use crate::CompilationError;
pub struct Tokenizer{
    tokens: Vec<TokenType>,
    current_token: usize,
    content: String,
    tipo: Type,
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
        let mut coso = Tokenizer{
            content:String::new(),
            tokens: TokenType::tokenize(file)?,
            current_token: 0,
            tipo: Type::Identifier,
        };
        coso.update_content();
        Ok(coso)
    }
    pub fn current_token(&self)->&TokenType{
        &self.tokens[self.current_token]
    }
    pub fn ret_and_advance(&mut self)->&TokenType{
        self.current_token += 1;
        self.update_content();
        &self.tokens[self.current_token - 1]
    }
    fn update_content(&mut self){
        self.content = self.current_token().inner();
        self.tipo = match self.current_token(){
            TokenType::Identifier(_)=>Type::Identifier,
            TokenType::IntConst(_)=>Type::IntConst,
            TokenType::StringConst(_)=>Type::StringConst,
            TokenType::Symbol(_)=>Type::Symbol,
            TokenType::KeyWord(_)=>Type::Keyword,
        };
    }
    pub fn content(&self)->&String{
        &self.content
    }
}
#[derive(PartialEq,Debug)]
pub enum TokenType{
    KeyWord(String),
    Symbol(String),
    Identifier(String),
    IntConst(String),
    StringConst(String),
}
#[derive(PartialEq,Clone,Copy)]
pub enum Type{
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}
impl TokenType{
    pub fn tokenize(file:&str)->Result<Vec<Self>,CompilationError>{
        if let Ok(valor) = fs::read_to_string(file) {
            let mut tokens = Vec::new();
            let mut flag = false;
            for line in valor.lines(){
                if flag{
                    flag = TokenType::is_not_closing_multiline_comment(line);
                    continue;
                }
                if TokenType::is_opening_multiline_comment(line){
                    flag = true;
                    continue;
                }
                if let Some(mut value) = Self::parse_token(line) {
                    tokens.append(&mut value);
                }
            }
            Ok(tokens)
        }else{
            Err(CompilationError::FileAccessingError(file.to_string()))
        }
    }
    fn inner(&self)->String{
        match self{
            TokenType::Symbol(coso)=>format!("{}",coso),
            TokenType::StringConst(coso)=>coso.to_string(),
            TokenType::KeyWord(coso)=>coso.to_string(),
            TokenType::IntConst(coso)=>coso.to_string(),
            TokenType::Identifier(coso)=>coso.to_string(),
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
                    let caracter = match caracter{
                        '<'=>"&lt;".to_string(),
                        '>'=>"&gt;".to_string(),
                        '&'=>"&amp;".to_string(),
                        c=>format!("{}",c),
                    };
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
    fn is_opening_multiline_comment(line:&str)->bool{
        let line = line.trim();
        line.starts_with("/*") && !line.ends_with("*/")
    }
    fn is_not_closing_multiline_comment(line:&str)->bool{
        let line = line.trim();
        !line.ends_with("*/")
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
        format!("<{}>{}</{}>\n",tipo,valor,tipo)
    }
}
