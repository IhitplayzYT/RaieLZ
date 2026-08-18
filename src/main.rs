use std::fs;


use crate::{config::Config::{Config, EmailAccount}, data::DB::db::Database, helper::Helper::CLI, tui::app::app::App};

mod helper;
mod protocol;
mod data;
mod tui;
mod config;
mod parsers;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }
    let db_url = format!("mysql://{}:{}@localhost:{}/{}",clargs.db_user.unwrap_or("root".to_string()),clargs.db_pass.unwrap_or("".to_string()),clargs.db_port.unwrap_or(3306),clargs.db.unwrap_or("mydb".to_string()));
    let mut email_acc= EmailAccount::new(clargs.mail.unwrap(), clargs.smtp_srvr.unwrap(), clargs.imap_srvr.unwrap(),clargs.pop3_srvr.unwrap(), clargs.username.clone().unwrap(),clargs.pass.unwrap(), clargs.smtp_port.unwrap(), clargs.imap_port.unwrap(),clargs.pop3_port.unwrap());
    let conf:Config = if let Some(x) = clargs.config{match fs::read_to_string(&x){Ok(y) => {serde_json::from_str(&y).unwrap()},Err(_) => {panic!("Config Path is Invalid")}}}else{Config::default()};
    let mut app = App::new(email_acc, Database::new(&db_url).unwrap(),conf);
    





    println!("Hello, world!");
}
