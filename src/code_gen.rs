use crate::common::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn generate_asm(parser: RootNode, source_path: &str) {
    
    //find main function
    let main_function = parser.functions.iter().find(|x| x.name == "main");

    let mut file_content = String::new();

    file_content.push_str("BITS 64\n");
    file_content.push_str("section .text\n");
    file_content.push_str("global _start\n");


    for function in parser.functions.iter(){
        file_content.push_str(&format!("{}:\n", function.name));

        for term in function.terms.iter(){
            match term {
                Term::Return(term_return) => {
                    match &term_return.value {
                        Expr::IntLit(expr_int_lit) => {
                            file_content.push_str(&format!("    mov rdi, {}\n", expr_int_lit.value));
                            file_content.push_str("    ret\n");
                        }
                    }
                }
            }
        }

        file_content.push_str("    ret\n");
    }


    file_content.push_str("_start:\n");

    match main_function {
        Some(_) => {
            file_content.push_str("    call main\n");
        }
        None => {
            file_content.push_str("    mov rdi, 0\n");
        }
    }

    file_content.push_str("    mov rax, 60\n");
    file_content.push_str("    syscall\n");


    //replace .bo with .asm
    let path = format!("{}", source_path.replace(".bo", ".asm"));
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(&path){
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(file_content.as_bytes()){
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    println!("Generated ASM");

    // use nasm to compile
    let output = std::process::Command::new("nasm")
        .arg("-felf64")
        .arg(&format!("{}", source_path.replace(".bo", ".asm")))
        .output()
        .expect("failed to execute process");

    // use ld to link
    let output = std::process::Command::new("ld")
        .arg("-o")
        .arg(&format!("{}", source_path.replace(".bo", "")))
        .arg(&format!("{}", source_path.replace(".bo", ".o")))
        .output()
        .expect("failed to execute process");

    println!("Generated executable");
}