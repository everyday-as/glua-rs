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
    pub fn span(this: &Self) -> Span {
        this.span.clone()
    }

    pub fn morph<U>(this: &Self, inner: U) -> Node<U> {
        Node {
            span: this.span.clone(),
            inner,
        }
    }

    pub fn as_ref(this: &Self) -> Node<&T> {
        Node {
            span: this.span.clone(),
            inner: &this.inner,
        }
    }
}

impl ToString for Node<Exp> {
    fn to_string(&self) -> String {
        let mut renderer = Renderer::default();

        walk_exp(&mut renderer, Node::as_ref(self));

        renderer.into_inner()
    }
}

impl ToString for Node<Member> {
    fn to_string(&self) -> String {
        let mut renderer = Renderer::default();

        renderer.visit_member_exp(Node::as_ref(self));

        renderer.into_inner()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
