mod kitchen_area {
    pub(super) fn make_dish(dish: String) {
        println!("Preparing {}...", dish);
    }
}

mod dining_area {
    pub(super) fn order_apple() {
        println!("Ordering apple.");
        super::kitchen_area::make_dish("apple".to_string());
    }
}

fn main() {
    dining_area::order_apple();
}
