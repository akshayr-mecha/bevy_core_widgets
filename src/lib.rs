use bevy::{
    app::{App, Plugin, Update},
    input_focus::{tab_navigation, InputDispatchPlugin},
};
mod core_barrier;
mod core_button;
mod core_checkbox;
mod core_radio;
mod core_radio_group;
mod core_slider;
mod cursor;
mod events;
pub mod hover;
mod interaction_disabled;

pub use core_barrier::CoreBarrier;
pub use core_button::{CoreButton, CoreButtonPressed};
pub use core_checkbox::CoreCheckbox;
pub use core_radio::CoreRadio;
pub use core_radio_group::CoreRadioGroup;
pub use core_slider::CoreSlider;
pub use events::{ButtonClicked, ValueChange};
pub use interaction_disabled::InteractionDisabled;

pub struct CoreWidgetsPlugin;

use core_button::CoreButtonPlugin;
use core_checkbox::CoreCheckboxPlugin;
use core_radio::CoreRadioPlugin;
use core_radio_group::CoreRadioGroupPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputDispatchPlugin)
            .add_plugins(tab_navigation::TabNavigationPlugin)
            .add_plugins((
                CoreButtonPlugin,
                CoreCheckboxPlugin,
                CoreRadioPlugin,
                CoreRadioGroupPlugin,
            ))
            .add_systems(Update, (hover::update_hover_states, cursor::update_cursor))
            .add_observer(core_barrier::barrier_on_key_input)
            .add_observer(core_barrier::barrier_on_pointer_down)
            .add_observer(core_slider::slider_on_drag_start)
            .add_observer(core_slider::slider_on_drag_end)
            .add_observer(core_slider::slider_on_drag);
    }
}
