use std::sync::atomic::{AtomicUsize, Ordering};

use crate::parser::Program;
use crate::parser::Function;
use crate::parser::Statement;
use crate::parser::Expression;

use crate::parser::UnaryOperation;
use crate::parser::BinaryOperation;

static END_COUNTER: AtomicUsize = AtomicUsize::new(1);
static RIGHT_EQ_COUNTER: AtomicUsize = AtomicUsize::new(1);

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
        },
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
        },
        Expression::BinOp(operation,left_exp , right_exp) => {
            asm_string = expression_generator(*left_exp);
            match operation {
                BinaryOperation::And => {
                let end_count = END_COUNTER.fetch_add(1, Ordering::SeqCst);
                let rq_count = RIGHT_EQ_COUNTER.fetch_add(1, Ordering::SeqCst);
                asm_string.push_str("cmpl $0, %eax\n");
                asm_string.push_str(&format!("jne _righteq{rq_count}\n"));
                asm_string.push_str(&format!("jmp _end{end_count}\n"));
                asm_string.push_str(&format!("_righteq{rq_count}:\n"));
                asm_string.push_str(&expression_generator(*right_exp));
                asm_string.push_str("cmpl $0, %eax\n");
                asm_string.push_str("movl $0, %eax\n");
                asm_string.push_str("setne %al\n");
                asm_string.push_str(&format!("_end{end_count}:\n"));
                asm_string
                }
                BinaryOperation::Or => {
                    let end_count = END_COUNTER.fetch_add(1, Ordering::SeqCst);
                    let rq_count = RIGHT_EQ_COUNTER.fetch_add(1, Ordering::SeqCst);
                    asm_string.push_str("cmpl $0, %eax\n");
                    asm_string.push_str(&format!("je _righteq{rq_count}\n"));
                    asm_string.push_str("movl $1, %eax\n");
                    asm_string.push_str(&format!("jmp _end{end_count}\n"));
                    asm_string.push_str(&format!("_righteq{rq_count}:\n"));
                    asm_string.push_str(&expression_generator(*right_exp));
                    asm_string.push_str("cmpl $0, %eax\n");
                    asm_string.push_str("movl $0, %eax\n");
                    asm_string.push_str("jmp _end\n");
                    asm_string.push_str("setne %al\n");
                    asm_string.push_str(&format!("_end{end_count}:\n"));
                    asm_string
                } 
                _ => {
                    asm_string.push_str("pushq %rax\n");
                    asm_string.push_str(&expression_generator(*right_exp));
                    asm_string.push_str("popq %rcx\n");

                    match operation {
                        BinaryOperation::Addition => {
                            asm_string.push_str("addl %ecx, %eax\n");
                        },
                        BinaryOperation::Subtraction => {
                            asm_string.push_str("subl %eax, %ecx\n");
                            asm_string.push_str("movl %ecx, %eax\n");
                        },
                        BinaryOperation::Multiplication => {
                            asm_string.push_str("imul %ecx, %eax\n");
                        },
                        BinaryOperation::Division => {
                            asm_string.push_str("pushq %rax\n");
                            asm_string.push_str("movl %ecx, %eax\n");
                            asm_string.push_str("popq %rcx\n");
                            asm_string.push_str("cdq\n");
                            asm_string.push_str("idivl %ecx\n");
                        },
                        BinaryOperation::And => {

                        },
                        BinaryOperation::Or => {

                        },
                        BinaryOperation::Equal => {
                            asm_string.push_str("cmpl %eax, %ecx\n");
                            asm_string.push_str("movl $0, %eax\n");
                            asm_string.push_str("sete %al\n");
                        },
                        BinaryOperation::NotEqual => {
                            asm_string.push_str("cmpl %eax, %ecx\n");
                            asm_string.push_str("movl $0, %eax\n");
                            asm_string.push_str("setne %al\n");
                        },
                        BinaryOperation::LessThan => {
                            asm_string.push_str("cmpl %eax, %ecx\n");
                            asm_string.push_str("movl $0, %eax\n");
                            asm_string.push_str("setl %al\n");
                        },
                        BinaryOperation::LessThanEqual => {
                            asm_string.push_str("cmpl %eax, %ecx\n");
                            asm_string.push_str("movl $0, %eax\n");
                            asm_string.push_str("setle %al\n");
                        },
                        BinaryOperation::GreaterThan => {
                            asm_string.push_str("cmpl %eax, %ecx\n");
                            asm_string.push_str("movl $0, %eax\n");
                            asm_string.push_str("setg %al\n");
                        },
                        BinaryOperation::GreaterThanEqual => {
                            asm_string.push_str("cmpl %eax, %ecx\n");
                            asm_string.push_str("movl $0, %eax\n");
                            asm_string.push_str("setge %al\n");
                        },
                    }
                    asm_string
                }
            }
            
        }
    }
}