pub mod order;
pub mod orderlist;

fn main() {
    let mut waitinglist: orderlist::OrderList = orderlist::OrderList::new();
    let user_order: order::Order = order::Order {
        pizzatype: order::PizzaType::Regina,
        size: order::PizzaSize::XL,
        nb: 5
    };

    println!("waitinglist: {:?}", waitinglist);
    println!("user_order: {:?}", user_order);

    waitinglist.push_order(user_order);
    println!();
    println!("waitinglist: {:?}", waitinglist);

    let finished_order: order::Order = waitinglist.pop_order().unwrap_or_else(|| panic!());
    println!();
    println!("waitinglist: {:?}", waitinglist);
    println!("finished_order: {:?}", finished_order);
}
