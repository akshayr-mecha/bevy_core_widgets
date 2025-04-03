use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    input_focus::{InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::{interaction_states::Checked, ButtonClicked, InteractionDisabled};

/// Headless widget implementation for radio buttons. Note that this does not handle the mutual
/// exclusion of radio buttons in the same group; that should be handled by the parent component.
/// (This is relatively easy if the parent is a reactive widget.)
///
/// The widget emits a `ButtonClick` event when clicked, or when the `Enter` or `Space` key is
/// pressed while the radio button is focused. This event is normally handled by the parent
/// `CoreRadioGroup` component.
///
/// According to the WAI-ARIA best practices document, radio buttons should not be focusable,
/// but rather the enclosing group should be focusable.
/// See https://www.w3.org/WAI/ARIA/apg/patterns/radio/
#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::RadioButton)), Checked)]
pub struct CoreRadio;

fn radio_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&Checked, Has<InteractionDisabled>), With<CoreRadio>>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((checked, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if checked.0 || disabled {
            // If the radio is already checked, or disabled, we do nothing.
            return;
        }
        commands.trigger_targets(ButtonClicked, trigger.target());
    }
}

pub struct CoreRadioPlugin;

impl Plugin for CoreRadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(radio_on_pointer_click);
    }
}
