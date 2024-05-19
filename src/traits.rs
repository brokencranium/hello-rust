use crate::structures::Stock;

trait CanTransfer {
    fn transfer_stock(&self) -> ();

    fn print(&self) {
        println!("Transfer is being started");
    }
}

impl CanTransfer for Stock {
    fn transfer_stock(&self) {
        println!(
            "Stock {} is being transferred for {}",
            self.name, self.open_price
        )
    }
}

fn print_messages(stock: impl CanTransfer) {
    stock.print();
    stock.transfer_stock();
}

pub(crate) fn traits() {
    let stock = Stock::new("BK", 2000.0);
    print_messages(stock);
}
