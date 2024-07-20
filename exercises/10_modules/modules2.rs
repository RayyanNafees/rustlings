// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

#[allow(dead_code)]
mod delicious_snacks {
    
  

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}
use self::delicious_snacks::fruits::PEAR as fruit;
use self::delicious_snacks::veggies::CUCUMBER as veggie;

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie,
    );
}
