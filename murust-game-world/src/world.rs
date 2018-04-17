pub struct GameWorld {
  dispatcher: Dispatcher,
  world: World;
}

impl GameWorld {
  pub fn new() -> Self {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Movement>();

    world.add_resource(terrain);

    let mut dispatcher = DispatcherBuilder::new()
      .add(MovementSystem, "movement_system", &[])
      .add(MovementPostSystem, "movement_post_system", &["movement_system"])
      .build();
  }

  pub fn add_player(&mut self) {
    const MoveDuration = Duration::from_millis(400);
  }

  pub fn remove_player(&mut self) {
  }

  pub fn update(&mut self) {
    self.dispatcher.dispatch(&self.world);
    world.maintain();
  }
}
