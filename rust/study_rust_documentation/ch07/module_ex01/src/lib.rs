mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Apple"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house :: Breakfast :: summer("Pizza");

    meal.toast = String :: from("Chicken");
    println!("Give me {} Toast!", meal.toast);

    // meal.seasonal_fruit = String :: from("Banana")

    let order1 = back_of_house :: Appetizer :: Soup;
    let order2 = back_of_house :: Appetizer :: Salad;
}