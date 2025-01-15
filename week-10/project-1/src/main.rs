struct Computer { 
    brand: String,
    price: u32,
    quantity: u32,
}

impl Computer {
    fn cost(&self) -> u32 {
        self.price * self.quantity
    }
}

fn main() {
    let hp = Computer {
        brand: String::from("HP"),
        price: 650_000,
        quantity: 3,
    };

    let ibm = Computer {
        brand: String::from("IBM"),
        price: 755_000,
        quantity: 3,
    };

    let toshiba = Computer {
        brand: String::from("Toshiba"),
        price: 550_000,
        quantity: 3,
    };

    let dell = Computer {
        brand: String::from("DELL"),
        price: 850_000,
        quantity: 3,
    };

    let total = hp.cost() + ibm.cost() + toshiba.cost() + dell.cost();
    println!(
        "Total cost is {}. Individual costs are: HP = {}, IBM = {}, Toshiba = {}, DELL = {}.",
        total,
        hp.cost(),
        ibm.cost(),
        toshiba.cost(),
        dell.cost()
    );
}
