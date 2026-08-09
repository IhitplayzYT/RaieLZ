use crate::helper::Helper::CLI;

mod helper;
mod protocol;
mod data;
mod tui;
mod config;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }
    let db_url = format!("mysql://{}:{}@localhost:{}/{}",clargs.db_user.unwrap_or("root".to_string()),clargs.db_pass.unwrap_or("".to_string()),clargs.db_port.unwrap_or(3306),clargs.db.unwrap_or("mydb".to_string()));





    println!("Hello, world!");
}
