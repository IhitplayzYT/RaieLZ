pub mod Config{
    use crossterm::event::{KeyCode, KeyEvent};
use lettre::transport::smtp::authentication::Credentials;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::tui::app::app::MyColor;

    #[derive(Debug,Deserialize,Serialize)]
    pub struct Config{
        pub colors: [MyColor;5],// Primary,Secondary,Text,Urls,Borders,
        pub win_size: Vec<(usize,usize)>,// Normalize this total vector to get ratio dims of each window,
        pub win_toggle: KeyCode,
    }

    impl Default for Config{
        fn default() -> Self {
            Self { colors: [MyColor::White,MyColor::Black,MyColor::RGB(50, 50, 50),MyColor::Blue,MyColor::BrightRed], win_size: vec![], win_toggle:KeyCode::Tab}
        }
    }

    #[derive(Debug)]
    pub struct EmailAccount {
        pub address: String,
        pub smtp_server: String,
        pub imap_server: String,
        pub pop3_server: String,
        pub smtp_port: u16,
        pub imap_port: u16,
        pub pop3_port: u16,
        pub username: String,
        pass: String,
        pub credentials: Credentials,
    }


    impl EmailAccount{
        pub fn new(address: String,smtp_server: String,imap_server: String,pop3_server: String,username: String,pass:String,smtp_port:u16,imap_port:u16,pop3_port:u16) -> Self{
            Self { address, smtp_server, imap_server, username:username.clone(), credentials: Credentials::new(username, pass.clone()),smtp_port,imap_port,pass,pop3_server,pop3_port}
        }

        pub fn get_pass(&self) -> &str{
            &self.pass
        }

        // This is a exposed setter since multiple fields need updating
        pub fn update_creds(&mut self,username: Option<String>,pass:Option<String>){
            if let Some(x) = username.clone(){
                self.username = x;
            }
            if let Some(x) = pass.clone(){
                self.pass = x;
            }
            match (username,pass) {
                (Some(x),Some(y)) => {self.credentials = Credentials::new(x,y)},
                (Some(x),None) => {self.credentials = Credentials::new(x,self.pass.clone())},
                (None,Some(y)) => {self.credentials = Credentials::new(self.username.clone(),y)},
                _ => {}
            }
        }


    }


}