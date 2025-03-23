// imports
use rand::Rng;
use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU8, Ordering},
};
use std::thread;
use std::time::Duration;

// customer class
struct Customer {
    need: String,
    customer_id: u8,
}

impl Customer {
    fn new() -> Customer {
        Customer {
            need: String::new(),
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
    line: Arc<Mutex<VecDeque<Customer>>>,
    line_len: Arc<AtomicU8>,
    tasks: HashMap<String, u8>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            counter_id: 0,
            line: Arc::new(Mutex::new(VecDeque::new())), // Changed to VecDeque
            line_len: Arc::new(AtomicU8::new(0)),
            tasks: HashMap::new(),
            thread_handle: None,
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

        let mut line = self.line.lock().unwrap();
        line.push_back(customer);
    }

    fn get_task_duration(&self, task: &str) -> Option<u8> {
        self.tasks.get(task).copied()
    }

    fn generate_thread(&mut self) {
        let line_len = Arc::clone(&self.line_len);
        let tasks = self.tasks.clone();
        let line = Arc::clone(&self.line);
        let counter_id = self.counter_id;

        let handle = thread::spawn(move || {
            while line_len.load(Ordering::SeqCst) == 0 {
                thread::sleep(Duration::from_secs(1));
            }

            while line_len.load(Ordering::SeqCst) != 0 {
                let mut line = line.lock().unwrap();

                if let Some(customer) = line.front() {
                    let task = &customer.need;

                    if let Some(duration) = tasks.get(task) {
                        println!(
                            "Found customer {} with need: {} at counter: {}",
                            customer.customer_id, customer.need, counter_id
                        );
                        thread::sleep(Duration::from_secs(*duration as u64));
                        line.pop_front();
                        line_len.fetch_sub(*duration, Ordering::SeqCst);
                    }
                }

                thread::sleep(Duration::from_secs(5));
            }
            println!("Line processed for counter {}", counter_id);
        });

        self.thread_handle = Some(handle);
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
    }

    let mut handles = Vec::new();

    for counter in master_counter.counters.iter_mut() {
        if let Some(handle) = counter.thread_handle.take() {
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    println!("All customers tasks have been satisifed.");
}
