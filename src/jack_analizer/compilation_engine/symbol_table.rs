use std::collections::HashMap;
#[derive(PartialEq,Clone,Copy,Debug)]
pub enum VarKind{
    Argument,
    Var,
    Field,
    Static,
}
impl VarKind{
    pub fn segment_of(&self)->&'static str{
        match self{
            VarKind::Argument=>"argument",
            VarKind::Field=>"this",
            VarKind::Static=>"static",
            VarKind::Var=>"local",
        }
    }
}
#[derive(Debug)]
struct VariableData{
    tipo: String,
    kind: VarKind,
    index:usize,
}
#[derive(Debug)]
pub struct SymbolTable{
    n_arg: usize,
    n_var: usize,
    n_static: usize,
    n_field: usize,
    subrutine_table: HashMap<String,VariableData>,
    class_table: HashMap<String,VariableData>,
}
impl SymbolTable{
    pub fn new()->Self{
        SymbolTable{
            n_arg: 0,
            n_var: 0,
            n_static: 0,
            n_field: 0,
            subrutine_table: HashMap::new(),
            class_table: HashMap::new(), 
        }
    }
    pub fn new_subroutine(&mut self){
        self.subrutine_table.clear();
        self.n_var = 0;
        self.n_arg = 0;
    }
    pub fn insert(&mut self,tipo:String,kind:VarKind,name:String){
        match &kind{
            VarKind::Var=>{
                self.subrutine_table.insert(name,
                    VariableData{
                        tipo,
                        kind,
                        index: self.n_var,
                    }
                 );
                 self.n_var += 1;
            },
            VarKind::Static=>{
                self.class_table.insert(name,
                    VariableData{
                        tipo,
                        kind,
                        index: self.n_static,
                    }
                 );
                 self.n_static += 1;
            },
            VarKind::Field=>{
                self.class_table.insert(name,
                    VariableData{
                        tipo,
                        kind,
                        index: self.n_field,
                    }
                 );
                 self.n_field += 1;
            },
            VarKind::Argument=>{
                self.subrutine_table.insert(name,
                    VariableData{
                        tipo,
                        kind,
                        index: self.n_arg,
                    }
                 );
                 self.n_arg += 1;
            },

        }
    }
    pub fn var_count(&self,kind:VarKind)->usize{
        match kind{
            VarKind::Argument=>self.n_arg,
            VarKind::Field=>self.n_field,
            VarKind::Static=>self.n_static,
            VarKind::Var=>self.n_var,
        }
    }
    pub fn kind_of(&self,name:&String)->Option<VarKind>{
        if let Some(v)=self.retrieve(name){
            Some(v.kind)
        }else{
            None
        }
    }
    pub fn type_of(&self,name:&String)->Option<&String>{
        if let Some(v)=self.retrieve(name){
            Some(&v.tipo)
        }else{
            None
        }
    }
    pub fn index_of(&self,name:&String)->Option<usize>{
        if let Some(v)=self.retrieve(name){
            Some(v.index)
        }else{
            None
        }
    }
    fn retrieve(&self,name:&String)->Option<&VariableData>{
        if let Some(valor) = self.subrutine_table.get(name){
            Some(valor)
        }else{
            self.class_table.get(name)
        }
    }
}