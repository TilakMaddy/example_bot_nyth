use core::fmt;
use std::{collections::BTreeMap, error::Error, fmt::Display};

use aderyn_driver::{
    context::workspace_context::{ASTNode, WorkspaceContext},
    core_ast::NodeID,
    detection_modules::capture,
};

#[derive(Default)]
pub struct Helper {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

// Notice that this is not a trait.
// Nothing is stopping me from naming detect function any other name
impl Helper {
    fn detect(
        &mut self,
        context: &WorkspaceContext,
        _within: &[ASTNode],
        _using: &[ASTNode],
        _custom_1: &[ASTNode],
    ) -> Result<bool, Box<dyn Error>> {
        // Use the `context` to find nodes, then capture them as shown below
        // capture!(self, context, ast_node);

        for vd in context.assignments.keys() {
            capture!(self, context, vd);
        }

        Ok(!self.found_instances.is_empty())
    }
}

#[derive(Debug)]
enum MyError {
    AssignmentsNotFound,
}

impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to fetch!")
    }
}

// I am free to accept more arguments like within, using, etc, here in addition to context
pub fn capture_assignments(
    context: &WorkspaceContext,
    _within: &[ASTNode],
    _using: &[ASTNode],
    _custom_1: &[ASTNode],
    _custom_2: &[ASTNode],
) -> Result<BTreeMap<(String, usize), NodeID>, Box<dyn Error>> {
    let mut help_mate = Helper::default();
    if let Ok(found) = help_mate.detect(context, _within, _using, _custom_1) {
        if found {
            return Ok(help_mate.found_instances);
        }
    }
    Err(Box::new(MyError::AssignmentsNotFound))
}
