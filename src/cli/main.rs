// src/cli/main.rs


use clap::{App, SubCommand};

fn main() {
    let matches = App::new("cli_zeus")
        .version("0.1")
        .author("NopAngel")

        .about("cli init")

        .subcommand(SubCommand::with_name("show")

            .about("show init cli commands"))

        // ... otros subcomandos ...

        .get_matches();


    // Manejo de subcomandos

    match matches.subcommand() {

        ("show", Some(_)) => {

            println!("commands:");

            println!("  show - Show command in the cli");


        },
        _ => {
            println!("Command Not Found(404), use 'show' for show commands.");

        }

    }
}
