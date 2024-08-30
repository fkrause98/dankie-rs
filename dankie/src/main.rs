pub type Bot = tbot::EventLoop;
pub type BotCommand = std::sync::Arc<tbot::contexts::Command>;
pub type TxtMsg = std::sync::Arc<tbot::contexts::Text>;
pub mod entities;
#[macro_use]
pub mod module;
pub mod triggers;

use module::Module;
use once_cell::sync::Lazy;
use sea_orm::{Database, DatabaseConnection};
use tokio::{runtime::Handle, task};

pub static DB: Lazy<DatabaseConnection> = Lazy::new(init_db);

fn init_db() -> DatabaseConnection {
    task::block_in_place(move || {
        Handle::current().block_on(async {
            Database::connect("postgres://postgres:postgres@127.0.0.1:5432")
                .await
                .unwrap()
        })
    })
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
    let modules = modules![crate::triggers::Triggers];
    for module in modules {
        module.load(&mut bot);
    }
    bot.webhook(env!("WEBHOOK_URL"), env!("WEBHOOK_PORT"))
        .http()
        .start()
        .await
        .unwrap();
}
