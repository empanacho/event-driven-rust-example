use crosstown_bus::{CrosstownBus, QueueProperties};
use std::error::Error;

#[path = "../../models/lib.rs"]
mod models;
use models::ItemCreatedHandler;

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
