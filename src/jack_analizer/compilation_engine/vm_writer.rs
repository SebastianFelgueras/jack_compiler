pub struct VMWriter{
    output: String,
}
impl VMWriter{
    pub fn new()->Self{
        VMWriter{
            output: String::new(),
        }
    }
    pub fn push(&mut self,segment: &str,index:usize){
        self.output.push_str(&format!("push {} {}\n",segment,index));
    }
    pub fn pop(&mut self,segment: &str,index:usize){
        self.output.push_str(&format!("pop {} {}\n",segment,index));
    }
    pub fn arithmetic(&mut self,op:&str){
        self.output.push_str(op);
        self.output.push('\n')
    }
    pub fn label(&mut self,label:&str){
        self.output.push_str(&format!("label {}\n",label));
    }
    pub fn goto(&mut self,label:&str){
        self.output.push_str(&format!("goto {}\n",label));
    }
    pub fn write_if(&mut self,label:&str){
        self.output.push_str(&format!("if-goto {}\n",label));
    }
    pub fn call(&mut self,function_name:&str,n_args:usize){
        self.output.push_str(&format!("call {} {}\n",function_name,n_args));
    }
    pub fn function(&mut self,function:&str,n_locals:usize){
        self.output.push_str(&format!("function {} {}\n",function,n_locals));

    }
    pub fn write_return(&mut self){
        self.output.push_str("return\n");
    }
    pub fn return_vm_code(self)->String{
        self.output
    }
}