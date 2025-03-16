// imports
use rand::Rng;
use rand::prelude::*;

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
        let tasks = [
            "Process Check",
            "Check Balance",
            "Open Account",
            "Deposit Money",
        ];

        let mut rng = rand::rng();
        self.need = tasks.choose(&mut rng).expect("").to_string();
    }
}

// counter classes
struct Counter {
    process_check_t: u8,
    check_balance_t: u8,
    open_account_t: u8,
    deposit_money_t: u8,
    line: Vec<Customer>,
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

    fn add_customer(&mut self, customer: Customer) {
        self.line.push(customer);
    }
}

// master counter
struct MasterCounter {
    counters: Vec<Counter>,
}

impl MasterCounter {
    fn new() -> MasterCounter {
        MasterCounter {
            counters: Vec::new(),
        }
    }

    fn addCounters(&mut counter: Counter) {
        self.counters.push(counter);
    }
}

// main function
fn main() {
    let mut masterCounter = MasterCounter::new();
    let mut customers: Vec<Customer> = Vec::new();

    for _n in 1..=3 {
        let mut customer = Customer::new();
        customer.seed();

        customers.push(customer);
    }

    let mut counters: Vec<Counter> = Vec::new();

    for _n in 1..=3 {
        let mut counter = Counter::new();
        counter.generate_durations();
        masterCounter.addCounters(counter);

        counters.push(counter);
    }

    for counter in counters.iter() {
        println!("{}", counter.process_check_t);
        println!("{}", counter.check_balance_t);
        println!("{}", counter.open_account_t);
        println!("{}", counter.deposit_money_t);
    }
}
