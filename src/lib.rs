pub mod console;
pub mod rsx;
pub mod utils;
pub mod widgets;

pub use std::io::Result;

pub type Element = std::sync::Arc<dyn Fn(&Frame) -> crate::Result<()>>;

pub trait Widget {
    fn render(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    center_x: bool,
    center_y: bool,
    width_auto: bool,
    height_auto: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    pub area: Area,
}

pub struct State<T>(*mut T);

impl Frame {
    pub fn draw<W>(&self, w: &W, mut props: Area) -> Result<()>
    where
        W: Widget,
    {
        let written = w.render();

        let (ww, wh) = utils::str_size(&written);

        if props.width_auto {
            props.width = ww;
        }
        if props.height_auto {
            props.height = wh;
        }

        if props.width > self.area.width {
            props.width -= props.width - self.area.width;
        }

        if props.width > self.area.width {
            props.width -= props.width - self.area.width;
        }

        if props.center_x {
            props.x = (self.area.width - ww) / 2;
        }
        if props.center_y {
            props.y = (self.area.height - wh) / 2;
        }

        for (i, line) in written.lines().enumerate() {
            if i as u16 > props.height {
                break;
            }

            println!(
                "\x1b[{};{}H{}",
                props.y + (i as u16) + 1,
                props.x + 1,
                line.chars().take(props.width as usize).collect::<String>()
            );
        }

        Ok(())
    }

    pub fn new((width, height): (u16, u16)) -> Self {
        let mut f = Self::default();
        f.area.width(width);
        f.area.height(height);
        f
    }
}

impl Area {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn center_x() -> Self {
        let mut s = Self::default();
        s.center_x = true;
        s
    }

    pub fn center_y() -> Self {
        let mut s = Self::default();
        s.center_y = true;
        s
    }

    pub fn center() -> Self {
        let mut s = Self::default();
        s.center_x = true;
        s.center_y = true;
        s
    }

    pub fn width(&mut self, w: u16) -> Self {
        self.width = w;
        self.width_auto = false;
        *self
    }

    pub fn height(&mut self, h: u16) -> Self {
        self.height = h;
        self.height_auto = false;
        *self
    }

    pub fn x(&mut self, x: u16) -> Self {
        self.x = x;
        *self
    }

    pub fn y(&mut self, y: u16) -> Self {
        self.y = y;
        *self
    }
}

impl Default for Area {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            center_x: false,
            center_y: false,
            width_auto: true,
            height_auto: true,
        }
    }
}

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

pub fn use_state<T>(v: T) -> State<T> {
    State(Box::into_raw(Box::new(v)))
}
