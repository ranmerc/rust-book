#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() > rectangle.area()
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {rectangle:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    let smaller_rectangle = Rectangle {
        height: 10,
        width: 20,
    };

    let bigger_rectangle = Rectangle::square(100);

    fit_check(&smaller_rectangle, &rectangle);
    fit_check(&bigger_rectangle, &rectangle);
}

fn fit_check(rectangle: &Rectangle, bounding_rectangle: &Rectangle) {
    let fits = bounding_rectangle.can_hold(&rectangle);

    if fits {
        print!("{bounding_rectangle:#?} can fit {rectangle:#?}")
    } else {
        print!("{bounding_rectangle:#?} cannot fit {rectangle:#?}")
    }
}
