use crate::entities::global_regex;
use crate::entities::prelude::GlobalRegex;
use crate::module::Module;
use crate::{Bot, BotCommand, TxtMsg, DB};
use anyhow::Result;
use regex::Regex;
use sea_orm::prelude::*;
use sea_orm::Set;
use std::ops::Deref;
use tbot::contexts::methods::Message;
pub async fn fetch_regexes() -> Result<Vec<String>> {
    Ok(GlobalRegex::find()
        .all(DB.deref())
        .await?
        .into_iter()
        .map(|r| r.regexp.to_string())
        .collect())
}
#[derive(Copy, Clone, Debug)]
pub struct Triggers;
impl Triggers {
    pub async fn agregar_trigger(context: BotCommand) {
        // FIXME: Comprobar que la regex sea valida con:
        // Regex::new(input).is_some()
        let trigger = global_regex::ActiveModel {
            regexp: Set(context.text.value.clone()),
        };

        let response = match trigger.insert(DB.deref()).await {
            Ok(_) => "Trigger añadido con exito",
            _ => "No se pudo añadir, revisá que no exista ya master",
        };

        context.send_message(response).call().await.unwrap();
    }
    pub async fn listar_triggers(context: BotCommand) {
        let response = fetch_regexes().await.unwrap().join("\n");
        context.send_message(response).call().await.unwrap();
    }
    pub async fn match_con_mensaje(txt: TxtMsg) {
        let captures = fetch_regexes()
            .await
            .unwrap()
            .into_iter()
            .map(|r| Regex::new(&r).unwrap())
            .filter_map(|reg| reg.captures(&txt.text.value))
            .collect::<Vec<_>>();
        log::debug!("Matched captures: {:?}", captures);
    }
}
impl Module for Triggers {
    fn load(&self, bot: &mut Bot) {
        bot.command_with_description(
            "agregar",
            "Agrega un trigger a la lista de triggers globales",
            Triggers::agregar_trigger,
        );
        bot.command_with_description(
            "listar",
            "Listar los triggers globales conocidos",
            Triggers::listar_triggers,
        );
        bot.text(Triggers::match_con_mensaje);
    }
}
