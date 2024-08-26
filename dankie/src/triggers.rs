use crate::entities::global_regex;
use crate::entities::prelude::GlobalRegex;
use crate::module::Module;
use crate::{Bot, BotCommand, DB};
use anyhow::Result;
use regex::{Regex, RegexSet};
use sea_orm::prelude::*;
use sea_orm::{NotSet, Set};
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
    pub async fn agregar_trigger(context: BotCommand) -> Result<String, String> {
        // Chequear que la regex sea válida.
        let _ = Regex::new(&context.text.value).map_err(|err| err.to_string());
        // Cambios para la db
        let trigger = global_regex::ActiveModel {
            regexp: Set(context.text.value.clone()),
            ..Default::default()
        };
        // Insertar los cambios.
        let db_res = trigger.insert(DB.deref()).await;

        let response = match dbg!(db_res) {
            Ok(_) => "Trigger añadido con exito",
            Err(_) => "No se pudo añadir, revisá que no exista ya master",
        };

        Ok(response.to_string())
    }
    pub async fn listar_triggers(context: BotCommand) {
        let response = fetch_regexes().await.unwrap().join("\n");
        context.send_message(response).call().await.unwrap();
    }
    pub async fn match_con_mensaje(txt: &str) -> Vec<String> {
        let regexes = fetch_regexes().await.unwrap();
        let set = RegexSet::new(regexes.iter()).unwrap();
        let matching_regexes: Vec<_> = set.matches(txt).into_iter().collect();
        matching_regexes
            .iter()
            .map(|&index| regexes[index].clone())
            .collect()
    }
}
impl Module for Triggers {
    fn load(&self, bot: &mut Bot) {
        bot.command_with_description(
            "agregar",
            "Agrega un trigger a la lista de triggers globales",
            |context| async move {
                let response = match Triggers::agregar_trigger(context.clone()).await {
                    Ok(success) => success,
                    Err(err_msg) => err_msg,
                };
                context.send_message(response).call().await.unwrap();
            },
        );
        bot.command_with_description(
            "listar",
            "Listar los triggers globales conocidos",
            Triggers::listar_triggers,
        );
        bot.text(|context| async move {
            let input = &context.text.value;
            let matches = Triggers::match_con_mensaje(&input).await;
            context
                .send_message(matches.join("\n"))
                .call()
                .await
                .unwrap();
        })
    }
}
