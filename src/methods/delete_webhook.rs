use super::send_method;
use crate::{connectors::Connector, errors, internal::Client, Token};

#[derive(Debug, Clone)]
#[must_use]
pub struct DeleteWebhook<'a, C> {
    client: &'a Client<C>,
    token: Token,
}

impl<'a, C> DeleteWebhook<'a, C> {
    pub(crate) const fn new(client: &'a Client<C>, token: Token) -> Self {
        Self { client, token }
    }
}

impl<C: Connector> DeleteWebhook<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            &self.token,
            "deleteWebhook",
            None,
            Vec::new(),
        )
        .await?;

        Ok(())
    }
}
