use regex::Regex;

pub struct Parser {
    value: String,
    re: Regex,
}
  
impl Parser {
    fn string_as_num(&self) -> Option<(&str, i32)> {
        let Some(found_num) = self.re.find(self.value.as_str()) else {
            return None
        };
    
        match found_num.as_str() {
            "one" => Some(("one", 1)),
            "two" =>  Some(("two", 2)),
            "three" => Some(("three", 3)),
            "four" => Some(("four", 4)),
            "five" => Some(("five", 5)),
            "six" => Some(("six", 6)),
            "seven" => Some(("seven", 7)),
            "eight" => Some(("eight", 8)),
            "nine" => Some(("nine", 9)),
            _ => None,
        }
    }
    pub fn check(&mut self, next: &char) -> Option<char> {
        self.value.push(*next);
        if let Some(num) = next.to_digit(10) {
            self.value.clear();
            return num.to_string().chars().next();
        } else if let Some((num_str, num)) = self.string_as_num() {
            self.value = self.value[self.value.len() - num_str.len() + 1 .. self.value.len()].to_string();
            return num.to_string().chars().next();
        }

        return None
    }

    pub fn new() -> Parser {
        Parser {
            value: String::new(),
            re: Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap(),
        }
    }
}
