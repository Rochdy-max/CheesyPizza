#[derive(Debug)]
pub enum PizzaType {
    Regina,
    Margarita,
    Americana,
    Fantasia
}

#[derive(Debug)]
pub enum PizzaSize {
    S,
    M,
    L,
    XL,
    XXL
}

#[derive(Debug)]
pub struct Order {
    pub pizzatype: PizzaType,
    pub size: PizzaSize,
    pub nb: i32
}
