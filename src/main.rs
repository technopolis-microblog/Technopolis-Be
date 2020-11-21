// SPDX-License-Identifier: AGPL-3.0-or-later

use docopt::Docopt;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use serde::Deserialize;
use std::{error::Error, process};

mod migrations;

mod infra;
mod presentation;
mod usecase;

const USAGE: &'static str = "
Technopolis backend

Usage:
    Technopolis
    Technopolis init
    Technopolis (-h | --help)
    Technopolis --version

Options:
    -h --help       Show this screen.
    --version       Show version.
";

const SPLASH_TEXT: &'static str = "
  __                .__                                .__  .__        
_/  |_  ____   ____ |  |__   ____   ____ ______   ____ |  | |__| ______
\\   __\\/ __ \\_/ ___\\|  |  \\ /    \\ /  _ \\\\____ \\ /  _ \\|  | |  |/  ___/
 |  | \\  ___/\\  \\___|   Y  \\   |  (  <_> )  |_> >  <_> )  |_|  |\\___ \\ 
 |__|  \\___  >\\___  >___|  /___|  /\\____/|   __/ \\____/|____/__/____  >
           \\/     \\/     \\/     \\/       |__|                       \\/ 
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct Args {
    cmd_init: bool,

    flag_help: bool,
    flag_version: bool,
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

    println!("{}", SPLASH_TEXT);
    println!("Backend Version: v{}", VERSION);

    dotenv().ok();

    // サブコマンドを処理

    // init
    if args.cmd_init {
        // データベースのマイグレーションを実行
        migration().expect("Migrate Failed!");

        process::exit(0);
    }
}
