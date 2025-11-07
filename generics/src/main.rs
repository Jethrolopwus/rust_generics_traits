// generics in rust allow us to defin functions, structs, enums and traits with placeholders for data types enable flexible and reusable code.



struct Point<T, U> {
    x: T,
    y: U,
}   
// adding the implementation block for Point struct to demonstrate usage
impl <T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    } 
}
// specialization allow us to implement methods for specific type combinations


impl Point<i32, i32> {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    
    }
    fn printing(&self) {
        println!("The value of the Point coordinates of int are: ({}, {})", self.x, self.y);
    }
}

//impl for floating point 
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64{
        self.x + self.y
    }
    fn printing(&self) {
        println!("the value of the point cordinate for floating point are: ({}, {})", self.x, self.y);
    }
}

//free function

fn add_points<T, U>(int_point: &Point<T, U>, float_point: &Point<T, U>) -> Point<T, U>{
    unimplemented!();
}

// for int_point
fn add_points_i32(int_point: &Point<i32, i32>, float_point: &Point<i32, i32>) ->Point<i32, i32> {
    unimplemented!();
}

fn add_points_f64(int_point: &Point<f64, f64>, float_point: &Point<f64, f64>) ->Point<f64, f64>{
    unimplemented!();
}
fn main() {
    let origin = Point::new( 3, 5); 
    let float_point = Point::new(1.5, 2.5);
    let int_point = Point::new(5, 5.0); 

    println!("Integer Point: ({}, {})", int_point.x, int_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
    println!("Mixed Point: ({}, {})", int_point.x, int_point.y);

    // using the specialized method for Point<i32, i32>
    origin.printing();
    // calling the fn for the distance in floating point types
    println!("Distance from origin: {}", origin.distance_from_origin());

    // using the specialized method for Point<f64, f64>

    float_point.printing();
    println!("The Distance from the origin: {}", float_point.distance_from_origin());


    add_points_i32(&origin, &origin);
    add_points_f64(&float_point, &float_point);

}
