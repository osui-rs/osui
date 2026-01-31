//! # Rendering Module
//!
//! This module provides low-level rendering primitives and data structures
//! for drawing content to the terminal. It includes geometric primitives
//! (Point, Area, Size) and drawing instructions.

use crate::View;

/// Represents a drawing instruction that can be executed by the rendering engine
#[derive(Clone)]
pub enum DrawInstruction {
    /// Draw text at a specific point
    Text(Point, String),
    /// Render a view within a specified area
    View(Area, View),
    /// Render a child drawing context at an offset
    Child(Point, DrawContext),
}

/// Represents the dimensions of a drawable area
#[derive(Clone)]
pub struct Size {
    /// Width in terminal columns
    pub width: u16,
    /// Height in terminal rows
    pub height: u16,
}

/// Represents a position in 2D space
#[derive(Clone)]
pub struct Point {
    /// X coordinate (column)
    pub x: u16,
    /// Y coordinate (row)
    pub y: u16,
}

/// Represents a rectangular area with position and dimensions
#[derive(Clone)]
pub struct Area {
    /// X coordinate (column) of the top-left corner
    pub x: u16,
    /// Y coordinate (row) of the top-left corner
    pub y: u16,
    /// Width in terminal columns
    pub width: u16,
    /// Height in terminal rows
    pub height: u16,
}

/// Context for drawing operations
///
/// Accumulates drawing instructions that are executed by the rendering engine.
/// Tracks allocated space within the drawable area.
#[derive(Clone)]
pub struct DrawContext {
    /// The total area available for drawing
    pub area: Area,
    /// The area that has been allocated for drawing (union of all allocations)
    pub allocated: Area,
    /// List of drawing instructions to execute
    pub drawing: Vec<DrawInstruction>,
}

impl DrawContext {
    /// Creates a new DrawContext with the specified area
    pub fn new(area: Area) -> Self {
        Self {
            area,
            allocated: Area {
                x: u16::MAX,
                y: u16::MAX,
                width: 0,
                height: 0,
            },
            drawing: Vec::new(),
        }
    }

    /// Allocates space within the drawable area and returns the allocated area
    /// Updates the allocated bounds to include this allocation
    pub fn allocate(&mut self, x: u16, y: u16, width: u16, height: u16) -> Area {
        self.allocated.x = self.allocated.x.min(x);
        self.allocated.y = self.allocated.y.min(y);
        self.allocated.width = self.allocated.width.max(width);
        self.allocated.height = self.allocated.height.max(height);

        Area {
            x,
            y,
            width,
            height,
        }
    }

    /// Adds a drawing instruction to be executed
    pub fn draw(&mut self, inst: DrawInstruction) {
        self.drawing.push(inst);
    }

    /// Draws text at the specified point
    pub fn draw_text(&mut self, point: Point, text: &str) {
        self.drawing
            .push(DrawInstruction::Text(point, text.to_string()));
    }

    /// Draws a view within the specified area
    pub fn draw_view(&mut self, area: Area, view: View) {
        self.drawing.push(DrawInstruction::View(area, view));
    }
}
