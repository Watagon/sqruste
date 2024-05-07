#[derive(Debug, PartialEq)]
pub struct MetaCommand {}

#[derive(Debug, PartialEq)]
pub enum Command {
    Exit,
}

#[derive(Debug, PartialEq)]
pub enum MetaCommandError {
    Unrecognized,
}

type Result<T> = core::result::Result<T, MetaCommandError>;

impl Command {
    pub fn guess(input: &str) -> Result<Command> {
        match input {
            ".exit" => Ok(Self::Exit),
            _ => Err(MetaCommandError::Unrecognized),
        }
    }
}

impl MetaCommand {
    pub fn run(input: &str) -> Result<()> {
        let cmd = Command::guess(input)?;
        match cmd {
            Command::Exit => std::process::exit(0),
        }
        #[allow(unreachable_code)]
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Ok(Command::Exit), Command::guess(".exit"));
    }
}
