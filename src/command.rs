use core::num::ParseIntError;

use crate::simple::User;

pub struct Command {
    stmt: Statement,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    INSERT(User),
    SELECT,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Unrecognized,
    ParseError,
    SyntaxError,
}

// impl<F> From<F::Err> for CommandError where F: FromStr{}
impl From<ParseIntError> for CommandError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseError
    }
}

type Result<T> = core::result::Result<T, CommandError>;

impl Statement {
    pub fn guess(input: &str) -> Result<Statement> {
        match input {
            s if s.starts_with("insert") => {
                let ["insert", id, uname, email] =
                    s.split_ascii_whitespace().collect::<Vec<_>>()[..]
                else {
                    Err(CommandError::SyntaxError)?
                };
                let id = id.parse()?;
                let username = uname.to_owned();
                let email = email.to_owned();
                Ok(Self::INSERT(User {
                    id,
                    username,
                    email,
                }))
            }
            s if s.starts_with("select") => Ok(Self::SELECT),
            _ => Err(CommandError::Unrecognized),
        }
    }
}

impl Command {
    pub fn prepare(input: &str) -> Result<Self> {
        let input = input.trim();
        let stmt = Statement::guess(input)?;
        match stmt {
            Statement::INSERT(ref a) => {
                println!("This is where we would do an insert.");
                println!("{a:#?}");
            }
            Statement::SELECT => {
                println!("This is where we would do a select.");
            }
        };
        Ok(Self { stmt })
    }

    pub fn run(input: &str) -> Result<()> {
        let stmt = Statement::guess(input)?;
        match stmt {
            Statement::INSERT { .. } => {
                println!("This is where we would do an insert.");
            }
            Statement::SELECT => {
                println!("This is where we would do a select.");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            Ok(Statement::SELECT),
            Statement::guess("select asdf asdfasdfadf"),
        );
        assert_eq!(
            Ok(Statement::INSERT),
            Statement::guess("insert asdf asdfasdfadf"),
        );
        assert_eq!(
            Err(MetaCommandError::Unrecognized),
            Statement::guess("sdgfdsglfdsal"),
        );
    }
}
