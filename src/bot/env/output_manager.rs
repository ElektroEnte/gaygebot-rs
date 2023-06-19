use std::sync::Arc;
use std::time::{Duration, Instant};

use async_std::task;

use tokio::sync::Mutex;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{SecureTCPTransport, TwitchIRCClient};

use crate::bot::Output;
use super::environment::BotPermissions;

#[derive(Clone, Debug)]
pub struct OutputQueue {
    queue: Vec<Output>,
    last_sent: Instant,
    pub is_running: bool,
}

impl OutputQueue {
    pub fn new() -> Self {
        OutputQueue {
            queue: vec![],
            last_sent: Instant::now(),
            is_running: false,
        }
    }
    
    pub async fn add(&mut self, output: Output) {
        self.queue.push(output);
    }
    
    pub fn length(&self) -> usize {
        self.queue.len()
    }
    
    pub async fn run(manager: Arc<Mutex<OutputQueue>>, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>) {
        let mut locked_manager = manager.lock().await;
        locked_manager.is_running = true;
        drop(locked_manager);
        
        loop {
            let mut locked_manager = manager.lock().await;
            
            if locked_manager.length() > 0 {
                let mut output = locked_manager.queue.remove(0);
                output.send_in_context_using(Arc::clone(&client)).await.unwrap();
                
                locked_manager.last_sent = Instant::now();
                
                if BotPermissions::Default == output.input.ctx.environment.lock().await.permissions {
                    task::sleep(Duration::from_secs(1)).await;
                }
            }
            drop(locked_manager);
            
            task::sleep(Duration::from_millis(10)).await;
        }
    }
}

impl Default for OutputQueue {
    fn default() -> Self {
        OutputQueue {
            queue: vec![],
            last_sent: Instant::now(),
            is_running: false
        }
    }
}