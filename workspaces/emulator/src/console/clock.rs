use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time::sleep;
use crate::console::Component;
use crate::event::Event;
use async_trait::async_trait;

pub struct Clock {
    period: Duration,
    pub event_bus_sender: Sender<Event>,
    event_bus_receiver: Receiver<Event>
}

impl Clock {
    pub fn new(tx: Sender<Event>) -> Clock {
        let rx = tx.subscribe();
        Clock {
            period: Duration::from_secs(1),
            event_bus_sender: tx,
            event_bus_receiver: rx,
        }

    }
}

#[async_trait]
impl Component for Clock {

    async fn run(&mut self) {
        loop {
            match self.event_bus_receiver.try_recv() {
                Ok(Event::EXIT) => break,
                _ => {}
            }
            self.event_bus_sender.send(Event::TICK).expect("Failed to send Tick event");
            sleep(Duration::from_secs(1)).await;
        }
    }
}