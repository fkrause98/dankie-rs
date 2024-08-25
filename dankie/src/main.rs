use std::sync::Arc;
pub mod entities;
use crate::entities::prelude::*;
use anyhow::{anyhow, bail};
use entities::global_regex;
use log::info;
use sea_orm::prelude::*;
use sea_orm::{Database, Set};
use std::cell::OnceCell;
use tbot::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
    bot.command_with_description(
        "agregar",
        "Agrega un trigger a la lista de triggers globales",
        |context| async move {
            let mut db = Database::connect("postgres://postgres:postgres@127.0.0.1:5432")
                .await
                .map_err(|_| "Fatal: could not reach db")
                .unwrap()
                .clone();
            let trigger = global_regex::ActiveModel {
                regexp: Set(context.text.value.clone()),
            };
            let trigger: global_regex::Model = trigger.insert(&db.clone()).await.unwrap();
        },
    );
    bot.polling().start().await.unwrap();
}
