// SPDX-License-Identifier: AGPL-3.0-or-later

use docopt::Docopt;
use postgres::{Client, NoTls};
use serde::Deserialize;
use std::{error::Error, process};

mod migrations;

const USAGE: &'static str = "
Technopolis backend

Usage:
    Technopolis
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

#[derive(Debug, Deserialize)]
struct Args {
    flag_help: bool,
    flag_version: bool,
}

fn migration<S: AsRef<str>>(host: S, user: S, password: S) -> Result<(), Box<dyn Error>> {
    let params = format!(
        "host={} user={} password={}",
        host.as_ref(),
        user.as_ref(),
        password.as_ref()
    );

    let mut client = Client::connect(&params, NoTls)?;
    migrations::migrations::runner().run(&mut client)?;

    Ok(())
}

fn main() {
    // コマンドラインを解析
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // あとは読んで字の如く
    if args.flag_version {
        let version = env!("CARGO_PKG_VERSION");
        println!("Technopolis Backend Version: {}", version);
        process::exit(0);
    }

    if args.flag_help {
        println!("{}", USAGE);
        process::exit(0);
    }

    println!("{}", SPLASH_TEXT);

    // データベースのマイグレーションを実行
    println!("Trying to migrate...");
    migration("localhost", "vagrant", "password").expect("Migrate Failed!");
    println!("Migrate complete!");
}
