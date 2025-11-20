use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, Clone)]
pub struct State<T> {
    inner: Arc<Mutex<T>>,
}

pub fn use_state<T>(v: T) -> State<T> {
    State {
        inner: Arc::new(Mutex::new(v)),
    }
}

impl<T: Clone> State<T> {
    /// Gets the cloned value, recommended for preventing deadlocks
    pub fn get_dl(&self) -> T {
        self.inner.lock().unwrap().clone()
    }
}

impl<T> State<T> {
    /// Gets a lock on the state for read/write access.
    pub fn get(&self) -> MutexGuard<'_, T> {
        self.inner.lock().unwrap()
    }

    /// Sets the value and marks it as changed.
    pub fn set(&self, v: T) {
        *self.inner.lock().unwrap() = v;
    }
}

impl<T: Display> Display for State<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.inner.lock().unwrap())
    }
}

impl<T: Add<Output = T> + Clone> Add<T> for State<T> {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        let v = self.inner.lock().unwrap().clone();
        v.add(rhs)
    }
}

impl<T: Sub<Output = T> + Clone> Sub<T> for State<T> {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        let v = self.inner.lock().unwrap().clone();
        v.sub(rhs)
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for State<T> {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let v = self.inner.lock().unwrap().clone();
        v.mul(rhs)
    }
}

impl<T: Div<Output = T> + Clone> Div<T> for State<T> {
    type Output = T;

    fn div(self, rhs: T) -> Self::Output {
        let v = self.inner.lock().unwrap().clone();
        v.div(rhs)
    }
}

// Assign

impl<T: AddAssign> AddAssign<T> for State<T> {
    fn add_assign(&mut self, rhs: T) {
        self.get().add_assign(rhs);
    }
}

impl<T: SubAssign> SubAssign<T> for State<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.get().sub_assign(rhs);
    }
}

impl<T: MulAssign> MulAssign<T> for State<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.get().mul_assign(rhs);
    }
}

impl<T: DivAssign> DivAssign<T> for State<T> {
    fn div_assign(&mut self, rhs: T) {
        self.get().div_assign(rhs);
    }
}
