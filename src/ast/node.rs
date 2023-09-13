use crate::ast::exps::Member;
use crate::ast::visitors::renderer::Renderer;
use crate::ast::visitors::{walk_exp, Visitor};
use crate::ast::Exp;
use logos::Span;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub span: Span,
    pub inner: T,
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
impl ToString for Node<Member> {
    fn to_string(&self) -> String {
        let mut renderer = Renderer::default();

        renderer.visit_member_exp(&self);

        renderer.into_inner()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
