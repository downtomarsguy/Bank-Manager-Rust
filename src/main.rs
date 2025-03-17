// imports
use rand::Rng;
use rand::prelude::*;
use std::collections::HashMap;

// customer class
struct Customer {
    need: String,
    location: String,
}

impl Customer {
    fn new() -> Customer {
        Customer {
            need: String::new(),
            location: String::new(),
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
    counter_id: u8,
    line: Vec<Customer>,
    line_len: u8,
    tasks: HashMap<String, u8>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            process_check_t: 0,
            check_balance_t: 0,
            open_account_t: 0,
            deposit_money_t: 0,
            counter_id: 0,
            line: Vec::new(),
            line_len: 0,
            tasks: HashMap::new(),
        }
    }

    fn generate_durations(&mut self, counter_id: u8) {
        let mut rng = rand::rng();
        self.tasks
            .insert("Process Check".to_string(), rng.random_range(1..=10));
        self.tasks
            .insert("Check Balance".to_string(), rng.random_range(1..=10));
        self.tasks
            .insert("Open Account".to_string(), rng.random_range(1..=10));
        self.tasks
            .insert("Deposit money".to_string(), rng.random_range(1..=10));

        self.counter_id = counter_id;
    }

    fn add_customer(&mut self, customer: Customer) {
        self.line.push(customer);
    }

    fn get_task_duration(&self, task: &str) -> Option<u8> {
        self.tasks.get(task).copied()
    }
}

// master counter
struct MasterCounter {
    counters: Vec<Counter>,
    customers: Vec<Customer>,
}

impl MasterCounter {
    fn new() -> MasterCounter {
        MasterCounter {
            counters: Vec::new(),
            customers: Vec::new(),
        }
    }

    fn add_counter(&mut self, counter: Counter) {
        self.counters.push(counter);
    }
}

// main function
fn main() {
    let mut master_counter = MasterCounter::new();

    for n in 1..=3 {
        let mut counter = Counter::new();
        counter.generate_durations(n);
        master_counter.add_counter(counter);
    }

    for _n in 1..=10 {
        let mut customer = Customer::new();
        customer.seed();
    }

    for counter in master_counter.counters.iter() {
        println!("{}", counter.process_check_t);
        println!("{}", counter.check_balance_t);
        println!("{}", counter.open_account_t);
        println!("{}", counter.deposit_money_t);
    }
}
