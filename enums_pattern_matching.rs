/*
    ENUMS 

    Enums are used to create custom types representing a fixed set of possible values.

    The primary advantage of using enums is type safety, ensuring that the value is known at compile time.
    Enums can also hold associated data, making them versatile for various applications.

    Benefits of Enums:
    - Type safety
    - Improved code clarity
    - Pattern matching
    - Associated data
*/

// Let's create a custom type for vehicles

enum Vehicle {
    Car,
    Bus,
    Truck,
    Motorcycle,
}

// Example implementation
fn main() {
    let car = Vehicle::Car;
    display_vehicle(car);

    let bus = Vehicle::Bus;
    display_vehicle(bus);
}

fn display_vehicle(vehicle: Vehicle) {
    match vehicle {
        Vehicle::Car => println!("Vehicle: Car"),
        Vehicle::Bus => println!("Vehicle: Bus"),
        Vehicle::Truck => println!("Vehicle: Truck"),
        Vehicle::Motorcycle => println!("Vehicle: Motorcycle"),
    }
}

/*
    Rust enums can also store associated data, allowing enums to act as algebraic data types.
    This makes them more flexible and powerful for pattern matching.

    Associated Data Example:
    - Rectangle(height, width)
    - Circle(radius)
*/

enum Shape {
    Rectangle(f64, f64), // Rectangle(height, width)
    Circle(f64),        // Circle(radius)
    Triangle(f64, f64), // Triangle(base, height)
}

fn main_shapes() {
    let circle = Shape::Circle(1.0);
    let rectangle = Shape::Rectangle(1.0, 2.0);
    let triangle = Shape::Triangle(3.0, 4.0);

    println!("Circle Area: {:.2}", calculate_area(&circle));
    println!("Rectangle Area: {:.2}", calculate_area(&rectangle));
    println!("Triangle Area: {:.2}", calculate_area(&triangle));
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(a, b) => a * b,
        Shape::Triangle(b, h) => 0.5 * b * h,
    }
}

/*
    Matching with Enums:
    - The match expression helps safely destructure the enum variants.
    - Each arm of the match covers a different variant.

    Advanced Features:
    - Enum Methods
    - Default Implementations
    - Deriving Traits
*/

impl Shape {
    fn describe(&self) {
        match self {
            Shape::Circle(r) => println!("Circle with radius {:.2}", r),
            Shape::Rectangle(a, b) => println!("Rectangle with dimensions {:.2}x{:.2}", a, b),
            Shape::Triangle(b, h) => println!("Triangle with base {:.2} and height {:.2}", b, h),
        }
    }
}

fn main_descriptions() {
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(5.0, 6.0);

    circle.describe();
    rectangle.describe();
    triangle.describe();
}

/*
    ✅ Enums combined with pattern matching allow you to represent different types of data safely and efficiently.
    ✅ The use of associated data with enums makes Rust highly expressive and type-safe.
    ✅ Enum methods provide a clean way to add functionality to enums.
*/
