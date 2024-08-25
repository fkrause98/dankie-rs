pub mod entities;
use std::collections::HashSet;
use std::ops::Deref;

use anyhow::Result;
use entities::global_regex;
use entities::prelude::GlobalRegex;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::prelude::*;
use sea_orm::{Database, DatabaseConnection, Set};
use tbot::prelude::*;
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

pub async fn fetch_regexes() -> Result<Vec<String>> {
    Ok(GlobalRegex::find()
        .all(DB.deref())
        .await?
        .into_iter()
        .map(|r| r.regexp.to_string())
        .collect())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
    bot.command_with_description(
        "agregar",
        "Agrega un trigger a la lista de triggers globales",
        |context| async move {
            let trigger = global_regex::ActiveModel {
                regexp: Set(context.text.value.clone()),
            };
            let response = match trigger.insert(DB.deref()).await {
                Ok(_) => "Trigger añadido con exito",
                _ => "No se pudo añadir, revisá que no exista ya master",
            };
            context.send_message(response).call().await.unwrap();
        },
    );
    bot.command_with_description(
        "listar",
        "Listar los triggers globales conocidos",
        |context| async move {
            // FIXME: Comprobar que la regex sea valida con:
            // Regex::new(input).is_some()
            let response = fetch_regexes().await.unwrap().join("\n");
            context.send_message(response).call().await.unwrap();
        },
    );
    bot.text(|context| async move {
        let captures = fetch_regexes()
            .await
            .unwrap()
            .into_iter()
            .map(|r| Regex::new(&r).unwrap())
            .filter_map(|reg| reg.captures(&context.text.value))
            .collect::<Vec<_>>();
        log::debug!("{:?}", captures);
        // let response = captures.collect::<>
    });
    bot.polling().start().await.unwrap();
}
