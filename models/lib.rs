use crosstown_bus::{HandleError, MessageHandler};
use borsh::{BorshSerialize, BorshDeserialize};

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
