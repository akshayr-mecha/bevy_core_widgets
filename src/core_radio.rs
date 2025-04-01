use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, system::SystemId, world::DeferredWorld},
    input_focus::{InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::{ButtonClicked, InteractionDisabled};

/// Headless widget implementation for radio buttons. Note that this does not handle the mutual
/// exclusion of radio buttons in the same group; that should be handled by the parent component.
/// (This is relatively easy if the parent is a reactive widget.)
///
/// The `on_click` field is a system that will be run when the button is clicked, or when the
/// `Enter` or `Space` key is pressed while the radio button is focused. If the `on_click` field is
/// `None`, the radio button will emit a `ButtonClicked` event when clicked.
///
/// According to the WAI-ARIA best practices document, radio buttons should not be focusable,
/// but rather the enclosing group should be focusable.
/// See https://www.w3.org/WAI/ARIA/apg/patterns/radio/
#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::RadioButton)))]
#[component(on_add = on_add_radio, on_replace = on_add_radio)]
pub struct CoreRadio {
    pub checked: bool,
    pub on_click: Option<SystemId>,
}

// Hook to set the a11y "checked" state when the radio is added or updated.
fn on_add_radio(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let radio = entt.get::<CoreRadio>().unwrap();
    let checked = radio.checked;
    let mut accessibility = entt.get_mut::<AccessibilityNode>().unwrap();
    accessibility.set_toggled(match checked {
        true => accesskit::Toggled::True,
        false => accesskit::Toggled::False,
    });
}

fn radio_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreRadio, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((radio, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if radio.checked || disabled {
            // If the radio is already checked, or disabled, we do nothing.
            return;
        }
        if let Some(on_click) = radio.on_click {
            commands.run_system(on_click);
        } else {
            commands.trigger_targets(ButtonClicked, trigger.target());
        }
    }
}

pub struct CoreRadioPlugin;

impl Plugin for CoreRadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(radio_on_pointer_click);
    }
}
