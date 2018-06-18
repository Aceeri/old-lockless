pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (ReadStorage<'a, Player>,);
    fn run(&mut self, (players,): Self::SystemData) {
         for (player,) in (&players).join() {
            println!("{:?} tagged as player", player.entity);
         }
    }
}
