#[derive(Debug)]
struct Food {
    pub calories: u64,
}

#[derive(Debug)]
struct Elf {
    pub food: Vec<Food>,
}

impl Elf {
    fn parse(input: &str) -> Self {
        let lines = input.split('\n');

        let food_calories = lines.map(|l| l.parse::<u64>().unwrap());
        let food: Vec<Food> = food_calories.map(|calories| Food { calories }).collect();

        Elf { food }
    }

    fn calories(&self) -> u64 {
        self.food.iter().map(|f| f.calories).sum()
    }
}

fn part_1(input: &str) -> u64 {
    let split_by_elf = input.trim().split("\n\n");

    let elves: Vec<Elf> = split_by_elf.map(|s| s.trim()).map(Elf::parse).collect();

    elves.iter().map(|e| e.calories()).max().unwrap()
}

fn part_2(input: &str) -> u64 {
    let split_by_elf = input.trim().split("\n\n");

    let elves: Vec<Elf> = split_by_elf.map(|s| s.trim()).map(Elf::parse).collect();
    let mut calories: Vec<u64> = elves.iter().map(|e| e.calories()).collect();

    calories.sort();
    calories.reverse();

    calories[0..3].iter().sum()
}

fn main() {
    let example_input = include_str!("example.input");
    let example_part_1_ans = part_1(example_input);

    println!("Example Input Part 1 Answer: {example_part_1_ans}");

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);

    println!("My Input Part 1 Answer: {my_part_1_ans}");

    println!();
    println!();

    let example_input = include_str!("example.input");
    let example_part_2_ans = part_2(example_input);

    println!("Example Input Part 2 Answer: {example_part_2_ans}");

    let my_input = include_str!("my.input");
    let my_part_2_ans = part_2(my_input);

    println!("My Input Part 2 Answer: {my_part_2_ans}");
}
