use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use std::error::Error;

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
    let mut publisher =
        CrosstownBus::new_queue_publisher("amqp://guest:guest@0.0.0.0:5672".to_owned())?;

    _ = publisher.publish_event(
        "item_created".to_owned(),
        ItemCreatedEventMessage {
            item_id: "919".to_owned(),
            item_title: "Horizon Zero Dawn".to_owned(),
        },
    );

    _ = publisher.publish_event(
        "item_created".to_owned(),
        ItemCreatedEventMessage {
            item_id: "191".to_owned(),
            item_title: "Horizon Forbidden West".to_owned(),
        },
    );

    Ok(())
}
