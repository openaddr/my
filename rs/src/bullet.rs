use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Bullet {
    base: Base<Area2D>,

    #[export]
    #[init(val = 300.0)]
    speed: f32,
}

#[godot_api]
impl IArea2D for Bullet {
    fn physics_process(&mut self, delta: f64) {
        let v = Vector2::new(self.speed, 0.0);
        let position = self.base().get_position() + v * delta as f32;
        self.base_mut().set_position(position)
    }

    fn ready(&mut self) {
        let base = self.base();
        let mut visible_on_screen =
            base.get_node_as::<VisibleOnScreenNotifier2D>("VisibleOnScreenNotifier2D");
        let mut gd = self.to_gd();
        visible_on_screen
            .signals()
            .screen_exited()
            .connect(move || {
                gd.queue_free()
            });
    }
}
