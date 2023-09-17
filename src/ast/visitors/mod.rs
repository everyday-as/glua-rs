pub mod renderer;

use crate::ast::exps::{
    Binary, Function, FunctionCall, Index, Member, MethodCall, TableConstructor, Unary,
};
use crate::ast::node::Node;
use crate::ast::stats::{
    Assignment, Do, For, ForIn, FunctionDef, Goto, IfElse, Label, RepeatUntil, Return, VarDef,
    While,
};
use crate::ast::{Block, Exp, Stat};

pub trait Visitor {
    // Statements
    fn visit_stat(&mut self, v: &Node<Stat>) {
        walk_stat(self, v);
    }

    fn visit_assignment_stat(&mut self, v: &Node<Assignment>) {
        walk_assignment_stat(self, &v);
    }

    fn visit_do_stat(&mut self, v: &Node<Do>) {
        walk_do_stat(self, &v);
    }

    fn visit_for_stat(&mut self, v: &Node<For>) {
        walk_for_stat(self, &v);
    }

    fn visit_for_in_stat(&mut self, v: &Node<ForIn>) {
        walk_for_in_stat(self, &v);
    }

    fn visit_function_def_stat(&mut self, v: &Node<FunctionDef>) {
        walk_function_def_stat(self, &v);
    }

    fn visit_goto_stat(&mut self, v: &Node<Goto>) {}

    fn visit_if_else_stat(&mut self, v: &Node<IfElse>) {
        walk_if_else_stat(self, &v);
    }

    fn visit_label_stat(&mut self, v: &Node<Label>) {}

    fn visit_repeat_until_stat(&mut self, v: &Node<RepeatUntil>) {
        walk_repeat_until_stat(self, &v);
    }

    fn visit_return_stat(&mut self, v: &Node<Return>) {
        walk_return_stat(self, &v);
    }

    fn visit_var_def_stat(&mut self, v: &Node<VarDef>) {
        walk_var_def_stat(self, &v);
    }

    fn visit_while_stat(&mut self, v: &Node<While>) {
        walk_while_stat(self, &v);
    }

    fn visit_break_stat(&mut self) {}

    fn visit_continue_stat(&mut self) {}

    fn visit_none_stat(&mut self) {}

    // Expressions
    fn visit_exp(&mut self, v: &Node<Exp>) {
        walk_exp(self, &v);
    }

    fn visit_binary_exp(&mut self, v: &Node<Binary>) {
        walk_binary_exp(self, &v);
    }

    fn visit_function_exp(&mut self, v: &Node<Function>) {
        walk_function_exp(self, &v);
    }

    fn visit_index_exp(&mut self, v: &Node<Index>) {
        walk_index_exp(self, &v);
    }

    fn visit_member_exp(&mut self, v: &Node<Member>) {
        walk_member_exp(self, &v);
    }

    fn visit_table_exp(&mut self, v: &Node<TableConstructor>) {
        walk_table_exp(self, &v);
    }

    fn visit_unary_exp(&mut self, v: &Node<Unary>) {
        walk_unary_exp(self, &v);
    }

    fn visit_bool_exp(&mut self, _v: &Node<bool>) {}

    fn visit_nil_exp(&mut self) {}

    fn visit_number_exp(&mut self, _v: &Node<f64>) {}

    fn visit_ref_exp(&mut self, _v: &Node<String>) {}

    fn visit_string_exp(&mut self, _v: &Node<String>) {}

    fn visit_var_args_exp(&mut self) {}

    // Common
    fn visit_function_call(&mut self, v: &Node<FunctionCall>) {
        walk_function_call(self, &v);
    }

    fn visit_method_call(&mut self, v: &Node<MethodCall>) {
        walk_method_call(self, &v);
    }
}

// Statement walkers
pub fn walk_block<V: Visitor + ?Sized>(visitor: &mut V, v: &Block) {
    v.iter().for_each(|s| visitor.visit_stat(&s));
}

pub fn walk_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Stat>) {
    match &v.inner {
        Stat::Assignment(s) => visitor.visit_assignment_stat(&s),
        Stat::Do(s) => visitor.visit_do_stat(&s),
        Stat::For(s) => visitor.visit_for_stat(&s),
        Stat::ForIn(s) => visitor.visit_for_in_stat(&s),
        Stat::FunctionCall(s) => visitor.visit_function_call(&s),
        Stat::FunctionDef(s) => visitor.visit_function_def_stat(&s),
        Stat::Goto(s) => visitor.visit_goto_stat(&s),
        Stat::IfElse(s) => visitor.visit_if_else_stat(&s),
        Stat::Label(s) => visitor.visit_label_stat(&s),
        Stat::MethodCall(s) => visitor.visit_method_call(&s),
        Stat::RepeatUntil(s) => visitor.visit_repeat_until_stat(&s),
        Stat::Return(s) => visitor.visit_return_stat(&s),
        Stat::VarDef(s) => visitor.visit_var_def_stat(&s),
        Stat::While(s) => visitor.visit_while_stat(&s),
        Stat::Break => visitor.visit_break_stat(),
        Stat::Continue => visitor.visit_continue_stat(),
        Stat::None => visitor.visit_none_stat(),
    };
}

pub fn walk_assignment_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Assignment>) {
    v.exps.iter().for_each(|e| visitor.visit_exp(&e));
    v.vars.iter().for_each(|e| visitor.visit_exp(&e));
}

pub fn walk_do_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Do>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
}

pub fn walk_for_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<For>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
    visitor.visit_exp(&v.init.1);
    visitor.visit_exp(&v.test);

    if let Some(update) = &v.update {
        visitor.visit_exp(&update);
    }
}

pub fn walk_for_in_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<ForIn>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
    v.exps.iter().for_each(|e| visitor.visit_exp(&e));
}

pub fn walk_function_def_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<FunctionDef>) {
    v.body.body.iter().for_each(|s| visitor.visit_stat(&s));
}

pub fn walk_if_else_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<IfElse>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
    visitor.visit_exp(&v.cond);

    v.else_ifs.iter().for_each(|(exp, block)| {
        block.iter().for_each(|s| visitor.visit_stat(&s));
        visitor.visit_exp(&exp);
    });

    if let Some(else_block) = &v.else_block {
        else_block.iter().for_each(|s| visitor.visit_stat(&s));
    }
}

pub fn walk_repeat_until_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<RepeatUntil>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
    visitor.visit_exp(&v.cond);
}

pub fn walk_return_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Return>) {
    v.exps.iter().for_each(|e| visitor.visit_exp(&e));
}

pub fn walk_var_def_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<VarDef>) {
    if let Some(init_exps) = &v.init_exps {
        init_exps.iter().for_each(|e| visitor.visit_exp(&e));
    }
}

pub fn walk_while_stat<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<While>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
    visitor.visit_exp(&v.cond);
}

// Expression walkers
pub fn walk_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Exp>) {
    match &v.inner {
        Exp::Binary(e) => visitor.visit_binary_exp(&e),
        Exp::Function(e) => visitor.visit_function_exp(&e),
        Exp::FunctionCall(e) => visitor.visit_function_call(&e),
        Exp::Index(e) => visitor.visit_index_exp(&e),
        Exp::Member(e) => visitor.visit_member_exp(&e),
        Exp::MethodCall(e) => visitor.visit_method_call(&e),
        Exp::Table(e) => visitor.visit_table_exp(&e),
        Exp::Unary(e) => visitor.visit_unary_exp(&e),
        Exp::Bool(e) => visitor.visit_bool_exp(&e),
        Exp::Nil => visitor.visit_nil_exp(),
        Exp::Number(e) => visitor.visit_number_exp(&e),
        Exp::Ref(e) => visitor.visit_ref_exp(&e),
        Exp::String(e) => visitor.visit_string_exp(&e),
        Exp::VarArgs => visitor.visit_var_args_exp(),
    };
}

pub fn walk_binary_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Binary>) {
    visitor.visit_exp(&v.lhs);
    visitor.visit_exp(&v.rhs);
}

pub fn walk_function_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Function>) {
    v.body.iter().for_each(|s| visitor.visit_stat(&s));
}

pub fn walk_index_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Index>) {
    visitor.visit_exp(&v.lhs);
    visitor.visit_exp(&v.exp);
}

pub fn walk_member_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Member>) {
    visitor.visit_exp(&v.lhs);
}

pub fn walk_table_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<TableConstructor>) {
    v.fields.iter().for_each(|f| {
        visitor.visit_exp(&f.value);

        if let Some(key) = &f.key {
            visitor.visit_exp(&key);
        }
    })
}

pub fn walk_unary_exp<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<Unary>) {
    visitor.visit_exp(&v.exp);
}

// Common walkers
pub fn walk_function_call<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<FunctionCall>) {
    v.args.iter().for_each(|e| visitor.visit_exp(&e));
    visitor.visit_exp(&v.lhs);
}

pub fn walk_method_call<V: Visitor + ?Sized>(visitor: &mut V, v: &Node<MethodCall>) {
    v.args.iter().for_each(|e| visitor.visit_exp(&e));
    visitor.visit_exp(&v.lhs);
}
