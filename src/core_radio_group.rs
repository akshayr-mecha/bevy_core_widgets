use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::{keyboard::KeyboardInput, ButtonState},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::{ButtonClicked, Checked, CoreRadio, InteractionDisabled, ValueChange};

/// Headless widget implementation for a "radio group". This component is used to group multiple
/// `CoreRadio` components together, allowing them to behave as a single unit. It implements
/// the tab navigation logic and keyboard shortcuts for radio buttons.
///
/// The `CoreRadioGroup` component does not have any state itself, and makes no assumptions about
/// what, if any, value is associated with each radio button. Instead, it relies on the `CoreRadio`
/// components to trigger a `ButtonPress` event, and tranforms this into a `ValueChange` event
/// which contains the id of the selected button. The app can then derive the selected value
/// from this using app-specific data.
#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::RadioGroup)))]
pub struct CoreRadioGroup {
    pub on_change: Option<SystemId<In<Entity>>>,
}

fn radio_group_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_group: Query<(&CoreRadioGroup, &Children)>,
    q_radio: Query<(&Checked, Has<InteractionDisabled>), With<CoreRadio>>,
    mut commands: Commands,
) {
    if let Ok((CoreRadioGroup { on_change }, group_children)) = q_group.get(trigger.target()) {
        let event = &trigger.event().input;
        if event.state == ButtonState::Pressed
            && !event.repeat
            && matches!(
                event.key_code,
                KeyCode::ArrowUp
                    | KeyCode::ArrowDown
                    | KeyCode::ArrowLeft
                    | KeyCode::ArrowRight
                    | KeyCode::Home
                    | KeyCode::End
            )
        {
            let key_code = event.key_code;
            trigger.propagate(false);
            let radio_children = group_children
                .iter()
                .filter_map(|child_id| match q_radio.get(child_id) {
                    Ok((checked, false)) => Some((child_id, checked.0)),
                    Ok((_, true)) => None,
                    Err(_) => None,
                })
                .collect::<Vec<_>>();
            if radio_children.is_empty() {
                return; // No enabled radio buttons in the group
            }
            let current_index = radio_children
                .iter()
                .position(|(_, checked)| *checked)
                .unwrap_or(usize::MAX); // Default to invalid index if none are checked

            let next_index = match key_code {
                KeyCode::ArrowUp | KeyCode::ArrowLeft => {
                    // Navigate to the previous radio button in the group
                    if current_index == 0 {
                        // If we're at the first one, wrap around to the last
                        radio_children.len() - 1
                    } else {
                        // Move to the previous one
                        current_index - 1
                    }
                }
                KeyCode::ArrowDown | KeyCode::ArrowRight => {
                    // Navigate to the next radio button in the group
                    if current_index >= radio_children.len() - 1 {
                        // If we're at the last one, wrap around to the first
                        0
                    } else {
                        // Move to the next one
                        current_index + 1
                    }
                }
                KeyCode::Home => {
                    // Navigate to the first radio button in the group
                    0
                }
                KeyCode::End => {
                    // Navigate to the last radio button in the group
                    radio_children.len() - 1
                }
                _ => {
                    return;
                }
            };

            if current_index == next_index {
                // If the next index is the same as the current, do nothing
                return;
            }

            let (next_id, _) = radio_children[next_index];

            // Trigger the on_change event for the newly checked radio button
            if let Some(on_change) = on_change {
                commands.run_system_with(*on_change, next_id);
            } else {
                commands.trigger_targets(ValueChange(next_id), trigger.target());
            }
        }
    }
}

fn radio_group_on_button_click(
    mut trigger: Trigger<ButtonClicked>,
    q_group: Query<(&CoreRadioGroup, &Children)>,
    q_radio: Query<(&Checked, &ChildOf, Has<InteractionDisabled>), With<CoreRadio>>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    let radio_id = trigger.target();

    // Find the radio button that was clicked.
    let Ok((_, child_of, _)) = q_radio.get(radio_id) else {
        return;
    };

    // Find the parent CoreRadioGroup of the clicked radio button.
    let group_id = child_of.parent;
    let Ok((CoreRadioGroup { on_change }, group_children)) = q_group.get(group_id) else {
        // The radio button's parent is not a CoreRadioGroup, ignore the click
        warn!("Radio button clicked without a valid CoreRadioGroup parent");
        return;
    };

    // Set focus to group and hide focus ring
    focus.0 = Some(group_id);
    focus_visible.0 = false;

    // Get all the radio group children.
    let radio_children = group_children
        .iter()
        .filter_map(|child_id| match q_radio.get(child_id) {
            Ok((checked, _, false)) => Some((child_id, checked.0)),
            Ok((_, _, true)) => None,
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    if radio_children.is_empty() {
        return; // No enabled radio buttons in the group
    }

    trigger.propagate(false);
    let current_radio = radio_children
        .iter()
        .find(|(_, checked)| *checked)
        .map(|(id, _)| *id);

    if current_radio == Some(radio_id) {
        // If they clicked the currently checked radio button, do nothing
        return;
    }

    // Trigger the on_change event for the newly checked radio button
    if let Some(on_change) = on_change {
        commands.run_system_with(*on_change, radio_id);
    } else {
        commands.trigger_targets(ValueChange(radio_id), group_id);
    }
}

pub struct CoreRadioGroupPlugin;

impl Plugin for CoreRadioGroupPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(radio_group_on_key_input)
            .add_observer(radio_group_on_button_click);
    }
}
