use crate::data::abilities::AbilityType;
use crate::data::hero::HeroType;
use crate::data::item::ItemType;
use crate::data::level::Level;
use crate::data::map::MapId;
use crate::plugins::hud::HudRedraw;
use crate::state::AppState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::{EnumIter, IntoEnumIterator};

pub struct GameplayEffectsPlugin;

impl Plugin for GameplayEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameMenuMain), on_enter_game_main_menu)
            .add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(Update, on_update)
            .add_event::<GameplayEffectEvent>()
            .add_event::<GameplayStatsRecalculatedEvent>()
            .insert_resource(GameplayEffectPluginState::default());
    }
}

/// Reset all the data when we enter the main menu
fn on_enter_game_main_menu(mut state: ResMut<GameplayEffectPluginState>) {
    *state = GameplayEffectPluginState::default();
}

/// Runs when the game is initializing
fn on_enter_game_init(mut state: ResMut<GameplayEffectPluginState>) {
    // If the menu was skipped, we do not have stats, so we call select hero here
    if state.player_effects.move_speed <= 0. {
        state
            .player_effects
            .equip_hero(HeroType::Pepe.get_gameplay_effects())
    }
}

fn on_update(
    time: Res<Time>,
    mut state: ResMut<GameplayEffectPluginState>,
    mut rx_gameplay: EventReader<GameplayEffectEvent>,
    mut tx_recalculated: EventWriter<GameplayStatsRecalculatedEvent>,
    mut tx_hud: EventWriter<HudRedraw>,
) {
    let mut debug_count = 0;
    for ev in rx_gameplay.iter() {
        debug_count += 1;
        match ev {
            GameplayEffectEvent::HeroSelected(hero) => {
                state.player_effects.equip_hero(hero.get_gameplay_effects())
            }
            GameplayEffectEvent::MapSelected(map) => {
                state.player_effects.equip_map(map.get_gameplay_effects())
            }
            GameplayEffectEvent::NFTEquipped(id, item) => state
                .player_effects
                .equip_nft(id, item.get_gameplay_effects()),
            GameplayEffectEvent::NFTUnEquipped(id) => state.player_effects.unequip_nft(id),
            GameplayEffectEvent::ItemEquipped(entity, item) => state
                .player_effects
                .equip_item(entity.clone(), item.get_gameplay_effects()),
            GameplayEffectEvent::ItemUnEquipped(entity) => {
                state.player_effects.unequip_item(entity.clone())
            }
            GameplayEffectEvent::LevelUp(level) => {
                state.player_effects.level_up(level.get_gameplay_effects())
            }
            GameplayEffectEvent::AbilityLevelUp(ability, lvl) => {
                state
                    .player_effects
                    .ability_level_up(ability.get_gameplay_effects(*lvl));

                // Only redraw the hud if the ability is new
                if *lvl == 1 {
                    tx_hud.send(HudRedraw::AbilitySlots);
                }
            }
        }
    }
    if debug_count > 0 {
        eprintln!("DEBUG EFFECTS {:?}", state.player_effects)
    }

    // Update gameplay tags & temporary effects
    state.player_tags.tick(time.delta_seconds());
    state.player_effects.update_temporary(time.delta_seconds());

    // Emit a recalculated event
    tx_recalculated.send(GameplayStatsRecalculatedEvent {});
}

#[derive(Debug, Event)]
pub enum GameplayEffectEvent {
    HeroSelected(HeroType),
    MapSelected(MapId),
    NFTEquipped(String, ItemType),
    NFTUnEquipped(String),
    ItemEquipped(Entity, ItemType),
    ItemUnEquipped(Entity),
    LevelUp(Level),
    AbilityLevelUp(AbilityType, u8),
}

#[derive(Event)]
pub struct GameplayStatsRecalculatedEvent {}

#[derive(Default, Resource)]
pub struct GameplayEffectPluginState {
    pub player_effects: GameplayEffectContainer,
    pub player_tags: GameplayTagContainer,
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

#[derive(Clone, Copy, Debug, EnumIter, Hash, Eq, PartialEq)]
pub enum GameplayStat {
    MovementSpeed,
    AttackRate,
    HealthCap,
    HealthRegen,
    SpawnRate,
    Damage,
    OrcaCount,
    OrcaSpeed,
    OrcaDamage,
    PickupDistance,
    WhaleInterval,
    WhaleDamage,
    WhaleArea,
    ShitcoinInterval,
    ShitcoinMunitions,
    ShitcoinDamage,
    RugPullInterval,
    RugPullSpeed,
    RugPullTTL,
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

    pub fn new_add(stat: GameplayStat, val: f64) -> Self {
        Self {
            stat,
            op: GameplayEffectOperation::Add,
            val,
        }
    }

    pub fn new_sub(stat: GameplayStat, val: f64) -> Self {
        Self {
            stat,
            op: GameplayEffectOperation::Sub,
            val,
        }
    }

    pub fn new_mul(stat: GameplayStat, val: f64) -> Self {
        Self {
            stat,
            op: GameplayEffectOperation::Mul,
            val,
        }
    }

    pub fn new_div(stat: GameplayStat, val: f64) -> Self {
        Self {
            stat,
            op: GameplayEffectOperation::Div,
            val,
        }
    }
}

struct TemporaryGameplayEffectStack {
    effects: Vec<GameplayEffect>,
    duration: f32,
}

impl TemporaryGameplayEffectStack {
    pub fn new(effects: Vec<GameplayEffect>, duration: f32) -> Self {
        Self { effects, duration }
    }
}

#[derive(Default)]
pub struct GameplayEffectContainer {
    /// The heroes stats serve as the base stat and must be absolute values
    hero: Vec<GameplayEffect>,
    /// The map can also modify the stats
    map: Vec<GameplayEffect>,
    /// The NFTs that were equipped
    nfts: Vec<(String, GameplayEffect)>,
    /// Each equipped item has effects
    items: Vec<(Entity, GameplayEffect)>,
    /// Effects gained for each player level as well as each ability upgrade
    player_progression: Vec<GameplayEffect>,
    /// Temporary effects from abilities or shrines
    temporary: Vec<TemporaryGameplayEffectStack>,

    /// Used for fast access of final values
    flat_packed: HashMap<GameplayStat, f64>,

    pub move_speed: f32,
    pub orca_speed: f32,
    pub orca_damage: f32,
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
    pub fn equip_nft(&mut self, id: &String, effects: Vec<GameplayEffect>) {
        // Check if already equipped
        if self.nfts.iter().any(|(e, _)| e == id) {
            eprintln!("NFT was already equipped {:?}", id);
            return;
        }

        // Add effects of this NFT
        self.nfts
            .extend(effects.into_iter().map(|effect| (id.clone(), effect)));

        self.recalculate();
    }

    /// Un-apply the effects of some NFT from this container
    pub fn unequip_nft(&mut self, id: &String) {
        self.nfts.retain(|&(ref e, _)| e != id);
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

    /// For every level up we may add additional effects
    pub fn level_up(&mut self, effects: Vec<GameplayEffect>) {
        self.player_progression.extend(effects.into_iter());

        self.recalculate();
    }

    /// Every ability can be level up. This adds additional effects
    pub fn ability_level_up(&mut self, effects: Vec<GameplayEffect>) {
        self.player_progression.extend(effects.into_iter());

        self.recalculate();
    }

    /// Adds a stack of temporary effects with the given duration
    pub fn apply_temporary(&mut self, effects: Vec<GameplayEffect>, duration: f32) {
        self.temporary
            .push(TemporaryGameplayEffectStack::new(effects, duration));
        self.recalculate();
    }

    /// Update temporary effects and remove if applicable
    pub fn update_temporary(&mut self, delta: f32) {
        let before = self.temporary.len();
        for stack in self.temporary.iter_mut() {
            stack.duration -= delta;
        }
        self.temporary.retain(|stack| stack.duration > 0.0);

        if self.temporary.len() != before {
            self.recalculate();
        }
    }

    /// This function will iterate through all effects and store final stat values for lookup operations
    fn recalculate(&mut self) {
        // Can use this for change detection
        let _before = std::mem::replace(
            &mut self.flat_packed,
            HashMap::from_iter(GameplayStat::iter().map(|stat| (stat, 0.))),
        );

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

        for (_itm, effect) in &self.nfts {
            self.flat_packed
                .entry(effect.stat)
                .and_modify(|e| effect.op.apply(e, effect.val));
        }

        for (_itm, effect) in &self.items {
            self.flat_packed
                .entry(effect.stat)
                .and_modify(|e| effect.op.apply(e, effect.val));
        }

        for effect in &self.player_progression {
            self.flat_packed
                .entry(effect.stat)
                .and_modify(|e| effect.op.apply(e, effect.val));
        }

        for stack in &self.temporary {
            for effect in &stack.effects {
                self.flat_packed
                    .entry(effect.stat)
                    .and_modify(|e| effect.op.apply(e, effect.val));
            }
        }

        // Cache additional values we might need every tick
        self.move_speed = *self
            .flat_packed
            .get(&GameplayStat::MovementSpeed)
            .unwrap_or(&100.) as f32;
    }

    /// Get some stat
    pub fn get_stat(&self, stat: GameplayStat) -> f64 {
        *self.flat_packed.get(&stat).unwrap_or(&0.)
    }
}

impl std::fmt::Debug for GameplayEffectContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameplayEffectContainer packed:\r\n{}", {
            let stat_repr: Vec<String> = GameplayStat::iter()
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

#[derive(Clone, Debug, PartialEq)]
pub enum GameplayTag {
    Attack,
    Dash,
}

impl GameplayTag {
    /// Returns a list of tags that prevent this one from being applied
    pub fn blocked_by(&self) -> Vec<GameplayTag> {
        vec![self.clone()]
    }
}

#[derive(Debug)]
struct GameplayTagWrapped {
    tag: GameplayTag,
    cooldown: f32,
}

type Cooldown = f32;

#[derive(Default)]
pub struct GameplayTagContainer {
    tags: Vec<GameplayTagWrapped>,
}

impl GameplayTagContainer {
    pub fn has_tag(&self, search_tag: &GameplayTag) -> bool {
        for tag in &self.tags {
            if tag.tag == *search_tag {
                return true;
            }
        }

        false
    }

    pub fn add_tag(&mut self, tag: GameplayTag, cooldown: Cooldown) -> bool {
        if self.has_tag(&tag) {
            //TODO use GameplayTag::blocked_by or some new invention
            return false;
        }

        self.tags.push(GameplayTagWrapped { tag, cooldown });

        true
    }

    pub fn tick(&mut self, delta: f32) {
        for tag in self.tags.iter_mut() {
            tag.cooldown -= delta;
        }
        self.tags.retain(|tag| tag.cooldown > 0.0);
    }
}
