use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct Location {
    x: i32,
    y: i32,
    direction: i32,
}

fn help() {
    let multiline = r#"
Hello! Robot coming online.
Command the robot with:
  L - turn left
  R - turn right
  M - move forward
  ? - this message
  Q - quit
    "#;

    println!("{}", multiline);
}

fn fmove(x: &mut i32, y: &mut i32, direction: String) -> (i32, i32) {
    if direction == "North" {
        *y = *y + 1
    } else if direction == "West" {
        *x = *x - 1
    } else if direction == "East" {
        *x = *x + 1
    } else {
        *y = *y - 1
    }
    (*x, *y)
}

fn print_status(x: i32, y: i32, direction: String) -> String {
    format!("Robot at ({}, {}) facing {}", x, y, direction)
}

fn turn(input: &String, turn: &Location) -> i32 {
    if input == "L" {
        if turn.direction == 0 {
            3
        } else {
            turn.direction - 1 % 4
        }
    } else {
        if turn.direction == 3 {
            0
        } else {
            turn.direction + 1 % 4
        }
    }
}

fn main() {
    let mut mapping = HashMap::new();
    mapping.insert("0", "North");
    mapping.insert("1", "East");
    mapping.insert("2", "South");
    mapping.insert("3", "West");
    let mut location = Location {
        x: 0,
        y: 0,
        direction: 0,
    };
    help();
    loop {
        print!("{}", '>');
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();
        if input.trim() == "?" {
            help();
            continue;
        } else if input == "M" {
            fmove(
                &mut location.x,
                &mut location.y,
                mapping[&*location.direction.to_string()].to_string(),
            );
        } else if input == "L" || input == "R" {
            location.direction = turn(&input.to_string(), &location)
        } else {
            println!("Robot shutting down.");
            std::process::exit(0)
        }
        println!(
            "{}",
            print_status(
                location.x,
                location.y,
                mapping[&*location.direction.to_string()].to_string(),
            )
        );
    }
}

#[cfg(test)]
mod tests {

    use super::fmove;
    use super::print_status;
    use super::Location;
    use crate::turn;

    #[test]

    fn test_print_status() {
        assert_eq!(
            print_status(1, 1, "North".to_string()),
            "Robot at (1, 1) facing North"
        );
    }
    #[test]
    fn test_move_n() {
        assert_eq!(
            fmove(&mut (0 as i32), &mut (0 as i32), "North".to_string()),
            (0, 1)
        );
    }
    #[test]
    fn test_move_w() {
        assert_eq!(
            fmove(&mut (0 as i32), &mut (1 as i32), "West".to_string()),
            (-1, 1)
        );
    }
    #[test]
    fn test_turn() {
        assert_eq!(
            turn(
                &"L".to_string(),
                &Location {
                    x: 0,
                    y: 0,
                    direction: 0
                },
            ),
            3
        );
        assert_eq!(
            turn(
                &"L".to_string(),
                &Location {
                    x: 0,
                    y: 0,
                    direction: 3
                },
            ),
            2
        );
        assert_eq!(
            turn(
                &"R".to_string(),
                &Location {
                    x: 0,
                    y: 0,
                    direction: 3
                },
            ),
            0
        );
    }
}
