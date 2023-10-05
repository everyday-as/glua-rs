use std::fmt::{Formatter, Write};

use byteyarn::YarnRef;

use crate::ast::{
    Exp,
    exps::{
        Binary, Function, FunctionCall, Index, Member, MethodCall, table::Field, TableConstructor,
        Unary,
    },
    node::Node,
    Stat,
    stats::{
        Assignment, Do, For, ForIn, FunctionDef, Goto, IfElse, RepeatUntil, Return, VarDef, While,
    }, visitors::{Visitor, walk_block, walk_exp, walk_stat, walk_unary_exp},
};

const INDENT: &'static str = "    ";

pub struct Renderer<'a, 'f> {
    f: &'a mut Formatter<'f>,
    line: bool,
    indent: usize,
    result: std::fmt::Result,
}

impl<'a, 'f: 'a> Renderer<'a, 'f> {
    pub fn fmt<N, W>(node: N, f: &'a mut Formatter<'f>, walker: W) -> std::fmt::Result
    where
        W: FnOnce(&mut Self, N),
    {
        let mut renderer = Self {
            f,
            line: false,
            indent: 0,
            result: Ok(()),
        };

        walker(&mut renderer, node);

        renderer.result
    }

    pub fn list<'b, W, N: 'b, V, I>(&mut self, mut walker: W, values: V)
    where
        W: FnMut(&mut Self, N),
        V: IntoIterator<Item = N, IntoIter = I>,
        I: ExactSizeIterator<Item = N>,
    {
        let iter = values.into_iter();

        let len = iter.len();

        for (i, value) in iter.enumerate() {
            walker(self, value);

            if len - i > 1 {
                self.str(", ")
            }
        }
    }

    fn line(&mut self) {
        if !self.line {
            self.line = true;
            return;
        }

        let n = self.indent;

        self.try_fmt(|f| {
            f.write_char('\n')?;

            for _ in 0..n {
                f.write_str(INDENT)?
            }

            Ok(())
        })
    }

    fn indented<W, V>(&mut self, walker: W, value: V)
    where
        W: FnOnce(&mut Self, V),
    {
        self.indent += 1;
        walker(self, value);
        self.indent -= 1;
    }

    fn try_fmt<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Formatter) -> std::fmt::Result,
    {
        self.result = self.result.and_then(|_| f(self.f))
    }

    fn ch(&mut self, c: char) {
        self.try_fmt(|f| f.write_char(c))
    }

    fn str(&mut self, s: &str) {
        self.try_fmt(|f| f.write_str(s))
    }
}

impl Visitor<'_> for Renderer<'_, '_> {
    fn visit_stat(&mut self, v: &Node<&Stat>) {
        self.line();

        walk_stat(self, v);

        if matches!(
            v.into_inner(),
            Stat::FunctionCall(..) | Stat::MethodCall(..)
        ) {
            self.ch(';')
        }
    }

    fn visit_assignment_stat(&mut self, v: &Node<&Assignment>) {
        self.list(walk_exp, v.vars);

        self.str(" = ");

        self.list(walk_exp, v.exps);

        self.ch(';');
    }

    fn visit_do_stat(&mut self, v: &Node<&Do>) {
        if v.body.is_empty() {
            self.str("do end");

            return;
        }

        self.str("do");

        self.indented(walk_block, v.body);

        self.line();
        self.str("end");
    }

    fn visit_for_stat(&mut self, v: &Node<&For>) {
        self.try_fmt(|f| write!(f, "for {} = ", v.init.0));
        walk_exp(self, &v.init.1);

        self.str(", ");
        walk_exp(self, &v.test);

        if let Some(ref update) = v.update {
            self.str(", ");
            walk_exp(self, update);
        }

        if v.body.is_empty() {
            self.str(" do end");
            return;
        }

        self.str(" do");

        self.indented(walk_block, v.body);

        self.line();
        self.str("end");
    }

    fn visit_for_in_stat(&mut self, v: &Node<&ForIn>) {
        self.str("for ");

        self.list(str_walker, v.names);

        self.str(" in ");

        self.list(walk_exp, v.exps);

        if v.body.is_empty() {
            self.str(" do end");

            return;
        }

        self.str(" do");

        self.indented(walk_block, v.body);

        self.line();
        self.str("end")
    }

    fn visit_function_def_stat(&mut self, v: &Node<&FunctionDef>) {
        if v.local {
            self.str("local ")
        }

        self.try_fmt(|f| write!(f, "function {}(", v.name));

        self.list(str_walker, v.body.params);

        self.ch(')');

        self.indented(walk_block, v.body.body);

        self.line();
        self.str("end");
    }

    fn visit_goto_stat(&mut self, v: &Node<&Goto>) {
        self.str("goto ");

        self.str(v.label);

        self.ch(';')
    }

    fn visit_if_else_stat(&mut self, v: &Node<&IfElse>) {
        self.str("if ");
        walk_exp(self, &v.cond);
        self.str(" then");

        self.indented(walk_block, v.body);

        for (cond, body) in v.else_ifs {
            self.line();

            self.str("elseif ");
            walk_exp(self, cond);
            self.str(" then");

            self.indented(walk_block, *body);
        }

        self.line();
        self.str("end")
    }

    fn visit_label_stat(&mut self, v: &Node<&str>) {
        self.str("::");
        self.str(v.into_inner());
        self.str("::")
    }

    fn visit_repeat_until_stat(&mut self, v: &Node<&RepeatUntil>) {
        self.str("repeat");

        if v.body.is_empty() {
            self.str(" until ")
        } else {
            self.indented(walk_block, v.body);
            self.line();
            self.str("until ");
        }

        walk_exp(self, &v.cond)
    }

    fn visit_return_stat(&mut self, v: &Node<&Return>) {
        self.str("return");

        if !v.exps.is_empty() {
            self.ch(' ');

            self.list(walk_exp, v.exps)
        }

        self.ch(';')
    }

    fn visit_var_def_stat(&mut self, v: &Node<&VarDef>) {
        self.str("local ");

        self.list(str_walker, v.names);

        if let Some(init_exps) = v.init_exps {
            self.str(" = ");

            self.list(walk_exp, init_exps);
        }

        self.ch(';')
    }

    fn visit_while_stat(&mut self, v: &Node<&While>) {
        self.str("while ");
        walk_exp(self, &v.cond);
        self.str(" do");

        walk_block(self, v.body);

        self.line();
        self.str("end")
    }

    fn visit_break_stat(&mut self, _v: &Node<()>) {
        self.str("break;")
    }

    fn visit_continue_stat(&mut self, _v: &Node<()>) {
        self.str("continue;")
    }

    fn visit_binary_exp(&mut self, v: &Node<&Binary>) {
        self.ch('(');

        walk_exp(self, &v.lhs);

        self.try_fmt(|f| write!(f, " {} ", v.op));

        walk_exp(self, &v.rhs);

        self.ch(')');
    }

    fn visit_function_exp(&mut self, v: &Node<&Function>) {
        self.str("function(");
        self.list(str_walker, v.params);
        self.ch(')');

        if v.body.is_empty() {
            self.str(" end");

            return;
        }

        self.indented(walk_block, v.body);

        self.line();
        self.str("end")
    }

    fn visit_index_exp(&mut self, v: &Node<&Index>) {
        walk_exp(self, &v.lhs);
        self.ch('[');
        walk_exp(self, &v.exp);
        self.ch(']');
    }

    fn visit_member_exp(&mut self, v: &Node<&Member>) {
        walk_exp(self, &v.lhs);

        self.try_fmt(|f| write!(f, ".{}", v.name));
    }

    fn visit_table_exp(&mut self, v: &Node<&TableConstructor>) {
        fn walk_field(r: &mut Renderer, Field { key, value }: &Field) {
            r.line();

            if let Some(key) = key {
                match key.into_inner() {
                    Exp::String(_) | Exp::Number(_) | Exp::Bool(_) | Exp::Ref(_) => {
                        walk_exp(r, key)
                    }

                    _ => {
                        r.ch('[');
                        walk_exp(r, key);
                        r.ch(']');
                    }
                }

                r.str(" = ");
            }

            walk_exp(r, value);
        }

        self.ch('{');

        if v.fields.is_empty() {
            self.ch('}');

            return;
        }

        self.list(|r, v| r.indented(walk_field, v), v.fields);

        self.line();
        self.ch('}');
    }

    fn visit_unary_exp(&mut self, v: &Node<&Unary>) {
        self.try_fmt(|f| write!(f, "{} ", v.op));

        walk_unary_exp(self, v);
    }

    fn visit_bool_exp(&mut self, v: &Node<&bool>) {
        if *v.into_inner() {
            self.str("true")
        } else {
            self.str("false")
        }
    }

    fn visit_nil_exp(&mut self, _v: &Node<()>) {
        self.str("nil");
    }

    fn visit_number_exp(&mut self, v: &Node<&f64>) {
        self.try_fmt(|f| write!(f, "{}", **v))
    }

    fn visit_ref_exp(&mut self, v: &Node<&str>) {
        self.str(v);
    }

    fn visit_string_exp(&mut self, v: &Node<YarnRef<[u8]>>) {
        self.ch('"');

        for res in v.utf8_chunks() {
            match res {
                Ok(chunk) => self.try_fmt(|f| {
                    let mut b = 0;

                    for offset in memchr::memchr_iter(b'"', chunk.as_bytes()) {
                        if b < offset {
                            f.write_str(&chunk[b..offset])?;
                        }

                        f.write_str("\\\"")?;

                        b = offset;
                    }

                    if b < chunk.len().wrapping_sub(1) {
                        f.write_str(&chunk[b..])?;
                    }

                    Ok(())
                }),

                Err(chunk) => {
                    for b in chunk {
                        self.try_fmt(|f| {
                            f.write_char('\\')?;
                            write!(f, "{}", b)
                        })
                    }
                }
            }
        }

        self.ch('"');
    }

    fn visit_function_call(&mut self, v: &Node<&FunctionCall>) {
        walk_exp(self, &v.lhs);

        self.ch('(');

        self.list(walk_exp, v.args);

        self.ch(')')
    }

    fn visit_method_call(&mut self, v: &Node<&MethodCall>) {
        walk_exp(self, &v.lhs);

        self.ch(':');

        self.str(v.name);

        self.ch('(');

        self.list(walk_exp, v.args);

        self.ch(')');
    }
}

fn str_walker<V>(r: &mut Renderer, v: V)
where
    V: AsRef<str>,
{
    r.str(v.as_ref())
}
