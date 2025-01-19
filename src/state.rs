//! The `state` module provides the `State` type for managing mutable state across UI components
//! and operations. It wraps a raw pointer to enable shared mutability and integrates with common
//! traits for ease of use.

/// A wrapper around a raw pointer to a mutable value, allowing shared state management.
///
/// # Safety
/// This type uses unsafe code to dereference raw pointers. Proper care must be taken
/// to ensure the validity and lifetime of the referenced value.
///
/// # Examples
/// ```
/// let value = 42;
/// let state = State(Box::into_raw(Box::new(value)));
/// assert_eq!(*state, 42);
/// ```
#[derive(Clone, Copy)]
pub struct State<T>(pub(crate) *mut T);

impl<T: std::fmt::Display> std::fmt::Display for State<T> {
    /// Formats the value of the state for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { &*self.0 })
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for State<T> {
    /// Formats the state for debugging purposes.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State({:?})", unsafe { &*self.0 })
    }
}

//// State Operations ////

impl<T: std::ops::Add<Output = T> + Clone> std::ops::Add<T> for State<T> {
    type Output = T;

    /// Adds a value to the state, returning the result.
    fn add(self, rhs: T) -> Self::Output {
        unsafe {
            let lhs = &mut *self.0;
            lhs.clone() + rhs
        }
    }
}

impl<T: std::ops::Sub<Output = T> + Clone> std::ops::Sub<T> for State<T> {
    type Output = T;

    /// Subtracts a value from the state, returning the result.
    fn sub(self, rhs: T) -> Self::Output {
        unsafe {
            let lhs = &mut *self.0;
            lhs.clone() - rhs
        }
    }
}

impl<T: std::ops::Div<Output = T> + Clone> std::ops::Div<T> for State<T> {
    type Output = T;

    /// Divides the state by a value, returning the result.
    fn div(self, rhs: T) -> Self::Output {
        unsafe {
            let lhs = &mut *self.0;
            lhs.clone() / rhs
        }
    }
}

impl<T: std::ops::AddAssign + Clone> std::ops::AddAssign<T> for State<T> {
    /// Adds a value to the state in place.
    fn add_assign(&mut self, rhs: T) {
        unsafe { *self.0 += rhs }
    }
}

impl<T: std::ops::SubAssign + Clone> std::ops::SubAssign<T> for State<T> {
    /// Subtracts a value from the state in place.
    fn sub_assign(&mut self, rhs: T) {
        unsafe { *self.0 -= rhs }
    }
}

impl<T> std::ops::Deref for State<T> {
    type Target = T;

    /// Dereferences the state to access the underlying value.
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> std::ops::DerefMut for State<T> {
    /// Dereferences the state to access the underlying mutable value.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> State<T> {
    /// Returns a copy of the current state.
    pub fn copy_state(&self) -> Self {
        Self(self.0)
    }
}

impl<T: PartialEq<T>> PartialEq<T> for State<T> {
    /// Compares the state with another value for equality.
    fn eq(&self, other: &T) -> bool {
        unsafe { &*self.0 == other }
    }
}
