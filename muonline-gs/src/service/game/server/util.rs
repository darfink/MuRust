use futures::Future;
use std::io;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

pub fn delay(delay: Duration) -> impl Future<Item = (), Error = io::Error> {
  Delay::new(Instant::now() + delay).map_err(|_| io::ErrorKind::Interrupted.into())
}
