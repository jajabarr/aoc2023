use std::collections::HashMap;
use std::cmp;
use std::fmt;

pub struct PartDelimiter {
    pub value: char,
    pub coordinate: (usize, usize),
    pub len: usize
}

pub struct SuspectedPart {
    pub value: i32,
    pub coordinate: (usize, usize),
    pub len: usize,
}

pub struct MachinePart {
    pub value: i32,
    pub coordinate: (usize, usize),
    pub len: usize,
}

impl SuspectedPart {
    pub fn new(value: i32, coordinate: (usize, usize), len: usize) -> SuspectedPart {
        SuspectedPart {
            value,
            coordinate,
            len,
        }
    }
}

pub struct Parser {
    pub machine_parts: Vec<MachinePart>,
    schematic_entries: Vec<Vec<char>>,
    suspected_parts: HashMap<usize, HashMap<usize, SuspectedPart>>,
    part_delimiters: HashMap<usize, HashMap<usize, PartDelimiter>>,
}

impl Parser {
    pub fn new() -> Parser {
        let machine_parts: Vec<MachinePart> = Vec::new();
        let schematic_entries: Vec<Vec<char>> = Vec::new();
        let suspected_parts: HashMap<usize, HashMap<usize, SuspectedPart>> = HashMap::new();
        let part_delimiters: HashMap<usize, HashMap<usize, PartDelimiter>> = HashMap::new();

        Parser {
            machine_parts,
            schematic_entries,
            suspected_parts,
            part_delimiters,
        }
    }

    pub fn read(&mut self, row: usize, line: &str) {
        self.schematic_entries.push(Vec::new());
        let mut num_str = String::new();

        for (i, c) in line.chars().enumerate() {
            if let Some(entry_row) = self.schematic_entries.last_mut() {
                entry_row.push(c);
            }
            if c.is_numeric() {
                num_str.push(c);
            } else {

                self.maybe_add_suspected_part_entry(row, i, &num_str);

                num_str.clear();

                if c == '.' {
                    continue;
                } 

                self.part_delimiters
                    .entry(row)
                    .or_insert(HashMap::new())
                        .entry(i)
                        .or_insert(PartDelimiter {value: c, coordinate: (row, i), len: 1 });
            }
        }

        self.maybe_add_suspected_part_entry(row, line.len() - 1, &num_str);
    }

    pub fn find_machine_parts(&mut self) {
        let mut gears: HashMap<usize, HashMap<usize, Vec<i32>>> = HashMap::new();

        for (row, suspected_parts) in self.suspected_parts.iter() {
            'parts: for ( col, part) in suspected_parts.iter() {
                let _row_minus_1: usize = cmp::max(*row as i32 - 1, 0) as usize;
                let _col_minus_1: usize = cmp::max(*col as i32 - 1, 0) as usize;
                for i in _row_minus_1 ..= row + 1 {
                    for j in _col_minus_1 ..= col + part.len {
                        let Some(part_delimiters) = self.part_delimiters.get(&i) else {
                            continue;
                        };

                        let Some(delimiter) = part_delimiters.get(&j) else {
                            continue;
                        };

                        if delimiter.value == '*' {
                            gears.entry(i).or_insert(HashMap::new()).entry(j).or_insert(Vec::new()).push(part.value);
                        }

                        let part =  MachinePart {
                            value: part.value,
                            coordinate: (*row, *col),
                            len: part.len,
                        };

                        self.machine_parts.push({
                            part
                        });

                        // continue 'parts;
                    }
                }

            }
        }
        let mut gear_ratios = 0;

        for gearset in gears.values() {
            for gear_entries in gearset.values() {
                if gear_entries.len() == 2 {
                    gear_ratios += gear_entries[0] * gear_entries[1];
                }
            }
        }
        println!("found {} parts", self.machine_parts.len());
        println!("gear ratios: {}", gear_ratios);
    }

    fn print_part_area(&self, part: &MachinePart) {
        let (row, col) = part.coordinate;

        let _row_minus_1: usize = cmp::max(row as i32 - 1, 0) as usize;
        let _col_minus_1: usize = cmp::max(col as i32 - 1, 0) as usize;
        for i in _row_minus_1 ..= row + 1 {
            for j in _col_minus_1 ..= col + part.len {
                let Some(row_entries) = self.schematic_entries.get(i) else {continue;};
                let Some(entry) = row_entries.get(j) else {continue;};
                print!("{}", entry);
            }
            println!();
        }
        println!();
    }

    fn maybe_add_suspected_part_entry(&mut self, row: usize, col: usize, num_str: &str) {
        if let Ok(num_result) = num_str.parse::<i32>() {
            let start_pos = cmp::max(col as i32 - num_str.len() as i32, 0) as usize;
            self.suspected_parts
                .entry(row)
                .or_insert(HashMap::new())
                    .entry(start_pos)
                    .or_insert(SuspectedPart::new(num_result, (row, start_pos), num_str.len()));
        };
    }
}


impl fmt::Display for MachinePart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> ({},{}) {}", self.value, self.coordinate.0, self.coordinate.1, self.len)
    }
}

impl fmt::Display for SuspectedPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> ({},{}) {}", self.value, self.coordinate.0, self.coordinate.1, self.len)
    }
}

impl fmt::Display for PartDelimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> ({},{}) {}", self.value, self.coordinate.0, self.coordinate.1, self.len)
    }
}