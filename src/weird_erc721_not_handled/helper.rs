use core::fmt;
use std::{collections::BTreeMap, error::Error, fmt::Display};

use aderyn_driver::{
    context::workspace_context::WorkspaceContext, core_ast::NodeID, detection_modules::capture,
};

#[derive(Default)]
pub struct Helper {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl Helper {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
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

pub fn capture_assignments(
    context: &WorkspaceContext,
) -> Result<BTreeMap<(String, usize), NodeID>, Box<dyn Error>> {
    let mut help_mate = Helper::default();
    if let Ok(found) = help_mate.detect(context) {
        if found {
            return Ok(help_mate.found_instances);
        }
    }
    Err(Box::new(MyError::AssignmentsNotFound))
}
