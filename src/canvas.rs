use std::fmt::Display;

use crate::braille;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            pixels: vec![
                Pixel {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0
                };
                (width * height) as usize
            ],
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[(x + y * self.width) as usize] = pixel;
    }

    #[allow(dead_code)]
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, pixel: Pixel) {
        let dx = (x1 as isize - x0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 as isize - y0 as isize).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        loop {
            self.draw_pixel(x as usize, y as usize, pixel);
            if x == x1 as isize && y == y1 as isize {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        for y in (0..self.height).step_by(4) {
            s.push('║');
            for x in (0..self.width).step_by(2) {
                let braille = [
                    if x + 0 < self.width && y + 0 < self.height {
                        Some((x + 0) + (y + 0) * self.width)
                    } else {
                        None
                    },
                    if x + 0 < self.width && y + 1 < self.height {
                        Some((x + 0) + (y + 1) * self.width)
                    } else {
                        None
                    },
                    if x + 0 < self.width && y + 2 < self.height {
                        Some((x + 0) + (y + 2) * self.width)
                    } else {
                        None
                    },
                    if x + 1 < self.width && y + 0 < self.height {
                        Some((x + 1) + (y + 0) * self.width)
                    } else {
                        None
                    },
                    if x + 1 < self.width && y + 1 < self.height {
                        Some((x + 1) + (y + 1) * self.width)
                    } else {
                        None
                    },
                    if x + 1 < self.width && y + 2 < self.height {
                        Some((x + 1) + (y + 2) * self.width)
                    } else {
                        None
                    },
                    if x + 0 < self.width && y + 3 < self.height {
                        Some((x + 0) + (y + 3) * self.width)
                    } else {
                        None
                    },
                    if x + 1 < self.width && y + 3 < self.height {
                        Some((x + 1) + (y + 3) * self.width)
                    } else {
                        None
                    },
                ]
                .iter()
                .map(|&i| {
                    if let Some(i) = i {
                        let pixel = self.pixels[i as usize];
                        (((pixel.r | pixel.g | pixel.b) & pixel.a) > 0) as u8
                    } else {
                        0
                    }
                })
                .collect::<Vec<u8>>();
                s.push(braille::Braille::from(braille.as_slice()).0);
            }
            s.push('║');
            s.push('\n');
        }
        let width = (self.width as usize + 1) / 2 as usize;
        write!(f, "╔{:═^width$}╗\n{s}╚{:═^width$}╝", "", "")
    }
}

#[test]
fn test() {
    let mut canvas = Canvas::new(10, 10);
}
