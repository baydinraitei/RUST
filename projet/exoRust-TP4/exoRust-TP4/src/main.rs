use crate::Instruction::*;
use std::{
    env,
    io::{self, Read},
};
use substring::Substring;
enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
    Lis,
    Boucle(Vec<Instruction>),
}

fn parse(source: String) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;
    let mut i = 0;
    for symbol in source.chars() {
        if loop_stack == 0 {
            match symbol {
                '>' => program.push(Droite),
                '<' => program.push(Gauche),
                '+' => program.push(Plus),
                '-' => program.push(Moins),
                '.' => program.push(Affiche),
                ',' => program.push(Lis),
                '[' => {
                    loop_start = i;
                    loop_stack += 1;
                }
                _ => (),
            };
        } else {
            match symbol {
                '[' => {
                    loop_stack += 1;
                }
                ']' => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Boucle(parse(
                            source.substring(loop_start + 1, i).to_string(),
                        )));
                    }
                }
                _ => (),
            }
        }
        i += 1
    }
    program
}

fn interpreteur(mut memoire: &mut Vec<i32>, instructions: &Vec<Instruction>, curpos: &mut usize) {
    if 0 == memoire.len() {
        memoire.push(0)
    }
    for i in instructions {
        match i {
            Plus => memoire[*curpos] += 1,
            Moins => memoire[*curpos] -= 1,
            Droite => {
                if (*curpos + 1) < memoire.len() {
                    *curpos += 1
                } else {
                    memoire.push(0);
                    *curpos += 1
                }
            }
            Gauche => {
                if (*curpos - 1) == 0 {
                    *curpos = 0
                } else {
                    *curpos -= 1
                }
            }
            Affiche => print!(
                "{}",
                std::char::from_u32(memoire[*curpos] as u32).unwrap_or('?')
            ),
            Lis => {
                let mut character = [0];
                match io::stdin().read(&mut character) {
                    Ok(_) => memoire[*curpos] = character[0] as i32,
                    Err(_) => println!("Erreur de lecture"),
                }
            }
            Boucle(x) => {
                while memoire[*curpos] != 0 {
                    interpreteur(&mut memoire, &x, curpos)
                }
            }
        }
    }
}

fn runbrainfuck(file: &str) {
    let mut mem = vec![];

    match std::fs::read_to_string(file.to_string()) {
        Ok(x) => interpreteur(&mut mem, &parse(x), &mut 0),
        Err(_) => println!("Fichier introuvable"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg.substring(0, 6) == "/file:" {
            runbrainfuck(arg.substring(6, arg.len()));
        }
    }
}
