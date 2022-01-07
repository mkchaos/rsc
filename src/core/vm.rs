use super::types::{calc_op_1, calc_op_2, get_op_param_num, Code, CodeAddr, MemAddr, Op};
use super::compiler::Program;

pub struct VM {
    pc: usize,
    ps: usize,
    pd: usize,
    codes: Vec<Code>,
    datas: Vec<i32>,
    stop: bool,
}

#[allow(dead_code)]
impl VM {
    pub fn new(data_stack_size: usize, prog: Program) -> Self {
        let mut vm = VM {
            pc: 0,
            pd: 0,
            ps: 0,
            datas: vec![0; data_stack_size],
            codes: prog.codes,
            stop: false,
        };
        for _ in prog.memory.iter() {
            vm.ps += 1;
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

    pub fn execute_once(&mut self) {
        let code = self.codes[self.pc].clone();
        self.pc += 1;
        match code {
            Code::Push(addr) => {
                let v = self.getv(addr);
                self.datas[self.ps] = v;
                self.ps += 1;
            }
            Code::Pop(addr) => {
                self.setv(addr, self.datas[self.ps]);
                self.ps -= 1;
            }
            Code::Op(op) => match get_op_param_num(op) {
                1 => {
                    self.datas[self.ps - 1] = calc_op_1(op, self.datas[self.ps - 1]);
                }
                2 => {
                    self.datas[self.ps - 1] =
                        calc_op_2(op, self.datas[self.ps - 2], self.datas[self.ps - 1]);
                    self.ps -= 1;
                }
                _ => {
                    panic!("No {} param num", get_op_param_num(op));
                }
            },
            Code::Call(off) => {
                self.datas[self.ps] = self.pc as i32;
                self.datas[self.ps + 1] = self.pd as i32;
                self.ps += 2;
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
            Code::Ret => {
                self.pc = self.datas[self.ps - 2] as usize;
                self.pd = self.datas[self.ps - 1] as usize;
                self.ps -= 2;
            }
            Code::Exit => {
                self.stop = true;
            }
        }
    }

    pub fn execute(&mut self) {
        while !self.stop {
            self.execute_once()
        }
    }
}
