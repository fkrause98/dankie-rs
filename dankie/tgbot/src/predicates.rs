//! Useful predicates and utilities for them.

// `tbot`'s types are `Send`, and users can't implement `tbot`'s traits anyway,
// so adding `+ Send + Sync` will only make docs too explicit
#![allow(clippy::future_not_send)]
// Functions defined in this module are intended to be used in places where
// async functions are expected. It will be inconvinient for users to wrap
// manually our ready-made predicates in async functions.
#![allow(clippy::unused_async)]

pub mod chat;
pub mod media;
pub mod message;
mod traits;

use futures::{future::BoxFuture, Future};
use std::sync::Arc;
pub use traits::{
    PredicateBooleanOperations, StatefulPredicateBooleanOperations,
};

/// Allows running stateless predicates in the stateful event loop.
pub fn without_state<'a, C, P, S, F>(
    predicate: P,
) -> impl Fn(Arc<C>, Arc<S>) -> BoxFuture<'a, bool> + Send + Sync + 'a
where
    P: PredicateBooleanOperations<C, F>,
    F: Future<Output = bool> + Send,
    C: Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    let predicate = Arc::new(predicate);

    move |ctx, _state| {
        let predicate = Arc::clone(&predicate);
        Box::pin(async move { predicate(ctx).await })
    }
}
