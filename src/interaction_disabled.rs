use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::Component,
};

/// A marker component to indicate that a widget is disabled and should be "grayed out".
/// This is used to prevent user interaction with the widget. It should not, however, prevent
/// the widget from being updated or rendered, or from acquiring keyboard focus.
///
/// For apps which support a11y: if a widget (such as a slider) contains multiple entities,
/// the `InteractionDisabled` component should be added to the root entity of the widget - the
/// same entity that contains the `AccessibilityNode` component. This will ensure that
/// the a11y tree is updated correctly.
#[derive(Component, Debug, Clone, Copy)]
#[component(on_add = on_add_disabled, on_remove = on_remove_disabled)]
pub struct InteractionDisabled;

// Hook to set the a11y "disabled" state when the widget is disabled.
fn on_add_disabled(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_disabled();
    }
}

// Hook to remove the a11y "disabled" state when the widget is enabled.
fn on_remove_disabled(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.clear_disabled();
    }
}
