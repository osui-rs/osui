use crossterm::ExecutableCommand;

pub mod key;
pub mod macros;
pub mod ui;
pub mod utils;

pub trait Element: std::fmt::Debug {
    fn get_child(&mut self) -> Option<&mut Box<dyn Element>>;
    fn get_data(&self) -> ElementData;
    fn set_data(&mut self, _: ElementData);
    fn render(&mut self) -> String {
        String::new()
    }
    fn update(&mut self, _: crate::key::Key) -> UpdateResponse {
        UpdateResponse::None
    }
    fn tick(&mut self, _: usize) {}
}

#[derive(Debug)]
pub struct ElementData {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub style: ui::styles::Style,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateResponse {
    Exit,
    Done,
    None,
    Tick(Vec<u32>),
}

pub struct App {
    element: Box<dyn Element>,
}

impl App {
    /// Creates a new screen to render components
    pub fn new() -> App {
        App {
            element: ui::text(),
        }
    }

    /// Sets a component
    pub fn set_component(&mut self, element: Box<dyn Element>) {
        let (width, height) = crossterm::terminal::size().unwrap();
        self.element = element;
        let mut data = self.element.get_data();
        data.style.is_active = true;
        if data.width == 0 {
            data.width = width as usize;
        }
        if data.height == 0 {
            data.height = height as usize;
        }
        self.element.set_data(data);
    }

    /// Render to the screen
    pub fn render(&mut self) {
        let (width, height) = crossterm::terminal::size().unwrap();
        let mut data = self.element.get_data();
        if data.width == 0 {
            data.width = width as usize;
        }
        if data.height == 0 {
            data.height = height as usize;
        }
        self.element.set_data(data);
        let mut frame: Vec<String> = create_frame!(width as usize, height as usize);
        utils::render_to_frame(&mut frame, &mut self.element);
        utils::clear();
        print!("{}", frame.join(""));
        utils::flush();
        let mut data = self.element.get_data();
        if data.width == width as usize {
            data.width = 0;
        }
        if data.height == height as usize {
            data.height = 0;
        }
        self.element.set_data(data);
    }

    /// Run the screen
    pub fn run(&mut self) {
        utils::hide_cursor();
        utils::clear();
        let mut stdout = std::io::stdout();
        stdout
            .execute(crossterm::terminal::EnterAlternateScreen)
            .unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            self.render();
            match self.element.update(key::read_key()) {
                UpdateResponse::Exit => {
                    crossterm::terminal::disable_raw_mode().unwrap();
                    utils::clear();
                    utils::show_cursor();
                    println!("");
                    return;
                }
                UpdateResponse::Tick(ticks) => {
                    for (i, n) in ticks.into_iter().enumerate() {
                        std::thread::sleep(std::time::Duration::from_millis(n as u64));
                        self.element.tick(i);
                        self.render();
                    }
                }
                _ => {}
            }
        }
    }
}
