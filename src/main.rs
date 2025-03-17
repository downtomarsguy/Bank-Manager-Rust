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
            .insert("process_check_t".to_string(), rng.random_range(1..=10));
        self.tasks
            .insert("check_balance_t".to_string(), rng.random_range(1..=10));
        self.tasks
            .insert("open_account_t".to_string(), rng.random_range(1..=10));
        self.tasks
            .insert("deposit_money_t".to_string(), rng.random_range(1..=10));

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

    fn sectionalize(&mut self, customer: Customer) {
        self.customers.push(customer);

        let variable = translate(self.customers[self.customers.len() - 1].need.clone());

        let mut found = false;
        for counter in self.counters.iter() {
            if let Some(duration) = counter.get_task_duration(&variable) {
                println!("The value of {} is {}", variable, duration);
                found = true;
                break;
            }
        }

        if !found {
            println!("No field named {} found in any counter", variable);
        }
    }
}

// translate function
fn translate(input: String) -> String {
    input
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("_")
        + "_t"
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
        master_counter.sectionalize(customer);
    }

    for counter in master_counter.counters.iter() {
        for (task, duration) in &counter.tasks {
            println!("{} {}", task, duration);
        }
    }
}
