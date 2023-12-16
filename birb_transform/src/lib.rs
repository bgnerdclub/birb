#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use birb::{App, TypedEntityID};
#[allow(clippy::wildcard_imports)]
use birb_maths::two::*;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard};

#[derive(Clone, Debug)]
pub struct Transform<T> {
    pub translation: Vector<T>,
    pub rotation: Rotor<T>,
    pub scale: Vector<T>,
    parent: Option<TypedEntityID<Transform<T>>>,
}

impl<T: 'static + Copy + Send + Sync> Transform<T> {
    pub const fn new(
        translation: Vector<T>,
        rotation: Rotor<T>,
        scale: Vector<T>,
        parent: Option<TypedEntityID<Self>>,
    ) -> Self {
        Self {
            translation,
            rotation,
            scale,
            parent,
        }
    }

    pub fn get_parent<'a>(&'a self, app: &'a App) -> Option<MappedRwLockReadGuard<'a, Self>> {
        app.get_entity(self.parent?)
    }

    pub fn get_parent_mut<'a>(&'a self, app: &'a App) -> Option<MappedRwLockWriteGuard<'a, Self>> {
        app.get_entity_mut(self.parent?)
    }
}

impl<T: 'static + Unit + Zero + Copy + Send + Sync> Transform<T> {
    #[must_use]
    pub fn identity(parent: Option<TypedEntityID<Self>>) -> Self {
        Self::new(Vector::zero(), Rotor::identity(), Vector::one(), parent)
    }
}
