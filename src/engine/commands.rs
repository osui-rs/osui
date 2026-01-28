use crate::engine::Command;

pub struct Stop;

impl Command for Stop {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
