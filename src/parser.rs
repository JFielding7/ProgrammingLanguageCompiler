// use crate::lexer::Token;
// use std::rc::Rc;
// use std::slice::Iter;
// 
// pub struct Parser<'a> {
//     tokens: Iter<'a, Token>,
//     curr: usize,
// }
// 
// impl<'a> Parser<'a> {
//     pub fn new(tokens: &'a Vec<Token>) -> Self {
//         let x = tokens.iter();
//         
//         Parser {
//             tokens: x,
//             curr: 0,
//         }
//     }
// }
// 
// impl<'a> Iterator for Parser<'a> {
//     type Item = &'a Token;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.curr == self.tokens.len() {
//             None
//         } else {
//             let token = self.tokens[self.curr];
//             self.curr += 1;
//             Some(token)
//         }
//     }
// }
