// imports
use rand::Rng;

// customer class
struct Customer{
    need: String,
}

impl Customer {
    fn new() -> Customer {
        Customer {
            need: String::new(),
        }
    }

    fn seed(&mut self) {
        let tasks = vec!["Process Check".to_string(), "Check Balance".to_string(), "Open Account".to_string(), "Deposit Money".to_string()];
        let mut rng = rand::rng();
        self.need = tasks[rng.random_range(0..=tasks.len()-1)].to_string()
    }
}

// main function
fn main() {
    let mut customer = Customer::new();

    customer.seed();

    println!("{}", customer.need);
}
