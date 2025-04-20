use crate::game::global_mut;

use crate::player::Player;
use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct Monster {
    base: Base<Area2D>,
    #[export]
    #[init(val = -50.0)]
    speed: f32,
}

#[godot_api]
impl Monster {
    #[func]
    fn _on_body_entered(&mut self, body: Gd<Node2D>) {
        if let Ok(mut player) = body.try_cast::<Player>() {
            player.bind_mut().over();
        }
    }

    #[func]
    fn _on_area_entered(&mut self, &mut area: Gd<Area2D>) {
        if area.is_in_group("bullet") {
            self.on_dead();
            area.queue_free();
        }
    }

    #[func]
    fn _on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    fn on_dead(&mut self) {
        let mut animator = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animator.set_animation("death");
        let collision = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        self.base_mut().remove_child(&collision);
        self.speed = 0.0;
        let mut gd = self.to_gd();
        animator
            .signals()
            .animation_finished()
            .connect(move || gd.queue_free());
        global_mut().score += 1;
    }
}

#[godot_api]
impl IArea2D for Monster {
    fn process(&mut self, delta: f64) {
        let v = Vector2::new(self.speed, 0.0);
        let position = self.base().get_position() + v * delta as f32;
        self.base_mut().set_position(position);
    }
}
