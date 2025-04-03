use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::{keyboard::KeyboardInput, ButtonState},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::{interaction_states::Checked, InteractionDisabled, ValueChange};

/// Headless widget implementation for checkboxes. The `checked` represents the current state
/// of the checkbox. The `on_change` field is a system that will be run when the checkbox
/// is clicked, or when the Enter or Space key is pressed while the checkbox is focused.
/// If the `on_change` field is `None`, the checkbox will emit a `ValueChange` event instead.
#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::CheckBox)), Checked)]
pub struct CoreCheckbox {
    pub on_change: Option<SystemId<In<bool>>>,
}

fn checkbox_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<(&CoreCheckbox, &Checked, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((checkbox, checked, disabled)) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if !disabled
            && event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
        {
            let is_checked = checked.0;
            trigger.propagate(false);
            if let Some(on_change) = checkbox.on_change {
                commands.run_system_with(on_change, !is_checked);
            } else {
                commands.trigger_targets(ValueChange(!is_checked), trigger.target());
            }
        }
    }
}

fn checkbox_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreCheckbox, &Checked, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((checkbox, checked, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if !disabled {
            let is_checked = checked.0;
            if let Some(on_change) = checkbox.on_change {
                commands.run_system_with(on_change, !is_checked);
            } else {
                commands.trigger_targets(ValueChange(!is_checked), trigger.target());
            }
        }
    }
}

pub struct CoreCheckboxPlugin;

impl Plugin for CoreCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(checkbox_on_key_input)
            .add_observer(checkbox_on_pointer_click);
    }
}
