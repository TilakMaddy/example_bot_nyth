use std::{collections::BTreeMap, error::Error};

use aderyn_driver::context::workspace_context::WorkspaceContext;
use aderyn_driver::core_ast::NodeID;
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{IssueDetector, IssueSeverity};

#[derive(Default)]
pub struct WeirdErc20NotHandledDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for WeirdErc20NotHandledDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Use the `context` to find nodes, then capture them as shown below
        // capture!(self, context, ast_node);

        for a in context.assignments.keys() {
            capture!(self, context, a);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Title for WeirdErc20NotHandledDetector")
    }

    fn description(&self) -> String {
        String::from("Description for WeirdErc20NotHandledDetector")
    }

    fn severity(&self) -> IssueSeverity {
        // Choose the appropriate severity
        IssueSeverity::NC
    }

    fn name(&self) -> String {
        "weird-erc20-not-handled".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod weird_erc20_not_handled_tests {

    use crate::config_tests::tests_configuration;

    use super::WeirdErc20NotHandledDetector;

    use aderyn_driver::context::workspace_context::WorkspaceContext;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::IssueDetector;

    fn test_weird_erc20_not_handled_for(
        _contract_file: String,
        context: WorkspaceContext,
        mut detector: impl IssueDetector,
    ) {
        // assert that the detector finds instances
        let found = detector.detect(&context).unwrap();
        assert!(found);
    }

    #[test]
    fn test_weird_erc20_not_handled() {
        let detector = WeirdErc20NotHandledDetector::default();
        let contracts = tests_configuration().get_contracts_for(detector.name());

        for contract_file in contracts {
            let detector = WeirdErc20NotHandledDetector::default();
            let context = load_contract(&contract_file);
            test_weird_erc20_not_handled_for(contract_file, context, detector);
        }
    }
}
