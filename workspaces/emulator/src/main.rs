use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use emulator::console::clock::Clock;
use emulator::console::Component;
use emulator::console::cpu::Cpu;
use emulator::console::memory::Memory;
use futures::future::join_all;


#[tokio::main]
async fn main() {
    println!("Booting NES Emulator...");

    let (tx, _) = broadcast::channel(16);

    let mut handles = vec![];

    let clock_tx = tx.clone();
    handles.push(
        tokio::spawn(async move {
            Clock::new(clock_tx).run().await;
        })
    );

    let cpu_tx = tx.clone();
    handles.push(
        tokio::spawn(async move {
            Cpu::new(cpu_tx).run().await;
        })
    );

    let mem_tx = tx.clone();
    handles.push(
        tokio::spawn(async move {
            Memory::new(mem_tx).run().await;
        })
    );

    join_all(handles).await;

}
