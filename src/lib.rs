#![allow(unused, unused_imports)]

use std::{env, process, io, fs};


pub mod files;
pub mod dir;

#[derive(Debug)]
pub struct CliParams<'a> {
    command: &'a str,
    category: &'a str,
    name: &'a str,
    running: bool
}

impl<'a> CliParams<'a> {
    fn build(args: &'a [String]) -> Result<CliParams, &'static str> {
        if args.len() < 3 {
            return Err("please check the argments, should be 2 (search string & file name)");
        }
        Ok(CliParams{
            category: &args[1],
            command: &args[2],
            name: "",
            running: true
        })
    }

    fn new () -> CliParams <'a> {
        CliParams{
            category: "",
            command: "",
            name: "",
            running: true
        }
    }
}

pub fn start () {
    println!("Please enter directory/file name and command >");
    loop {
        let mut config_var = CliParams::new();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.contains("end") {
                    println!("Cli tool is shutting down on command");
                    break;
                }
                let inp = &input;
                let instructions: Vec<&str> = inp.trim().split(' ').collect();
                if instructions.len() < 3 {
                    println!("please specify the correct number of command arguments !!!");
                } else {
                    config_var.command = instructions[0];
                    config_var.category = instructions[1];
                    config_var.name = instructions[2];
                    match config_var.category {
                        "file" => files::call(&config_var),
                        "dir" => dir::call(&config_var),
                        _ => println!("Invalid command: please specify correct command arguments..")
                    };
                }
            },
            Err(e) => println!("There was an error: {e}")
        }
        println!(">");
    }
    
}



