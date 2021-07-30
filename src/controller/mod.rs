mod node;

/// Defines functionality required by Gubernator controllers.
pub trait Controller {
    fn reconcile(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
