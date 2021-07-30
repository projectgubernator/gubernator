use super::Controller;

/// The Gubernator node controller.
///
/// This controller manages the lifecycle of Gubernator worker nodes.
pub struct NodeController;

impl NodeController {}

impl Controller for NodeController {
    fn reconcile(&mut self) {
        println!("reconciling")
    }
}
