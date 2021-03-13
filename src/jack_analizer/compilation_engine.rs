use super::tokenizer::*;
use crate::CompilationError;
const SUBRUTDELC:[&str;3] = ["constructor","function","method"];
const STATEMENTS:[&str;5] = ["if","while","do","let","return"];
const OP:[&str;9] = ["+","-","*","/","&amp;","|","&lt;","&gt;","="];
const UNARY_OP:[&str;2]=["-","~"];
type Resul = Result<(),CompilationError>;
struct CompilationEngine{
    tokens: Tokenizer,
    compiled: String,
}
impl CompilationEngine{
    fn new(tokens: Tokenizer)->Self{
        CompilationEngine{
            tokens,
            compiled: String::new(),
        }
    }
    fn push_str(&mut self,str:&str){
        self.compiled.push_str(str);
    }
    fn advance_and_compile(&mut self){
        self.compiled.push_str(&self.tokens.ret_and_advance().to_xml());
    }
    pub fn compile_class(tokens: Tokenizer)->Result<String,CompilationError>{
        //tokens.print();
        let mut engine = CompilationEngine::new(tokens);
        if engine.tokens.current_token() == &TokenType::KeyWord("class".to_string()){
            engine.push_str("<class>\n");
            for _ in 0..3{ //class className{
                engine.advance_and_compile();
            }
            while (engine.tokens.content() == "static") || (engine.tokens.content() == "field"){
                engine.compile_class_decl_var()?;
            }
            while SUBRUTDELC.contains(&engine.tokens.content().as_str()){
                engine.compile_subroutine()?;
            }
            //NO REEMPLAZAR POR advance_and_compile porque ese metodo llama a ret_and_advance que llama a update content y
            //al ser el último token del archivo, cuando trata de cargar el siguente update hacxe un out of bounds
            engine.compiled.push_str(&engine.tokens.current_token().to_xml()); //}
            engine.push_str("</class>\n");
            Ok(engine.compiled)
        }else{
            Err(CompilationError::SintaxError)
        }
    }
    fn compile_class_decl_var(&mut self)->Resul{
        self.push_str("<classVarDec>\n");
        while self.tokens.content() != ";"{
            self.advance_and_compile();
        }
        self.advance_and_compile(); //agrega el ;
        self.push_str("</classVarDec>\n");
        Ok(())
    }
    fn compile_subroutine(&mut self)->Resul{
        self.push_str("<subroutineDec>\n");
        for _ in 0..4{ //agrego hasta el ( inclusive
            self.advance_and_compile();
        } 
        self.compile_parameter_list()?;
        self.advance_and_compile(); //el )
        self.push_str("<subroutineBody>\n");
        self.advance_and_compile();//{
        while self.tokens.content() == "var"{
            self.compile_var_dec();
        }
        self.compile_statements()?;
        self.advance_and_compile();//}
        self.push_str("</subroutineBody>\n");
        self.push_str("</subroutineDec>\n");
        Ok(())
    }
    fn compile_var_dec(&mut self){
        self.push_str("<varDec>\n");
        while self.tokens.content() != ";"{
            self.advance_and_compile();
        }
        self.advance_and_compile(); //;
        self.push_str("</varDec>\n");
    }
    fn compile_statements(&mut self)->Resul{
        self.push_str("<statements>\n");
        while STATEMENTS.contains(&self.tokens.content().as_str()){
            match self.tokens.content().as_str(){
                "let"=>self.compile_let()?,
                "if"=>self.compile_if()?,
                "while"=>self.compile_while()?,
                "do"=>self.compile_do()?,
                "return"=>self.compile_return()?,
                _=>return Err(CompilationError::SintaxError),
            }
        }
        self.push_str("</statements>\n");
        Ok(())
    }
    fn compile_expression(&mut self)->Resul{
        self.push_str("<expression>\n");
        self.compile_term()?;
        while OP.contains(&self.tokens.content().as_str()){
            self.advance_and_compile(); //compila la operacion
            self.compile_term()?;
        }
        self.push_str("</expression>\n");
        Ok(())
    }
    fn compile_term(&mut self)->Resul{
        self.push_str("<term>\n");
        if UNARY_OP.contains(&self.tokens.content().as_str()){
            self.advance_and_compile();self.compile_term()?; //unaryOp term
            self.push_str("</term>\n");
            return Ok(());
        }else if self.tokens.content() == "("{
            self.advance_and_compile();//(
            self.compile_expression()?;
            self.advance_and_compile();//)
            self.push_str("</term>\n");
            return Ok(())
        }
        self.advance_and_compile();
        if self.tokens.content() == "["{
            self.advance_and_compile(); //[
            self.compile_expression()?;
            self.advance_and_compile();//]
        }else if self.tokens.content() == "."{
            self.advance_and_compile();self.advance_and_compile();self.advance_and_compile(); //.subroutineName(
            self.compile_expression_list()?;
            self.advance_and_compile();//)
        }else if self.tokens.content() == "("{
            self.advance_and_compile();//(
            self.compile_expression_list()?;
            self.advance_and_compile();//)
        }
        self.push_str("</term>\n");
        Ok(())
    }
    fn compile_let(&mut self)->Resul{
        self.push_str("<letStatement>\n");
        self.advance_and_compile();self.advance_and_compile(); //let varName
        if self.tokens.content() == "["{
            self.advance_and_compile();
            self.compile_expression()?;
            self.advance_and_compile(); //]
        }
        self.advance_and_compile();//=
        self.compile_expression()?;
        self.advance_and_compile(); //;
        self.push_str("</letStatement>\n");
        Ok(())
    }
    fn compile_while(&mut self)->Resul{
        self.push_str("<whileStatement>\n");
        self.advance_and_compile(); //while
        self.advance_and_compile();//(
        self.compile_expression()?;
        self.advance_and_compile();//)
        self.advance_and_compile();//{
        self.compile_statements()?;
        self.advance_and_compile();//}
        self.push_str("</whileStatement>\n");
        Ok(())
    }
    fn compile_if(&mut self)->Resul{
        self.push_str("<ifStatement>\n");
        self.advance_and_compile();self.advance_and_compile(); //if (
        self.compile_expression()?;
        self.advance_and_compile();self.advance_and_compile(); //){
        self.compile_statements()?;
        self.advance_and_compile(); //}
        if self.tokens.content() == "else"{
            self.advance_and_compile();self.advance_and_compile(); //else{
            self.compile_statements()?;
            self.advance_and_compile();//}
        }
        self.push_str("</ifStatement>\n");
        Ok(())
    }
    fn compile_parameter_list(&mut self)->Resul{
        self.push_str("<parameterList>\n");
        while self.tokens.content() != ")"{
            self.advance_and_compile();
        }
        self.push_str("</parameterList>\n");
        Ok(())
    }
    fn compile_return(&mut self)->Resul{
        self.push_str("<returnStatement>\n");
        self.advance_and_compile(); //return
        if self.tokens.content() !=";"{
            self.compile_expression()?;
        }
        self.advance_and_compile();//;
        self.push_str("</returnStatement>\n");
        Ok(())
    }
    fn compile_do(&mut self)->Resul{
        self.push_str("<doStatement>\n");
        self.advance_and_compile();//do
        self.advance_and_compile(); //varName || className
        if self.tokens.content() == "."{
            self.advance_and_compile();//.
            self.advance_and_compile();//subroutineName
        }
        self.advance_and_compile();//(
        self.compile_expression_list()?;
        self.advance_and_compile();//)
        self.advance_and_compile();//;
        self.push_str("</doStatement>\n");
        Ok(())
    }
    ///corta en el primer )
    fn compile_expression_list(&mut self)->Resul{
        self.push_str("<expressionList>\n");
        if self.tokens.content() !=")"{
            self.compile_expression()?;
        }
        while self.tokens.content() ==","{
            self.advance_and_compile();//,
            self.compile_expression()?;
        }
        self.push_str("</expressionList>\n");
        Ok(())
    }
}
pub fn compile(tokens: Tokenizer)->Result<String,CompilationError>{
    CompilationEngine::compile_class(tokens)
}
