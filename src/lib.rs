use crossterm::ExecutableCommand;

pub mod key;
pub mod macros;
pub mod ui;
pub mod utils;

pub trait Element: std::fmt::Debug {
    fn get_child(&mut self) -> Option<&mut Box<dyn Element>>;
    fn get_data(&self) -> ElementData;
    fn set_data(&mut self, _: ElementData);
    fn clear_ticks(&mut self);
    fn render(&mut self, _tick: usize) -> String {
        String::new()
    }
    fn update(&mut self, _ctx: &mut UpdateContext) {}
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
}

pub struct UpdateContext {
    pub key: key::Key,
    pub tick: usize,
    pub response: UpdateResponse,
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

    /// Creates a new screen to render components with a pre-existing component
    pub fn from(elem: Box<dyn Element>) -> App {
        let mut app = App::new();
        app.set_component(elem);
        app
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
    fn render(&mut self, tick: usize) {
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
        utils::render_to_frame(tick, &mut frame, &mut self.element);
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

    fn update(&mut self, ctx: &mut UpdateContext) {
        self.element.update(ctx);
        match ctx.response {
            UpdateResponse::Exit => {
                crossterm::terminal::disable_raw_mode().unwrap();
                utils::clear();
                utils::show_cursor();
                println!("");
                return;
            }
            _ => {}
        }
    }

    /// Run the screen
    pub fn run(&mut self) {
        // Initialize
        utils::hide_cursor();
        utils::clear();
        let mut stdout = std::io::stdout();
        stdout
            .execute(crossterm::terminal::EnterAlternateScreen)
            .unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();

        // Start the update thread
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || loop {
            tx.send(key::read_key()).unwrap();
        });

        // Start the render loop
        let mut tick: usize = 0;
        loop {
            if tick > 99 {
                tick = 0;
            }
            self.render(tick);
            tick += 1;
            match rx.try_recv() {
                Ok(k) => self.update(&mut UpdateContext {
                    key: k,
                    tick,
                    response: UpdateResponse::None,
                }),
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    panic!("disconnected")
                }
            }
        }
    }
}
