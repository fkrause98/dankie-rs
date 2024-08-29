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
use tbot::types::parameters::ImplicitChatId;
pub async fn fetch_regexes() -> Result<Vec<global_regex::Model>> {
    Ok(GlobalRegex::find()
        .all(DB.deref())
        .await?
        .into_iter()
        .collect())
}
#[derive(Copy, Clone, Debug)]
pub struct Triggers;
impl Triggers {
    pub async fn agregar_trigger(context: BotCommand) -> Result<String, String> {
        let Some(trigger_msg) = &context.reply_to else {
            return Err("Tenés que responder a algo, bro".to_string());
        };
        // Chequear que la regex sea válida.
        let _ = Regex::new(&context.text.value).map_err(|err| err.to_string())?;
        // Cambios para la db
        let trigger = global_regex::ActiveModel {
            regexp: Set(context.text.value.clone()),
            chat_id: Set(trigger_msg.chat.id.0),
            // FIXME: This should be u32 on the db
            msg_id: Set(trigger_msg.id.0 as i64),
            ..Default::default()
        };
        // Insertar los cambios.
        let db_res = trigger.insert(DB.deref()).await;

        let response = match db_res {
            Ok(_) => "Trigger añadido con exito",
            Err(_) => "No se pudo añadir, revisá que no exista ya master",
        };

        Ok(response.to_string())
    }
    pub async fn listar_triggers(context: BotCommand) {
        let regexes: Vec<String> = fetch_regexes().await.unwrap().into_iter().map(|r| r.regexp).collect();
        let response = format!("Triggers conocidos: \n {}", regexes.join(""));
        context.send_message(response).call().await.unwrap();
    }
    pub async fn match_con_mensaje(txt: &str) -> Vec<global_regex::Model> {
        let regexes = fetch_regexes().await.unwrap();
        let set = RegexSet::new(regexes.iter().map(|r| &r.regexp)).unwrap();
        let matching_regexes: Vec<_> = set.matches(txt).into_iter().collect();
        matching_regexes
            .iter()
            .map(|&index| regexes[index].clone())
            .collect()
    }
    pub async fn recuperar_un_trigger(id: i64) -> Option<global_regex::Model> {
        GlobalRegex::find_by_id(id)
            .one(DB.deref())
            .await
            .ok()?
    }
}
impl Module for Triggers {
    fn load(&self, bot: &mut Bot) {
        bot.command_with_description(
            "agregar",
            "Agrega un trigger a la lista de triggers globales",
            |context| async move {
                // Comprobar que agregar sea en base a una respuesta
                let response = match Triggers::agregar_trigger(context.clone()).await {
                    Ok(success) => success,
                    Err(err_msg) => {
                        format!("Wachin, hubo un error con tu regex: \n {}", err_msg)
                    }
                };
                context.send_message(response).call().await.unwrap();
            },
        );
        bot.command_with_description(
            "listar",
            "Listar los triggers globales conocidos",
            Triggers::listar_triggers,
        );
        bot.command_with_description(
            "triggered",
            "Ver qué triggers matchean con este mensaje",
            |context| async move {
                let input = &context.text.value;
                let matches = Triggers::match_con_mensaje(&input).await;
                if matches.len() > 0 {
                    context
                        .send_message(format!("Triggers que matchean: \n "))
                        .call()
                        .await
                        .unwrap();
                }
            },
        );
        bot.text(|context| async move {
            let input = &context.text.value;
            let matches = Triggers::match_con_mensaje(&input).await;
            if let Some(trigger) = matches.first() {
                if let Some(r) = Triggers::recuperar_un_trigger(trigger.id).await {
                    let chat_id = tbot::types::chat::Id(r.chat_id);
                    let msg_id = tbot::types::message::Id(r.msg_id as u32);
                    context.forward_here(chat_id, msg_id).call().await;
                }
            } else {}
        })
    }
}
