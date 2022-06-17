use std::fmt;

#[derive(Copy, Clone)]
pub enum Choice {
    X,
    O
}
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Choice::O => write!(f, "O"),
            Choice::X => write!(f, "X")
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pub table: Vec<char>
}

impl Table {
    pub fn new() -> Table {
        Table {
            table: vec![
               '1', '2', '3',
               '4', '5', '6',
               '7', '8', '9'
            ]
        }
    }

    /**
     * 
     */
    pub fn add_choice(&mut self, choice: Choice, position: usize) -> bool {
        self.validate_position(position);
        // Add validation
        if self.table[position] != 'X' || self.table[position] != 'O' {
            return false;
        }
        match choice {
            Choice::X => {
                self.table.remove(position);
                self.table.insert(position, 'X');
            },
            Choice::O => {
                self.table.remove(position);
                self.table.insert(position, 'O');
            }
        }
        true
    }

    pub fn print_table(&self) {
        println!("{} | {} | {}", self.table[0], self.table[1], self.table[2]);
        println!("--|---|--");
        println!("{} | {} | {}", self.table[3], self.table[4], self.table[5]);
        println!("--|---|--");
        println!("{} | {} | {}", self.table[6], self.table[7], self.table[8]);

    }

    pub fn check_for_winner(&self, choice: Choice) -> bool {
        /* Winner: 
        0 1 2, 3 4 5, 6 7 8, 0 3 6, 1 4 7, 2 5 8, 0 4 8, 2 4 6,
        */
        let choice = match choice {
            Choice::O => 'O',
            Choice::X => 'X',
        };
        if self.table[0] == choice && self.table[1] == choice && self.table[2] == choice {
            return true
        }
        if self.table[3] == choice && self.table[4] == choice && self.table[5] == choice {
            return true
        }
        if self.table[6] == choice && self.table[7] == choice && self.table[8] == choice {
            return true
        }
        if self.table[0] == choice && self.table[3] == choice && self.table[6] == choice {
            return true
        }
        if self.table[1] == choice && self.table[4] == choice && self.table[7] == choice {
            return true
        }
        if self.table[2] == choice && self.table[5] == choice && self.table[8] == choice {
            return true
        }
        if self.table[0] == choice && self.table[4] == choice && self.table[8] == choice {
            return true
        }
        if self.table[2] == choice && self.table[4] == choice && self.table[6] == choice {
            return true
        }

        false
    }

    pub fn check_for_draw(&self) -> bool {
        for ch in &self.table {
            if *ch != 'X' || *ch != 'O' {
                return false
            }
        }
        true
    }

    /**
     * 
     */
    fn validate_position(&self, pos: usize) {
        if pos > 8 {
            panic!("Position should be between 0 and 9")
        }
    }
}