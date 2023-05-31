use crosstown_bus::{CrosstownBus};
use std::error::Error;

#[path = "../../models/lib.rs"]
mod models;
use models::ItemCreatedEventMessage;

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
