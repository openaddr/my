use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Start,
    Over,
}

pub struct Game {
    pub state: GameState,
    pub score: i64,
}

static GAME: LazyLock<RwLock<Game>> = LazyLock::new(|| {
    RwLock::new(Game {
        state: GameState::Start,
        score: 0,
    })
});

pub fn global() -> RwLockReadGuard<'static, Game> {
    GAME.read()
        .expect("Failed to acquire lock, another thread panicked while holding it")
}
pub fn global_mut() -> RwLockWriteGuard<'static, Game> {
    GAME.write()
        .expect("Failed to acquire lock, another thread panicked while holding it")
}
