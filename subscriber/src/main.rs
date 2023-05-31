use std::error::Error;
use borsh::{BorshSerialize, BorshDeserialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler, QueueProperties};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct ItemCreatedEventMessage {
    pub item_id: String,
    pub item_title: String,
}

pub struct ItemCreatedHandler;

impl MessageHandler<ItemCreatedEventMessage> for ItemCreatedHandler {
    fn handle(&self, message: Box<ItemCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler: {:?}", message);
        Ok(())
    }

    fn get_handler_action(&self) -> String { "item_created".to_owned() }
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener =
        CrosstownBus::new_queue_listener("amqp://guest:guest@0.0.0.0:5672".to_owned())?;
    _ = listener.listen(
        "item_created".to_owned(),
        ItemCreatedHandler,
        QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    );

    loop {}
}
