#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.height * self.width
    }

    fn square(size: usize) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
    println!("Rectangle struct is {:#?}", rect2);
    dbg!(&rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    let square1 = Rectangle::square(45);
    println!(
        "The area of the rectangle is {} square pixels.",
        square1.area()
    );
}

fn area_tuple(dimensions: (usize, usize)) -> usize {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> usize {
    rect.width * rect.height
}
