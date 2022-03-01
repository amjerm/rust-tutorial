mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // With use
    hosting::add_to_waitlist();

    // Order appetizers
    // let order1 = back_of_house::Appetizer::Soup;
    // let order2 = back_of_house::Appetizer::Salad;

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// fn serve_order() {}
