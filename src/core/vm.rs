use super::compiler::Program;
use super::types::{calc_op_1, calc_op_2, get_op_param_num, Code, CodeAddr, ErrKind, MemAddr};

pub struct VM {
    pc: usize,
    ps: usize,
    pd: usize,
    codes: Vec<Code>,
    datas: Vec<i32>,
    control_stack: Vec<usize>,
    stop: bool,
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
            control_stack: Vec::new(),
            stop: false,
        };
        for (i, x) in prog.memory.iter().enumerate() {
            vm.datas[i] = *x;
        }
        vm
    }

    fn get_code_addr(&self, addr: CodeAddr) -> usize {
        match addr {
            CodeAddr::Direct(a) => a,
            _ => panic!("name addr in vm!"),
        }
    }

    fn geta(&self, addr: MemAddr) -> usize {
        match addr {
            MemAddr::Direct(a) => a,
            MemAddr::Indirect(a) => self.pd + a,
        }
    }

    fn pushv(&mut self, x: i32) -> Result<(), ErrKind> {
        if self.ps >= self.datas.len() {
            Err(ErrKind::StackOverFlow)
        } else {
            self.datas[self.ps] = x;
            self.ps += 1;
            Ok(())
        }
    }

    fn popv(&mut self) -> i32 {
        self.ps -= 1;
        self.datas[self.ps]
    }

    // ret output
    pub fn execute_once(&mut self) -> Result<Option<i32>, ErrKind> {
        let code = self.codes[self.pc].clone();
        self.pc += 1;
        match code {
            Code::PushValue(x) => {
                self.pushv(x)?;
            }
            Code::Push(addr) => {
                let a = self.geta(addr);
                self.pushv(self.datas[a])?;
            }
            Code::Pop(sz) => {
                self.ps -= sz;
            }
            Code::PopMov(addr) => {
                let a = self.geta(addr);
                self.datas[a] = self.popv();
            }
            Code::Op(op) => match get_op_param_num(op) {
                1 => {
                    self.datas[self.ps - 1] = calc_op_1(op, self.datas[self.ps - 1])?;
                }
                2 => {
                    self.datas[self.ps - 2] =
                        calc_op_2(op, self.datas[self.ps - 2], self.datas[self.ps - 1])?;
                    self.ps -= 1;
                }
                _ => {
                    panic!("No {} param num", get_op_param_num(op));
                }
            },
            Code::Call(code_addr, num_params) => {
                self.control_stack.push(self.pc);
                self.control_stack.push(self.pd);
                self.pd = self.ps - num_params;
                self.pc = self.get_code_addr(code_addr);
            }
            Code::Jump(code_addr) => {
                self.pc = self.get_code_addr(code_addr);
            }
            Code::CondJump(code_addr) => {
                if self.popv() == 0 {
                    self.pc = self.get_code_addr(code_addr);
                }
            }
            Code::Print => {
                let v = self.datas[self.ps - 1];
                println!("Print: {}", v);
                return Ok(Some(v));
            }
            Code::Ret(sz) => {
                // ret main
                if self.control_stack.len() == 0 {
                    self.stop = true;
                } else {
                    let nps = self.pd;
                    self.pd = self.control_stack.pop().unwrap();
                    self.pc = self.control_stack.pop().unwrap();
                    self.ps -= sz;
                    for i in 0..sz {
                        self.datas[nps + i] = self.datas[self.ps + i];
                    }
                    self.ps = nps + sz;
                }
            }
        }
        Ok(None)
    }

    pub fn execute(&mut self) -> Result<Vec<i32>, ErrKind> {
        let mut outs = Vec::new();
        while !self.stop {
            if let Some(x) = self.execute_once()? {
                outs.push(x);
            }
        }
        Ok(outs)
    }
}

#[cfg(test)]
mod tests {
    use super::VM;
    use crate::core::compiler::compile;
    use crate::utils::load_code_from_file;

    #[test]
    fn test_vm_basic() {
        let code = load_code_from_file("test_cfiles/vm/basic.c");
        let prog = compile(&code);
        assert!(prog.is_ok());
        let mut vm = VM::new(1000, prog.unwrap());
        let res = vm.execute();
        assert!(res.is_ok());
        assert_eq!(vec![0, 23], res.unwrap());
    }

    #[test]
    fn test_vm_while() {
        let code = load_code_from_file("test_cfiles/vm/while.c");
        let prog = compile(&code);
        assert!(prog.is_ok());
        let mut vm = VM::new(1000, prog.unwrap());
        let res = vm.execute();
        assert!(res.is_ok());
        assert_eq!(vec![10, 7, 4, 1], res.unwrap());
    }

    #[test]
    fn test_vm_if() {
        let code = load_code_from_file("test_cfiles/vm/if.c");
        let prog = compile(&code);
        assert!(prog.is_ok());
        let mut vm = VM::new(1000, prog.unwrap());
        let res = vm.execute();
        assert!(res.is_ok());
        assert_eq!(vec![10, 8, 6, 4, 2, 0], res.unwrap());
    }
}
