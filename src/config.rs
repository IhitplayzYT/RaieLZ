pub mod Config{
    use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};


    pub struct Config{
        pub colors: [Color;5],// Primary,Secondary,Text,Urls,Borders,
        pub win_size: Vec<(usize,usize)>,// Normalize this total vector to get ratio dims of each window,
        pub win_toggle: KeyCode,
    }

    impl Default for Config{
        fn default() -> Self {
            Self { colors: [Color::White,Color::Black,Color::Gray,Color::Blue,Color::LightRed], win_size: vec![], win_toggle:KeyCode::Tab}
        }
    }


}