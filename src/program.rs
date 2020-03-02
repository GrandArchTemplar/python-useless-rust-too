use std::ops::Add;
use std::borrow::Borrow;
use std::iter::repeat;


pub struct Expr { pub expr: String }

pub struct VarVar { pub name: String, pub val: Expr }

pub struct If { condition: Expr, left: Box<Action>, right: Box<Action> }

pub struct ForEach { var: String, object: String, body: Box<Action> }

pub struct Function { name: String, args: Vec<String>, body: Box<Program> }

pub struct Class { pub name: String, pub stat_fields: Vec<Box<VarVar>>, pub cons: Option<Box<Function>>, pub method: Vec<Box<Function>> }

pub enum Action {
    VarVar(VarVar),
    If(If),
    ForEach(ForEach),
    Function(Function),
    Class(Class),
}

pub enum Program {
    Act { act: Action, next: Box<Program> },
    EOF,
}

fn ident(width: i64, level: i64) -> String {
    return repeat(" ").take((width * level) as usize).collect::<String>();
}

fn process_action(input: Action, width: i64, level: i64) -> String {
    let program = "".to_string();
    return match input {
        Action::VarVar(inner) =>
            program.add([
                ident(width, level).borrow(),
                inner.name.borrow(),
                " = ",
                inner.val.expr.borrow(),
                "\n"
            ].concat().as_str()),
        Action::If(inner) =>
            program.add([
                [
                    ident(width, level).borrow(),
                    "if ",
                    inner.condition.expr.borrow(),
                    ":\n"
                ].concat(),
                process_action(*inner.left, width, level + 1).to_string(),
                [
                    ident(width, level).borrow(),
                    "else:\n"
                ].concat(),
                process_action(*inner.right, width, level + 1).to_string()
            ].concat().as_str())
        ,
        Action::ForEach(inner) =>
            program.add([
                [
                    ident(width, level).borrow(),
                    "for ", inner.var.borrow(),
                    " in ", inner.object.borrow()
                ].concat(),
                process_action(*inner.body, width, level + 1).to_string()
            ].concat().as_str()),
        Action::Function(inner) =>
            program
                .add([
                    [
                        ident(width, level).borrow(),
                        "def ",
                        inner.name.borrow(),
                        "(",
                        inner.args.join(", ").borrow(),
                        ")"
                    ].concat(),
                    process_program(*inner.body, width, level + 1).to_string()
                ].concat().as_str()),
        Action::Class(inner) => program
            .add([
                [
                    "class ",
                    inner.name.borrow(),
                    ":\n"
                ].concat(),
                match inner.stat_fields.len() {
                    0 => "".to_string(),
                    _ => [
                        inner
                            .stat_fields
                            .into_iter()
                            .map(|x| process_action(Action::VarVar(*x), width, level + 1))
                            .collect::<Vec<String>>().concat(),
                        "\n".to_string()
                    ].concat().to_string()
                },
                match inner.cons {
                    None => "".to_string(),
                    Some(cns) => [
                        process_action(Action::Function(*cns), width, level + 1),
                        "\n".to_string()
                    ].concat()
                },
                match inner.method.len() {
                    0 => "".to_string(),
                    _ => [
                        inner
                            .method
                            .into_iter()
                            .map(|x| process_action(Action::Function(*x), width, level + 1))
                            .collect::<Vec<String>>().concat(),
                        "\n".to_string()
                    ].concat().to_string()
                }
            ].concat().as_str())
    };
}

pub fn process_program(prog: Program, width: i64, level: i64) -> String {
    let program = "".to_string();
    return match prog {
        Program::Act { act, next } =>
            program
                .add([
                    process_action(act, width, 0),
                    process_program(*next, width, level)
                ].concat().as_str()),
        Program::EOF =>
            program.add("//Created by GrandArchTemplar")
    };
}

pub fn add_program(left: Program, right: Program) -> Program {
    return match left {
        Program::Act { act, next } =>
            Program::Act { act, next: Box::from(add_program(*next, right)) },
        Program::EOF => return match right {
            Program::Act { act, next } =>
                Program::Act { act, next },
            Program::EOF =>
                Program::EOF,
        },
    };
}

