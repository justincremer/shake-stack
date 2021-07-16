use stack::Stack;
use store::Order;

pub mod stack;
pub mod store;

const DATA: [(&'static str, &'static str); 10] = [
    ("justin", "strawberry milkshake"),
    ("loaf", "veggie burger"),
    ("liana", "pizza??"),
    ("sunny", "weapon"),
    ("angela", "chocolate milkshake"),
    ("paul", "vanilla milkshake"),
    ("fluzz", "Arc<Mutex<Vec<Bugs>>>"),
    ("kevin", "nap"),
    ("ari", "10-12 hours of quality League gameplay"),
    ("Siyam", "wifi"),
];

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn populate<'a>(stack: &mut Stack<Order<'a>>, data: [(&'a str, &'a str); 10]) {
    for i in data {
        stack.push(Order::new(i.0, i.1));
        sleep(1100);
    }
}

fn main() {
    let mut shake_stack: Stack<Order> = Stack::new();
    populate(&mut shake_stack, DATA);

    while !shake_stack.is_empty() {
        println!("{}", shake_stack.pop().unwrap().serve());
    }
}
