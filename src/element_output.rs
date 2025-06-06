use crate::{element::Element, error::Error, item_output::ItemOutput};
use serde::Serialize;

/// An element output.
#[derive(Debug, Serialize)]
pub struct ElementOutput {
    element: Element,
    results: Vec<Result<ItemOutput, Error>>,
}

impl ElementOutput {
    pub const fn new(element: Element, results: Vec<Result<ItemOutput, Error>>) -> Self {
        Self { element, results }
    }

    /// Returns an element.
    pub const fn element(&self) -> &Element {
        &self.element
    }

    /// Returns validation results.
    pub fn results(&self) -> impl ExactSizeIterator<Item = &Result<ItemOutput, Error>> {
        self.results.iter()
    }
}
