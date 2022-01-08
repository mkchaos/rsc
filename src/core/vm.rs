use super::compiler::Program;
use super::types::{
    calc_op_1, calc_op_2, get_op_param_num, Code, CodeAddr, ErrKind, MemAddr, Value,
};

pub struct VM {
    pc: usize,
    ps: usize,
    pd: usize,
    codes: Vec<Code>,
    datas: Vec<i32>,
    stop: bool,
    global_mem_size: usize,
}

#[allow(dead_code)]
impl VM {
    pub fn new(data_stack_size: usize, prog: Program) -> Self {
        let mem_len = prog.memory.len();
        if data_stack_size < mem_len + 100 {
            panic!("Too low vm size");
        }
        // println!("memory {:?}", prog.memory);
        let mut vm = VM {
            pc: prog.start_pc,
            pd: mem_len,
            ps: mem_len,
            datas: vec![0; data_stack_size],
            codes: prog.codes,
            stop: false,
            global_mem_size: mem_len,
        };
        for (i, x) in prog.memory.iter().enumerate() {
            vm.datas[i] = *x;
        }
        vm
    }

    fn getv(&self, addr: MemAddr) -> i32 {
        match addr {
            MemAddr::Direct(a) => self.datas[a],
            MemAddr::Indirect(a) => self.datas[self.pd + a],
            MemAddr::Value(a) => a,
        }
    }

    fn setv(&mut self, addr: MemAddr, v: i32) {
        match addr {
            MemAddr::Direct(a) => self.datas[a] = v,
            MemAddr::Indirect(a) => self.datas[self.pd + a] = v,
            MemAddr::Value(_) => {}
        }
    }

    pub fn execute_once(&mut self) -> Result<(), ErrKind> {
        let code = self.codes[self.pc].clone();
        self.pc += 1;
        match code {
            Code::Push(addr) => {
                if self.ps >= self.datas.len() {
                    return Err(ErrKind::StackOverFlow);
                }
                let v = self.getv(addr);
                // println!(
                //     "psuh {:?} {:?} {:?} {:?} {:?}",
                //     v, addr, self.pd, self.datas[2], self.ps
                // );
                self.datas[self.ps] = v;
                self.ps += 1;
            }
            Code::Pop(addr) => {
                self.ps -= 1;
                self.setv(addr, self.datas[self.ps]);
            }
            Code::Op(op) => match get_op_param_num(op) {
                1 => {
                    self.datas[self.ps - 1] = calc_op_1(op, self.datas[self.ps - 1])?;
                }
                2 => {
                    // println!(
                    //     "Calc {:?} {:?} {:?}",
                    //     op,
                    //     self.datas[self.ps - 2],
                    //     self.datas[self.ps - 1]
                    // );
                    self.datas[self.ps - 2] =
                        calc_op_2(op, self.datas[self.ps - 2], self.datas[self.ps - 1])?;
                    self.ps -= 1;
                }
                _ => {
                    panic!("No {} param num", get_op_param_num(op));
                }
            },
            Code::Call(off, num_params) => {
                if self.ps + 2 >= self.datas.len() {
                    return Err(ErrKind::StackOverFlow);
                }
                self.ps -= num_params;
                for i in (0..num_params).rev() {
                    self.datas[self.ps + i + 2] = self.datas[self.ps - i];
                }
                self.datas[self.ps] = self.pc as i32;
                self.datas[self.ps + 1] = self.pd as i32;
                self.pd = self.ps + 2;
                self.ps += 2 + num_params;
                match off {
                    CodeAddr::Direct(off) => self.pc = off,
                    _ => panic!("Call err"),
                }
            }
            Code::Jump(_off) => {}
            Code::CondJump(_off) => {}
            Code::Print => {
                println!("Print: {}", self.datas[self.ps - 1]);
            }
            Code::Ret(v) => {
                // ret main
                if self.pd <= self.global_mem_size {
                    self.stop = true;
                } else {
                    self.ps = self.pd;
                    self.pc = self.datas[self.ps - 2] as usize;
                    self.pd = self.datas[self.ps - 1] as usize;
                    self.ps -= 2;
                    match v {
                        Value::Int(v) => {
                            self.datas[self.ps] = v;
                            self.ps += 1;
                        }
                        _ => {}
                    }
                }
            }
            Code::Exit => {
                self.stop = true;
            }
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), ErrKind> {
        while !self.stop {
            self.execute_once()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;
    use crate::core::compiler::compile;
    use crate::utils::load_code_from_file;

    #[test]
    fn test_vm() {
        let code = load_code_from_file("example/test_vm_1.c");
        let prog = compile(&code);
        if prog.is_err() {
            println!("{:?}", prog.err());
        } else {
            let prog = prog.unwrap();
            for c in prog.codes.iter() {
                println!("{:?}", c);
            }
            let mut vm = VM::new(1000, prog);
            match vm.execute() {
                Err(e) => {
                    println!("vm {:?}", e);
                }
                _ => {}
            }
            println!("Big ok");
        }
    }
}
