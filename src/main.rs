use crate::program::{Expr, Action, Program, process_program, Class, VarVar};

mod parser;
mod program;
mod utils;

fn main() {
    let class =
        Action::Class(
            Class {
                name: "Abc".to_string(),
                stat_fields: vec![
                    Box::from(
                        VarVar {
                            name: "a".to_string(),
                            val: Expr {
                                expr: "200".to_string()
                            },
                        }
                    ),
                    Box::from(
                        VarVar {
                            name: "b".to_string(),
                            val: Expr {
                                expr: "3".to_string()
                            }
                        }
                    )
                ],
                cons: None,
                method: vec![],
            });
    let prog = Program::Act { act: class, next: Box::from(Program::EOF) };

    println!("{}", process_program(prog, 2, 0));
}
