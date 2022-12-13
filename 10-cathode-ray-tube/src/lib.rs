use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
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

pub fn be_a_cpu(input: &str) -> isize {
    let mut commands: VecDeque<Command> = input.lines().map(Command::parse).collect();

    let mut x_register = 1;
    let mut cycle_count = 1;

    let important_cycle_counts: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut current_important_cycle_index = 0;

    let mut important_signal_strenghts: Vec<isize> = vec![];

    let mut current_addx_command = None;

    loop {
        // Starting a CPU Cycle
        let c = if let Some(current_command) = current_addx_command {
            current_addx_command = None;

            current_command
        } else {
            let next_command = commands.pop_front().expect("We ran out of commands");

            current_addx_command = if let Command::Addx(amount) = next_command {
                Some(Command::Addx(amount))
            } else {
                None
            };

            Command::Noop
        };

        // Middle of Cycle
        // Record Signal Strengths
        if current_important_cycle_index < important_cycle_counts.len()
            && cycle_count == important_cycle_counts[current_important_cycle_index] as isize
        {
            let signal_strength =
                ((important_cycle_counts[current_important_cycle_index]) as isize) * x_register;

            important_signal_strenghts.push(signal_strength);

            current_important_cycle_index += 1;
        }
        // Draw Board
        let m = (cycle_count - 1) % 40;
        if (x_register - 1..=x_register + 1).contains(&m) {
            print!("#");
        } else {
            print!(".");
        }
        if cycle_count % 40 == 0 {
            println!();
        }
        if cycle_count == 240 {
            break;
        }

        // End of Cycle
        if let Command::Addx(amount) = c {
            x_register += amount;
        }
        cycle_count += 1;
    }

    important_signal_strenghts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let input = include_str!("example.input");
        let ans = be_a_cpu(input);

        assert_eq!(ans, 13140);
    }

    #[test]
    fn test_my_input_part_1() {
        let input = include_str!("my.input");
        let ans = be_a_cpu(input);

        assert_eq!(ans, 17940);
    }
}
