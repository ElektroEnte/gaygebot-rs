use std::sync::Arc;
use std::time::{Duration, Instant};

use async_std::task;

use tokio::sync::Mutex;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{SecureTCPTransport, TwitchIRCClient};

use crate::bot::Output;
use super::environment::BotPermissions;

#[derive(Clone, Debug, PartialEq)]
pub struct CacheItem {
    text: String,
    timestamp: Instant,
}

impl CacheItem {
    pub fn new(text: String) -> Self {
        CacheItem {
            text,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OutputQueue {
    queue: Vec<Output>,
    last_sent: Instant,
    cache: Vec<CacheItem>,
    pub is_running: bool,
}

impl OutputQueue {
    pub fn new() -> Self {
        OutputQueue {
            queue: vec![],
            last_sent: Instant::now(),
            cache: vec![],
            is_running: false,
        }
    }
    
    pub async fn add(&mut self, output: Output) {
        self.queue.push(output);
    }
    
    pub fn length(&self) -> usize {
        self.queue.len()
    }
    
    pub async fn run(queue: Arc<Mutex<OutputQueue>>, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>) {
        let mut locked_queue = queue.lock().await;
        locked_queue.is_running = true;
        drop(locked_queue);
        
        'outer: loop {
            let mut locked_queue = queue.lock().await;
            
            if locked_queue.length() > 0 {
                let mut output = locked_queue.queue.remove(0);
            
                let mut to_remove: usize = 0;
                
                for item in locked_queue.cache.iter() {
                    if item.timestamp.elapsed() > Duration::from_secs(30) {
                        to_remove += 1;
                    } else if item.text == output.text {
                        output.text.push('ㅤ');
                    }
                }
                
                for _ in 0..to_remove {
                    locked_queue.cache.remove(0);
                }
                
                locked_queue.cache.push(CacheItem::new(output.text.clone()));
                
                output.send_in_context_using(Arc::clone(&client)).await.unwrap();
                
                locked_queue.last_sent = Instant::now();
                
                if BotPermissions::Default == output.input.ctx.environment.lock().await.permissions {
                    task::sleep(Duration::from_secs(1)).await;
                }
            }
            drop(locked_queue);
            
            task::sleep(Duration::from_millis(10)).await;
        }
    }
}

impl Default for OutputQueue {
    fn default() -> Self {
        OutputQueue {
            queue: vec![],
            last_sent: Instant::now(),
            cache: vec![],
            is_running: false
        }
    }
}