struct MovementSystem;

impl<'a> System for MovementSystem {
  type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Movement>);

  fn run(&mut self, (mut position, mut movement): Self::SystemData) {
    let time = Instant::now();

    for (position, movement) in (&mut position, &mut movement).join() {
      if time.duration_since(movement.last_movement) <= step_delay {
        continue;
      }

      if let Some(new_position) = movement.path.pop() {
        movement.last_movement = time;
        position = new_position;
      }
    }
  }
}

struct MovementPostSystem;

impl<'a> System for MovementPostSystem {
  fn run() {
    for (entity, movement) in (entities, movement).join() {
      if movement.path.is_empty() {
        movement.remove(entity)
      }
    }
  }
}
