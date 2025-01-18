#[derive(Clone, Copy)]
pub struct State<T>(pub(crate) *mut T);

impl<T: std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { &*self.0 })
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State({:?})", unsafe { &*self.0 })
    }
}

//// State ops ////

impl<T: std::ops::Add<Output = T> + Clone> std::ops::Add<T> for State<T> {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        unsafe {
            let lhs = &mut *self.0;
            lhs.clone() + rhs
        }
    }
}

impl<T: std::ops::Sub<Output = T> + Clone> std::ops::Sub<T> for State<T> {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        unsafe {
            let lhs = &mut *self.0;
            lhs.clone() - rhs
        }
    }
}

impl<T: std::ops::Div<Output = T> + Clone> std::ops::Div<T> for State<T> {
    type Output = T;

    fn div(self, rhs: T) -> Self::Output {
        unsafe {
            let lhs = &mut *self.0;
            lhs.clone() / rhs
        }
    }
}

impl<T: std::ops::AddAssign + Clone> std::ops::AddAssign<T> for State<T> {
    fn add_assign(&mut self, rhs: T) {
        unsafe { *self.0 += rhs }
    }
}

impl<T: std::ops::SubAssign + Clone> std::ops::SubAssign<T> for State<T> {
    fn sub_assign(&mut self, rhs: T) {
        unsafe { *self.0 -= rhs }
    }
}

impl<T> std::ops::Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> std::ops::DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> State<T> {
    pub fn copy_state(&self) -> Self {
        Self(self.0)
    }
}

impl<T: PartialEq<T>> PartialEq<T> for State<T> {
    fn eq(&self, other: &T) -> bool {
        unsafe { &*self.0 == other }
    }
}
