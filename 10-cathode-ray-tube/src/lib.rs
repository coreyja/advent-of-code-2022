#[derive(Debug)]
enum Command {
    Addx(isize),
    Noop,
}

impl Command {
    fn parse(input: &str) -> Self {
        let mut split = input.split(' ');
        let command = split.next().unwrap();

        match command {
            "noop" => Command::Noop,
            "addx" => {
                let amount = split.next().unwrap().parse().unwrap();
                Command::Addx(amount)
            }
            _ => panic!("Command {command:?} not implemented"),
        }
    }

    fn cycle_count(&self) -> isize {
        match self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }
}

pub fn part_1(input: &str) -> isize {
    let commands: Vec<Command> = input.lines().map(Command::parse).collect();

    let mut x_register = 1;
    let mut cycle_count = 0;

    let important_cycle_counts: [usize; 6] = [20, 60, 100, 140, 180, 220].map(|x| x - 1);
    let mut current_important_cycle_index = 0;

    let mut important_signal_strenghts: Vec<isize> = vec![];

    for c in commands {
        let original_x = x_register;

        if let Command::Addx(amount) = c {
            x_register += amount;
        }

        cycle_count += c.cycle_count();

        if cycle_count >= important_cycle_counts[current_important_cycle_index] as isize {
            let chosen_x_register =
                if cycle_count == important_cycle_counts[current_important_cycle_index] as isize {
                    x_register
                } else {
                    original_x
                };
            let signal_strength = ((important_cycle_counts[current_important_cycle_index] + 1)
                as isize)
                * chosen_x_register;

            important_signal_strenghts.push(signal_strength);

            current_important_cycle_index += 1;

            if current_important_cycle_index >= important_cycle_counts.len() {
                break;
            }
        }
    }

    important_signal_strenghts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 13140);
    }

    #[test]
    fn test_my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 17940);
    }
}
