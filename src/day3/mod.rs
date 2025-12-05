pub struct Day3Results {
    pub part_1_joltage: u32,
    pub part_2_joltage: u32,
}

impl Day3Results {
    fn new() -> Self {
        Self {
            part_1_joltage: 0,
            part_2_joltage: 0,
        }
    }
}

pub fn day3(input: String) -> Day3Results {
    calc()
}
fn calc() -> Day3Results {
    let output = Day3Results::new();

    return output;
}

#[cfg(test)]
mod tests {}
