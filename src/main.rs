// SPDX-License-Identifier: AGPL-3.0-or-later

use docopt::Docopt;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use serde::{
    de::{Deserializer, Visitor},
    Deserialize,
};
use std::fmt;
use std::{error::Error, process};

mod migrations;

mod infra;
mod presentation;
mod usecase;

const USAGE: &'static str = "
Technopolis backend

Usage:
    Technopolis [<command>]
    Technopolis [options]

Options:
    -h --help       Show this screen.
    --version       Show version.
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct CommandVisitor;
impl<'de> Visitor<'de> for CommandVisitor {
    type Value = Command;

    fn expecting(&self, fomatter: &mut fmt::Formatter) -> fmt::Result {
        fomatter.write_str("a string INIT")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match s.to_uppercase().as_str() {
            "" => Command::None,
            "INIT" => Command::Init,
            s => Command::Unknown(s.to_string()),
        })
    }
}

#[derive(Debug)]
enum Command {
    Init,
    None,
    Unknown(String),
}
impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(d: D) -> Result<Command, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_str(CommandVisitor)
    }
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Command,

    flag_help: bool,
    flag_version: bool,
}

fn print_splash() {
    const SPLASH_TEXT: &'static str = "
  __                .__                                .__  .__        
_/  |_  ____   ____ |  |__   ____   ____ ______   ____ |  | |__| ______
\\   __\\/ __ \\_/ ___\\|  |  \\ /    \\ /  _ \\\\____ \\ /  _ \\|  | |  |/  ___/
 |  | \\  ___/\\  \\___|   Y  \\   |  (  <_> )  |_> >  <_> )  |_|  |\\___ \\ 
 |__|  \\___  >\\___  >___|  /___|  /\\____/|   __/ \\____/|____/__/____  >
           \\/     \\/     \\/     \\/       |__|                       \\/ 
";

    println!("{}", SPLASH_TEXT);
    println!("Backend Version: v{}", VERSION);
}

fn migration() -> Result<(), Box<dyn Error>> {
    println!("Trying to migrate...");

    let params = format!(
        "host={} user={} password={} dbname={}",
        dotenv::var("TECHNOPOLIS_POSTGRES_HOST")
            .expect("Environment \"TECHNOPOLIS_POSTGRES_HOST\" is not set!"),
        dotenv::var("TECHNOPOLIS_POSTGRES_USER")
            .expect("Environment \"TECHNOPOLIS_POSTGRES_USER\" is not set!"),
        dotenv::var("TECHNOPOLIS_POSTGRES_PASSWD")
            .expect("Environment \"TECHNOPOLIS_POSTGRES_PASSWD\" is not set!"),
        dotenv::var("TECHNOPOLIS_POSTGRES_DBNAME")
            .expect("Environment \"TECHNOPOLIS_POSTGRES_DBNAME\" is not set!"),
    );

    let mut client = Client::connect(&params, NoTls)?;
    migrations::migrations::runner().run(&mut client)?;

    println!("Migrate complete!");

    Ok(())
}

fn main() {
    // コマンドラインを解析
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // 付けたら処理せず終了する系のフラグを処理

    // --version
    if args.flag_version {
        println!("Technopolis Backend Version: v{}", VERSION);

        process::exit(0);
    }

    // --help
    if args.flag_help {
        println!("{}", USAGE);

        process::exit(0);
    }

    dotenv().ok();

    // サブコマンドを処理
    match args.arg_command {
        Command::Init => {
            print_splash();

            // データベースのマイグレーションを実行
            migration().expect("Migrate Failed!");

            process::exit(0);
        }
        Command::Unknown(s) => {
            println!("{} is unkown command.", s);
            println!("{}", USAGE);

            process::exit(1);
        }
        Command::None => {
            println!("{}", USAGE);

            process::exit(0);
        }
    }
}
