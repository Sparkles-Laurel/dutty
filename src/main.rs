use atty::Stream;
use clap::{App, Shell};
use crossterm::style::{self, Color, Stylize};

fn main() {
    let _cli = App::new("dutty") // Configure the CLI
        .version("1.0.0") // Version 1.0.0
        .author("Y. Cemal Öztürk <ozturkyigitcemal@gmail.com>") // Author information
        .about("inspects the states of terminal and teletype stdio streams") // What dutty is
        .usage("dutty [-h | --help] [-V | --version]"); // and how to use it
    let (color_terminal, color_teletype) = ("TERMINAL".green(), "TELETYPE".red());
    let (is_stdout, is_stdin, is_stderr) = (
        atty::is(Stream::Stdout),
        atty::is(Stream::Stdin),
        atty::is(Stream::Stderr),
    ); // Check wheter the streams are piped
    if is_stdout {
        println!("Standard output is a {}", color_terminal); //stdout is terminal
    } else {
        eprintln!("Standard output is a {}", color_teletype); //stdout is tty
    }

    if is_stdin {
        if !is_stderr {
            println!("Standard input is a {}", color_terminal); // stdin is terminal
        } else {
            eprintln!("Standard input is a {}", color_terminal);
        }
    } else {
        if is_stderr {
            eprintln!("Standard input is a {}", color_teletype); // stdin is tty
        } else {
            println!("Standard input is a {}", color_teletype);
        }
    }

    if is_stderr {
        eprintln!("Standard error is a {}", color_terminal); // stderr is terminal
    } else {
        println!("Standard error is a {}", color_teletype); // stderr is tty
    }
}
