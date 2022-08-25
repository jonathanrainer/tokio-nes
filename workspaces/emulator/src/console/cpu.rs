use tokio::sync::broadcast::{Receiver, Sender};
use crate::console::Component;
use crate::event::Event;
use async_trait::async_trait;

pub struct Cpu {
    event_bus_sender: Sender<Event>,
    event_bus_receiver: Receiver<Event>

}

impl Cpu {
    pub fn new(tx: Sender<Event>) -> Self {
        let rx = tx.subscribe();
        Cpu { event_bus_sender: tx, event_bus_receiver: rx }

    }
}

#[async_trait]
impl Component for Cpu {

    async fn run(&mut self) {
        loop {
            match self.event_bus_receiver.recv().await {
                Ok(Event::EXIT) => break,
                Ok(Event::TICK) => {
                    println!("Got a Tick in CPU!")
                }
                _ => println!("Error!")
            }
        }
    }
}