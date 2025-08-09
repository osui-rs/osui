use std::sync::{Arc, Mutex};

use crate::component;

use super::Extension;

use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};

component!(AlwaysFocused);
component!(Focused);

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct RelativeFocusExtension {
    cursor: usize,
    rendered: Arc<Mutex<Vec<(usize, u16, u16)>>>,
}

impl Extension for RelativeFocusExtension {
    fn init(&mut self, _ctx: &super::Context) {
        for widget in _ctx.get_widgets().iter() {
            if let Some(Focused) = widget.get() {
                widget.set_focused(true);
            }
        }
    }

    fn event(&mut self, ctx: &super::Context, event: &dyn super::Event) {
        if let Some(e) = event.get::<CrosstermEvent>() {
            match e {
                CrosstermEvent::Key(k) => {
                    if k.modifiers.contains(KeyModifiers::SHIFT) {
                        match k.code {
                            KeyCode::Right => {
                                let rendered = self.rendered.lock().unwrap();
                                let (_, current_x, current_y) = rendered
                                    .iter()
                                    .find(|r| r.0 == self.cursor)
                                    .unwrap_or(&(0, 0, 0));

                                if let Some(index) = find_closest_in_direction(
                                    Direction::Right,
                                    *current_x,
                                    *current_y,
                                    &rendered,
                                    self.cursor,
                                ) {
                                    self.cursor = index;
                                    for (i, w) in ctx.get_widgets().iter().enumerate() {
                                        w.set_focused(i == self.cursor);
                                    }
                                }
                            }

                            KeyCode::Left => {
                                let rendered = self.rendered.lock().unwrap();
                                let (_, current_x, current_y) = rendered
                                    .iter()
                                    .find(|r| r.0 == self.cursor)
                                    .unwrap_or(&(0, 0, 0));

                                if let Some(index) = find_closest_in_direction(
                                    Direction::Left,
                                    *current_x,
                                    *current_y,
                                    &rendered,
                                    self.cursor,
                                ) {
                                    self.cursor = index;
                                    for (i, w) in ctx.get_widgets().iter().enumerate() {
                                        w.set_focused(i == self.cursor);
                                    }
                                }
                            }

                            KeyCode::Up => {
                                let rendered = self.rendered.lock().unwrap();
                                let (_, current_x, current_y) = rendered
                                    .iter()
                                    .find(|r| r.0 == self.cursor)
                                    .unwrap_or(&(0, 0, 0));

                                if let Some(index) = find_closest_in_direction(
                                    Direction::Up,
                                    *current_x,
                                    *current_y,
                                    &rendered,
                                    self.cursor,
                                ) {
                                    self.cursor = index;
                                    for (i, w) in ctx.get_widgets().iter().enumerate() {
                                        w.set_focused(i == self.cursor);
                                    }
                                }
                            }

                            KeyCode::Down => {
                                let rendered = self.rendered.lock().unwrap();
                                let (_, current_x, current_y) = *rendered
                                    .iter()
                                    .find(|r| r.0 == self.cursor)
                                    .unwrap_or(&(0, 0, 0));

                                if let Some(index) = find_closest_in_direction(
                                    Direction::Down,
                                    current_x,
                                    current_y,
                                    &rendered,
                                    self.cursor,
                                ) {
                                    self.cursor = index;

                                    for (i, w) in ctx.get_widgets().iter().enumerate() {
                                        if let Some(AlwaysFocused) = w.get() {
                                            w.set_focused(true);
                                        } else {
                                            w.set_focused(i == self.cursor);
                                        }
                                    }
                                }
                            }

                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn render(&mut self, _ctx: &super::Context, _scope: &mut crate::prelude::RenderScope) {
        self.rendered.lock().unwrap().clear();
    }

    fn after_render_widget(
        &mut self,
        ctx: &super::Context,
        scope: &mut crate::prelude::RenderScope,
        widget: &std::sync::Arc<crate::prelude::Widget>,
    ) {
        let t = scope.get_transform().clone();
        let ctx = ctx.clone();
        let widget = widget.clone();

        let rendered = self.rendered.clone();

        std::thread::spawn(move || {
            if let Some(p) = ctx
                .get_widgets()
                .iter()
                .position(|w| Arc::ptr_eq(w, &widget) && !w.is_ghost())
            {
                rendered.lock().unwrap().push((p, t.x, t.y));
            }
        });
    }
}

impl RelativeFocusExtension {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            rendered: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

fn find_closest_in_direction(
    direction: Direction,
    current_x: u16,
    current_y: u16,
    rendered: &[(usize, u16, u16)],
    cursor: usize,
) -> Option<usize> {
    let mut closest_index: Option<usize> = None;
    let mut closest_distance = u16::MAX;
    let mut same_line_or_col_found = false;

    for &(i, x, y) in rendered {
        if i == cursor {
            continue;
        }

        match direction {
            Direction::Right if x > current_x => {
                if y == current_y {
                    let dx = x - current_x;
                    if !same_line_or_col_found || dx < closest_distance {
                        closest_index = Some(i);
                        closest_distance = dx;
                        same_line_or_col_found = true;
                    }
                } else if !same_line_or_col_found {
                    let dx = x - current_x;
                    let dy = (y as isize - current_y as isize).abs() as u16;
                    let dist = dx * dx + dy * dy;
                    if dist < closest_distance
                        || (dist == closest_distance && i < closest_index.unwrap_or(usize::MAX))
                    {
                        closest_index = Some(i);
                        closest_distance = dist;
                    }
                }
            }

            Direction::Left if x < current_x => {
                if y == current_y {
                    let dx = current_x - x;
                    if !same_line_or_col_found || dx < closest_distance {
                        closest_index = Some(i);
                        closest_distance = dx;
                        same_line_or_col_found = true;
                    }
                } else if !same_line_or_col_found {
                    let dx = current_x - x;
                    let dy = (y as isize - current_y as isize).abs() as u16;
                    let dist = dx * dx + dy * dy;
                    if dist < closest_distance
                        || (dist == closest_distance && i < closest_index.unwrap_or(usize::MAX))
                    {
                        closest_index = Some(i);
                        closest_distance = dist;
                    }
                }
            }

            Direction::Down if y > current_y => {
                if x == current_x {
                    let dy = y - current_y;
                    if !same_line_or_col_found || dy < closest_distance {
                        closest_index = Some(i);
                        closest_distance = dy;
                        same_line_or_col_found = true;
                    }
                } else if !same_line_or_col_found {
                    let dx = (x as isize - current_x as isize).abs() as u16;
                    let dy = y - current_y;
                    let dist = dx * dx + dy * dy;
                    if dist < closest_distance
                        || (dist == closest_distance && i < closest_index.unwrap_or(usize::MAX))
                    {
                        closest_index = Some(i);
                        closest_distance = dist;
                    }
                }
            }

            Direction::Up if y < current_y => {
                if x == current_x {
                    let dy = current_y - y;
                    if !same_line_or_col_found || dy < closest_distance {
                        closest_index = Some(i);
                        closest_distance = dy;
                        same_line_or_col_found = true;
                    }
                } else if !same_line_or_col_found {
                    let dx = (x as isize - current_x as isize).abs() as u16;
                    let dy = current_y - y;
                    let dist = dx * dx + dy * dy;
                    if dist < closest_distance
                        || (dist == closest_distance && i < closest_index.unwrap_or(usize::MAX))
                    {
                        closest_index = Some(i);
                        closest_distance = dist;
                    }
                }
            }

            _ => {}
        }
    }

    closest_index
}
