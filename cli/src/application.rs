/// Trait representing general functionality of a running application. 
// TODO: Move to a generic location
pub trait Application {
    fn signal_terminate(&self);
}