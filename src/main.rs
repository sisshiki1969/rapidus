extern crate rapidus;
use rapidus::extract_anony_func;
use rapidus::fv_finder;
use rapidus::fv_solver;
use rapidus::lexer;
use rapidus::parser;
use rapidus::vm;
use rapidus::vm_codegen;

extern crate clap;
use clap::{App, Arg};

use std::fs::OpenOptions;
use std::io::prelude::*;

const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("Rapidus")
        .version(VERSION_STR)
        .author("uint256_t")
        .about("A toy JavaScript engine")
        .arg(
            Arg::with_name("easyrun")
                .help("Enable easy-run")
                .long("easy-run"),
        )
        .arg(Arg::with_name("file").help("Input file name").index(1));
    let app_matches = app.clone().get_matches();

    if let Some(filename) = app_matches.value_of("file") {
        if app_matches.is_present("easyrun") {
            easy_run(filename);
            return;
        }

        vm::vm2_test();
        
        return;

        let mut file_body = String::new();

        match OpenOptions::new().read(true).open(filename) {
            Ok(mut ok) => ok
                .read_to_string(&mut file_body)
                .ok()
                .expect("cannot read file"),
            Err(e) => {
                println!("error: {}", e);
                return;
            }
        };

        let mut lexer = lexer::Lexer::new(file_body.clone());

        println!("Lexer:");
        while let Ok(token) = lexer.next() {
            println!("{:?}", token);
        }

        let mut parser = parser::Parser::new(file_body);

        println!("Parser:");
        let mut nodes = vec![];
        while let Ok(node) = parser.next() {
            println!("{:?}", node);
            nodes.push(node);
        }

        let mut node = nodes[0].clone();
        extract_anony_func::AnonymousFunctionExtractor::new().run_toplevel(&mut node);
        fv_finder::FreeVariableFinder::new().run_toplevel(&mut node);
        fv_solver::FreeVariableSolver::new().run_toplevel(&mut node);

        println!("extract_anony_func, fv_finder, fv_solver:\n {:?}", node);
        let mut vm_codegen = vm_codegen::VMCodeGen::new();
        let mut insts = vec![];
        vm_codegen.compile(&node, &mut insts);

        for inst in insts.clone() {
            println!("{:?}", inst);
        }

        // println!("Result:");
        // let mut vm = vm::VM::new();
        // vm.global_objects.extend(vm_codegen.global_varmap);
        // vm.run(insts);

        // println!("VM CodeGen Test:");
        // vm_codegen::test();
    }
}

fn easy_run(file_name: &str) {
    let mut file_body = String::new();

    match OpenOptions::new().read(true).open(file_name) {
        Ok(mut ok) => ok
            .read_to_string(&mut file_body)
            .ok()
            .expect("cannot read file"),
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    let mut parser = parser::Parser::new(file_body);

    let mut node_list = vec![];
    while let Ok(ok) = parser.next() {
        node_list.push(ok)
    }

    extract_anony_func::AnonymousFunctionExtractor::new().run_toplevel(&mut node_list[0]);
    fv_finder::FreeVariableFinder::new().run_toplevel(&mut node_list[0]);
    fv_solver::FreeVariableSolver::new().run_toplevel(&mut node_list[0]);

    let mut vm_codegen = vm_codegen::VMCodeGen::new();
    let mut insts = vec![];
    vm_codegen.compile(&node_list[0], &mut insts);

    // for inst in insts.clone() {
    //     println!("{:?}", inst);
    // }

    println!("Result:");
    let mut vm = vm::VM::new();
    (*vm.global_objects)
        .borrow_mut()
        .extend(vm_codegen.global_varmap);
    vm.run(insts);
}
