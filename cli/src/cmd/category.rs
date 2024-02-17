use super::{Cmd, CmdErrorType, SyntaxErrorType};

pub struct Category { }

impl Cmd for Category {
    fn new() -> Self where Self: Sized {
        Category { }
    }

    fn execute(&self, args: &[&str], ledger: &mut ledger::Ledger, app: &mut crate::app::Application) -> Result<super::CmdResult, super::CmdError> {
        match args.first() {
            Some(&"--new") => {
                // TODO RESUME
                panic!("todo");
            },
            Some(&"--list") => {
                let categories = ledger.get_transaction_categories();
                for category in categories.categories() {
                    writeln!(app.out(), "  {}", category.name())?;
                }
                Ok(super::CmdResult::Ok)
            },
            Some(unhandled_subcommand) => {
                Err(self.new_error(CmdErrorType::Syntax(SyntaxErrorType::InvalidSubcommand(unhandled_subcommand.to_string()))))
            }
            None => {
                Err(self.new_error(CmdErrorType::Syntax(SyntaxErrorType::MissingSubcommand)))
            }
        }
    }

    fn names(&self) -> Vec<&'static str> {
        vec!["category", "cat"]
    }

    fn help_text(&self) -> &'static str {
"Usage: category [OPTION] CATEGORY_NAME CATEGORY_PARENT
Add new transaction caregories or list existing ones

Options:
  --new   Create a new category with CATEGORY_NAME.
          If CATEGORY_PARENT is provided, then the
          new category will be a sub-category of the
          parent.
  --list  List existing transaction categories"
    }
}