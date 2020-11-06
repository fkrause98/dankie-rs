use super::{Bot, InnerBot};
use crate::{
    connectors::Client, errors, methods::LogOut, proxy::Proxy, token::Token,
};
use std::sync::Arc;

/// A builder for a [`Bot`] with advanced configuration.
///
/// [`Bot`]: ./struct.Bot.html
#[derive(Debug)]
#[must_use]
pub struct Builder(InnerBot);

impl Builder {
    /// Starts constructing a `Bot` with the provided token.
    pub fn with_string_token(token: String) -> Self {
        Self(InnerBot::new(Token(token), Client::https()))
    }

    /// Starts constructing a `Bot`, extracting the token from the provided
    /// environment variable.
    pub fn with_env_token(env_var: &'static str) -> Self {
        let token = std::env::var(env_var).unwrap_or_else(|_| {
            panic!("[tbot] Bot's token in {} was not specified", env_var)
        });

        Self::with_string_token(token)
    }

    /// Configures a proxy through which all the request will go.
    pub fn proxy(mut self, proxy: impl Into<Proxy>) -> Self {
        let proxy: Proxy = proxy.into();
        self.0.set_client(proxy.into());
        self
    }

    // I don't think marking `localhost` as a link is a good idea
    #[allow(clippy::doc_markdown)]
    /// Configures the URI where the bot will make requests.
    ///
    /// You only need to use this if you're going to use a self-hosted Bot API
    /// server. The provided URI may be `http` or `https`, it also may contain
    /// a path (e.g. `http://localhost/self-hosted-bot-api/`), and `tbot` will
    /// append `bot$TOKEN/$METHOD` to it, in case the server is behind a reverse
    /// proxy. The URI may also contain a query (e.g. `https://localhost/?foo`),
    /// in which case `tbot` will move it after the `bot$TOKEN/$METHOD` part.
    /// For example:
    ///
    /// The provided URI        | A URI generated by `tbot`
    /// ------------------------|-----------------------------------------
    /// `http://localhost`      | `http://localhost/bot$TOKEN/$METHOD`
    /// `http://localhost/foo`  | `http://localhost/foo/bot$TOKEN/$METHOD`
    /// `http://localhost/?foo` | `http://localhost/bot$TOKEN/$METHOD?foo`
    ///
    /// Note that `tbot` itself does not use the query part. `tbot` allows you
    /// to set it just in case your self-hosted Bot API server is behind
    /// a reverse proxy and you need to set the query for some reason. In this
    /// case, the query part is supposed to be removed  when it gets to the
    /// Bot API server.
    ///
    /// Do not forget to call [`log_out`] when you're moving from the cloud Bot
    /// API server, or [`close`] when you're moving from one self-hosted server
    /// to another. This method calls neither method and assumes that you've
    /// already migrated to the server you're configuring.
    ///
    /// # Example
    ///
    /// Say that you've started your local Bot API server on
    /// `http://localhost:8081`, and this is the first time you configure your
    /// bot to use your Bot API server. First, you need to call [`log_out`],
    /// and only then you call `server_uri`:
    ///
    /// ```no_run
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error> {
    /// use tbot::BotBuilder;
    ///
    /// let bot = BotBuilder::with_env_token("BOT_TOKEN")
    ///     .log_out().await? // log out from cloud Bot API first
    ///     .server_uri("http://localhost:8081".parse()?)
    ///     .build();
    /// # }
    /// ```
    ///
    /// Now `tbot` will make requests to your Bot API server. You only need
    /// to call [`log_out`] once (unless you use the cloud Bot API server
    /// again), so after you did it once, you can remove that line:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error> {
    /// use tbot::BotBuilder;
    ///
    /// let bot = BotBuilder::with_env_token("BOT_TOKEN")
    ///     .server_uri("http://localhost:8081".parse()?)
    ///     .build();
    /// # }
    /// ```
    ///
    /// [`log_out`]: #method.log_out
    ///
    /// If you're moving from one local server to another, you're going to call
    /// this method twice:
    ///
    /// ```no_run
    /// # async fn foo() -> Result<(), Box<dyn std::any::Any> {
    /// use tbot::BotBuilder;
    ///
    /// let bot = BotBuilder::with_env_token("BOT_TOKEN")
    ///     .server_uri("http://other-server:8081".parse()?)
    ///     .close().await? // close the bot on the old server first
    ///     .server_uri("http://localhost:8081".parse()?)
    ///     .build();
    /// # }
    /// ```
    ///
    /// Just like with logging out from the cloud Bot API server, you only have
    /// to do it once, and after that you can leave only the last `server_uri`
    /// call. If you use webhooks, do not forget to call [`delete_webhook`]
    /// before calling [`close`] to ensure that your bot isn't launched
    /// on the old server when it restarts.
    ///
    /// [`close`]: #method.close
    /// [`delete_webhook`]: #method.delete_webhook
    pub fn server_uri(mut self, uri: hyper::Uri) -> Self {
        self.0.set_uri(uri);
        self
    }

    /// Logs out from the cloud Bot API server.
    ///
    /// Note that after calling this method you must change the URI where `tbot`
    /// makes requests to your local Bot API server using [`server_uri`]. Once
    /// you log out, you cannot log back in the cloud server for 10 minutes.
    ///
    /// [`server_uri`]: #method.server_uri
    ///
    /// In case of an error, a tuple of `(`[`errors::MethodCall`]`, Self)` is
    /// returned in case you expect an error and can recover from it.
    ///
    /// [`errors::MethodCall`]: ./errors/enum.MethodCall.html
    pub async fn log_out(self) -> Result<Self, (errors::MethodCall, Self)> {
        match LogOut::new(&self.0).call().await {
            Ok(()) => Ok(self),
            Err(error) => Err((error, self)),
        }
    }

    /// Finishes constructing the [`Bot`].
    ///
    /// [`Bot`]: ./struct.Bot.html
    pub fn build(self) -> Bot {
        Bot {
            inner: Arc::new(self.0),
        }
    }
}
