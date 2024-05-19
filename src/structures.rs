#[derive(Debug)]
pub(crate) struct Stock {
    pub name: String,
    pub open_price: f32,
    stop_loss: f32,
    take_profit: f32,
    current_price: f32,
}

impl Stock {
    pub fn new(name: &str, price: f32) -> Stock {
        return Stock {
            name: String::from(name),
            open_price: price,
            stop_loss: 0.0,
            take_profit: 0.0,
            current_price: price,
        };
    }

    fn with_stop_loss(mut self, value: f32) -> Stock {
        self.stop_loss = value;
        return self;
    }

    fn with_take_profit(mut self, value: f32) -> Stock {
        self.take_profit = value;
        return self;
    }
}

pub(crate) fn structures() {
    let stock = Stock::new("BOA", 100.0)
        .with_stop_loss(80.0)
        .with_take_profit(20.0);
    println!("{0:#?}", stock);
}
