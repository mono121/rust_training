trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;

impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

pub fn run(){
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
}
fn get_price<T: Fruits>(fruits: T) {
    println!("price is {}", fruits.price());
}