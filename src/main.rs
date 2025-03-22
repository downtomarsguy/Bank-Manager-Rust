// imports
use rand::Rng;
use rand::prelude::*;
use std::collections::HashMap;
use std::sync::{
    Arc,
    atomic::{AtomicU8, AtomicU64, Ordering},
};
use std::thread;
use std::time::Duration;

// customer class
struct Customer {
    need: String,
    location: String,
    customer_id: u8,
}

impl Customer {
    fn new() -> Customer {
        Customer {
            need: String::new(),
            location: String::new(),
            customer_id: 0,
        }
    }

    fn seed(&mut self, customer_id: u8) {
        let tasks = [
            "Process Check",
            "Check Balance",
            "Open Account",
            "Deposit Money",
        ];

        self.customer_id = customer_id;

        let mut rng = rand::rng();
        self.need = tasks.choose(&mut rng).expect("").to_string();
    }
}

// counter classes
struct Counter {
    counter_id: u8,
    line: Vec<Customer>,
    line_len: Arc<AtomicU8>,
    tasks: HashMap<String, u8>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            counter_id: 0,
            line: Vec::new(),
            line_len: Arc::new(AtomicU8::new(0)),
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
            .insert("Deposit Money".to_string(), rng.random_range(1..=10));

        self.counter_id = counter_id;

        self.generate_thread();
    }

    fn add_customer(&mut self, customer: Customer) {
        let variable = customer.need.clone();

        if let Some(duration) = self.get_task_duration(&variable) {
            self.line_len.fetch_add(duration, Ordering::SeqCst);
        }

        self.line.push(customer);
    }

    fn get_task_duration(&self, task: &str) -> Option<u8> {
        self.tasks.get(task).copied()
    }

    fn generate_thread(&mut self) {
        let line_len = Arc::clone(&self.line_len);

        thread::spawn(move || {
            while line_len.load(Ordering::SeqCst) == 0 {
                thread::sleep(Duration::from_secs(1));
            }
            println!("line_len updated");
        });
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

    fn add_counter(&mut self, counter: Counter) {
        self.counters.push(counter);
    }

    fn sectionalize(&mut self, customer: Customer) {
        let mut line_lengths: Vec<u8> = Vec::new();

        for counter in self.counters.iter() {
            line_lengths.push(counter.line_len.load(Ordering::SeqCst));
        }

        let min_index = line_lengths
            .iter()
            .enumerate()
            .min_by_key(|&(_, &val)| val)
            .map(|(index, _)| index)
            .unwrap_or(0);

        self.counters[min_index].add_customer(customer);
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

    for n in 1..=10 {
        let mut customer = Customer::new();
        customer.seed(n);
        master_counter.sectionalize(customer);
        thread::sleep(Duration::from_secs(rand::rng().random_range(1..=5)));
    }

    for counter in master_counter.counters.iter() {
        for (task, duration) in &counter.tasks {
            println!("{} {} {}", counter.counter_id, task, duration);
        }
    }

    for counter in master_counter.counters.iter() {
        for customer in &counter.line {
            println!("{} {}", counter.counter_id, customer.customer_id);
        }
    }
}
