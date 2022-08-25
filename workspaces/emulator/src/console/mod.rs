pub mod cpu;
pub mod clock;
pub mod memory;

use async_trait::async_trait;

pub fn goodbye() {
    println!("Goodbye world!")
}

#[async_trait]
pub trait Component {

    async fn run(&mut self);
}