use queue::Queue;
use stack::Stack;
use store::Order;

pub mod queue;
pub mod stack;
pub mod store;

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_micros(ms));
}

fn populate_stack<'a>(data: &mut Stack<Order<'a>>, input: &Vec<(&'a str, &'a str)>) {
    for i in input {
        sleep(550);
        data.push(Order::new(i.0, i.1));
    }
}

fn populate_queue<'a>(data: &mut Queue<Order<'a>>, input: &Vec<(&'a str, &'a str)>) {
    for i in input {
        sleep(550);
        data.push(Order::new(i.0, i.1));
    }
}

fn run_stack<'a>(data: &mut Stack<Order>) -> String {
    let mut res = String::from("\nShake Stack:\n");
    while !data.is_empty() {
        sleep(550);
        res += format!("{}\n", data.pop().unwrap().serve()).as_str();
    }

    res

    // println!("{}", res);
}

fn run_queue<'a>(data: &mut Queue<Order>) -> String {
    let mut res = String::from("\nShake Queue:\n");
    while !data.is_empty() {
        sleep(550);
        res += format!("{}\n", data.pop().unwrap().serve()).as_str();
    }

    res
}

fn gen_data<'a>() -> (Stack<Order<'a>>, Queue<Order<'a>>) {
    let data: Vec<(&'static str, &'static str)> = vec![
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
    let data: Vec<Order<'_>> = data.iter().map(|i| Order::from(*i)).collect();

    let stack = Stack::from(data.clone());
    let queue = Queue::from(data.clone());

    (stack, queue)
}

fn main() {
    use std::sync::mpsc;
    use std::thread::spawn;

    let (mut stack, mut queue) = gen_data();
    let (s1, rx) = mpsc::channel::<String>();
    let s2 = s1.clone();

    spawn(move || s1.clone().send(run_stack(&mut stack)));
    spawn(move || s2.clone().send(run_queue(&mut queue)));

    for _ in 0..2 {
        println!("{}", rx.recv().unwrap());
    }
}
