use crate::{arc, key::Key, Element};

/// Enum defining commands that can be issued by an `Element`.
#[derive(Clone)]
pub enum Command {
    /// Renders the element with the specified state.
    Render(usize),
    /// Exits the application.
    Exit,
    /// Updates the element with the specified state.
    Event(Event),
    /// Pauses execution for the specified duration in milliseconds.
    Sleep(u64),
}

/// Enum representing the possible responses from an element event.
#[derive(Clone)]
pub enum EventResponse {
    /// Indicates that the event is complete.
    Done,
    /// Indicates no response.
    None,
    /// Issues a single command.
    /// Issues a list of commands.
    CommandList(Vec<Command>),
    /// Update the current element
    UpdateSelf(Box<dyn Element>),
    /// Update a element by a id
    UpdateElementById(String, Box<dyn Element>),
    /// Multiple Responses
    /// # DO NOT USE MANUALLY
    Mul(Vec<EventResponse>),
}

impl EventResponse {
    pub fn add_command(&mut self, command: Command) {
        match self {
            EventResponse::CommandList(commands) => {
                commands.push(command);
            }
            EventResponse::Mul(responses) => {
                for response in responses {
                    match response {
                        EventResponse::CommandList(commands) => {
                            commands.push(command.clone());
                            return;
                        }
                        _ => {}
                    }
                }
                self.add_response(EventResponse::CommandList(vec![command.clone()]));
            }
            _ => {}
        }
    }
    pub fn update_self(&mut self, elem: Box<dyn Element>) {
        self.add_response(Self::UpdateSelf(elem));
    }
    pub fn update_element_by_id(&mut self, id: &str, elem: Box<dyn Element>) {
        self.add_response(Self::UpdateElementById(id.to_string(), elem));
    }
    pub fn add_response(&mut self, response: EventResponse) {
        match self {
            EventResponse::Mul(responses) => {
                responses.insert(0, response);
            }
            _ => *self = EventResponse::Mul(vec![response, self.clone()]),
        }
    }
    pub fn execute(&self) -> Vec<EventResponse> {
        return match self {
            EventResponse::Mul(v) => v.clone(),
            _ => vec![self.clone()],
        };
    }
}

/// Enum representing the possible responses from an element update.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(Key),
    State(usize),
    Hover,
}

/// A handler is a function that runs on a event of a element
///
/// # Params
/// - &mut T: The element,
/// - &mut EventResponse: The event response
pub type Handler<T> = std::sync::Arc<std::sync::Mutex<dyn FnMut(&mut T, &mut EventResponse)>>;

/// Creates a new `Handler<T>` from a given function closure.
///
/// This function takes a closure and wraps it in an `Arc<Mutex<dyn FnMut>>`, allowing it to be
/// used as a thread-safe, mutable handler that can be shared across threads. The handler function
/// receives a mutable reference to a `T` type and a mutable reference to an `EventResponse`, allowing
/// it to mutate both the state and response.
///
/// # Type Parameters
///
/// - `T`: The type of the data that the handler will operate on.
/// - `F`: The type of the handler function, which must implement `FnMut(&mut T, &mut EventResponse) + 'static`.
///
/// # Parameters
///
/// - `handler_fn`: The closure or function that will serve as the handler, taking mutable references
///   to both the `T` data and an `EventResponse`.
///
/// # Returns
///
/// Returns a `Handler<T>`, which is an `Arc<Mutex<dyn FnMut(&mut T, &mut EventResponse)>>`.
///
/// This type alias makes it easy to manage the handler in a multi-threaded context, allowing shared ownership
/// with safe, mutable access to the underlying function.
///
/// # Examples
///
/// ```rust
/// use osui::ui::button;
/// rsx! {
///     button {
///         // Create a new handler and assign it to on_click
///         on_click: new_handler(move |btn: &mut Button, _response| {
///             btn.text = "Clicked".to_string();
///         }
///     ), "click me" }
/// }
/// ```
pub fn new_handler<T, F>(handler_fn: F) -> Handler<T>
where
    F: FnMut(&mut T, &mut EventResponse) + 'static,
{
    arc!(handler_fn)
}
