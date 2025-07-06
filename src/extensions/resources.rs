use crate::{event, widget::Component};

#[derive(Debug, Clone)]
pub struct Resource<T>(pub T);

impl<T: Send + Sync + 'static> Component for Resource<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

event!(ResourceUpdate);
