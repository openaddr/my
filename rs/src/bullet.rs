use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
struct Bullet {
    base: Base<Area2D>,

    #[init(val = 300.0)]
    speed: f32,

    #[init(node = "VisibleOnScreenNotifier2D")]
    visible_on_screen: OnReady<Gd<VisibleOnScreenNotifier2D>>,
}

#[godot_api]
impl IArea2D for Bullet {
    fn physics_process(&mut self, delta: f64) {
        let v = Vector2::new(self.speed, 0.0);
        let position = self.base().get_position() + v * delta as f32;
        self.base_mut().set_position(position)
    }

    fn ready(&mut self) {
        let mut gd = self.to_gd();
        self.visible_on_screen
            .signals()
            .screen_exited()
            .connect(move || gd.queue_free());
    }
}
