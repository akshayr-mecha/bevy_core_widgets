use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    input::{keyboard::KeyboardInput, ButtonState},
    input_focus::FocusedInput,
    prelude::*,
};

use crate::{ButtonClicked, CoreRadio, InteractionDisabled};

/// Headless widget implementation for a "radio group". This component is used to group multiple
/// `CoreRadio` components together, allowing them to behave as a single unit. It implements
/// the mutual exclusion and tab navigation logic for radio buttons.
///
/// The `CoreRadioGroup` component does not have any state itself, and makes no assumptions about
/// what, if any, value is associated with each radio button. Instead, it relies on the `CoreRadio`
/// components to trigger a `ButtonPress` event, and (after handling the mutual exclusion logic)
/// this event is propagated to the application.
///
/// TODO: This currently does not work with radio buttons that use an `on_click` handler.
#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::RadioGroup)))]
pub struct CoreRadioGroup;

fn radio_group_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_group: Query<&Children, With<CoreRadioGroup>>,
    mut q_radio: Query<(&mut CoreRadio, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok(group_children) = q_group.get(trigger.target()) {
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
                    Ok((radio, false)) => Some((child_id, radio.checked, radio.on_click)),
                    Ok((_, true)) => None,
                    Err(_) => None,
                })
                .collect::<Vec<_>>();
            if radio_children.is_empty() {
                return; // No enabled radio buttons in the group
            }
            let current_index = radio_children
                .iter()
                .position(|(_, checked, _)| *checked)
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

            // Uncheck the current radio button
            if current_index != usize::MAX {
                q_radio
                    .get_mut(radio_children[current_index].0)
                    .expect("Current radio button should exist")
                    .0
                    .checked = false;
            }

            // Check the next radio button
            let (next_id, _, on_click) = radio_children[next_index];
            q_radio
                .get_mut(next_id)
                .expect("Next radio button should exist")
                .0
                .checked = true;

            // Trigger the on_click event for the newly checked radio button
            if let Some(on_click) = on_click {
                commands.run_system(on_click);
            } else {
                commands.trigger_targets(ButtonClicked, trigger.target());
            }
        }
    }
}

fn radio_group_on_button_click(
    trigger: Trigger<ButtonClicked>,
    q_group: Query<&Children, With<CoreRadioGroup>>,
    mut q_radio: Query<(&mut CoreRadio, &ChildOf, Has<InteractionDisabled>)>,
) {
    // Find the radio button that was clicked.
    let Ok((_, child_of, _)) = q_radio.get(trigger.target()) else {
        return;
    };

    // Find the parent CoreRadioGroup of the clicked radio button.
    let Ok(group_children) = q_group.get(child_of.parent) else {
        // The radio button's parent is not a CoreRadioGroup, ignore the click
        return;
    };

    // Get all the radio group children.
    let radio_children = group_children
        .iter()
        .filter_map(|child_id| match q_radio.get(child_id) {
            Ok((radio, _, false)) => Some((child_id, radio.checked, radio.on_click)),
            Ok((_, _, true)) => None,
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    if radio_children.is_empty() {
        return; // No enabled radio buttons in the group
    }

    let current_index = radio_children
        .iter()
        .position(|(_, checked, _)| *checked)
        .unwrap_or(usize::MAX); // Default to invalid index if none are checked

    let next_index = radio_children
        .iter()
        .position(|(id, _, _)| *id == trigger.target())
        .unwrap_or(current_index); // Default to the current index if not found

    if current_index == next_index {
        // If the next index is the same as the current, do nothing
        return;
    }

    // Uncheck the current radio button
    if current_index != usize::MAX {
        q_radio
            .get_mut(radio_children[current_index].0)
            .expect("Current radio button should exist")
            .0
            .checked = false;
    }

    // Check the next radio button
    let (next_id, _, _) = radio_children[next_index];
    q_radio
        .get_mut(next_id)
        .expect("Next radio button should exist")
        .0
        .checked = true;

    // Trigger the on_click event for the newly checked radio button
    // if let Some(on_click) = on_click {
    //     commands.run_system(on_click);
    // } else {
    //     commands.trigger_targets(ButtonClicked, trigger.target());
    // }
}

pub struct CoreRadioGroupPlugin;

impl Plugin for CoreRadioGroupPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(radio_group_on_key_input)
            .add_observer(radio_group_on_button_click);
    }
}
