use std::fmt;
use colored::Colorize;

pub struct Game {
    pub id: i32,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Game {
    pub fn new(id: i32, red: i32, green: i32, blue: i32) -> Game {
        Game {
            id,
            red,
            green,
            blue,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {} red, {} green, {} blue", 
            self.id, 
            if self.red > 12 {self.red.to_string().red()} else {self.red.to_string().white()}, 
            if self.green > 13 {self.green.to_string().green()} else {self.green.to_string().white()},
            if self.blue > 14 {self.blue.to_string().blue()} else {self.blue.to_string().white()},
        )
    }
}