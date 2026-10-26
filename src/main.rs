use {todo::*, dsc_lib::*};
use std::{fs::File, io::Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let user = match std::env::var("USER") {
        Ok(name) => name,
        Err(_) => return Err(String::from("Can't get your username")),
    };
    let dir = format!("/home/{}/.config/todo", &user);
    let file = format!("{}/list", &dir);

    let _ = func::check_dir(&dir)?;

    match args.len() {
        2 => {
            match &args[1][..] {
                "help" | "h" => println!("{}", consts::HELP_INFO),
                "list" | "l" => {
                    let list = match std::fs::read_to_string(&file) {
                        Ok(st) => st,
                        Err(_) => {

                            match File::create(&file) {
                                Ok(_) => (),
                                Err(_) => return Err(format!("Can't create the {} file", &file)),
                            }

                            return Ok(());
                        },
                    };
                    println!("{}", list);
                },
                _ => return Err(String::from(consts::ERROR)),
            }
        },
        3 => {
            let st = match std::fs::read_to_string(&file) {
                Ok(s) => s,
                Err(_) => return Err(format!("Can't read the {} file", &file)),
            };

            match &args[1][..] {
                "new" | "n" => {
                    let mut v = Var::collect_to_vec(&st);
                    let new = Var::parse_line(&args[2][..]);
                    v.push(new);
                    let _ = Var::write_var_vec(&v, &file)?;
                },
                "delete" | "d" => {
                    let mut buffer = String::new();
                    for i in st.lines() {
                        let var = Var::parse_line(i);
                        if var.get_name() == &args[2] {
                            buffer.push_str(i);
                        }
                    }

                    let mut f = match File::create(&file) {
                        Ok(f) => f,
                        Err(_) => return Err(String::from("Error: can't create the file!")),
                    };

                    match f.write_all(buffer.as_bytes()) {
                        Ok(()) => (),
                        Err(_) => return Err(String::from("Error: can't write to the file!")),
                    }
                },
                "search" | "s" => {
                    for i in st.lines() {
                        let var = Var::parse_line(i);
                        if var.get_name() == &args[2] {
                            println!("{}", i);
                        }
                    }
                },
                _ => return Err(String::from(consts::ERROR)),
            }
        },
        _ => return Err(String::from(consts::ERROR)),
    }

    Ok(())
}
