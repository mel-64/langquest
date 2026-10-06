//! This module contains various utils, abstracted for use in other modules.

use std::io::{self, Write};

/// Builder for yes/no questions.
///
/// ```no_run
/// use lq::utils::Question;
///
/// let answer = Question::ask("Continue?").default(true).prompt().unwrap();
/// if answer {
///   println!("Question answered with yes!");
/// } else {
///   println!("Question answered with no!");
/// }
/// ```
pub struct Question {
  msg: String,
  default: Option<bool>,
  exact_yes: Option<String>,
}

impl Question {
  /// Create a yes/no question with the given prompt text.
  pub fn ask(msg: impl Into<String>) -> Self {
    Self {
      msg: msg.into(),
      default: None,
      exact_yes: None,
    }
  }

  /// Set the exact required answer to count as 'yes'.
  /// The answer set here is still case-insensitive.
  /// Mutually exclusive with `default(true)`.
  pub fn exact_yes(mut self, exact_yes: impl Into<String>) -> Self {
    self.exact_yes = Some(exact_yes.into().to_ascii_lowercase());
    self
  }

  /// Set the default answer, used when the user presses Enter.
  /// It is shown in the prompt as [y/N] or [Y/n].
  /// `default(true)` is mutually exclusive with `exact_yes`.
  pub fn default(mut self, default: bool) -> Self {
    self.default = Some(default);
    self
  }

  /// Print the prompt and read an answer from stdin.
  /// Accepts y/yes and n/no by default, enter accepts the default.
  pub fn prompt(self) -> io::Result<bool> {
    assert!(self.default != Some(true) || self.exact_yes.is_none());
    let exact_yes = self.exact_yes.unwrap_or_default();
    let yes_answers = if !exact_yes.is_empty() { vec![exact_yes.as_str()] } else { vec!["y", "yes"] };

    let hint: Option<String> = match self.default {
      Some(true) => Some("Y/n".into()),
      Some(false) => Some("y/N".into()),
      None => {
        if exact_yes.is_empty() {
          Some("y/n".into())
        } else {
          Some(format!("{}/n", exact_yes))
        }
      }
    };

    let prompt = hint.map_or(self.msg.clone(), |h| format!("{} [{}]", self.msg, h));

    loop {
      print!("{prompt}: ");
      io::stdout().flush()?;

      let mut input = String::new();
      io::stdin().read_line(&mut input)?;
      let input = input.trim().to_ascii_lowercase();

      match input.as_str() {
        "" => {
          if let Some(default) = self.default {
            return Ok(default);
          }
        }
        val if yes_answers.contains(&val) => return Ok(true),
        "n" | "no" => return Ok(false),
        _ => {}
      }
      println!("Invalid answer. Try again..");
    }
  }
}
