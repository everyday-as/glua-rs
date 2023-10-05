use byteyarn::YarnRef;

use crate::ast::{
    Block,
    Exp,
    exps::{Binary, Function, FunctionCall, Index, Member, MethodCall, TableConstructor, Unary},
    node::Node, Stat, stats::{
        Assignment, Do, For, ForIn, FunctionDef, Goto, IfElse, RepeatUntil, Return, VarDef, While,
    },
};

pub mod renderer;

pub trait Visitor<'ast> {
    // Statements
    fn visit_stat(&mut self, v: &Node<&'ast Stat<'ast>>) {
        walk_stat(self, v);
    }

    fn visit_assignment_stat(&mut self, v: &Node<&'ast Assignment<'ast>>) {
        walk_assignment_stat(self, v);
    }

    fn visit_do_stat(&mut self, v: &Node<&'ast Do<'ast>>) {
        walk_do_stat(self, v);
    }

    fn visit_for_stat(&mut self, v: &Node<&'ast For<'ast>>) {
        walk_for_stat(self, v);
    }

    fn visit_for_in_stat(&mut self, v: &Node<&'ast ForIn<'ast>>) {
        walk_for_in_stat(self, v);
    }

    fn visit_function_def_stat(&mut self, v: &Node<&'ast FunctionDef<'ast>>) {
        walk_function_def_stat(self, v);
    }

    fn visit_goto_stat(&mut self, v: &Node<&'ast Goto<'ast>>) {
        let _ = v;
    }

    fn visit_if_else_stat(&mut self, v: &Node<&'ast IfElse<'ast>>) {
        walk_if_else_stat(self, v);
    }

    fn visit_label_stat(&mut self, v: &Node<&'ast str>) {
        let _ = v;
    }

    fn visit_repeat_until_stat(&mut self, v: &Node<&'ast RepeatUntil<'ast>>) {
        walk_repeat_until_stat(self, v);
    }

    fn visit_return_stat(&mut self, v: &Node<&'ast Return<'ast>>) {
        walk_return_stat(self, v);
    }

    fn visit_var_def_stat(&mut self, v: &Node<&'ast VarDef<'ast>>) {
        walk_var_def_stat(self, v);
    }

    fn visit_while_stat(&mut self, v: &Node<&'ast While<'ast>>) {
        walk_while_stat(self, v);
    }

    fn visit_break_stat(&mut self, v: &Node<()>) {
        let _ = v;
    }

    fn visit_continue_stat(&mut self, v: &Node<()>) {
        let _ = v;
    }

    fn visit_none_stat(&mut self) {}

    // Expressions
    fn visit_exp(&mut self, v: &Node<&'ast Exp<'ast>>) {
        walk_exp(self, v);
    }

    fn visit_binary_exp(&mut self, v: &Node<&'ast Binary<'ast>>) {
        walk_binary_exp(self, v);
    }

    fn visit_function_exp(&mut self, v: &Node<&'ast Function<'ast>>) {
        walk_function_exp(self, v);
    }

    fn visit_index_exp(&mut self, v: &Node<&'ast Index<'ast>>) {
        walk_index_exp(self, v);
    }

    fn visit_member_exp(&mut self, v: &Node<&'ast Member<'ast>>) {
        walk_member_exp(self, v);
    }

    fn visit_table_exp(&mut self, v: &Node<&'ast TableConstructor<'ast>>) {
        walk_table_exp(self, v);
    }

    fn visit_unary_exp(&mut self, v: &Node<&'ast Unary<'ast>>) {
        walk_unary_exp(self, v);
    }

    fn visit_bool_exp(&mut self, v: &Node<&'ast bool>) {
        let _ = v;
    }

    fn visit_nil_exp(&mut self, v: &Node<()>) {
        let _ = v;
    }

    fn visit_number_exp(&mut self, v: &Node<&'ast f64>) {
        let _ = v;
    }

    fn visit_ref_exp(&mut self, v: &Node<&'ast str>) {
        let _ = v;
    }

    fn visit_string_exp(&mut self, v: &Node<YarnRef<'ast, [u8]>>) {
        let _ = v;
    }

    fn visit_var_args_exp(&mut self, v: &Node<()>) {
        let _ = v;
    }

    // Common
    fn visit_function_call(&mut self, v: &Node<&'ast FunctionCall<'ast>>) {
        walk_function_call(self, v);
    }

    fn visit_method_call(&mut self, v: &Node<&'ast MethodCall<'ast>>) {
        walk_method_call(self, v);
    }
}

// Statement walkers
pub fn walk_block<'ast, V>(visitor: &mut V, v: Block<'ast>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.iter().for_each(|s| visitor.visit_stat(s));
}

pub fn walk_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast Stat<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    match **v {
        Stat::Assignment(s) => visitor.visit_assignment_stat(&Node::morph(v, s)),
        Stat::Do(s) => visitor.visit_do_stat(&Node::morph(v, s)),
        Stat::For(s) => visitor.visit_for_stat(&Node::morph(v, s)),
        Stat::ForIn(s) => visitor.visit_for_in_stat(&Node::morph(v, s)),
        Stat::FunctionCall(s) => visitor.visit_function_call(&Node::morph(v, s)),
        Stat::FunctionDef(s) => visitor.visit_function_def_stat(&Node::morph(v, s)),
        Stat::Goto(s) => visitor.visit_goto_stat(&Node::morph(v, s)),
        Stat::IfElse(s) => visitor.visit_if_else_stat(&Node::morph(v, s)),
        Stat::Label(s) => visitor.visit_label_stat(&Node::morph(v, s)),
        Stat::MethodCall(s) => visitor.visit_method_call(&Node::morph(v, s)),
        Stat::RepeatUntil(s) => visitor.visit_repeat_until_stat(&Node::morph(v, s)),
        Stat::Return(s) => visitor.visit_return_stat(&Node::morph(v, s)),
        Stat::VarDef(s) => visitor.visit_var_def_stat(&Node::morph(v, s)),
        Stat::While(s) => visitor.visit_while_stat(&Node::morph(v, s)),
        Stat::Break => visitor.visit_break_stat(&Node::morph(v, ())),
        Stat::Continue => visitor.visit_continue_stat(&Node::morph(v, ())),
        Stat::None => visitor.visit_none_stat(),
    };
}

pub fn walk_assignment_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast Assignment>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.exps.iter().for_each(|e| visitor.visit_exp(e));
    v.vars.iter().for_each(|e| visitor.visit_exp(e));
}

pub fn walk_do_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast Do<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
}

pub fn walk_for_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast For<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
    visitor.visit_exp(&v.init.1);
    visitor.visit_exp(&v.test);

    if let Some(ref update) = v.update {
        visitor.visit_exp(update);
    }
}

pub fn walk_for_in_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast ForIn<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
    v.exps.iter().for_each(|e| visitor.visit_exp(e));
}

pub fn walk_function_def_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast FunctionDef<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    visitor.visit_function_exp(&v.body);
}

pub fn walk_if_else_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast IfElse<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
    visitor.visit_exp(&v.cond);

    v.else_ifs.iter().for_each(|(exp, block)| {
        block.iter().for_each(|s| visitor.visit_stat(s));
        visitor.visit_exp(exp);
    });

    if let Some(else_block) = &v.else_block {
        else_block.iter().for_each(|s| visitor.visit_stat(s));
    }
}

pub fn walk_repeat_until_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast RepeatUntil<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
    visitor.visit_exp(&v.cond);
}

pub fn walk_return_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast Return<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.exps.iter().for_each(|e| visitor.visit_exp(e));
}

pub fn walk_var_def_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast VarDef<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    if let Some(init_exps) = v.init_exps {
        init_exps.iter().for_each(|e| visitor.visit_exp(e));
    }
}

pub fn walk_while_stat<'ast, V>(visitor: &mut V, v: &Node<&'ast While<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
    visitor.visit_exp(&v.cond);
}

// Expression walkers
pub fn walk_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast Exp<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    match **v {
        Exp::Binary(e) => visitor.visit_binary_exp(&Node::morph(v, e)),
        Exp::Function(e) => visitor.visit_function_exp(&Node::morph(v, e)),
        Exp::FunctionCall(e) => visitor.visit_function_call(&Node::morph(v, e)),
        Exp::Index(e) => visitor.visit_index_exp(&Node::morph(v, e)),
        Exp::Member(e) => visitor.visit_member_exp(&Node::morph(v, e)),
        Exp::MethodCall(e) => visitor.visit_method_call(&Node::morph(v, e)),
        Exp::Table(e) => visitor.visit_table_exp(&Node::morph(v, e)),
        Exp::Unary(e) => visitor.visit_unary_exp(&Node::morph(v, e)),
        Exp::Bool(e) => visitor.visit_bool_exp(&Node::morph(v, e)),
        Exp::Nil => visitor.visit_nil_exp(&Node::morph(v, ())),
        Exp::Number(e) => visitor.visit_number_exp(&Node::morph(v, e)),
        Exp::Ref(e) => visitor.visit_ref_exp(&Node::morph(v, e)),
        Exp::String(e) => visitor.visit_string_exp(&Node::morph(v, *e)),
        Exp::VarArgs => visitor.visit_var_args_exp(&Node::morph(v, ())),
    };
}

pub fn walk_binary_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast Binary<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    visitor.visit_exp(&v.lhs);
    visitor.visit_exp(&v.rhs);
}

pub fn walk_function_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast Function<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.body.iter().for_each(|s| visitor.visit_stat(s));
}

pub fn walk_index_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast Index<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    visitor.visit_exp(&v.lhs);
    visitor.visit_exp(&v.exp);
}

pub fn walk_member_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast Member<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    visitor.visit_exp(&v.lhs);
}

pub fn walk_table_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast TableConstructor<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.fields.iter().for_each(|f| {
        visitor.visit_exp(&f.value);

        if let Some(key) = &f.key {
            visitor.visit_exp(key);
        }
    })
}

pub fn walk_unary_exp<'ast, V>(visitor: &mut V, v: &Node<&'ast Unary<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    visitor.visit_exp(&v.exp);
}

// Common walkers
pub fn walk_function_call<'ast, V>(visitor: &mut V, v: &Node<&'ast FunctionCall<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.args.iter().for_each(|e| visitor.visit_exp(e));
    visitor.visit_exp(&v.lhs);
}

pub fn walk_method_call<'ast, V>(visitor: &mut V, v: &Node<&'ast MethodCall<'ast>>)
where
    V: Visitor<'ast> + ?Sized,
{
    v.args.iter().for_each(|e| visitor.visit_exp(e));
    visitor.visit_exp(&v.lhs);
}
