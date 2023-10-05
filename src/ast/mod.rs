use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

pub use exp::Exp;
pub use stat::Stat;

use crate::ast::{
    node::Node,
    visitors::{renderer::Renderer, walk_block},
};

mod exp;
pub mod exps;
pub mod node;
mod stat;
pub mod stats;
pub mod visitors;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Block<'a>(pub &'a [Node<&'a Stat<'a>>]);

impl<'a> Deref for Block<'a> {
    type Target = [Node<&'a Stat<'a>>];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Display for Block<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Renderer::fmt(*self, f, walk_block)
    }
}
