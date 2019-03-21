use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, ComponentBox, Entity, SharedComponentBox, World};

use crate::application::Tree;

use crate::event::EventHandler;

use crate::{Layout, RenderObject, properties::*};

pub use self::build_context::*;
pub use self::context::*;
pub use self::message::*;
pub use self::state::*;
pub use self::template::*;
pub use self::widget_container::*;

mod build_context;
mod context;
mod message;
mod state;
mod template;
mod widget_container;

/// Adds the given `pseudo_class` to the css selector of the given `widget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.0.pseudo_classes.insert(String::from(pseudo_class));
        selector.0.set_dirty(true);
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `widget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.0.pseudo_classes.remove(pseudo_class);
        selector.0.set_dirty(true);
    }
}

/// The `Widget` trait is used to define a new widget.
pub trait Widget: Template {
    /// Creates a new widget.
    fn create() -> Self;

    /// Builds the widget and returns the template of the widget.
    fn build(self, context: &mut BuildContext) -> Entity;

    /// Inerts a new event handler.
    fn insert_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self;


    fn state(&self) -> Option<Rc<State>> {
        None
    }

    fn child(self, child: Entity) -> Self;
}

/// Used to create an entity for a widget with its properties as components.
pub struct BuildContext<'a> {
    world: &'a mut World<Tree>,
    render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl<'a> BuildContext<'a> {
    /// Creates a new `BuildContext`.
    pub fn new(
        world: &'a mut World<Tree>,
        render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    ) -> Self {
        BuildContext {
            world,
            render_objects,
            layouts,
            handlers,
            states,
        }
    }

    /// Creates a new entity.
    pub fn create_entity(&mut self) -> Entity {
        self.world.create_entity().build()
    }

    /// Appends a child to a parent.
    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        self.world
            .entity_container()
            .append_child(parent, child)
            .unwrap();
    }

    /// Registers a property as component.
    pub fn register_property<P: Component>(&mut self, widget: Entity, property: P) {
        self.world
            .entity_component_manager()
            .register_component(widget, property);
    }

    /// Registers a property box as component.
    pub fn register_property_box(&mut self, widget: Entity, property: ComponentBox) {
        self.world
            .entity_component_manager()
            .register_component_box(widget, property);
    }

    /// Registers a shared property.
    pub fn register_shared_property<P: Component>(&mut self, target: Entity, source: Entity) {
        self.world
            .entity_component_manager()
            .register_shared_component::<P>(target, source);
    }

    /// Registers a shared component box.
    pub fn register_property_shared_box(&mut self, widget: Entity, property: SharedComponentBox) {
        self.world
            .entity_component_manager()
            .register_shared_component_box(widget, property);
    }

    /// Registers a state with a widget.
    pub fn register_state(&self, widget: Entity, state: Rc<State>) {
        self.states.borrow_mut().insert(widget, state);
    }

    /// Registers a render object with a widget.
    pub fn register_render_object(&self, widget: Entity, render_object: Box<dyn RenderObject>) {
        self.render_objects
            .borrow_mut()
            .insert(widget, render_object);
    }

    /// Registers a event handler with a widget.
    pub fn register_handler(&self, widget: Entity, handler: Rc<dyn EventHandler>) {
        if !self.handlers.borrow().contains_key(&widget) {
            self.handlers.borrow_mut().insert(widget, vec![]);
        }

        self.handlers.borrow_mut().get_mut(&widget).unwrap().push(handler);
    }

    /// Registers a layout object with a widget.
    pub fn register_layout(&self, widget: Entity, layout: Box<dyn Layout>) {
        self.layouts.borrow_mut().insert(widget, layout);
    }
}