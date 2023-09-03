struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

pub fn run() {
    let mut point = Point { x: 2.0, y: 3.5 };
    println!("Before translation: x = {}, y = {}", point.x, point.y);
    point.translate(1.5, -0.5);
    println!("After translation: x = {}, y = {}", point.x, point.y);
}
