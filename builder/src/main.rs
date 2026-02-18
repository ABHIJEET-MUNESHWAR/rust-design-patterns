// 1. The Components available for our object
#[derive(Debug)]
enum BurgerComponent {
    BottomBun,
    Patty,
    Tomato,
    Lettuce,
    Cheese,
    TopBun,
}
// 2. The Final Product we want to build
#[derive(Debug)]
struct Burger {
    layers: Vec<BurgerComponent>,
}

// 3. The Builder itself
struct BurgerBuilder {
    components: Vec<BurgerComponent>,
}

impl BurgerBuilder {
    // Initializes the builder with a default starting state (Bottom Bun)
    fn new() -> BurgerBuilder {
        BurgerBuilder {
            components: vec![BurgerComponent::BottomBun],
        }
    }

    // A fluent method to add ingredients
    // Returning 'self' allows for method chaining
    fn add_component(mut self, component: BurgerComponent) -> BurgerBuilder {
        self.components.push(component);
        self
    }

    // The final step: Adds the Top Bun and consumes the builder
    // to return the finished Burger
    fn build(mut self) -> Burger {
        self.components.push(BurgerComponent::TopBun);
        Burger {
            layers: self.components,
        }
    }
}

fn main() {
    // Using the builder with method chaining (Fluent Interface)
    let my_burger = BurgerBuilder::new()
        .add_component(BurgerComponent::Patty)
        .add_component(BurgerComponent::Tomato)
        .add_component(BurgerComponent::Cheese)
        .add_component(BurgerComponent::Lettuce)
        .build();
    println!("Enjoy your burger: {:?}", my_burger);
}
