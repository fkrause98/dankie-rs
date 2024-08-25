pub static DB: Lazy<DatabaseConnection> = Lazy::new(init_db);
pub type Bot = tbot::EventLoop;

pub mod entities;
pub mod module;
pub mod triggers;
