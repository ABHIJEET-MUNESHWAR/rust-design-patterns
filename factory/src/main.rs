// 1. Define the interface (Trait)
trait Toy {
    fn log(&self);
}

// 2. Define concrete products
struct Robot;
struct Car;

// 3. Implement the interface for each product
impl Toy for Robot {
    fn log(&self) {
        println!("This is a Robot toy.");
    }
}

impl Toy for Car {
    fn log(&self) {
        println!("This is a Car toy.");
    }
}

// 4. Define the types of toys the factory can make
enum ToyType {
    Robot,
    Car,
}

// 5. The Factory
struct ToyFactory;

impl ToyFactory {
    // Returns a Trait Object (dyn Toy) wrapped in a Box
    // because the size isn't known at compile time.
    fn build_toy(toy_type: ToyType) -> Box<dyn Toy> {
        match toy_type {
            ToyType::Robot => Box::new(Robot),
            ToyType::Car => Box::new(Car),
        }
    }
}

fn main() {
    // 6. Use the factory to create objects without knowing their concrete types
    let robot = ToyFactory::build_toy(ToyType::Robot);
    let car = ToyFactory::build_toy(ToyType::Car);
    robot.log();
    car.log();
}
