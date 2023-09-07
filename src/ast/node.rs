use std::ops::Deref;
use logos::Span;
use crate::ast::Exp;
use crate::ast::visitors::renderer::Renderer;
use crate::ast::visitors::{walk_exp};

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub span: Span,
    pub inner: T
}

impl<T> Node<T> {
    pub fn span(node: &Node<T>) -> Span {
        node.span.clone()
    }
}

impl ToString for Node<Exp> {
    fn to_string(&self) -> String {
        let mut renderer = Renderer::default();

        walk_exp(&mut renderer, &self);

        renderer.into_inner()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}