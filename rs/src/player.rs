use crate::game::{GameState, global, global_mut};
use crate::game_manager::GameManager;
use gdext_coroutines::prelude::{StartCoroutine, seconds};
use godot::classes::*;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[init(val = 100.0)]
    speed: f32,

    #[export]
    bullet_scene: Option<Gd<PackedScene>>,
}

#[godot_api]
impl Player {
    
    #[func]
    fn _on_fire(&self) {
        if self.base().get_velocity() != Vector2::ZERO || global().state == GameState::Over {
            return;
        }
        let mut audio = self.base().get_node_as::<AudioStreamPlayer>("FireSound");
        audio.play();
        let mut bullet: Gd<Node2D> = self
            .bullet_scene
            .clone()
            .unwrap()
            .instantiate_as::<Node2D>();
        bullet.set_position(self.base().get_position() + Vector2::new(6.0, 6.0));
        

        self.base()
            .get_tree()
            .unwrap()
            .get_current_scene()
            .unwrap()
            .add_child(&bullet);
    }
    /// 重载场景
    pub fn over(&mut self) {
        // 1. 获取场景树并创建定时器 [[1]][[5]]
        let gd = self.to_gd();
        self.start_coroutine(
            #[coroutine]
            move || {
                yield seconds(3.0);
                gd.get_tree().unwrap().reload_current_scene();
                global_mut().state = GameState::Start;
                global_mut().score = 0;
            },
        );
        let mut gm = self
            .base()
            .get_tree()
            .unwrap()
            .get_current_scene()
            .unwrap()
            .cast::<GameManager>();
        let mut animator = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let collision = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        self.base_mut().remove_child(&collision);
        animator.set_animation("over");
        global_mut().state = GameState::Over;
        gm.bind_mut().show_game_over();
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn physics_process(&mut self, _: f64) {
        // todo 移动范围需要限制
        if global().state == GameState::Over {
            return;
        }
        let direction = Input::singleton().get_vector("left", "right", "up", "down");
        let v = direction * self.speed; // 矢量速度

        let speed_sq = v.length_squared(); // 避免开平方运算
        let animation = if speed_sq > 0.1 {
            "run" // 移动时播放run动画
        } else {
            "idle" // 静止时播放idle动画
        };
        let mut animator = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animator.set_animation(animation);

        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();
    }
}
