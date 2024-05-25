use iota_client::ClientBuilder;
use iota_client::Seed;
use serde::{Deserialize, Serialize};
use std::fs;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    seed: String,
    address: String,
    balance: u64,
}

impl User {
    fn new(username: String) -> Self {
        let seed = Self::generate_seed();
        User {
            username,
            seed,
            address: String::new(),
            balance: 0,
        }
    }

    fn generate_seed() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(81)
            .map(char::from)
            .collect()
    }

    fn generate_address(&mut self) {
        let seed_bytes = match hex::decode(&self.seed) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error decoding seed: {}", e);
                return;
            }
        };

        let seed = match Seed::from_bytes(&seed_bytes) {
            Ok(seed) => seed,
            Err(e) => {
                eprintln!("Error creating Seed from bytes: {}", e);
                return;
            }
        };

        let client_builder = match ClientBuilder::new()
            .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")
        {
            Ok(client_builder) => client_builder,
            Err(e) => {
                eprintln!("Error building client builder: {}", e);
                return;
            }
        };

        let client = match client_builder.build() {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Error building client: {}", e);
                return;
            }
        };

        let addresses = match client.get_addresses(&seed).with_range(0..1) {
            Ok(addresses) => addresses,
            Err(e) => {
                eprintln!("Error generating addresses: {}", e);
                return;
            }
        };

        self.address = addresses[0].to_string();
        println!("Generated Address: {}", self.address);
    }

    fn check_balance(&mut self) {
        let client_builder = match ClientBuilder::new()
            .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")
        {
            Ok(client_builder) => client_builder,
            Err(e) => {
                eprintln!("Error building client builder: {}", e);
                return;
            }
        };

        let client = match client_builder.build() {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Error building client: {}", e);
                return;
            }
        };

        let balance = match client.get_address().balance(&self.address) {
            Ok(balance) => balance.balance,
            Err(e) => {
                eprintln!("Error checking balance: {}", e);
                return;
            }
        };

        self.balance = balance;
        println!("Balance: {} Mi", self.balance);
    }

    fn save_to_file(&self, filename: &str) {
        let data = serde_json::to_string(&self).unwrap();
        fs::write(filename, data).expect("Unable to write file");
    }

    fn load_from_file(filename: &str) -> Self {
        let data = fs::read_to_string(filename).expect("Unable to read file");
        serde_json::from_str(&data).unwrap()
    }
}

fn main() {
    let filename = "user.json";
    let mut user: User;

    if !std::path::Path::new(filename).exists() {
        // Crear un nuevo usuario
        let username = "test_user".to_string();
        user = User::new(username);
        println!("Generated Seed for {}: {}", user.username, user.seed);

        // Generar una dirección IOTA
        user.generate_address();
        user.check_balance();

        // Guardar el usuario en un archivo
        user.save_to_file(filename);
    } else {
        // Cargar el usuario desde un archivo
        user = User::load_from_file(filename);
        println!("Loaded User Address: {}", user.address);

        // Verificar el balance
        user.check_balance();
    }
}
