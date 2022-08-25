use tokio::sync::broadcast::{Receiver, Sender};
use crate::console::Component;
use crate::event::Event;
use async_trait::async_trait;

pub struct Memory {
    event_bus_sender: Sender<Event>,
    event_bus_receiver: Receiver<Event>
}

impl Memory {
    pub fn new(tx: Sender<Event>) -> Memory {
        let rx = tx.subscribe();
        Memory {
            event_bus_sender: tx,
            event_bus_receiver: rx
        }

    }
}

#[async_trait]
impl Component for Memory {


    async fn run(&mut self) {
        loop {
            match self.event_bus_receiver.recv().await {
                Ok(Event::EXIT) => break,
                Ok(Event::TICK) => {
                    println!("Got a Tick in Memory!")
                }
                _ => println!("Error!")
            }
        }
    }
}