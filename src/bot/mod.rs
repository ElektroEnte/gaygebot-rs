pub use context::Context;
pub use chatter::Chatter;
pub use channel::Channel;
pub use message::Message;
pub use bot_input::Input;
pub use bot_output::Output;
pub use response_type::ResponseType;

pub mod context;

pub mod channel;
pub mod message;
pub mod chatter;
pub mod bot_input;
pub mod bot_output;
pub mod env;
pub mod response_type;
pub mod identifier;

use std::collections::HashMap;
use std::sync::Arc;

use tokio;
use tokio::sync::Mutex;

use twitch_irc::{ClientConfig, Error, login::StaticLoginCredentials, message::ServerMessage, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::message::PrivmsgMessage;

use env::environment::Environment;
use crate::bot::env::output_manager::OutputQueue;

use crate::job::{
    job_manager::JobManager,
    job_parameter::JobParameter,
    job_pattern::JobPattern,
};

pub enum LogMode {
    None,
    Debug,
}

#[derive(Clone)]
pub struct Bot {
    pub login: String,
    oauth: Option<String>,
}

impl Bot {
    pub fn new(login: String, oauth: Option<String>) -> Self {
        Bot {
            login,
            oauth,
        }
    }
    
    pub fn generate_config(&self) -> ClientConfig<StaticLoginCredentials> {
        ClientConfig::new_simple(StaticLoginCredentials::new(self.login.to_owned(), self.oauth.to_owned()))
    }
    
    /// runs the bot instance
    pub async fn run(&mut self, initial_channels: Vec<String>, log_mode: LogMode) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let config = self.generate_config();
        let (mut incoming_messages, mut unsave_client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        
        let client = Arc::new(Mutex::new(unsave_client));
        
        let mut envs: Arc<Mutex<HashMap<String, Arc<Mutex<Environment>>>>> = Arc::new(Mutex::new(HashMap::new()));
        
        envs.lock().await.insert("default".to_string(), Arc::new(Mutex::new(Environment::default())));
        
        let queue_arc = Arc::clone(&envs.lock().await.get("default").unwrap().lock().await.output_queue);
        let client_arc = Arc::clone(&client);
        
        tokio::spawn(async move {
            OutputQueue::run(queue_arc, client_arc).await;
        });
        
        let join_handle = tokio::spawn(async move {
            for channel in initial_channels {
                let locked_client = client.lock().await;
                locked_client.join(channel).unwrap();
            }
            
            while let Some(message) = incoming_messages.recv().await {
                match log_mode {
                    LogMode::Debug => { println!("{:?}", message) }
                    LogMode::None => {}
                };
                
                match message {
                    ServerMessage::Privmsg(msg) => {
                        let client_clone = Arc::clone(&client);
                        let env_clone = Arc::clone(&envs);

                        Bot::on_private_message(client_clone, msg, env_clone).await.unwrap();
                    }
                    _ => {}
                };
            };
            
            Ok(())
        });
        
        join_handle.await.unwrap()?;
        
        Ok(())
    }
    
    async fn on_private_message(
        client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>,
        msg: PrivmsgMessage,
        envs: Arc<Mutex<HashMap<String, Arc<Mutex<Environment>>>>>,
    ) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let now = std::time::Instant::now();
        
        let mut ctx = Context::from(msg);
        
        let locked_envs = envs.lock().await;
        let env = Arc::clone(&locked_envs.get("default").unwrap());
        drop(locked_envs);
        
        ctx.set_env(&env);
        
        let mut input = Input::generate_from_context(ctx).await;
        
        let locked_env = env.lock().await;
        let queue_arc = Arc::clone(&locked_env.output_queue);
        drop(locked_env);
        
        let mut locked_queue = queue_arc.lock().await;
        locked_queue.add(input.execute_as_job().await).await;
        drop(locked_queue);
        
        
        println!("{}", now.elapsed().as_micros());
        
        Ok(())
    }
}
