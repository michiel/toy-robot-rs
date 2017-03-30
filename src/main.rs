#[allow(dead_code)]
mod toyrobot_grammar {
    include!(concat!(env!("OUT_DIR"), "/toyrobot_grammar.rs"));
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Move,
    Left,
    Right,
    Report,
    Place(i64, i64, Direction),
}

#[derive(Debug, PartialEq)]
pub enum CommandParseError {
    InvalidCommand(String),
}

pub fn read_command(s: &str) -> Result<Command, CommandParseError> {
    match toyrobot_grammar::command(s) {
        Ok(command) => Ok(command),
        Err(_) => {
            Err(CommandParseError::InvalidCommand(format!("ERROR :Invalid command : {:?}", s)))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_basic_commands() {
        assert_eq!(read_command(&"MOVE").unwrap(), Command::Move);
        assert_eq!(read_command(&"LEFT").unwrap(), Command::Left);
        assert_eq!(read_command(&"RIGHT").unwrap(), Command::Right);
        assert_eq!(read_command(&"REPORT").unwrap(), Command::Report);
    }

    #[test]
    fn can_read_place_command() {
        let res1 = read_command(&"PLACE 1,2,WEST");
        assert_eq!(res1.unwrap(), Command::Place(1, 2, Direction::West));

        let res2 = read_command(&"PLACE 2,4,NORTH");
        assert_eq!(res2.unwrap(), Command::Place(2, 4, Direction::North));

        let res3 = read_command(&"PLACE X,Y,Z");
        assert_eq!(res3.is_err(), true);
    }
}
