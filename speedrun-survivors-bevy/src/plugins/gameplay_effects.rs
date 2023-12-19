use crate::data::hero::HeroType;
use crate::data::map::MapId;
use crate::state::AppState;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct GameplayEffectsPlugin;

impl Plugin for GameplayEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(Update, (on_update))
            .add_event::<GameplayEffectEvent>()
            .insert_resource(GameplayEffectPluginState::default());
    }
}

fn on_enter_game_running(mut state: ResMut<GameplayEffectPluginState>) {}

fn on_exit_game_running(mut state: ResMut<GameplayEffectPluginState>) {}

fn on_update(
    mut state: ResMut<GameplayEffectPluginState>,
    mut event_reader: EventReader<GameplayEffectEvent>,
) {
    let mut debug_count = 0;
    for ev in event_reader.iter() {
        match ev {
            GameplayEffectEvent::HeroSelected(hero) => {
                debug_count += 1;
                state.player.equip_hero(hero.get_gameplay_effects())
            }
            GameplayEffectEvent::MapSelected(map) => {
                debug_count += 1;
                state.player.equip_map(map.get_gameplay_effects())
            }
            _ => {
                eprintln!(
                    "ERROR UNHANDLED: Gameplay Effect Plugin read event {:?}",
                    ev
                )
            }
        }
    }
    if debug_count > 0 {
        eprintln!("CALC DEBUG {:?}", state.player)
    }
}

#[derive(Debug, Event)]
pub enum GameplayEffectEvent {
    HeroSelected(HeroType),
    MapSelected(MapId),
    NFTEquipped(String, u64),
    NFTUnEquipped(String, u64),
    ItemEquipped(Entity, u64),
    ItemUnEquipped(Entity, u64),
    LevelUp(u64),
}

#[derive(Default, Resource)]
pub struct GameplayEffectPluginState {
    pub player: GameplayEffectContainer,
}

#[derive(PartialEq)]
pub enum GameplayEffectOperation {
    Abs,
    Add,
    Sub,
    Mul,
    Div,
}

impl GameplayEffectOperation {
    fn apply(&self, operand: &mut f64, operator: f64) {
        match &self {
            GameplayEffectOperation::Abs => *operand = operator,
            GameplayEffectOperation::Add => *operand += operator,
            GameplayEffectOperation::Sub => *operand -= operator,
            GameplayEffectOperation::Mul => *operand *= operator,
            GameplayEffectOperation::Div => *operand /= operator,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameplayStat {
    MovementSpeed,
    AttackRate,
    Health,
    HealthRegen,
    SpawnRate,
    Damage,
}

impl GameplayStat {
    pub fn into_iter() -> core::array::IntoIter<GameplayStat, 6> {
        [
            Self::MovementSpeed,
            Self::AttackRate,
            Self::Health,
            Self::HealthRegen,
            Self::SpawnRate,
            Self::Damage,
        ]
        .into_iter()
    }
}

pub struct GameplayEffect {
    pub stat: GameplayStat,
    pub op: GameplayEffectOperation,
    pub val: f64,
}

impl GameplayEffect {
    pub fn new(stat: GameplayStat, op: GameplayEffectOperation, val: f64) -> Self {
        Self { stat, op, val }
    }

    pub fn new_abs(stat: GameplayStat, val: f64) -> Self {
        Self {
            stat,
            op: GameplayEffectOperation::Abs,
            val,
        }
    }
}

#[derive(Default)]
pub struct GameplayEffectContainer {
    /// The heroes stats serve as the base stat and must be absolute values
    pub hero: Vec<GameplayEffect>,
    /// The map can also modify the stats
    pub map: Vec<GameplayEffect>,
    /// The NFTs that were equipped
    pub nfts: Vec<(String, GameplayEffect)>, //TODO think this though.. could be good to display stats in the mnu screen - then we need this here and messages from the UI
    /// Each equipped item has effects
    pub items: Vec<(Entity, GameplayEffect)>,
    /// With each level up, additional effects can be added
    pub levels: Vec<GameplayEffect>, //TODO implement below

    /// Used for fast access of final values
    flat_packed: HashMap<GameplayStat, f64>,
}

impl GameplayEffectContainer {
    /// Apply the hero base stats into this container
    pub fn equip_hero(&mut self, effects: Vec<GameplayEffect>) {
        self.hero = effects;
        self.recalculate();
    }

    /// Apply the map effects into this container
    pub fn equip_map(&mut self, effects: Vec<GameplayEffect>) {
        self.map = effects;
        self.recalculate();
    }

    /// Apply the effects of an item into this container
    pub fn equip_item(&mut self, entity: Entity, effects: Vec<GameplayEffect>) {
        // Check if already equipped
        if self.items.iter().any(|(e, _)| *e == entity) {
            eprintln!("Item was already equipped {:?}", entity);
            return;
        }

        // Add effects of this item
        self.items
            .extend(effects.into_iter().map(|effect| (entity, effect)));

        self.recalculate();
    }

    /// Un-apply the effects of some item from this container
    pub fn unequip_item(&mut self, entity: Entity) {
        self.items.retain(|&(e, _)| e != entity);
        self.recalculate();
    }

    /// This function will iterate through all effects and store final stat values for lookup operations
    fn recalculate(&mut self) {
        self.reset_flat_packed();

        for effect in &self.hero {
            if effect.op != GameplayEffectOperation::Abs {
                panic!("GameplayStat error: Heroes Gameplay stats must only use ABS operator as they are the base for all calculations");
            }

            self.flat_packed
                .entry(effect.stat)
                .and_modify(|e| effect.op.apply(e, effect.val));
        }

        // if !flat.are_valid_base_stats() {
        //     panic!("GameplayStat error: Hero stats are not properly configured");
        // }

        for effect in &self.map {
            self.flat_packed
                .entry(effect.stat)
                .and_modify(|e| effect.op.apply(e, effect.val));
        }

        for (_itm, effect) in &self.items {
            self.flat_packed
                .entry(effect.stat)
                .and_modify(|e| effect.op.apply(e, effect.val));
        }
    }

    /// Delete the cached values to recalculate from scratch
    fn reset_flat_packed(&mut self) {
        self.flat_packed = HashMap::from_iter(GameplayStat::into_iter().map(|stat| (stat, 0.)));
    }
}

impl std::fmt::Debug for GameplayEffectContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameplayEffectContainer packed: {}", {
            let stat_repr: Vec<String> = GameplayStat::into_iter()
                .map(|stat| {
                    format!(
                        " - {:?} -> {}",
                        stat,
                        self.flat_packed.get(&stat).unwrap_or(&0.)
                    )
                })
                .collect();

            stat_repr.join("\r\n")
        })
    }
}
