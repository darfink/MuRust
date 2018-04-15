use failure::{Error, Fail};
use futures::Future;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

pub fn delay(delay: Duration) -> impl Future<Item = (), Error = Error> {
  Delay::new(Instant::now() + delay)
    .map_err(|error| error.context("Could not create task timer").into())
}
