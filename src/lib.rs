pub mod arg;
pub mod cli;
pub mod command;
pub mod errors;
mod seqalin;

#[cfg(test)]
mod tests {
    use crate::cli::Cli;
    use crate::arg::{Positional, Flag};
    use super::*;

    /// Helper test `fn` to write vec of &str as iterator for Cli parameter.
    fn args<'a>(args: Vec<&'a str>) -> Box<dyn Iterator<Item=String> + 'a> {
        Box::new(args.into_iter().map(|f| f.to_string()).into_iter())
    }

    mod radd {
        use crate::command::{FromCli, Command, Runner};
        use super::*;

        /// `Radd` is 'rust-add' that can add two unsigned 8-bit values together.
        #[derive(PartialEq, Debug)]
        struct Radd {
            lhs: u8,
            rhs: u8, 
            verbose: bool,
        }

        impl Radd {
            fn run(&self) -> u16 {
                self.lhs as u16 + self.rhs as u16
            }
        }

        impl Runner<()> for Radd {}

        impl FromCli for Radd {
            fn from_cli<'c>(cli: &'c mut Cli) -> Result<Self, errors::CliError<'c>> where Self: Sized {
                // set help text in case of an error
                cli.help(HELP, Some(3))?;
                let radd = Radd {
                    verbose: cli.check_flag(Flag::new("verbose"))?,
                    lhs: cli.require_positional(Positional::new("lhs"))?,
                    rhs: cli.require_positional(Positional::new("rhs"))?,
                };
                // optional: verify the cli has no additional arguments if this is the top-level command being parsed
                cli.is_empty()?;
                Ok(radd)
            }
        }

        impl Command<()> for Radd {
            type Status = ();

            fn exec(&self, _: &()) -> Self::Status {
                let sum: u16 = self.run();
                if self.verbose == true {
                    println!("{} + {} = {}", self.lhs, self.rhs, sum);
                } else {
                    println!("{}", sum);
                }
            }
        }

        #[test]
        fn program_radd() {
            let mut cli = Cli::new()
                .threshold(4)
                .tokenize(args(vec!["radd", "9", "10"]));
            let program = Radd::from_cli(&mut cli).unwrap();
            std::mem::drop(cli);

            assert_eq!(program.run(), 19);
        }

        const HELP: &str = "\
Adds two numbers together.

Usage:
    radd [options] <lhs> <rhs> 

Args:
    <lhs>       left-hand operand
    <rhs>       right-hand operand
    
Options:
    --verbose   display computation work
";
    }
}
