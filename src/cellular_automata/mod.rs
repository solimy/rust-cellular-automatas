mod conway;
mod highlife;
mod gravity;


use rayon::prelude::IntoParallelRefIterator;

use crate::canvas::{Canvas, Pixel};

#[derive(Debug, Clone)]
pub struct Cell {
    pub is_alive: bool,
    pub is_protected: bool,
}


impl Cell {
    pub fn new() -> Cell {
        Cell { is_alive: false, is_protected: true }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::new()
    }
}


pub struct Neighbors<'a> {
    up: Option<&'a Cell>,
    up_right: Option<&'a Cell>,
    right: Option<&'a Cell>,
    down_right: Option<&'a Cell>,
    down: Option<&'a Cell>,
    down_left: Option<&'a Cell>,
    left: Option<&'a Cell>,
    up_left: Option<&'a Cell>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rules {
    Conway,
    HighLife,
    Gravity(bool),
}



#[derive(Debug)]
pub struct World
{
    pub width: usize,
    pub height: usize,
    pub rule: Rules,
    pub cells: Vec<Cell>,
}


impl World {
    pub fn new(rules: Rules, width: usize, height: usize) -> World {
        World {
            width: width,
            height: height,
            rule: rules,
            cells: (0..(width * height))
                .into_iter()
                .map(|_| Cell::default())
                .collect(),
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();
        for (i, cell) in self.cells.iter().enumerate() {
            let neighbors = Neighbors {
                up: if i >= self.width { Some(&self.cells[i - self.width]) } else { None },
                up_right: if i >= self.width && i % self.width != self.width - 1 { Some(&self.cells[i - self.width + 1]) } else { None },
                right: if i % self.width != self.width - 1 { Some(&self.cells[i + 1]) } else { None },
                down_right: if i < self.width * (self.height - 1) && i % self.width != self.width - 1 { Some(&self.cells[i + self.width + 1]) } else { None },
                down: if i < self.width * (self.height - 1) { Some(&self.cells[i + self.width]) } else { None },
                down_left: if i < self.width * (self.height - 1) && i % self.width != 0 { Some(&self.cells[i + self.width - 1]) } else { None },
                left: if i % self.width != 0 { Some(&self.cells[i - 1]) } else { None },
                up_left: if i >= self.width && i % self.width != 0 { Some(&self.cells[i - self.width - 1]) } else { None },
            };
            match self.rule {
                Rules::Conway => conway::tick(&mut new_cells[i], &neighbors),
                Rules::HighLife => highlife::tick(&mut new_cells[i], &neighbors),
                Rules::Gravity(stack) => gravity::tick(&mut new_cells[i], &neighbors, stack),
            };
        }
        self.cells = new_cells;
        match self.rule {
            Rules::Gravity(_) => self.populate(0.01),
            _ => (),
        }
    }

    pub fn kill(&mut self, x: usize, y: usize) {
        self.cells[y * self.width + x].is_alive = false;
    }

    pub fn revive(&mut self, x: usize, y: usize) {
        self.cells[y * self.width + x].is_alive = true;
    }

    pub fn populate(&mut self, p: f32) {
        for (index, cell) in self.cells.iter_mut().enumerate() {
            let y = index / self.width;
            match self.rule {
                Rules::Gravity(_) => gravity::populate(y, cell, p),
                _ => cell.is_alive = rand::random::<f32>() < p,
            }
        }
    }
}


impl std::fmt::Display for World
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Canvas::from(self))
    }
}


impl From<&World> for Canvas
{
    fn from(world: &World) -> Canvas {
        let mut canvas = Canvas::new(world.width as usize, world.height as usize);
        for (i, cell) in world.cells.iter().enumerate() {
            let x = i % world.width as usize;
            let y = i / world.width as usize;
            // println!("xy({},{}) -> {}", x, y, cell.is_alive);
            if cell.is_alive {
                canvas.draw_pixel(x, y, Pixel { r: 255, g: 255, b: 255, a: 255 });
            }
        }
        canvas
    }
}
