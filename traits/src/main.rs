

struct Drawing_info {
     line_width: u8,
    color: String,
}
struct Square {
    side: f32,
    line_width: u8,
    color: String,
}



struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}




// traits definition
trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter is Not implemented returning a dummy value");
        0.0
    }

}

// traits implementation

impl Shape for Rectangle {
    fn area(&self) ->f32 {
        let area_of_rectangle = self.length * self.width;
        println!("The area of the rectangle is: {}", area_of_rectangle);
        area_of_rectangle
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rectangle = 2.0 * (self.length + self.width);
        println!("The Rectangle Perimeter is: {}", perimeter_of_rectangle);
        perimeter_of_rectangle


}

}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("The area of the square is: {}", area_of_square);
        area_of_square

    }
}


fn main() {
    let rect = Rectangle {
        length: 10.0,
        width: 5.0,
        line_width: 2,
        color: String::from("Red"),
    };

    let sqr = Square {
        side: 4.0,
        line_width: 1,
        color: String::from("Blue"),
    };

    rect.area();
    sqr.area();
    rect.perimeter();
    sqr.perimeter();
}
