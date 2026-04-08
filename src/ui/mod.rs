pub mod terminal;

use crate::types::UIResult;

/// Abstraction for user interaction and display/rendering
///
/// This trait defines the interface for rendering prompts, selections, confirmations, and input in a way that can be implemented by different UI backends (e.g., terminal, GUI, web)
#[cfg_attr(test, mockall::automock)]
pub trait UserInterface {
    /// Renders a selection prompt and returns the selected index
    fn render_select(&self, prompt: &str, options: &[String]) -> UIResult<usize>;

    /// Renders a multi selection prompt and returns the selected index
    fn render_multi_select(&self, prompt: &str, options: &[String]) -> UIResult<Vec<usize>>;

    /// Renders a confirmation (yes/no) prompt
    fn render_confirm(&self, prompt: &str) -> UIResult<bool>;

    /// Reads a line of input from the user
    #[allow(dead_code)]
    fn render_input(&self, prompt: &str) -> UIResult<String>;
}
