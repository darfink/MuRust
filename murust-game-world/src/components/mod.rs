#[derive(Component, Debug)]
#[component(VecStorage)]
struct Movement {
  path: Vec<Position>,
  last_movement: Instant,
}
