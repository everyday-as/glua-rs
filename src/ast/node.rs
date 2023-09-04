use logos::Span;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub span: Span,
    pub inner: T
}