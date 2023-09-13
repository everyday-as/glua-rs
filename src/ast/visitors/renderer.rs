use crate::ast::exps::Member;
use crate::ast::node::Node;
use crate::ast::visitors::{walk_exp, Visitor};

pub struct Renderer {
    pub inner: String,
}

impl Default for Renderer {
    fn default() -> Self {
        let inner = String::new();

        Self { inner }
    }
}

impl Renderer {
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl Visitor for Renderer {
    fn visit_member_exp(&mut self, v: &Node<Member>) {
        walk_exp(self, &v.lhs);

        self.inner.push_str(&format!(".{}", v.name));
    }

    fn visit_ref_exp(&mut self, v: &Node<String>) {
        self.inner.push_str(v);
    }
}
