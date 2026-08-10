pub mod app{
    use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use serde_json::de::SliceRead;

use crate::{config::Config::{Config, EmailAccount}, data::DB::db::Database};


    #[derive(Debug)]
    pub struct App{
        pub email_conf: EmailAccount,
        pub db: Database,
        pub settings: Config
    }

    impl App{
        pub fn new(email_conf:EmailAccount,db:Database,settings:Config) -> Self{
            Self { email_conf, db, settings }
        }
    }


 #[derive(Debug, Clone,Copy,PartialEq, Eq,Hash,Deserialize,Serialize)]
pub enum MyColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8,u8,u8)
}


pub enum Color_channel{
    R,G,B
}


impl MyColor{
    pub fn new(r:u8,g:u8,b:u8) -> Self{
        Self::RGB(r, g, b)
    }

    pub fn rgb_str(&self) -> String{
        match self{
    MyColor::Black => format!("{:02X}{:02X}{:02X}",0,0,0),
    MyColor::Red => format!("{:02X}{:02X}{:02X}",125,0,0),
    MyColor::Green => format!("{:02X}{:02X}{:02X}",0,125,0),
    MyColor::Yellow => format!("{:02X}{:02X}{:02X}",181, 165, 54),
    MyColor::Blue => format!("{:02X}{:02X}{:02X}",0,0,125),
    MyColor::Magenta => format!("{:02X}{:02X}{:02X}",76, 51, 102),
    MyColor::Cyan => format!("{:02X}{:02X}{:02X}",44, 167, 184),
    MyColor::White => format!("{:02X}{:02X}{:02X}",212, 210, 195),
    MyColor::BrightBlack => format!("{:02X}{:02X}{:02X}",33,33,33),
    MyColor::BrightRed => format!("{:02X}{:02X}{:02X}",255,0,0),
    MyColor::BrightGreen => format!("{:02X}{:02X}{:02X}",0,255,0),
    MyColor::BrightYellow => format!("{:02X}{:02X}{:02X}",255,230,0),
    MyColor::BrightBlue => format!("{:02X}{:02X}{:02X}",0,0,255),
    MyColor::BrightMagenta => format!("{:02X}{:02X}{:02X}",123, 0, 255),
    MyColor::BrightCyan => format!("{:02X}{:02X}{:02X}",0, 218, 247),
    MyColor::BrightWhite => format!("{:02X}{:02X}{:02X}",255,255,255),
    MyColor::RGB(r,g,b) => format!("{:02X}{:02X}{:02X}",r,g,b)
        }
    }

    pub fn get_rgb(&self) -> (u8,u8,u8){
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };
        (r,g,b)
    }

    

        pub fn channel(&self, chan:Color_channel) -> u8 {
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };

        match chan {
            Color_channel::R => r,
            Color_channel::G => g,
            Color_channel::B => b,
        }
    }

    pub fn with_channel(&self,chan:Color_channel,v:u8) -> MyColor{
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };

        match chan {
            Color_channel::R => MyColor::RGB(v,g,b),
            Color_channel::G => MyColor::RGB(r,v,b),
            Color_channel::B => MyColor::RGB(r,g,v),
        }
    }

    pub fn to_color(&self) -> Color{
        let (r, g, b) = match *self {
            MyColor::Black => (0, 0, 0),
            MyColor::Red => (125, 0, 0),
            MyColor::Green => (0, 125, 0),
            MyColor::Yellow => (181, 165, 54),
            MyColor::Blue => (0, 0, 125),
            MyColor::Magenta => (76, 51, 102),
            MyColor::Cyan => (44, 167, 184),
            MyColor::White => (212, 210, 195),
            MyColor::BrightBlack => (33, 33, 33),
            MyColor::BrightRed => (255, 0, 0),
            MyColor::BrightGreen => (0, 255, 0),
            MyColor::BrightYellow => (255, 230, 0),
            MyColor::BrightBlue => (0, 0, 255),
            MyColor::BrightMagenta => (123, 0, 255),
            MyColor::BrightCyan => (0, 218, 247),
            MyColor::BrightWhite => (255, 255, 255),
            MyColor::RGB(r, g, b) => (r, g, b),
        };
        Color::Rgb(r, g, b)
    }


}


impl From<String> for MyColor{
    fn from(value: String) -> Self {
        let (r,g,b) = (&value[..2],&value[2..4],&value[4..6]);
        let (r,g,b) = (u8::from_str_radix(r, 16).unwrap(),u8::from_str_radix(g, 16).unwrap(),u8::from_str_radix(b, 16).unwrap());
        match (r,g,b) {
            (0,0,0) => MyColor::Black,
            (125,0,0) => MyColor::Red,
            (0,125,0) => MyColor::Green,
            (181,165,54) => MyColor::Yellow,
            (0,0,125) => MyColor::Blue,
            (76,51,102) => MyColor::Magenta,
            (44,167,184) => MyColor::Cyan,
            (212,210,195) => MyColor::White,
            (33,33,33) => MyColor::BrightBlack,
            (255,0,0) => MyColor::BrightRed,
            (0,255,0) => MyColor::BrightGreen,
            (255,230,0) => MyColor::BrightYellow,
            (0,0,255) => MyColor::BrightBlue,
            (123,0,255) => MyColor::BrightMagenta,
            (0,218,247) => MyColor::BrightCyan,
            (255,255,255) => MyColor::BrightWhite,
            _ => MyColor::RGB(r, g, b)
        }
         
    }

}


   



}