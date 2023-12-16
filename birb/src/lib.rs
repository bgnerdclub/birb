#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(downcast_unchecked)]

use list_any::VecAny;
use parking_lot::{
    lock_api::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard},
    RawRwLock, RwLock,
};
use rayon::prelude::*;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub trait Module: Any + Debug + Send + Sync {
    fn tick(&mut self, _: &App) {}
}

pub trait MainThreadModule: Any + Debug {
    fn tick(&mut self, _: &MainThreadApp) {}
}

#[derive(Default)]
pub struct MainThreadApp {
    app: App,
    modules: HashMap<TypeId, Box<RwLock<dyn MainThreadModule>>>,
}

impl MainThreadApp {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_main_thread_module<T: 'static + MainThreadModule>(&mut self, module: T) {
        self.modules
            .insert(TypeId::of::<T>(), Box::new(RwLock::new(module)));
    }

    pub fn register<T, F: FnOnce(&mut T)>(&mut self, func: F)
    where
        for<'a> &'a mut T: From<&'a mut Self>,
    {
        func(self.into());
    }

    pub fn tick(&self) {
        self.modules
            .iter()
            .for_each(|(_, module)| module.write().tick(self));
        self.app.tick();
    }

    pub fn run(&self) {
        *self.app.running.write() = true;
        while *self.app.running.read() {
            self.tick();
        }
    }
}

impl Deref for MainThreadApp {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for MainThreadApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<'a> From<&'a mut MainThreadApp> for &'a mut App {
    fn from(value: &'a mut MainThreadApp) -> Self {
        value
    }
}

#[derive(Default)]
pub struct App {
    entities: HashMap<TypeId, RwLock<VecAny>>,
    modules: HashMap<TypeId, Box<RwLock<dyn Module>>>,
    running: RwLock<bool>,
}

impl App {
    #[must_use]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> MainThreadApp {
        MainThreadApp::default()
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    pub fn register_entity<T: 'static + Sync + Send>(&mut self, entity: T) {
        let id = TypeId::of::<T>();
        match self.entities.get_mut(&id) {
            Some(ents) => ents.write().downcast_mut().unwrap().push(entity),
            None => {
                self.entities.insert(id, RwLock::new(vec![entity].into()));
            }
        }
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    pub fn register_entities<T: 'static + Sync + Send + Clone>(&mut self, entities: &[T]) {
        let id = TypeId::of::<T>();
        match self.entities.get_mut(&id) {
            Some(ents) => ents
                .write()
                .downcast_mut()
                .unwrap()
                .extend_from_slice(entities),
            None => {
                self.entities
                    .insert(id, RwLock::new(entities.to_vec().into()));
            }
        }
    }

    pub fn register_module<T: 'static + Module>(&mut self, module: T) {
        self.modules
            .insert(TypeId::of::<T>(), Box::new(RwLock::new(module)));
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    #[must_use]
    pub fn get_entity<T: 'static + Send + Sync>(
        &self,
    ) -> Option<MappedRwLockReadGuard<'_, RawRwLock, [T]>> {
        self.entities.get(&TypeId::of::<T>()).map(|entities| {
            RwLockReadGuard::map(entities.read(), |entities| {
                entities.downcast_slice().unwrap()
            })
        })
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    #[must_use]
    pub fn get_entity_mut<T: 'static + Send + Sync>(
        &self,
    ) -> Option<MappedRwLockWriteGuard<'_, RawRwLock, [T]>> {
        self.entities.get(&TypeId::of::<T>()).map(|entities| {
            RwLockWriteGuard::map(entities.write(), |entities| {
                entities.downcast_slice_mut().unwrap()
            })
        })
    }

    /// # Panics
    /// Panics if modules map has a mismatch between the type stated in the key and the type
    /// stated in the value
    #[must_use]
    pub fn get_module<T: 'static>(&self) -> Option<MappedRwLockReadGuard<'_, RawRwLock, T>> {
        self.modules.get(&TypeId::of::<T>()).map(|module| {
            RwLockReadGuard::map(module.read(), |module| {
                (module as &dyn Any).downcast_ref().unwrap()
            })
        })
    }

    /// # Panics
    /// Panics if modules map has a mismatch between the type stated in the key and the type
    /// stated in the value
    #[must_use]
    pub fn get_module_mut<T: 'static>(&self) -> Option<MappedRwLockWriteGuard<'_, RawRwLock, T>> {
        self.modules.get(&TypeId::of::<T>()).map(|module| {
            RwLockWriteGuard::map(module.write(), |module| {
                (module as &mut dyn Any).downcast_mut().unwrap()
            })
        })
    }

    pub fn tick(&self) {
        self.modules.par_iter().for_each(|(_, module)| {
            module.write().tick(self);
        });
    }

    pub fn exit(&self) {
        *self.running.write() = false;
    }
}
