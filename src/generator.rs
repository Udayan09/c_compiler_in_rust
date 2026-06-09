use crate::parser::Program;
use crate::parser::Function;
use crate::parser::Statement;
use crate::parser::Expression;

use crate::parser::UnaryOperation;

pub fn program_generator(prog: Program) -> String {
    let curr_func = prog.function;
    function_generator(curr_func)
    
}

pub fn function_generator(func: Function) -> String {
    let function_name = func.name;
    let asm_string = format!(".globl {function_name}\n{function_name}:\n");
    let statement_string = statement_generator(func.body);
    let asm_string = asm_string + &statement_string;
    asm_string
}

pub fn statement_generator(statement: Statement) -> String {
    let mut asm_string = String::new();
    match statement {
        Statement::Return(exp) => {
            asm_string = expression_generator(exp);
            asm_string.push_str("ret\n");
            asm_string
        }
    }
}

pub fn expression_generator(exp: Expression) -> String {
    let mut asm_string = String::new();
    match exp {
        Expression::Constant(int_literal) => {
            asm_string = format!("movl ${int_literal}, %eax\n");
            asm_string
        }
        Expression::UnOp(operation, inner_exp) => {
            asm_string = expression_generator(*inner_exp);

            match operation {
                UnaryOperation::Negation => {
                    asm_string.push_str("negl %eax\n");
                },
                UnaryOperation::Complement => {
                    asm_string.push_str("not %eax\n");
                },
                UnaryOperation::LogicalNegation => {
                    asm_string.push_str("cmpl $0, %eax\n");
                    asm_string.push_str("movl $0, %eax\n");
                    asm_string.push_str("sete %al\n");
                },
            }
            asm_string
        }
    }
}