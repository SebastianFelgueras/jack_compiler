mod tokenizer;
mod symbol_table;
mod vm_writer;
use symbol_table::*;
use tokenizer::*;
use crate::CompilationError;
const SUBRUTDELC:[&str;3] = ["constructor","function","method"];
const STATEMENTS:[&str;5] = ["if","while","do","let","return"];
const OP:[&str;9] = ["+","-","*","/","&amp;","|","&lt;","&gt;","="];
const UNARY_OP:[&str;2]=["-","~"];
type Resul = Result<(),CompilationError>;
struct CompilationEngine{
    tokens: Tokenizer,
    compiled: vm_writer::VMWriter,
    class: String,
    tables: SymbolTable,
    label_id_number: usize,
}
impl CompilationEngine{
    fn new(tokens: Tokenizer)->Self{
        CompilationEngine{
            tokens,
            class: String::new(),
            compiled: vm_writer::VMWriter::new(),
            tables: SymbolTable::new(),
            label_id_number: 0,
        }
    }
    fn generate_unique_label(&mut self)->String{
        self.label_id_number += 1;
        format!("L{}",self.label_id_number)
    }
    fn advance(&mut self){
        self.tokens.advance();
    }
    fn content_and_advance(&mut self)->String{
        let return_val = self.tokens.content().clone(); //lifetime problem y la verdad que no me importa el rendimiento por ahora
        self.advance();
        return_val
    }
    pub fn compile_class(tokens: Tokenizer)->Result<String,CompilationError>{
        //tokens.print();
        let mut engine = CompilationEngine::new(tokens);
        if engine.tokens.current_token() == &TokenType::KeyWord("class".to_string()){
            engine.advance();//class
            engine.class = engine.content_and_advance().clone();//class name
            engine.advance();//{
            while (engine.tokens.content() == "static") || (engine.tokens.content() == "field"){
                engine.compile_class_decl_var()?;
            }
            while SUBRUTDELC.contains(&engine.tokens.content().as_str()){
                engine.compile_subroutine()?;
            }
            //el ultimo } lo dejo pagando, total no compila a VM
            Ok(engine.compiled.return_vm_code())
        }else{
            Err(CompilationError::SintaxError)
        }
    }
    fn compile_class_decl_var(&mut self)->Resul{
        let kind = match self.content_and_advance().as_str(){
            "static"=>VarKind::Static,
            "field"=>VarKind::Field,
            _=>return Err(CompilationError::SintaxError),
        };
        let tipo = self.content_and_advance().clone();
        while self.tokens.content() != ";"{
            if self.tokens.content() != ","{
                let nombre = self.content_and_advance().clone();
                self.tables.insert(tipo.clone(),kind,nombre);
            }
        }
        self.advance(); //agrega el ;
        Ok(())
    }
    fn compile_subroutine(&mut self)->Resul{
        self.tables.new_subroutine();
        let tipo = self.content_and_advance();
        let is_void = self.content_and_advance() == "void";
        let nombre = self.content_and_advance();
        if tipo == "method"{
            self.tables.insert(self.class.clone(),VarKind::Argument,"this".to_string());
        }
        self.advance();//(
        self.compile_parameter_list()?;
        self.advance(); //el )
        self.advance();//{
        while self.tokens.content() == "var"{
            self.compile_var_dec();
        }
        self.compiled.function(&format!("{}.{}",self.class,nombre), self.tables.var_count(VarKind::Var));
        if tipo == "method"{
            self.compiled.push("argument",0);
            self.compiled.pop("pointer",0);
        }else if tipo == "constructor"{
            self.compiled.push("constant", self.tables.var_count(VarKind::Field));
            self.compiled.call("Memory.alloc", 1);
            self.compiled.pop("pointer",0);
        }
        //lo hago antes de los statements ya que es lo mismo, si el stack lo dejan en cero, entonces queda el push este,
        //y sino, quedará el valor de más arriba que igual el do lo tiene que descartar.
        if is_void{
            self.compiled.push("constant",0);
        }
        self.compile_statements()?;
        self.advance();//}
        Ok(())
    }
    fn compile_var_dec(&mut self){
        self.advance(); //var
        let tipo = self.content_and_advance();
        while self.tokens.content() != ";"{
            if self.tokens.content() == ","{
                self.advance();
            }
            let nombre = self.content_and_advance();
            //println!("{}",nombre);
            self.tables.insert(tipo.clone(), VarKind::Var,nombre );
        }
        self.advance(); //;
    }
    fn compile_statements(&mut self)->Resul{
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
        Ok(())
    }
    fn compile_expression(&mut self)->Resul{
        self.compile_term()?;
        while OP.contains(&self.tokens.content().as_str()){
            let op = self.content_and_advance(); //compila la operacion
            self.compile_term()?;
            self.compiled.arithmetic(parse_op(&op)?);
        }
        Ok(())
    }
    fn compile_term(&mut self)->Resul{
        if UNARY_OP.contains(&self.tokens.content().as_str()){
            let op = self.content_and_advance();
            self.compile_term()?; //term
            if op == "-"{
                self.compiled.arithmetic("neg");
            }else{
                self.compiled.arithmetic("not");
            }
            return Ok(());
        }else if self.tokens.content() == "("{
            self.advance();//(
            self.compile_expression()?;
            self.advance();//)
            return Ok(())
        }
        match self.tokens.tipo(){
            Type::IntConst=>{
                let contenido = self.content_and_advance();
                self.compiled.push("constant",contenido.parse().unwrap());
            },
            Type::Keyword=>{
                match self.content_and_advance().as_str(){
                    "true"=>{
                        self.compiled.push("constant",1);
                        self.compiled.arithmetic("neg");
                    },
                    "false"=>{
                        self.compiled.push("constant",0);
                    },
                    "null"=>{
                        self.compiled.push("constant",0);
                    }
                    "this"=>{
                        self.compiled.push("pointer",0);
                    },
                    _=>return Err(CompilationError::SintaxError),
                }
            },
            Type::StringConst=>{
                let nousar = self.content_and_advance(); //el borrow checker se enoja sino
                let string = nousar.as_bytes();
                self.compiled.push("constant",string.len());
                self.compiled.call("String.new", 1);
                self.compiled.pop("temp",1);
                for caracter in string{
                    self.compiled.push("temp", 1);
                    self.compiled.push("constant",*caracter as usize);
                    self.compiled.call("String.appendChar", 2);
                }
                self.compiled.push("temp",1);
            },
            Type::Symbol=>return Err(CompilationError::SintaxError),
            Type::Identifier=>{
                let primer_id = self.content_and_advance();
                if self.tokens.content() == "["{
                    self.advance(); //[
                    self.compile_expression()?;
                    let kind = self.tables.kind_of(&primer_id).unwrap().segment_of();
                    let index = self.tables.index_of(&primer_id).unwrap();
                    self.compiled.push(kind, index);
                    self.compiled.arithmetic("add");
                    self.compiled.pop("pointer", 1);
                    self.compiled.push("that",0);
                    self.advance();//]
                }else if self.tokens.content() == "."{
                    self.advance();//.
                    if self.tables.kind_of(&primer_id) == None{ //no es un metodo
                        let segundo_id = self.content_and_advance();
                        self.advance(); //(
                        let n_args = self.compile_expression_list()?;
                        self.compiled.call(&format!("{}.{}",primer_id,segundo_id), n_args);
                    }else{
                        let kind = self.tables.kind_of(&primer_id).unwrap().segment_of();
                        let index = self.tables.index_of(&primer_id).unwrap();
                        let tipo = self.tables.type_of(&primer_id).unwrap().clone();
                        self.compiled.push(kind,index);
                        let segundo_id = self.content_and_advance();
                        self.advance(); //(
                        let n_args = self.compile_expression_list()?+1;
                        self.compiled.call(&format!("{}.{}",tipo,segundo_id), n_args);
                    }
                    self.advance();//)
                }else if self.tokens.content() == "("{
                    self.advance();//(
                    let n_args = self.compile_expression_list()?;
                    self.advance();//)
                    self.compiled.call(&format!("{}.{}",self.class.clone(),primer_id), n_args);
                }else{
                    let index = self.tables.index_of(&primer_id).unwrap();
                    let kind = self.tables.kind_of(&primer_id).unwrap().segment_of();
                    self.compiled.push(kind,index);
                }
            }
        }
        Ok(())
    }
    fn compile_let(&mut self)->Resul{
        self.advance();//let 
        let mut is_arr = false;
        let variable = self.content_and_advance(); 
        let kind = self.tables.kind_of(&variable).unwrap().segment_of();
        let index = self.tables.index_of(&variable).unwrap();
        if self.tokens.content() == "["{
            self.advance();
            self.compile_expression()?;
            self.advance(); //]
            self.compiled.push(kind, index);
            self.compiled.arithmetic("add");
            self.compiled.pop("temp",3);
            is_arr = true;
        }
        self.advance();//=
        self.compile_expression()?;
        if is_arr{
            self.compiled.push("temp",3);
            self.compiled.pop("pointer",1);
            self.compiled.pop("that",0);
        }else{
            self.compiled.pop(kind,index);
        }
        self.advance(); //;
        Ok(())
    }
    fn compile_while(&mut self)->Resul{
        let label = self.generate_unique_label();
        self.advance(); //while
        self.advance();//(
        self.compile_expression()?;
        self.compiled.arithmetic("not");
        self.compiled.write_if(&label);
        self.advance();//)
        self.advance();//{
        self.compile_statements()?;
        self.advance();//}
        self.compiled.label(&label);
        Ok(())
    }
    fn compile_if(&mut self)->Resul{
        let label1 = self.generate_unique_label();
        let label2 = self.generate_unique_label();
        self.advance();self.advance(); //if (
        self.compile_expression()?;
        self.compiled.arithmetic("not");
        self.compiled.write_if(&label1);
        self.advance();self.advance(); //){
        self.compile_statements()?;
        self.advance(); //}
        self.compiled.goto(&label2);
        self.compiled.label(&label1);
        if self.tokens.content() == "else"{
            self.advance();self.advance(); //else{
            self.compile_statements()?;
            self.advance();//}
        }
        self.compiled.label(&label2);
        Ok(())
    }
    fn compile_parameter_list(&mut self)->Resul{
        while self.tokens.content() != ")"{
            if self.tokens.content() == ","{
                self.advance();
            }
            let tipo = self.content_and_advance();
            let nombre = self.content_and_advance();
            self.tables.insert(tipo, VarKind::Argument,nombre);
        }
        Ok(())
    }
    fn compile_return(&mut self)->Resul{
        self.advance(); //return
        if self.tokens.content() !=";"{
            self.compile_expression()?;
        }
        self.compiled.write_return();
        self.advance();//;
        Ok(())
    }
    fn compile_do(&mut self)->Resul{
        self.advance();//do
        let mut nombre = self.content_and_advance(); //varName || className
        if self.tokens.content() == "."{
            self.advance();//.
            nombre = format!("{}.{}",nombre,self.content_and_advance());//subroutineName
        }
        self.advance();//(
        let n_args = self.compile_expression_list()?;
        self.advance();//)
        self.advance();//;
        self.compiled.call(&nombre, n_args);
        self.compiled.pop("temp",0);
        Ok(())
    }
    ///corta en el primer )
    fn compile_expression_list(&mut self)->Result<usize,CompilationError>{
        let mut i = 0;
        if self.tokens.content() !=")"{
            self.compile_expression()?;
            i +=1;
        }
        while self.tokens.content() ==","{
            self.advance();//,
            self.compile_expression()?;
            i+=1;
        }
        Ok(i)
    }
}
///Solo lo hace con funciones no unarias
fn parse_op(op:&str)->Result<&'static str,CompilationError>{
    match op{
        "+"=>Ok("add"),
        "-"=>Ok("sub"),
        "="=>Ok("eq"),
        "&lt;"=>Ok("lt"),
        "&gt;"=>Ok("gt"),
        "&amp;"=>Ok("and"),
        "|"=>Ok("or"),
        "*"=>Ok("call Math.multiply 2"),
        "/"=>Ok("call Math.divide 2"),
        _=>Err(CompilationError::SintaxError)
    }
}
pub fn compile(file: &str)->Result<String,CompilationError>{
   CompilationEngine::compile_class(tokenizer::Tokenizer::new(file)?)
}
