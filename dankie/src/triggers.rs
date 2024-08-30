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
use tbot::contexts::Text;
use tbot::types::message::Kind;
use tbot::types::parameters::ImplicitChatId;
pub async fn fetch_regexes() -> Result<Vec<global_regex::Model>> {
    Ok(GlobalRegex::find()
        .all(DB.deref())
        .await?
        .into_iter()
        .collect())
}

use thiserror::Error;
use TriggerError::*;

#[derive(Error, Debug)]
pub enum TriggerError {
    #[error("Tenes que responder a algo pa")]
    NoSelectedReply,
    #[error("Flaco, pasame bien la regex, me bardea el compilador \n: {0} ")]
    InvalidRegex(#[from] regex::Error),
    #[error("No se pudo añadir, revisá que no exista ya master")]
    GenericErr,
    #[error("No matchea ninguna man")]
    NoMatch,
}

#[derive(Copy, Clone, Debug)]
pub struct Triggers;
impl Triggers {
    pub async fn agregar_trigger(context: BotCommand) -> Result<(), TriggerError> {
        let Some(trigger_msg) = &context.reply_to else {
            return Err(NoSelectedReply);
        };
        // Chequear que la regex sea válida.
        Regex::new(&context.text.value).map_err(|err| InvalidRegex(err))?;
        // Cambios para la db
        let trigger = global_regex::ActiveModel {
            regexp: Set(context.text.value.clone()),
            chat_id: Set(trigger_msg.chat.id.0),
            // FIXME: This should be u32 on the db
            msg_id: Set(trigger_msg.id.0 as i64),
            ..Default::default()
        };
        // Insertar los cambios, y si no early return.
        trigger.insert(DB.deref()).await.map_err(|_| GenericErr)?;
        Ok(())
    }
    pub async fn listar_triggers(context: BotCommand) {
        let regexes: Vec<String> = fetch_regexes()
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.regexp)
            .collect();
        let response = format!("Triggers conocidos: \n {}", regexes.join("\n"));
        context.send_message(response).call().await.unwrap();
    }

    pub async fn match_con_mensaje(txt: &str) -> Result<Vec<global_regex::Model>, TriggerError> {
        let regexes = fetch_regexes().await.map_err(|_| TriggerError::NoMatch)?;
        let set =
            RegexSet::new(regexes.iter().map(|r| &r.regexp)).map_err(|_| TriggerError::NoMatch)?;
        let matching_regexes: Vec<_> = set.matches(txt).into_iter().collect();
        if matching_regexes.len() == 0 {
            Err(NoMatch)
        } else {
            Ok(matching_regexes
                .iter()
                .map(|&index| regexes[index].clone())
                .collect())
        }
    }
    pub async fn recuperar_un_trigger(id: i64) -> Option<global_regex::Model> {
        GlobalRegex::find_by_id(id).one(DB.deref()).await.ok()?
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
                    Ok(success) => "Trigger agregado, master",
                    Err(err) => &err.to_string(),
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
                if let Some(input) = &context.reply_to.clone() {
                    let input = match &input.kind {
                        Kind::Text(t) => &t.value,
                        _ => unreachable!(),
                    };
                    let response = match Triggers::match_con_mensaje(input).await {
                        Ok(found) => {
                            let matches: Vec<String> =
                                found.into_iter().map(|r| r.regexp).collect();
                            format!("Triggers que matchean: \n {}", matches.join("\n"))
                        }
                        Err(err) => err.to_string(),
                    };
                    context
                        .send_message_in_reply(response)
                        .call()
                        .await
                        .unwrap();
                }
            },
        );
        bot.text(|context| async move {
            let input = &context.text.value;
            let matches = Triggers::match_con_mensaje(&input).await;
            match matches {
                Ok(matches) if matches.first().is_some() => {
                    if let Some(r) = Triggers::recuperar_un_trigger(matches[0].id).await {
                        let chat_id = tbot::types::chat::Id(r.chat_id);
                        let msg_id = tbot::types::message::Id(r.msg_id as u32);
                        context.forward_here(chat_id, msg_id).call().await;
                    }
                }
                _ => {}
            }
        });
    }
}
