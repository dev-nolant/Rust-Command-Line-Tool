use std::io::{stdin,stdout,Write};
mod ping;

struct Command {
    aliases: Vec<&'static str>,
    func: fn(&[Command], Option<&[&str]>),
    description: &'static str,
}
#[allow(dead_code)]
fn error() {
    print!("Type 'help' to see a list of commands\n\n");
}

fn clear_terminal(_commands: &[Command], _arg: Option<&[&str]>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn directory_list(_commands: &[Command], _arg: Option<&[&str]>) {
    let paths = std::fs::read_dir("./").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}

fn help(_commands: &[Command], _arg: Option<&[&str]>) {
    for command in _commands {
        println!("{:10} - {}", command.aliases.join(", "), command.description);
    }
    println!();
}

fn exit(_commands: &[Command], _arg: Option<&[&str]>) {
    std::process::exit(0);
}

fn clerxit(_commands: &[Command], _arg: Option<&[&str]>) {
    clear_terminal(_commands, None);
    std::process::exit(0);
}


fn ping(_commands: &[Command], args: Option<&[&str]>) {
    let mut destination = None;
    let mut print_all = false;

    if let Some(args) = args {
        for arg in args {
            match *arg {
                "-A" => print_all = true,
                _ => destination = Some(*arg),
            }
        }
    }

    ping::main(destination, print_all);
}

fn main() {
    
    let commands: [Command; 6] = [
        Command {
            aliases: vec!["clear", "cls"],
            func: clear_terminal,
            description: "Clears the terminal screen",
        },
        Command {
            aliases: vec!["ls", "list"],
            func: directory_list,
            description: "Lists directory contents",
        },
        Command {
            aliases: vec!["help", "h"],
            func: help,
            description: "Displays this help message",
        },
        Command {
            aliases: vec!["clerxit"],
            func: clerxit,
            description: "Cleans and exits the terminal",
        },
        Command {
            aliases: vec!["exit"],
            func: exit,
            description: "Exits the terminal",
        },
        Command {
            aliases: vec!["ping"],
            func: ping,
            description: "ping {ip} {-A=All Data}",
        },
    ];
    clear_terminal(&commands, None);
    loop {
        print!("LIP > ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let command_input = parts[0];
        let args = if parts.len() > 1 { Some(&parts[1..]) } else { None };

        let found = commands.iter().find(|cmd| cmd.aliases.contains(&command_input));

        match found {
            Some(command) => (command.func)(&commands, args),
            None => println!("Unknown command, type 'help' to see a list of commands"),
        }
    }
}


