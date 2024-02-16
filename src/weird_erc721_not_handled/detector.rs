use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::workspace_context::WorkspaceContext;
use aderyn_driver::core_ast::NodeID;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{IssueDetector, IssueSeverity};

use super::helper::capture_assignments;

#[derive(Default)]
pub struct WeirdErc721NotHandledDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for WeirdErc721NotHandledDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Use the `context` to find nodes, then capture them as shown below
        // capture!(self, context, ast_node);

        for vd in context.variable_declarations.keys() {
            capture!(self, context, vd);
        }

        let assignments = capture_assignments(context).unwrap();

        for assign_key in assignments.keys() {
            self.found_instances
                .insert(assign_key.clone(), *assignments.get(assign_key).unwrap());
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Title for WeirdErc721NotHandledDetector")
    }

    fn description(&self) -> String {
        String::from("Description for WeirdErc721NotHandledDetector")
    }

    fn severity(&self) -> IssueSeverity {
        // Choose the appropriate severity
        IssueSeverity::NC
    }

    fn name(&self) -> String {
        "weird-erc721-not-handled".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod weird_erc721_not_handled_tests {

    use crate::config_tests::tests_configuration;

    use super::WeirdErc721NotHandledDetector;

    use aderyn_driver::context::workspace_context::WorkspaceContext;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::IssueDetector;

    fn test_weird_erc721_not_handled_for(
        _contract_file: String,
        context: WorkspaceContext,
        mut detector: impl IssueDetector,
    ) {
        // assert that the detector finds instances
        let found = detector.detect(&context).unwrap();
        assert!(found);
    }

    #[test]
    fn test_weird_erc721_not_handled() {
        let detector = WeirdErc721NotHandledDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.name());

        for contract_file in contracts {
            let detector = WeirdErc721NotHandledDetector::default();
            let context = load_contract(&contract_file);
            test_weird_erc721_not_handled_for(contract_file, context, detector);
        }
    }
}
