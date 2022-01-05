// use super::program::{Instrument, Program, V};
// use crate::token::Token;

// pub struct VM {
//     pc: usize,
//     pd: usize,
//     // po: usize,
//     codes: Vec<Instrument>,
//     datas: Vec<i32>,
// }

// // pub enum RuntimeErr {
// //     StackOverFlow,
// //     DivideZero,
// //     InvalidInstrument,
// // }

// #[allow(dead_code)]
// impl VM {
//     pub fn new(data_stack_size: usize, prog: Program) -> Self {
//         VM {
//             pc: 0,
//             pd: 0,
//             po: 0,
//             datas: vec![0; data_stack_size],
//             codes: prog.inss,
//         }
//     }

//     fn getv(&self, p: V) -> i32 {
//         match p {
//             V::Direct(a) => self.datas[a],
//             V::Indirect(a) => self.datas[self.pd + a],
//         }
//     }

//     fn setv(&mut self, p: V, v: i32) {
//         match p {
//             V::Direct(a) => self.datas[a] = v,
//             V::Indirect(a) => self.datas[self.pd + a] = v,
//         }
//     }

//     pub fn execute_once(&mut self) -> bool {
//         let ins = self.codes[self.pc].clone();
//         self.pc += 1;
//         match ins {
//             Instrument::Set(a, v) => {
//                 self.setv(a, v);
//             }
//             Instrument::Mov(a, b) => {
//                 self.setv(b, self.getv(a));
//             }
//             // Instrument::Op(_t, _a, _b) => {}
//             Instrument::BinOp(t, a, b, c) => {
//                 let a = self.getv(a);
//                 let b = self.getv(b);
//                 let mut v = 0;
//                 match t {
//                     Token::Add => v = a + b,
//                     Token::Minus => v = a - b,
//                     Token::Multiply => v = a * b,
//                     Token::Divide => v = a / b,
//                     _ => {}
//                 }
//                 self.setv(c, v);
//             }
//             Instrument::Print(a) => {
//                 let a = self.getv(a);
//                 println!("{}", a);
//             }
//             Instrument::Call(_a) => {

//             }
//             Instrument::Ret => {

//             }
//             Instrument::Exit => return false,
//             // _ => {}
//         }
//         true
//     }

//     pub fn execute(&mut self) {
//         while self.pc < self.codes.len() && self.execute_once() {}
//     }
// }
