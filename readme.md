# [`tbot`](https://tbot.rs)

Make cool Telegram bots with Rust easily. For example, here's a simple echo bot:

```rust
use tbot::prelude::*;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let echo = &context.text.value;
        let call_result = context.send_message(echo).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
```

There are many [examples] to see `tbot` in action. If you want to see real-world
use of `tbot`, check out [this list][projects].

If you're a newcomer, we recommend you go through the [tutorial] first. We also
have several [How-to guides][how-to] to help you use `tbot`. You can always
refer to our API docs on [_docs.rs_][api-docs] (also, docs for `master`
are available [here][master-docs]).

If you have a question, ask it in [our group] on Telegram. If you find a bug,
file an issue on either our [GitLab] or [GitHub] repository.

[examples]: ./examples/
[projects]: https://gitlab.com/SnejUgal/tbot/-/wikis/Projects-built-with-tbot

[tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
[how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
[api-docs]: https://docs.rs/tbot
[master-docs]: https://docs.tbot.rs

[our group]: https://t.me/tbot_group
[gitlab]: https://gitlab.com/SnejUgal/tbot
[github]: https://github.com/tbot-rs/tbot

## Features

- Full Telegram Bot API 4.9 support, as well as media download/upload, polling
  and [webhooks];
- `async`/`.await` support, built upon `tokio`;
- Type-safe and idiomatic API;
- Easy to use, while scalable and configurable.

[webhooks]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks

## Installation

Add `tbot` and `tokio` to your Cargo.toml:

```toml
[dependencies]
tbot = "0.6"
tokio = { version = "0.2", features = ["macros"] }
```

`tokio` is required to start the runtime. You'll also need the `macros` feature
of `tokio` if you're going to start the runtime using `#[tokio::main]`.

## Minimum supported Rust version

`tbot` inherits `tokio`'s MSRV policy: the last three minor versions are
supported. Last time this paragraph was updated, the latest version was 1.52,
therefore `tbot`'s MSRV is 1.49. You can also look up it in our
[`.gitlab-ci.yml`], as it shows what version we're actually testing on.

[`.gitlab-ci.yml`]: ./.gitlab-ci.yml

## Contribution

Glad you want to contribute to `tbot`! We develop the crate on [GitLab],
so create your merge request there if you can. We may accept pull requests
on [GitHub] as well, but we prefer [GitLab].
