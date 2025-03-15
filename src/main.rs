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
    process_check_t: u8,
    check_balance_t: u8,
    open_account_t: u8,
    deposit_money_t: u8,
    line: Vec<Counter>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            process_check_t: 0,
            check_balance_t: 0,
            open_account_t: 0,
            deposit_money_t: 0,
            line: Vec::new(),
        }
    }

    fn generate_durations(&mut self) {
        let mut rng = rand::rng();
        self.process_check_t = rng.random_range(1..=10);
        self.check_balance_t = rng.random_range(1..=10);
        self.open_account_t = rng.random_range(1..=10);
        self.deposit_money_t = rng.random_range(1..=10);
    }
}

// main function
fn main() {
    let mut customers: Vec<Customer> = Vec::new();

    for n in 1..=3 {
        let mut customer = Customer::new();
        customer.seed();

        customers.push(customer);
    }

    let mut counters: Vec<Counter> = Vec::new();

    for n in 1..=3 {
        let mut counter = Counter::new();
        counter.generate_durations();

        counters.push(counter);
    }

    for counter in counters.iter() {
        println!("{}", counter.process_check_t);
        println!("{}", counter.check_balance_t);
        println!("{}", counter.open_account_t);
        println!("{}", counter.deposit_money_t);
    }
}
