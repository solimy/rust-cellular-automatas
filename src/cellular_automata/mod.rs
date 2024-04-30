mod conway;
mod gravity;
mod highlife;

use crate::canvas::{Canvas, Pixel};

#[derive(Debug, Clone)]
pub struct Cell {
    pub is_alive: bool,
    pub is_protected: bool,
    pub age: u8,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            is_alive: false,
            is_protected: true,
            age: 0,
        }
    }

    pub fn get_older(&mut self) {
        self.age = if self.is_alive { if self.age < u8::MAX { self.age + 1 } else { u8::MAX } } else { 0 };
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
pub struct World {
    pub width: usize,
    pub height: usize,
    pub rule: Rules,
    pub cells: Vec<Cell>,
    pub epoch: u64,
    pub reset_at_epoch: u64,
    pub pop_rate: f32,
}

impl World {
    pub fn new(
        rule: Rules,
        width: usize,
        height: usize,
        reset_at_epoch: u64,
        pop_rate: f32,
    ) -> World {
        World {
            width,
            height,
            rule,
            reset_at_epoch,
            pop_rate,
            epoch: 0,
            cells: (0..(width * height))
                .into_iter()
                .map(|_| Cell::default())
                .collect(),
        }
    }

    pub fn tick(&mut self) {
        self.epoch += 1;

        if self.reset_at_epoch > 0 && self.epoch % self.reset_at_epoch == 0 {
            self.cells = self.cells.iter().map(|_| Cell::default()).collect();
            self.populate();
        }

        let mut new_cells = self.cells.clone();
        self.cells.iter().enumerate().for_each(|(i, _)| {
            let neighbors = Neighbors {
                up: if i >= self.width {
                    Some(&self.cells[i - self.width])
                } else {
                    None
                },
                up_right: if i >= self.width && i % self.width != self.width - 1 {
                    Some(&self.cells[i - self.width + 1])
                } else {
                    None
                },
                right: if i % self.width != self.width - 1 {
                    Some(&self.cells[i + 1])
                } else {
                    None
                },
                down_right: if i < self.width * (self.height - 1)
                    && i % self.width != self.width - 1
                {
                    Some(&self.cells[i + self.width + 1])
                } else {
                    None
                },
                down: if i < self.width * (self.height - 1) {
                    Some(&self.cells[i + self.width])
                } else {
                    None
                },
                down_left: if i < self.width * (self.height - 1) && i % self.width != 0 {
                    Some(&self.cells[i + self.width - 1])
                } else {
                    None
                },
                left: if i % self.width != 0 {
                    Some(&self.cells[i - 1])
                } else {
                    None
                },
                up_left: if i >= self.width && i % self.width != 0 {
                    Some(&self.cells[i - self.width - 1])
                } else {
                    None
                },
            };
            match self.rule {
                Rules::Conway => conway::tick(&mut new_cells[i], &neighbors),
                Rules::HighLife => highlife::tick(&mut new_cells[i], &neighbors),
                Rules::Gravity(stack) => gravity::tick(&mut new_cells[i], &neighbors, stack),
            };
        });
        self.cells = new_cells;
        match self.rule {
            Rules::Gravity(_) => {
                let old_pop_rate = self.pop_rate;
                self.pop_rate = self.pop_rate * 0.1;
                self.populate();
                self.pop_rate = old_pop_rate;
            }
            _ => (),
        }
    }

    #[allow(dead_code)]
    pub fn kill(&mut self, x: usize, y: usize) {
        self.cells[y * self.width + x].is_alive = false;
    }

    pub fn revive(&mut self, x: usize, y: usize) {
        self.cells[y * self.width + x].is_alive = true;
    }

    pub fn populate(&mut self) {
        for (index, cell) in self.cells.iter_mut().enumerate() {
            let y = index / self.width;
            match self.rule {
                Rules::Gravity(_) => gravity::populate(y, cell, self.pop_rate),
                _ => cell.is_alive = rand::random::<f32>() < self.pop_rate,
            }
        }
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Canvas::from(self))
    }
}

impl From<&World> for Canvas {
    fn from(world: &World) -> Canvas {
        let mut canvas = Canvas::new(world.width as usize, world.height as usize);
        for (i, cell) in world.cells.iter().enumerate() {
            let x = i % world.width as usize;
            let y = i / world.width as usize;
            // println!("xy({},{}) -> {}", x, y, cell.is_alive);
            if cell.is_alive {
                canvas.draw_pixel(
                    x,
                    y,
                    Pixel {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );
            }
        }
        canvas
    }
}
