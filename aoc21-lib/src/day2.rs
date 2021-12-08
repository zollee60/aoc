pub struct SubMarine {
    pub depth: u64,
    pub horizontal: u64,
    pub aim: u64
}

pub trait Move {
    fn execute_plan(&mut self, plan: &Vec<String>);
    fn execute_plan_v2(&mut self, plan: &Vec<String>);
    fn reset(&mut self);
}

impl Move for SubMarine {
    fn execute_plan(&mut self, plan: &Vec<String>) {
        for instruction in plan.iter() {
            let instruction_pieces: Vec<&str> = instruction.split(" ").collect();
            let value = instruction_pieces[1].parse::<u64>().unwrap();
            match instruction_pieces[0] {
                "up" => self.depth = self.depth - value,
                "down" => self.depth = self.depth + value,
                "forward" => self.horizontal = self.horizontal + value,
                _ => println!("Cannot execute instruction")
            }
        }
    }

    fn execute_plan_v2(&mut self, plan: &Vec<String>) {
        for instruction in plan.iter() {
            let instruction_pieces: Vec<&str> = instruction.split(" ").collect();
            let value = instruction_pieces[1].parse::<u64>().unwrap();
            match instruction_pieces[0] {
                "up" => self.aim = self.aim - value,
                "down" => self.aim = self.aim + value,
                "forward" => { self.horizontal = self.horizontal + value; self.depth = self.depth + (self.aim * value) },
                _ => println!("Cannot execute instruction")
            }
        }
    }

    fn reset(&mut self) {
        self.depth = 0;
        self.horizontal = 0;
        self.aim = 0;
    }
}