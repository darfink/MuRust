pub trait RemoveItemBy<T> {
  fn remove_item_by<P>(&mut self, predicate: P) -> Option<T>
  where
    P: FnMut(&T) -> bool;
}

impl<T> RemoveItemBy<T> for Vec<T> {
  fn remove_item_by<P>(&mut self, predicate: P) -> Option<T>
  where
    P: FnMut(&T) -> bool,
  {
    let index = (*self).iter().position(predicate)?;
    Some(self.remove(index))
  }
}
