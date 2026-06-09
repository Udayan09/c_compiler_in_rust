//Import required libraries
use std::env;
use std::fs;
use std::process::Command;

//Import modules
mod lexer;
mod parser;
mod generator;

use lexer::Token;
use lexer::lex;
use parser::program_parser;
use crate::generator::program_generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path= &args[1];

    println!("File Path: {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let tokens:Vec<Token> = lex(&contents);

    // println!("Tokens (Pretty): {:#?}", tokens);

    let prog = program_parser(tokens);
    println!("Program: {:#?}", prog);
    // dbg!(args);
    let asm_string = program_generator(prog);
    println!("Here is the asm code:");
    println!("{asm_string}");

    fs::write("assembly.s", asm_string)
        .expect("Write failed!");   

    run_gcc();

}

//Runs GCC to create the executable and delete the intermediate assembly file
pub fn run_gcc(){
    let mut gcc_command = Command::new("gcc");
    gcc_command.arg("assembly.s");
    gcc_command.arg("-o");
    gcc_command.arg("out");

    let status = gcc_command.status().expect("Failed to execute GCC");

    if !status.success() {
        panic!("GCC failed to assemble and link the file!");
    }

    fs::remove_file("assembly.s")
    .expect("Failed to remove file");
}

