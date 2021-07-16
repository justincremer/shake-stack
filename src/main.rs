use queue::Queue;
use stack::Stack;
use store::Order;

pub mod queue;
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
    std::thread::sleep(std::time::Duration::from_micros(ms));
}

fn populate_stack<'a>(data: &mut Stack<Order<'a>>, input: [(&'a str, &'a str); 10]) {
    for i in input {
        sleep(600);
        data.push(Order::new(i.0, i.1));
    }
}

fn populate_queue<'a>(data: &mut Queue<Order<'a>>, input: [(&'a str, &'a str); 10]) {
    for i in input {
        sleep(600);
        data.push(Order::new(i.0, i.1));
    }
}

fn run_stack() {
    let mut shake_stack: Stack<Order> = Stack::new();
    populate_stack(&mut shake_stack, DATA);

    let mut res = String::from("\nShake Stack:\n");
    while !shake_stack.is_empty() {
        sleep(600);
        res += format!("{}\n", shake_stack.pop().unwrap().serve()).as_str();
    }

    println!("{}", res);
}

fn run_queue() {
    let mut shake_queue: Queue<Order> = Queue::new();
    populate_queue(&mut shake_queue, DATA);

    let mut res = String::from("\nShake Queue:\n");
    while !shake_queue.is_empty() {
        sleep(600);
        res += format!("{}\n", shake_queue.pop().unwrap().serve()).as_str();
    }

    println!("{}", res);
}

fn main() {
    // use std::thread::spawn;

    // let mut handles = Vec::new();
    // handles.push(spawn(|| run_stack()));
    // handles.push(spawn(|| run_queue()));

    // for h in handles.iter() {
    //     h.join().unwrap();
    // }

    run_stack();
    run_queue();
}
