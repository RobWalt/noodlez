use bevy::prelude::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .register_type::<AppState>()
            .add_systems(Update, Self::change_app_state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States, Reflect)]
pub enum AppState {
    #[default]
    Select,
    SpawnNodes,
    SpawnEdges,
}

impl AppState {
    pub fn from_keys(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::KeyN => Some(Self::SpawnNodes),
            KeyCode::KeyE => Some(Self::SpawnEdges),
            KeyCode::KeyS => Some(Self::Select),
            _ => None,
        }
    }
}

impl AppStatePlugin {
    fn change_app_state(
        keys: Res<ButtonInput<KeyCode>>,
        state: Res<State<AppState>>,
        mut next_state: ResMut<NextState<AppState>>,
    ) {
        if let Some(next) = keys
            .get_just_pressed()
            .filter_map(|key| AppState::from_keys(*key))
            .filter(|new_state| state.get() != new_state)
            .next()
        {
            next_state.set(next);
        }
    }
}
