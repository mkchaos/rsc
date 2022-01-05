pub use crate::core::{calc_op_1, calc_op_2, get_op_param_num, Addr, Code, Op};

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
    // pub fn new(data_stack_size: usize, prog: Program) -> Self {
    //     VM {
    //         pc: 0,
    //         pd: 0,
    //         po: 0,
    //         datas: vec![0; data_stack_size],
    //         codes: prog.inss,
    //     }
    // }

    fn getv(&self, addr: Addr) -> i32 {
        match addr {
            Addr::Direct(a) => self.datas[a],
            Addr::Indirect(a) => self.datas[self.pd + a],
        }
    }

    fn setv(&mut self, addr: Addr, v: i32) {
        match addr {
            Addr::Direct(a) => self.datas[a] = v,
            Addr::Indirect(a) => self.datas[self.pd + a] = v,
        }
    }

    pub fn execute_once(&mut self) {
        let code = self.codes[self.pc].clone();
        self.pc += 1;
        match code {
            Code::MovA(addr, a) => {
                self.setv(addr, self.getv(a));
            }
            Code::MovV(addr, v) => {
                self.setv(addr, v);
            }
            Code::PushV(v) => {
                self.datas[self.ps] = v;
                self.ps += 1;
            }
            Code::PushA(a) => {
                self.datas[self.ps] = self.getv(a);
                self.ps += 1;
            }
            Code::Pop => {
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
                self.pc = off;
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
