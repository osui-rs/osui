use crate::key::Key;

/// Enum defining commands that can be issued by an `Element`.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum EventResponse {
    /// Indicates that the event is complete.
    Done,
    /// Indicates no response.
    None,
    /// Issues a single command.
    Command(Command),
    /// Issues a list of commands.
    CommandList(Vec<Command>),
}

/// Enum representing the possible responses from an element update.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(Key),
    State(usize),
}
