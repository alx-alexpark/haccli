use clap::{Parser, Subcommand};

/// Example description
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Subcommand
   #[clap(subcommand)]
   command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    ExampleCommand {
        example_arg: String
    }
}

fn main() {
    let args = Args::parse();

    println!("HACCli!");

    match args.command {
        SubCommand::ExampleCommand { example_arg } => {
            println!("{}", example_arg);
        }
    }
}
