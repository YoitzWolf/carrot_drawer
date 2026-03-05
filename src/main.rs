mod app_setup;

use std::time::Duration;
use app_setup::*;
mod core;
use core::*;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use tokio;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;
use tokio::time::sleep;

fn runner(proxy: EventLoopProxy<StateUpdate>) {
    tokio::task::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            println!("Boba");
        }
    });
}

pub async fn run() -> anyhow::Result<()> {
    env_logger::init();
    let mut event_loop = EventLoop::with_user_event().build()?;
    let mut proxy = event_loop.create_proxy();
    let mut app = App::new();
    runner(proxy);
    event_loop.run_app(&mut app)?;
    Ok(())
}


#[tokio::main]
async fn main() {
    run().await.unwrap();
}