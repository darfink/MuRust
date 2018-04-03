/// An interface for queryable servers.
pub trait QueryableGameServer {
  /// Returns the URI of the Game Server.
  fn uri(&self) -> &str;
}
