use std::fmt;
use std::time::Instant;

pub struct Order<'a> {
    name: &'a str,
    item: &'a str,
    time: Instant,
    status: OrderStatus,
}

enum OrderStatus {
    Hot,
    Warm,
    Cold,
}

impl<'a> Order<'a> {
    pub fn new(name: &'a str, item: &'a str) -> Self {
        let time = Instant::now();
        let status = OrderStatus::Hot;
        Order {
            name,
            item,
            time,
            status,
        }
    }

    pub fn serve(&mut self) -> &Self {
        let duration = self.time.elapsed();
        if duration.as_secs() > 10 {
            self.status = OrderStatus::Cold;
        } else if duration.as_secs() > 5 {
            self.status = OrderStatus::Warm;
        }

        self
    }
}

impl fmt::Display for Order<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} orderd a {}\nIt has been {}s and the food is {}.",
            self.name,
            self.item,
            self.time.elapsed().as_secs(),
            match self.status {
                OrderStatus::Hot => "hot",
                OrderStatus::Warm => "warm",
                OrderStatus::Cold => "cold",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn it_works() {
        let order = Order::new("justin", "vanilla milkshake");
        println!("{}", order);
    }
}
