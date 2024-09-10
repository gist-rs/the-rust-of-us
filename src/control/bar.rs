use bevy::prelude::*;
use bevy_stat_bars::*;
use std::marker::PhantomData;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCharacter;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stat<T>
where
    T: Component,
{
    pub value: f32,
    pub max: f32,
    #[reflect(ignore)]
    phantom: PhantomData<T>,
}

impl<T> Default for Stat<T>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            value: 10.0,
            max: 10.0,
            phantom: PhantomData,
        }
    }
}

impl<T> Stat<T>
where
    T: Component,
{
    pub fn new_full(value: f32) -> Self {
        assert!(0. < value);
        Self {
            value,
            max: value,
            ..Default::default()
        }
    }
}

impl<T> std::ops::AddAssign<f32> for Stat<T>
where
    T: Component,
{
    fn add_assign(&mut self, rhs: f32) {
        self.value = (self.value + rhs).clamp(0.0, self.max);
    }
}

impl<T> std::ops::SubAssign<f32> for Stat<T>
where
    T: Component,
{
    fn sub_assign(&mut self, rhs: f32) {
        self.value = (self.value - rhs).clamp(0.0, self.max);
    }
}

impl<T> StatbarObservable for Stat<T>
where
    T: Component,
{
    fn get_statbar_value(&self) -> f32 {
        self.value / self.max
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HealthValue;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MagicValue;

pub type Health = Stat<HealthValue>;
pub type Magic = Stat<MagicValue>;

pub fn adjust_stats(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hp_query: Query<&mut Health>,
    mut mp_query: Query<&mut Magic>,
) {
    let delta = 5.0 * time.delta_seconds();
    hp_query.iter_mut().for_each(|mut hp| {
        if keyboard_input.pressed(KeyCode::KeyA) {
            *hp -= delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            *hp += delta;
        }
    });
    mp_query.iter_mut().for_each(|mut mp| {
        if keyboard_input.pressed(KeyCode::KeyQ) {
            *mp -= delta;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            *mp += delta;
        }
    });
}
