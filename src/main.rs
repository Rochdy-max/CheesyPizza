pub mod order;

fn main() {
    let user_order: order::Order = order::Order {
        pizzatype: order::PizzaType::Regina,
        size: order::PizzaSize::XL,
        nb: 5
    };

    println!("user_order: {:?}", user_order);
}
