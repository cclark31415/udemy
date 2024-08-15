// Problem #1: Identify the error in the code and fix it

trait Drawable {
    fn draw(&self);
}

trait AnimatedDrawable: Drawable {
    fn animate(&self);
}

struct Circle;

impl Drawable for Circle {   // Implemented this
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl AnimatedDrawable for Circle {
    fn animate(&self) {
        println!("Animating a circle");
    }
}

fn main() {
    let circle = Circle;
    circle.draw();
    circle.animate();
}
