
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DustCommand {
    Put,
    Get,
    Mk,
    Cd,
    Show,
}

pub fn run(cmd: DustCommand) {
    match cmd {
        DustCommand::Put => put(),
        DustCommand::Get => get(),
        DustCommand::Mk => mk(),
        DustCommand::Cd => cd(),
        DustCommand::Show => show(),
    }
}

fn put() {
    println!("put");
}

fn get() {
    println!("get");
}

fn mk() {
    println!("mk");
}

fn cd() {
    println!("cd");
}

fn show() {
    println!("show");
}
