use anyhow::Result;
use serde::Deserialize;
use tbot::contexts::methods::Message;
use tbot::contexts::Text;
use tbot::types::message::Kind;
use tbot::types::parameters::ImplicitChatId;
use thiserror::Error;
use core::fmt::Display;
use core::fmt;

use crate::module::Module;
use crate::Bot;

#[derive(Copy, Clone, Debug)]
pub struct Dolar;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DolarInfo {
    nombre: String,
    compra: f64,
    venta: f64,
    fecha_actualizacion: String,
}

impl Display for DolarInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dolar {}\nCompra: {}\nVenta: {}\nHora del Update: {} \n \n", self.nombre.clone(), self.compra, self.venta, self.fecha_actualizacion)
    }
}

impl Dolar {
    pub async fn dolar_all() -> Result<Vec<DolarInfo>> {
        Ok(reqwest::get("https://dolarapi.com/v1/dolares")
                       .await?
                       .json::<Vec<DolarInfo>>()
                       .await?)
    }
}
impl Module for Dolar {
    fn load(&self, bot: &mut Bot) {
        bot.command_with_description(
            "dolar",
            "devuelve los precios del dolar",
            |context| async move {
                // Comprobar que agregar sea en base a una respuesta
                let response =
                    match Dolar::dolar_all().await {
                        Ok(info) => info.into_iter().map(|info| info.to_string()).collect::<String>(),
                        Err(err) => {
                            log::error!("No se pudo conectar con la api del dolar {err}");
                            "No se puedo conectar a la api, intenta de nuevo mostro :(".to_string()
                        }
                    };
                context.send_message(format!("{}", response)).call().await.unwrap();
            },
        );
    }
}
