use iota_client::Client;
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

    async fn generate_address(&mut self) {
        let seed = Seed::from_bytes(&hex::decode(&self.seed).unwrap()).unwrap();
        let client = Client::builder()
            .with_node("https://chrysalis-nodes.iota.org")
            .unwrap()
            .finish()
            .await
            .unwrap();

        let addresses = client.get_addresses(&seed)
            .with_range(0..1)
            .finish()
            .await
            .unwrap();

        self.address = addresses[0].to_string();
        println!("Generated Address: {}", self.address);
    }

    async fn check_balance(&mut self) {
        let client = Client::builder()
            .with_node("https://chrysalis-nodes.iota.org")
            .unwrap()
            .finish()
            .await
            .unwrap();

        let balance = client.get_address().balance(&self.address).await.unwrap().balance;
        self.balance = balance;
        println!("Balance: {} Mi", self.balance);
    }

    async fn send_tokens(&self, recipient_address: &str, amount: u64) {
        let seed = Seed::from_bytes(&hex::decode(&self.seed).unwrap()).unwrap();
        let client = Client::builder()
            .with_node("https://chrysalis-nodes.iota.org")
            .unwrap()
            .finish()
            .await
            .unwrap();

        let message = client
            .message()
            .with_seed(&seed)
            .with_output(recipient_address, amount)
            .unwrap() // Cambiado para usar unwrap aquí
            .finish()
            .await
            .unwrap();

        println!("Message ID: {:?}", message.id());
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

#[tokio::main]
async fn main() {
    // Crear un nuevo usuario
    let username = "test_user".to_string();
    let mut user = User::new(username);
    println!("Generated Seed for {}: {}", user.username, user.seed);

    // Generar una dirección IOTA
    user.generate_address().await;
    user.check_balance().await;

    // Guardar el usuario en un archivo
    user.save_to_file("user.json");

    // Cargar el usuario desde un archivo
    let mut loaded_user = User::load_from_file("user.json");
    println!("Loaded User Address: {}", loaded_user.address);

    // Simular el envío de tokens a otra dirección
    let recipient_address = "RECIPIENT_IOTA_ADDRESS_HERE"; // Reemplaza con la dirección del destinatario
    let amount = 1_000_000; // Cantidad de tokens a enviar (en IOTA)
    loaded_user.send_tokens(recipient_address, amount).await;
    loaded_user.check_balance().await;
}
