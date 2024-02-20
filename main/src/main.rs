use anyhow::Context;
use clap::Parser;

//
// (ref.) [clap - _derive](https://docs.rs/clap/latest/clap/_derive/index.html)
//

#[derive(clap::Subcommand)]
enum AlgorithmsCommand {
    GCD { x: u32, y: u32 },
    LCM { x: u32, y: u32 },
    Exponential { base: u32, power: u32 },
    PrimesUpTo { upper_bound: usize },
}

#[derive(clap::Subcommand)]
enum GamesCommand {
    Guessing,
}

#[derive(clap::Subcommand)]
enum ToolsCommand {
    BaseConverter,
    PersonGenerator {
        name: String,
        birthday_str: String,
        address_room: String,
        address_building: String,
        address_street: String,
        address_district: String,
        #[arg(short, long)]
        out_path_str: String,
    },
    QueryParamsParser {
        input: String,
    },
}

#[derive(clap::Subcommand)]
enum Command {
    Algorithms {
        #[command(subcommand)]
        command: AlgorithmsCommand,
    },
    Games {
        #[command(subcommand)]
        command: GamesCommand,
    },
    Tools {
        #[command(subcommand)]
        command: ToolsCommand,
    },
}

#[derive(Parser)]
#[command(about = "An example CLI")]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = CLI::parse();
    if let Err(err) = (|| -> anyhow::Result<()> {
        match cli.command {
            Command::Algorithms { command } => match command {
                AlgorithmsCommand::GCD { x, y } => {
                    println!("gcd of {x} and {y} = {}", algorithms::gcd(x, y));
                }
                AlgorithmsCommand::LCM { x, y } => {
                    println!("lcm of {x} and {y} = {}", algorithms::lcm(x, y));
                }
                AlgorithmsCommand::Exponential { base, power } => {
                    println!("{base}^{power} = {}", algorithms::exponential(base, power));
                }
                AlgorithmsCommand::PrimesUpTo { upper_bound } => {
                    println!("{:?}", algorithms::primes_up_to(upper_bound));
                }
            },
            Command::Games { command } => match command {
                GamesCommand::Guessing => games::guessing::main().context("guessing::main")?,
            },
            Command::Tools { command } => match command {
                ToolsCommand::BaseConverter => tools::base_converter::main(),
                ToolsCommand::PersonGenerator {
                    name,
                    birthday_str,
                    address_room,
                    address_building,
                    address_street,
                    address_district,
                    out_path_str: out_file,
                } => tools::person_generator::main(
                    name,
                    birthday_str,
                    address_room,
                    address_building,
                    address_street,
                    address_district,
                    out_file,
                )
                .context("person_generator::main")?,
                ToolsCommand::QueryParamsParser { input } => {
                    tools::query_params_parser::main(input.as_str())
                        .context("query_param_parser::main")?
                }
            },
        };
        Ok(())
    })() {
        common::print_err(err);
    }
}
