// imports
use rand::Rng;

// customer class
struct Customer {
    need: String,
}

impl Customer {
    fn new() -> Customer {
        Customer {
            need: String::new(),
        }
    }

    fn seed(&mut self) {
        let tasks = vec![
            "Process Check".to_string(),
            "Check Balance".to_string(),
            "Open Account".to_string(),
            "Deposit Money".to_string(),
        ];
        let mut rng = rand::rng();
        self.need = tasks[rng.random_range(0..=tasks.len() - 1)].to_string();
    }
}

// counter classes
struct Counter {
    process_check: u8,
    check_balance: u8,
    open_account: u8,
    deposit_money: u8,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            process_check: 0,
            check_balance: 0,
            open_account: 0,
            deposit_money: 0,
        }
    }

    fn generate_durations(&mut self) {
        let mut rng = rand::rng();
        self.process_check = rng.random_range(1..=10);
        self.check_balance = rng.random_range(1..=10);
        self.open_account = rng.random_range(1..=10);
        self.deposit_money = rng.random_range(1..=10);
    }
}

// main function
fn main() {
    let mut customer = Customer::new();

    customer.seed();

    let mut counter = Counter::new();

    counter.generate_durations();

    println!("{}", customer.need);
    println!("{}", counter.process_check);
    println!("{}", counter.check_balance);
    println!("{}", counter.open_account);
    println!("{}", counter.deposit_money);
}
