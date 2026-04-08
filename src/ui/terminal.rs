use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};

use crate::ui::UserInterface;

/// Terminal-based user interface implementation using the `dialoguer` crate
pub struct TerminalUserInterface {
    theme: ColorfulTheme,
}

impl TerminalUserInterface {
    pub fn new() -> Self {
        return Self {
            theme: ColorfulTheme::default(),
        };
    }
}

impl UserInterface for TerminalUserInterface {
    fn render_select(&self, prompt: &str, options: &[String]) -> super::UIResult<usize> {
        return Select::with_theme(&self.theme)
            .with_prompt(prompt)
            .items(options)
            .default(0)
            .interact()
            .map_err(|error| error.into());
    }

    fn render_multi_select(&self, prompt: &str, options: &[String]) -> super::UIResult<Vec<usize>> {
        return MultiSelect::with_theme(&self.theme)
            .with_prompt(prompt)
            .items(options)
            .defaults(&vec![false; options.len()])
            .interact()
            .map_err(|error| error.into());
    }

    fn render_confirm(&self, prompt: &str) -> super::UIResult<bool> {
        return Confirm::with_theme(&self.theme)
            .with_prompt(prompt)
            .interact()
            .map_err(|error| error.into());
    }

    fn render_input(&self, prompt: &str) -> super::UIResult<String> {
        return Input::with_theme(&self.theme)
            .with_prompt(prompt)
            .interact_text()
            .map_err(|error| error.into());
    }
}
