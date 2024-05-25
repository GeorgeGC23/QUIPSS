use iota_client::{
    Client as IotaClient, ClientBuilder, Result, Seed, MessageId,
    bee_message::payload::Payload,
    bee_transaction::bundled::{Address, BundledTransactionField},
};
use std::convert::TryInto;

// Estructura para representar el token
#[derive(Debug)]
struct Token {
    name: String,
    symbol: String,
    supply: u64,
}

impl Token {
    // Constructor para crear un nuevo token
    fn new(name: &str, symbol: &str, supply: u64) -> Self {
        Token {
            name: name.to_string(),
            symbol: symbol.to_string(),
            supply,
        }
    }

    // Función para transferir el token a una dirección IOTA
    async fn transfer(&self, recipient_address: &str, client: &IotaClient) -> Result<MessageId> {
        // Crea el payload del token
        let payload = Payload::Indexation(Box::new(self.symbol.clone().into_bytes()), vec![0u8; 0]);

        // Envía la transacción a la dirección del destinatario
        let message_id = client
            .message()
            .with_index("TOKEN_TRANSFER")
            .with_data(payload)
            .with_output(Address::Ed25519(recipient_address.try_into().unwrap()), self.supply)
            .finish()
            .await?;

        Ok(message_id)
    }
}


//TOKEN DE PRUEBA