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
    fn tick(&mut self, app: &mut App) {}
}

pub struct App {
    entities: HashMap<TypeId, VecAny>,
    modules: Vec<Rc<RefCell<dyn Module>>>,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            modules: Vec::new(),
            running: true,
        }
    }

    pub fn register_entity<T: 'static + Sync + Send>(&mut self, entity: T) {
        let id = TypeId::of::<T>();
        match self.entities.get_mut(&id) {
            Some(entities) => entities.downcast_mut().unwrap().push(entity),
            None => {
                self.entities.insert(id, vec![entity].into());
            }
        }
    }

    pub fn register_module<T: 'static>(&mut self, module: T)
    where
        T: Module,
    {
        self.modules.push(Rc::new(RefCell::new(module)));
    }

    pub fn get_entity<'a, T: 'static + Send + Sync>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        if let Some(entities) = self.entities.get(&TypeId::of::<T>()) {
            Box::new(entities.downcast_slice().unwrap().iter())
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn get_entity_mut<'a, T: 'static + Send + Sync>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut T> + 'a> {
        if let Some(entities) = self.entities.get_mut(&TypeId::of::<T>()) {
            Box::new(entities.downcast_slice_mut().unwrap().iter_mut())
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn get_module<T: 'static>(&self) -> Option<Ref<'_, T>> {
        self.modules.iter().find_map(|system| {
            Ref::filter_map(system.try_borrow().ok()?, |x| {
                (x as &dyn Any).downcast_ref::<T>()
            })
            .ok()
        })
    }

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
            self.tick()
        }
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
