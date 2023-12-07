#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(downcast_unchecked)]
#![feature(trait_upcasting)]

use list_any::VecAny;

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
};

pub trait Module: Any + Debug {
    fn tick(&mut self, _: &mut App) {}
}

#[derive(Default)]
pub struct App {
    entities: HashMap<TypeId, VecAny>,
    modules: Vec<Rc<RefCell<dyn Module>>>,
    running: bool,
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    pub fn register_entity<T: 'static + Sync + Send>(&mut self, entity: T) {
        let id = TypeId::of::<T>();
        match self.entities.get_mut(&id) {
            Some(ents) => ents.downcast_mut().unwrap().push(entity),
            None => {
                self.entities.insert(id, vec![entity].into());
            }
        }
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    pub fn register_entities<T: 'static + Sync + Send + Clone>(&mut self, entities: &[T]) {
        let id = TypeId::of::<T>();
        match self.entities.get_mut(&id) {
            Some(ents) => ents.downcast_mut().unwrap().extend_from_slice(entities),
            None => {
                self.entities.insert(id, entities.to_vec().into());
            }
        }
    }

    pub fn register_module<T: 'static + Module>(&mut self, module: T) {
        self.modules.push(Rc::new(RefCell::new(module)));
    }

    pub fn register<F: FnOnce(&mut Self)>(&mut self, func: F) {
        func(self);
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    #[must_use]
    pub fn get_entity<T: 'static + Send + Sync>(&self) -> &[T] {
        self.entities.get(&TypeId::of::<T>()).map_or_else(
            || &[] as &[T],
            |entities| entities.downcast_slice().unwrap(),
        )
    }

    /// # Panics
    /// Panics if entities map has a mismatch between the type stated in the key and the type
    /// stated in the value
    #[must_use]
    pub fn get_entity_mut<T: 'static + Send + Sync>(&mut self) -> &mut [T] {
        self.entities.get_mut(&TypeId::of::<T>()).map_or_else(
            || &mut [] as &mut [T],
            |entities| entities.downcast_slice_mut().unwrap(),
        )
    }

    #[must_use]
    pub fn get_module<T: 'static>(&self) -> Option<Ref<'_, T>> {
        self.modules.iter().find_map(|system| {
            Ref::filter_map(system.try_borrow().ok()?, |x| {
                (x as &dyn Any).downcast_ref::<T>()
            })
            .ok()
        })
    }

    #[must_use]
    pub fn get_module_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
        self.modules.iter().find_map(|system| {
            RefMut::filter_map(system.try_borrow_mut().ok()?, |x| {
                (x as &mut dyn Any).downcast_mut::<T>()
            })
            .ok()
        })
    }

    pub fn tick(&mut self) {
        for module in self.modules.clone() {
            module.borrow_mut().tick(self);
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.tick();
        }
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
