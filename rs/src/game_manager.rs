use crate::game::global;
use crate::monster::Monster;
use godot::classes::{Label, Timer};
use godot::global::{clampf, randf_range};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct GameManager {
    #[init(load = "res://Scenes/monster.tscn")]
    monster_scene: OnReady<Gd<PackedScene>>,

    #[init(node = "CanvasLayer/Label")]
    score_label: OnReady<Gd<Label>>,

    #[init(node = "CanvasLayer/Game Over")]
    game_over_label: OnReady<Gd<Label>>,

    base: Base<Node2D>,
}

const MONSTER_SPAWN_POSITION_Y_MAX: f64 = 115.0;
const MONSTER_SPAWN_POSITION_Y_MIN: f64 = 30.0;
const MONSTER_SPAWN_POSITION_X: f32 = 256.0;

#[godot_api]
impl GameManager {
    #[signal]
    fn game_start();
    #[signal]
    fn game_over();

    #[func]
    fn _on_timer_timeout(&self) {
        self.spawn_monster();
    }

    fn spawn_monster(&self) {
        godot_print!("生成怪物");
        let mut monster = self.monster_scene.instantiate_as::<Monster>();
        monster.set_position(Vector2::new(
            MONSTER_SPAWN_POSITION_X,
            randf_range(MONSTER_SPAWN_POSITION_Y_MIN, MONSTER_SPAWN_POSITION_Y_MAX) as real,
        ));
        self.base()
            .get_tree()
            .unwrap()
            .get_current_scene()
            .unwrap()
            .add_child(&monster);
    }

    fn update_wait_time(&mut self, delta: f64) {
        let mut timer = self.base().get_node_as::<Timer>("Timer");
        let new_wait_time = timer.get_wait_time() - 0.2 * delta; // 计算新的等待时间
        timer.set_wait_time(clampf(new_wait_time, 1.0, 3.0))
    }

    fn update_score_label(&mut self) {
        self.score_label
            .set_text(&format!("得分：{}", global().score));
    }

    pub fn show_game_over(&mut self) {
        self.game_over_label.clone().set_visible(true);
    }
}

#[godot_api]
impl INode2D for GameManager {
    fn process(&mut self, delta: f64) {
        self.update_wait_time(delta);
        self.update_score_label()
    }
}
