mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    // Pentagon
    let pentagon = gs::Pentagon::new(
        &gs::Point::new(700, 100),
        &gs::Point::new(800, 150),
        &gs::Point::new(750, 250),
        &gs::Point::new(650, 250),
        &gs::Point::new(600, 150),
    );
    pentagon.draw(&mut image);

    // Cube
    let cube = gs::Cube::new(
        &gs::Point::new(100, 400),
        &gs::Point::new(200, 400),
        &gs::Point::new(200, 500),
        &gs::Point::new(100, 500),
        &gs::Point::new(150, 350),
        &gs::Point::new(250, 350),
        &gs::Point::new(250, 450),
        &gs::Point::new(150, 450),
    );
    cube.draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}