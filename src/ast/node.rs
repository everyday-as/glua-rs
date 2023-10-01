use std::ops::Deref;

use logos::Span;

use crate::ast::{
    exps::Member,
    visitors::{renderer::Renderer, walk_exp, Visitor},
    Exp,
};

#[derive(Clone, Copy, Debug)]
pub struct Node<T> {
    span: (usize, usize),
    inner: T,
}

// impl<T> Copy for Node<T> where T: Copy {}

impl<T> Node<T> {
    pub fn new(span: Span, inner: T) -> Self {
        Self {
            span: (span.start, span.end),
            inner,
        }
    }

    pub fn span(&self) -> Span {
        self.span.0..self.span.1
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    #[inline(always)]
    pub fn map<U>(this: Self, cb: impl FnOnce(T) -> U) -> Node<U> {
        Node {
            span: this.span,
            inner: cb(this.inner),
        }
    }

    #[inline(always)]
    pub fn morph<U>(this: &Self, inner: U) -> Node<U> {
        Node {
            span: this.span,
            inner,
        }
    }
}

impl ToString for Node<&Exp<'_>> {
    fn to_string(&self) -> String {
        let mut renderer = Renderer::default();

        walk_exp(&mut renderer, self);

        renderer.into_inner()
    }
}

impl ToString for Node<&Member<'_>> {
    fn to_string(&self) -> String {
        let mut renderer = Renderer::default();

        renderer.visit_member_exp(self);

        renderer.into_inner()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
