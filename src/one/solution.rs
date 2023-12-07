use aoc2023::util::file::read_lines;

fn find_line_result(line: &String) -> i32 {
  let mut numbers = String::new();
  for item in line.chars() {
    if let Some(_) = item.to_digit(10) {
      if numbers.len() >= 2 {
        numbers.pop();
        numbers.push(item);
        continue;
      }
      numbers.push(item);
    }
  }

  if numbers.len() == 2 {
    return numbers.parse().unwrap()
  }

  if let Some(first) = numbers.chars().next() {
    numbers.push(first);
  }

  numbers.parse().unwrap()
}

pub fn solve() -> () {
  let mut total = 0;

  if let Err(_) = read_lines("src/one/input.txt", |line| {
    total += find_line_result(&line);
  }) {
    println!("Error reading file");
  }

  println!("Total: {}", total);
}