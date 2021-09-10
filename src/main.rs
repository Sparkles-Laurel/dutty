use atty::Stream;
use clap::{App, Shell};

fn main() {
    let _cli = App::new("dutty") // Configure the CLI
        .version("1.0.0") // Version 1.0.0
        .author("Y. Cemal Öztürk <ozturkyigitcemal@gmail.com>") // Author information
        .about("inspects the states of terminal and teletype stdio streams") // What dutty is
        .usage("dutty [-h | --help] [-V | --version]");  // and how to use it
        

    let (is_stdout, is_stdin, is_stderr) = (
        atty::is(Stream::Stdout),
        atty::is(Stream::Stdin),
        atty::is(Stream::Stderr)
    ); // Check wheter the streams are piped
    if is_stdout {
        println!("Standard output is a TERMINAL"); //stdout is terminal
    } else {
        eprintln!("Standard output is a TELETYPE"); //stdout is tty
    }

    if is_stdin {
        eprintln!("Standard input is a TERMINAL"); // stdin is terminal
    } else {
        eprintln!("Standard input is a TELETYPE"); // stdin is tty
    }

    if is_stderr {
        eprintln!("Standard error is a TERMINAL"); // stderr is terminal
    } else {
        println!("Standard error is a TELETYPE"); // stderr is tty
    }
}
