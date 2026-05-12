use raster::{Color, Image};
use rand::Rng;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0, width),
            y: rng.gen_range(0, height),
        }
    }
}


pub struct Line {
    pub p1: Point,
    pub p2: Point,
}


impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Line {
        Line {
            p1: Point { x: p1.x, y: p1.y },
            p2: Point { x: p2.x, y: p2.y },
        }
    }

    pub fn random(width: i32, height: i32) -> Line {
        Line::new(
            &Point::random(width, height),
            &Point::random(width, height),
        )
    }
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {                                                                                                                                          
    pub fn new(p1: &Point, p2: &Point,p3: &Point) -> Triangle {                                                                                                   
        Triangle {
            p1: Point { x: p1.x, y: p1.y },
            p2: Point { x: p2.x, y: p2.y },
            p3: Point { x: p3.x, y: p3.y },
        }                                                                                                                                            
    }
} 



pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

                                                                                                                                                    
impl Rectangle {                                                                                                                                     
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {                                                                                                
        Rectangle {                                                                                                                                  
            p1: Point { x: p1.x, y: p1.y },                                                                                                        
            p2: Point { x: p2.x, y: p2.y },                                                                                                          
        }
    }                                                                                                                                                
} 

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            center: Point { x: center.x, y: center.y },
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Circle {
        let mut rng = rand::thread_rng();
        Circle::new(
            &Point::random(width, height),
            rng.gen_range(10, 400),
        )
    }
}

pub struct Pentagon {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub p4: Point,
    pub p5: Point,
}

impl Pentagon {
    pub fn new(p1: &Point, p2: &Point, p3: &Point, p4: &Point, p5: &Point) -> Pentagon {
        Pentagon {
            p1: Point { x: p1.x, y: p1.y },
            p2: Point { x: p2.x, y: p2.y },
            p3: Point { x: p3.x, y: p3.y },
            p4: Point { x: p4.x, y: p4.y },
            p5: Point { x: p5.x, y: p5.y },
        }
    }
}

pub struct Cube {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub p4: Point,
    pub p5: Point,
    pub p6: Point,
    pub p7: Point,
    pub p8: Point,
}

impl Cube {
    pub fn new(
        p1: &Point, p2: &Point, p3: &Point, p4: &Point,
        p5: &Point, p6: &Point, p7: &Point, p8: &Point,
    ) -> Cube {
        Cube {
            p1: Point { x: p1.x, y: p1.y },
            p2: Point { x: p2.x, y: p2.y },
            p3: Point { x: p3.x, y: p3.y },
            p4: Point { x: p4.x, y: p4.y },
            p5: Point { x: p5.x, y: p5.y },
            p6: Point { x: p6.x, y: p6.y },
            p7: Point { x: p7.x, y: p7.y },
            p8: Point { x: p8.x, y: p8.y },
        }
    }
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

fn draw_line(image: &mut Image, p1: &Point, p2: &Point, color: Color) {
    let mut x0 = p1.x;
    let mut y0 = p1.y;
    let x1 = p2.x;
    let y1 = p2.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        image.display(x0, y0, color.clone());
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

impl Drawable for Point {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
}

impl Drawable for Line {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        draw_line(image, &self.p1, &self.p2, self.color());
    }
}

impl Drawable for Triangle {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        let color = self.color();
        draw_line(image, &self.p1, &self.p2, color.clone());
        draw_line(image, &self.p2, &self.p3, color.clone());
        draw_line(image, &self.p3, &self.p1, color.clone());
    }
}

impl Drawable for Rectangle {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let top_right = Point { x: self.p2.x, y: self.p1.y };
        let bottom_left = Point { x: self.p1.x, y: self.p2.y };
        draw_line(image, &self.p1, &top_right, color.clone());
        draw_line(image, &top_right, &self.p2, color.clone());
        draw_line(image, &self.p2, &bottom_left, color.clone());
        draw_line(image, &bottom_left, &self.p1, color.clone());
    }
}

impl Drawable for Circle {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;
        let cx = self.center.x;
        let cy = self.center.y;

        while x >= y {
            image.display(cx + x, cy + y, color.clone());
            image.display(cx + y, cy + x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx - x, cy - y, color.clone());
            image.display(cx - y, cy - x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx + x, cy - y, color.clone());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }
}

impl Drawable for Pentagon {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        let color = self.color();
        draw_line(image, &self.p1, &self.p2, color.clone());
        draw_line(image, &self.p2, &self.p3, color.clone());
        draw_line(image, &self.p3, &self.p4, color.clone());
        draw_line(image, &self.p4, &self.p5, color.clone());
        draw_line(image, &self.p5, &self.p1, color.clone());
    }
}

impl Drawable for Cube {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
            rng.gen_range(30, 255),
        );
        return color;
    }

    fn draw(&self, image: &mut Image) {
        let color = self.color();
        
        // Front face (p1, p2, p3, p4)
        draw_line(image, &self.p1, &self.p2, color.clone());
        draw_line(image, &self.p2, &self.p3, color.clone());
        draw_line(image, &self.p3, &self.p4, color.clone());
        draw_line(image, &self.p4, &self.p1, color.clone());
        
        // Back face (p5, p6, p7, p8)
        draw_line(image, &self.p5, &self.p6, color.clone());
        draw_line(image, &self.p6, &self.p7, color.clone());
        draw_line(image, &self.p7, &self.p8, color.clone());
        draw_line(image, &self.p8, &self.p5, color.clone());
        
        // Connect front to back
        draw_line(image, &self.p1, &self.p5, color.clone());
        draw_line(image, &self.p2, &self.p6, color.clone());
        draw_line(image, &self.p3, &self.p7, color.clone());
        draw_line(image, &self.p4, &self.p8, color.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RANDOM FUNCTION BOUNDS TESTS

    #[test]
    fn test_point_random_within_bounds() {
        let width = 500;
        let height = 600;
        
        for _ in 0..100 {
            let point = Point::random(width, height);
            assert!(point.x >= 0 && point.x < width, 
                    "Point x coordinate {} is out of bounds [0, {})", point.x, width);
            assert!(point.y >= 0 && point.y < height, 
                    "Point y coordinate {} is out of bounds [0, {})", point.y, height);
        }
    }

    #[test]
    fn test_point_random_edge_dimensions() {
        let point = Point::random(1, 1);
        assert_eq!(point.x, 0, "Point x should be 0 when width is 1");
        assert_eq!(point.y, 0, "Point y should be 0 when height is 1");
    }

    #[test]
    fn test_point_random_large_dimensions() {
        let width = 10000;
        let height = 10000;
        
        for _ in 0..50 {
            let point = Point::random(width, height);
            assert!(point.x >= 0 && point.x < width);
            assert!(point.y >= 0 && point.y < height);
        }
    }

    #[test]
    fn test_line_random_points_within_bounds() {
        let width = 800;
        let height = 600;
        
        for _ in 0..50 {
            let line = Line::random(width, height);
            assert!(line.p1.x >= 0 && line.p1.x < width);
            assert!(line.p1.y >= 0 && line.p1.y < height);
            assert!(line.p2.x >= 0 && line.p2.x < width);
            assert!(line.p2.y >= 0 && line.p2.y < height);
        }
    }

    #[test]
    fn test_circle_random_within_bounds() {
        let width = 1000;
        let height = 1000;
        
        for _ in 0..50 {
            let circle = Circle::random(width, height);
            assert!(circle.center.x >= 0 && circle.center.x < width, 
                    "Circle center x {} is out of bounds", circle.center.x);
            assert!(circle.center.y >= 0 && circle.center.y < height,
                    "Circle center y {} is out of bounds", circle.center.y);
            assert!(circle.radius > 0, "Circle radius should be positive");
            assert!(circle.radius < 400, "Circle radius should be less than 400");
        }
    }

    //SHAPE INITIALIZATION TESTS

    #[test]
    fn test_point_new() {
        let point = Point::new(42, 100);
        assert_eq!(point.x, 42);
        assert_eq!(point.y, 100);
    }

    #[test]
    fn test_point_new_with_zeros() {
        let point = Point::new(0, 0);
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn test_point_new_with_negative_values() {
        let point = Point::new(-50, -100);
        assert_eq!(point.x, -50);
        assert_eq!(point.y, -100);
    }

    #[test]
    fn test_line_new_different_points() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(30, 40);
        let line = Line::new(&p1, &p2);
        
        assert_eq!(line.p1.x, 10);
        assert_eq!(line.p1.y, 20);
        assert_eq!(line.p2.x, 30);
        assert_eq!(line.p2.y, 40);
    }

    #[test]
    fn test_triangle_new() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 0);
        let p3 = Point::new(5, 10);
        let triangle = Triangle::new(&p1, &p2, &p3);
        
        assert_eq!(triangle.p1.x, 0);
        assert_eq!(triangle.p2.y, 0);
        assert_eq!(triangle.p3.x, 5);
    }

    #[test]
    fn test_rectangle_new() {
        let p1 = Point::new(100, 200);
        let p2 = Point::new(300, 400);
        let rect = Rectangle::new(&p1, &p2);
        
        assert_eq!(rect.p1.x, 100);
        assert_eq!(rect.p1.y, 200);
        assert_eq!(rect.p2.x, 300);
        assert_eq!(rect.p2.y, 400);
    }

    #[test]
    fn test_rectangle_inverted_coordinates() {
        let p1 = Point::new(300, 400);
        let p2 = Point::new(100, 200);
        let rect = Rectangle::new(&p1, &p2);
        
        assert_eq!(rect.p1.x, 300);
        assert_eq!(rect.p1.y, 400);
        assert_eq!(rect.p2.x, 100);
        assert_eq!(rect.p2.y, 200);
    }

    #[test]
    fn test_circle_new() {
        let center = Point::new(500, 500);
        let circle = Circle::new(&center, 75);
        
        assert_eq!(circle.center.x, 500);
        assert_eq!(circle.center.y, 500);
        assert_eq!(circle.radius, 75);
    }

    #[test]
    fn test_circle_new_small_radius() {
        let center = Point::new(100, 100);
        let circle = Circle::new(&center, 1);
        assert_eq!(circle.radius, 1);
    }

    #[test]
    fn test_circle_new_large_radius() {
        let center = Point::new(0, 0);
        let circle = Circle::new(&center, 500);
        assert_eq!(circle.radius, 500);
    }

    //EDGE CASES TESTS

    #[test]
    fn test_line_same_start_and_end_point() {
        let p = Point::new(50, 50);
        let line = Line::new(&p, &p);
        
        assert_eq!(line.p1.x, line.p2.x);
        assert_eq!(line.p1.y, line.p2.y);
    }

    #[test]
    fn test_line_horizontal() {
        let p1 = Point::new(0, 50);
        let p2 = Point::new(100, 50);
        let line = Line::new(&p1, &p2);
        
        assert_eq!(line.p1.y, line.p2.y, "Horizontal line should have same y coordinates");
    }

    #[test]
    fn test_line_vertical() {
        let p1 = Point::new(50, 0);
        let p2 = Point::new(50, 100);
        let line = Line::new(&p1, &p2);
        
        assert_eq!(line.p1.x, line.p2.x, "Vertical line should have same x coordinates");
    }

    #[test]
    fn test_triangle_collinear_points() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(5, 5);
        let p3 = Point::new(10, 10);
        let triangle = Triangle::new(&p1, &p2, &p3);
        
        assert_eq!(triangle.p1.x, 0);
        assert_eq!(triangle.p2.x, 5);
        assert_eq!(triangle.p3.x, 10);
    }

    #[test]
    fn test_rectangle_zero_area() {
        let p = Point::new(100, 100);
        let rect = Rectangle::new(&p, &p);
        
        assert_eq!(rect.p1.x, rect.p2.x);
        assert_eq!(rect.p1.y, rect.p2.y);
    }

    //COLOR TESTS

    #[test]
    fn test_point_color() {
        let point = Point::new(0, 0);
        let color = point.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    #[test]
    fn test_line_color() {
        let line = Line::new(&Point::new(0, 0), &Point::new(10, 10));
        let color = line.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    #[test]
    fn test_triangle_color() {
        let triangle = Triangle::new(
            &Point::new(0, 0),
            &Point::new(10, 0),
            &Point::new(5, 10),
        );
        let color = triangle.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    #[test]
    fn test_rectangle_color() {
        let rect = Rectangle::new(&Point::new(0, 0), &Point::new(10, 10));
        let color = rect.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    #[test]
    fn test_circle_color() {
        let circle = Circle::new(&Point::new(0, 0), 50);
        let color = circle.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    //PENTAGON TESTS

    #[test]
    fn test_pentagon_new() {
        let p1 = Point::new(100, 50);
        let p2 = Point::new(200, 20);
        let p3 = Point::new(250, 120);
        let p4 = Point::new(180, 200);
        let p5 = Point::new(50, 180);
        
        let pentagon = Pentagon::new(&p1, &p2, &p3, &p4, &p5);
        
        assert_eq!(pentagon.p1.x, 100);
        assert_eq!(pentagon.p2.x, 200);
        assert_eq!(pentagon.p3.x, 250);
        assert_eq!(pentagon.p4.x, 180);
        assert_eq!(pentagon.p5.x, 50);
    }

    #[test]
    fn test_pentagon_color() {
        let p = Point::new(0, 0);
        let pentagon = Pentagon::new(&p, &p, &p, &p, &p);
        let color = pentagon.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    //CUBE TESTS

    #[test]
    fn test_cube_new() {
        let p1 = Point::new(100, 100);
        let p2 = Point::new(200, 100);
        let p3 = Point::new(200, 200);
        let p4 = Point::new(100, 200);
        let p5 = Point::new(150, 50);
        let p6 = Point::new(250, 50);
        let p7 = Point::new(250, 150);
        let p8 = Point::new(150, 150);
        
        let cube = Cube::new(&p1, &p2, &p3, &p4, &p5, &p6, &p7, &p8);
        
        assert_eq!(cube.p1.x, 100);
        assert_eq!(cube.p5.x, 150);
        assert_eq!(cube.p8.y, 150);
    }

    #[test]
    fn test_cube_color() {
        let p = Point::new(0, 0);
        let cube = Cube::new(&p, &p, &p, &p, &p, &p, &p, &p);
        let color = cube.color();
        
        assert!((0..255).contains(&color.r));
        assert!((0..255).contains(&color.g));
        assert!((0..255).contains(&color.b));
    }

    #[test]
    fn test_cube_all_vertices_stored() {
        let points: Vec<Point> = (0..8)
            .map(|i| Point::new(i * 10, i * 10))
            .collect();
        
        let cube = Cube::new(
            &points[0], &points[1], &points[2], &points[3],
            &points[4], &points[5], &points[6], &points[7],
        );
        
        assert_eq!(cube.p1.x, 0);
        assert_eq!(cube.p2.x, 10);
        assert_eq!(cube.p3.x, 20);
        assert_eq!(cube.p4.x, 30);
        assert_eq!(cube.p5.x, 40);
        assert_eq!(cube.p6.x, 50);
        assert_eq!(cube.p7.x, 60);
        assert_eq!(cube.p8.x, 70);
    }
}