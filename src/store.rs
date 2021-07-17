use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct Order<'a> {
    name: &'a str,
    item: &'a str,
    time: Instant,
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus {
    Hot,
    Warm,
    Cold,
    Inedibile,
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
        if duration.as_millis() > 20 {
            self.status = OrderStatus::Inedibile;
        } else if duration.as_millis() > 10 {
            self.status = OrderStatus::Cold;
        } else if duration.as_millis() > 5 {
            self.status = OrderStatus::Warm;
        }

        self
    }
}

impl<'a> From<(&'a str, &'a str)> for Order<'a> {
    fn from(i: (&'a str, &'a str)) -> Self {
        Order::new(i.0, i.1)
    }
}

impl fmt::Display for Order<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} orderd a {}\nIt has been {}m and the food is {}.",
            self.name,
            self.item,
            self.time.elapsed().as_millis(),
            match self.status {
                OrderStatus::Hot => "hot",
                OrderStatus::Warm => "warm",
                OrderStatus::Cold => "cold",
                OrderStatus::Inedibile => "inedible",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Order, OrderStatus};
    use std::time::Duration;

    #[test]
    fn status_depreciated() {
        let mut order = Order::new("justin", "vanilla milkshake");

        assert_eq!(order.status, OrderStatus::Hot);
        order.time = order.time - Duration::from_millis(6);
        let _ = order.serve();
        assert_eq!(order.status, OrderStatus::Warm);
        order.time = order.time - Duration::from_millis(6);
        let _ = order.serve();
        assert_eq!(order.status, OrderStatus::Cold);
        order.time = order.time - Duration::from_millis(10);
        let _ = order.serve();
        assert_eq!(order.status, OrderStatus::Inedibile);
    }
}
