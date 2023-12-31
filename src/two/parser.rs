use regex::Regex;
use super::game::Game;

pub struct Parser {
    re: Regex,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            re: Regex::new(r"Game (?P<game>\d+)|(?P<red>\d+) red|(?P<blue>\d+) blue|(?P<green>\d+) green").unwrap(),
        }
    }

    pub fn from(&self, line: &str) -> Game {
        let mut id: i32 = 0;
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;

        for captures in self.re.captures_iter(line) {
            if let Some(game_id) = captures.name("game") {

                if let Ok(game_id) = game_id.as_str().parse::<i32>() {
                    id = game_id;
                }
            }
            if let Some(c_red) = captures.name("red") {
                if let Ok(c_red) = c_red.as_str().parse::<i32>() {
                    if c_red > red {
                        red = c_red;
                    }
                }
            }
            if let Some(c_green) = captures.name("green") {
                if let Ok(c_green) = c_green.as_str().parse::<i32>() {
                    if c_green > green {
                        green = c_green;
                    }
                }
            }
            if let Some(c_blue) = captures.name("blue") {
                if let Ok(c_blue) = c_blue.as_str().parse::<i32>() {
                    if c_blue > blue {
                        blue = c_blue;
                    }
                }
            }
        }

        Game::new(id, red, green, blue)
    }
}