use raster::{Color, Image};

// Drawable which contains the methods draw and color.
pub trait Drawable {
    fn draw(&self, img: &mut Image) {}
    fn color() -> Color;
}

// Displayable which contains the method display.
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color) {}
}

// Point: a new point should be created from two i32 values.
#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
// Line: a new line should be created from references to two different points.
pub struct Line {
    pub point_a: Point,
    pub point_b: Point,
}

// Triangle: a new triangle should be created from references to three different points.
pub struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
}

// Rectangle: a new rectangle should be created from references to two different points.
pub struct Rectangle {
    pub point_a: Point,
    pub point_b: Point,
}

// Circle: a new circle should be created from a reference to a point representing the center, and an i32 value representing the circle's radius.
pub struct Circle {
    pub point: Point,
    pub radius: i32,
}

use rand::prelude::*;

fn draw_line(image: &mut Image, a: &Point, b: &Point, colour: &Color) {
    let mut x0 = a.x;
    let mut y0 = a.y;
    let x1 = b.x;
    let y1 = b.y;

    // Get absolute x/y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;
    loop {
        // Set pixel
        image.set_pixel(x0, y0, colour.clone());

        // Check end condition
        if x0 == x1 && y0 == y1 {
            break;
        };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}
// Point //
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn random(x: i32, y: i32) -> Self {
        let rng_x = rand::thread_rng().gen_range(0..x);
        let rng_y = rand::thread_rng().gen_range(0..y);
        Self { x: rng_x, y: rng_y }
    }
}
impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        // Set pixel
        let colour = Point::color();
        image.set_pixel(self.x, self.y, colour);
    }
    fn color() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0..=255),
            g: rand::thread_rng().gen_range(0..=255),
            b: rand::thread_rng().gen_range(0..=255),
            a: 255,
        }
    }
}
//....//

// Line //
impl Line {
    pub fn new(point_a: &Point, point_b: &Point) -> Self {
        Self {
            point_a: *point_a,
            point_b: *point_b,
        }
    }

    pub fn random(x: i32, y: i32) -> Self {
        let mut rng_x = rand::thread_rng().gen_range(0..x);
        let mut rng_y = rand::thread_rng().gen_range(0..y);
        let mut rng_x2 = rand::thread_rng().gen_range(0..x);
        let mut rng_y2 = rand::thread_rng().gen_range(0..y);
        Self {
            point_a: Point { x: rng_x, y: rng_y },
            point_b: Point {
                x: rng_x2,
                y: rng_y2,
            },
        }
    }
}
impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let colour = Line::color();
        draw_line(image, &self.point_a, &self.point_b, &colour);
    }
    fn color() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0..=255),
            g: rand::thread_rng().gen_range(0..=255),
            b: rand::thread_rng().gen_range(0..=255),
            a: 255,
        }
    }
}
//....//

// Circle //
impl Circle {
    pub fn new(point: &Point, radius: i32) -> Self {
        Self {
            point: *point,
            radius,
        }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng_x = rand::thread_rng().gen_range(0..width);
        let mut rng_y = rand::thread_rng().gen_range(0..height);
        let mut rng_radius = rand::thread_rng().gen_range(0..height);

        Self {
            point: Point { x: rng_x, y: rng_y },
            radius: rng_radius,
        }
    }
}
impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let x0 = self.point.x;
        let y0 = self.point.y;
        let colour = Circle::color();
        let pi = std::f64::consts::PI;
        let mut i: f64 = 0.;
        loop {
            let angle = i;
            let x: i32 = (self.radius as f64 * (angle * pi / 180.).cos()) as i32;
            let y: i32 = (self.radius as f64 * (angle * pi / 180.).sin()) as i32;
            image.set_pixel(x0 + x, y0 + y, colour.clone());
            i += 0.1;
            if i > 360. {
                break;
            }
        }
    }
    fn color() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0..=255),
            g: rand::thread_rng().gen_range(0..=255),
            b: rand::thread_rng().gen_range(0..=255),
            a: 255,
        }
    }
}
//....//

// Triangle //
impl Triangle {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Self {
        Self {
            point_a: *point_a,
            point_b: *point_b,
            point_c: *point_c,
        }
    }
}
impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let colour = Triangle::color();
        draw_line(image, &self.point_a, &self.point_b, &colour);
        draw_line(image, &self.point_b, &self.point_c, &colour);
        draw_line(image, &self.point_c, &self.point_a, &colour);
    }
    fn color() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0..=255),
            g: rand::thread_rng().gen_range(0..=255),
            b: rand::thread_rng().gen_range(0..=255),
            a: 255,
        }
    }
}
//....//

// Rectangle //
impl Rectangle {
    pub fn new(point_a: &Point, point_b: &Point) -> Self {
        Self {
            point_a: *point_a,
            point_b: *point_b,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let mut x0 = self.point_a.x;
        let mut x1 = self.point_b.x;
        let mut y0 = self.point_a.x;
        let mut y1 = self.point_b.y;

         
        // check if it's a square and changes either length or side to make it look like a rectangle we
        // are all familiar with. However, a square is a special kind of rectangle.
        // let is_sqaure = (x1 - x0) == (y1 - y0);
        // if is_sqaure{
        //     // x1 = x1*2;
        //     if y0 != 0{
        //         y0 = y0 * 2;
        //     }else if y1 > 0{
        //         y1 = y1*2;
        //     }else if x1 != 0{
        //         x1 = x1*2;
        //     }else{
        //         x0 = x0 * 2;
        //     }
        // }
        let point_a = Point { x: x0, y: y0 };
        let point_b = Point { x: x1, y: y1 };
        let point_ab = Point { x: x0, y: y1 };
        let point_ba = Point { x: x1, y: y0 };
        let colour = Rectangle::color();
        draw_line(image, &point_a, &point_ab, &colour);
        draw_line(image, &point_ab, &point_b, &colour);
        draw_line(image, &point_b, &point_ba, &colour);
        draw_line(image, &point_ba, &point_a, &colour);
    }
    fn color() -> Color {
        Color {
            r: rand::thread_rng().gen_range(0..=255),
            g: rand::thread_rng().gen_range(0..=255),
            b: rand::thread_rng().gen_range(0..=255),
            a: 255,
        }
    }
}
// ....//

// impl Displayable for Image {
//     fn display(&mut self, x: i32, y: i32, color: Color) {
//         if x >= 0 && x < self.width && y >= 0 && y < self.height {
//             self.set_pixel(x, y, color).unwrap();
//         }
//     }
// }
