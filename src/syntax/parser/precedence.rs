use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::*;

const BINARY_OPERATOR_GROUPS_COUNT: usize = 3;
const BINARY_OPERATOR_GROUPS: [&[TokenType]; BINARY_OPERATOR_GROUPS_COUNT] = [
    &[Equals],
    &[Plus, Minus],
    &[Star, Slash, Percent]
];

#[derive(Copy, Clone)]
pub struct BinaryOperatorGroup {
    group_index: usize,
}

impl BinaryOperatorGroup {
    fn new(group_index: usize) -> Self {
        Self { group_index }
    }

    pub fn lowest_precedence() -> Self {
        Self::new(0)
    }

    pub fn contains(&self, token: &Token) -> bool {
        for token_type in BINARY_OPERATOR_GROUPS[self.group_index] {
            if token == token_type {
                return true;
            }
        }

        false
    }

    pub fn next_lowest_precedence_group(&self) -> Option<Self> {
        if self.group_index + 1 == BINARY_OPERATOR_GROUPS_COUNT {
            None
        } else {
            Some(Self::new(self.group_index + 1))
        }
    }
}

pub struct BinaryOperatorGroupIterator {
    curr_group: Option<BinaryOperatorGroup>,
}

impl BinaryOperatorGroupIterator {
    pub fn starting_with(curr_group: Option<BinaryOperatorGroup>) -> Self {
        Self { curr_group }
    }
}

impl Iterator for BinaryOperatorGroupIterator {
    type Item = BinaryOperatorGroup;

    fn next(&mut self) -> Option<Self::Item> {
        let next_group = self.curr_group;
        
        if let Some(group) = next_group {
            self.curr_group = group.next_lowest_precedence_group();
            Some(group)
        } else {
            None
        }
    }
}
