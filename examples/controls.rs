//! Demonstrations of the various core widgets.
//!
//! Note that this example should not be used as a basis for a real application. A real application
//! would likely use a more sophisticated UI framework or library that includes composable styles,
//! templates, reactive signals, and other techniques that improve the developer experience. This
//! example has been written in a very brute-force, low-level style so as to demonstrate the
//! functionality of the widgets with minimal dependencies.

use bevy::{
    ecs::system::SystemId,
    input_focus::{
        tab_navigation::{TabGroup, TabIndex},
        InputFocus, InputFocusVisible,
    },
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use bevy_core_widgets::{
    hover::Hovering, ButtonClicked, CoreButton, CoreButtonPressed, CoreCheckbox, CoreRadio,
    CoreRadioGroup, CoreWidgetsPlugin, InteractionDisabled, ValueChange,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CoreWidgetsPlugin))
        .add_systems(Startup, setup_view_root)
        .add_systems(
            Update,
            (
                update_button_bg_colors,
                update_button_focus_rect,
                update_checkbox_colors,
                update_checkbox_focus_rect,
                update_radio_colors,
                update_radio_group_focus_rect,
                close_on_esc,
            ),
        )
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

    // Demonstration click handler.
    let on_click = commands.register_system(|| {
        info!("Button on_click handler called!");
    });

    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            left: ui::Val::Px(0.),
            top: ui::Val::Px(0.),
            right: ui::Val::Px(0.),
            bottom: ui::Val::Px(0.),
            padding: ui::UiRect::all(Val::Px(3.)),
            row_gap: ui::Val::Px(6.),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        UiTargetCamera(camera),
        TabGroup::default(),
        Children::spawn((
            Spawn(Text::new("Button")),
            Spawn(buttons_demo(on_click)),
            Spawn(Text::new("Checkbox")),
            Spawn(checkbox_demo()),
            Spawn(Text::new("Radio")),
            Spawn(radio_demo()),
            Spawn(Text::new("Slider")),
            // Spawn(Text::new("SpinBox")),
            // Spawn(Text::new("DisclosureToggle")),
            // (
            //     Spawn((
            //         Node::default(),
            //         Styles((style_column, |ec: &mut EntityCommands| {
            //             ec.entry::<Node>().and_modify(|mut node| {
            //                 node.align_items = ui::AlignItems::Stretch;
            //             });
            //         })),
            //         Children::spawn(Invoke(SliderDemo)),
            //     )),
            // ),
        )),
    ));

    // Observer for buttons that don't have an on_click handler.
    commands.add_observer(
        |mut trigger: Trigger<ButtonClicked>, q_button: Query<&CoreButton>| {
            // If the button doesn't exist or is not a CoreButton
            if q_button.get(trigger.target()).is_ok() {
                trigger.propagate(false);
                let button_id = trigger.target();
                info!("Got button click event: {:?}", button_id);
            }
        },
    );

    // Observer for checkboxes that don't have an on_change handler.
    commands.add_observer(
        |mut trigger: Trigger<ValueChange<bool>>, mut q_checkbox: Query<&mut CoreCheckbox>| {
            trigger.propagate(false);
            if let Ok(mut checkbox) = q_checkbox.get_mut(trigger.target()) {
                // Update checkbox state from event.
                checkbox.checked = trigger.event().0;
                info!("New checkbox state: {:?}", checkbox.checked);
            }
        },
    );
}

// struct SliderDemo;

// impl Template for SliderDemo {
//     fn build(&self, tc: &mut TemplateContext) {
//         let slider_value = tc.create_mutable::<f32>(50.);
//         let on_change_slider =
//             tc.create_callback_arg(move |new_value: In<f32>, mut world: DeferredWorld| {
//                 slider_value.set(&mut world, *new_value);
//             });
//         tc.spawn((
//             Node::default(),
//             Styles((style_column, |ec: &mut EntityCommands| {
//                 ec.entry::<Node>().and_modify(|mut node| {
//                     node.align_items = ui::AlignItems::Stretch;
//                 });
//             })),
//             Children::spawn((
//                 Invoke(
//                     Slider::new()
//                         .min(0.)
//                         .max(100.)
//                         .value(slider_value)
//                         .on_change(on_change_slider),
//                 ),
//                 Invoke(
//                     Slider::new()
//                         .min(0.)
//                         .max(100.)
//                         .value(slider_value)
//                         .label("Value:")
//                         .on_change(on_change_slider),
//                 ),
//             )),
//         ));
//     }
// }

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

/// The variant determines the button's color scheme
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    /// The default button apperance.
    #[default]
    Default,

    /// A more prominent, "call to action", appearance.
    Primary,

    /// An appearance indicating a potentially dangerous action.
    Danger,

    /// A button that is in a "toggled" state.
    Selected,
}

/// Create a row of demo buttons
fn buttons_demo(on_click: SystemId) -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            justify_content: ui::JustifyContent::Start,
            align_items: ui::AlignItems::Center,
            align_content: ui::AlignContent::Center,
            padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            column_gap: ui::Val::Px(6.0),
            ..default()
        },
        Children::spawn((
            Spawn(button("Open...", ButtonVariant::Default, Some(on_click))),
            Spawn(button("Save", ButtonVariant::Default, None)),
            Spawn(button("Create", ButtonVariant::Primary, None)),
        )),
    )
}

#[derive(Component, Default)]
struct DemoButton {
    variant: ButtonVariant,
}

/// Create a demo button
fn button(caption: &str, variant: ButtonVariant, on_click: Option<SystemId>) -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            justify_content: ui::JustifyContent::Center,
            align_items: ui::AlignItems::Center,
            align_content: ui::AlignContent::Center,
            padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            border: ui::UiRect::all(ui::Val::Px(0.0)),
            min_height: ui::Val::Px(24.0),
            ..default()
        },
        BorderRadius::all(ui::Val::Px(4.0)),
        Name::new("Button"),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoButton { variant },
        CoreButton { on_click },
        TabIndex(0),
        children![(
            Text::new(caption),
            TextFont {
                font_size: 14.0,
                ..default()
            }
        )],
    )
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
fn update_button_bg_colors(
    mut query: Query<
        (
            &DemoButton,
            &mut BackgroundColor,
            &Hovering,
            &CoreButtonPressed,
            Has<InteractionDisabled>,
        ),
        Or<(
            Added<DemoButton>,
            Changed<Hovering>,
            Changed<CoreButtonPressed>,
        )>,
    >,
) {
    for (button, mut bg_color, Hovering(is_hovering), CoreButtonPressed(is_pressed), is_disabled) in
        query.iter_mut()
    {
        // Update the background color based on the button's state
        let base_color = match button.variant {
            ButtonVariant::Default => colors::U3,
            ButtonVariant::Primary => colors::PRIMARY,
            ButtonVariant::Danger => colors::DESTRUCTIVE,
            ButtonVariant::Selected => colors::U4,
        };

        let new_color = match (is_disabled, is_pressed, is_hovering) {
            (true, _, _) => base_color.with_alpha(0.2),
            (_, true, true) => base_color.lighter(0.07),
            (_, false, true) => base_color.lighter(0.03),
            _ => base_color,
        };

        bg_color.0 = new_color.into();
    }
}

// Update the button's focus rectangle.
#[allow(clippy::type_complexity)]
fn update_button_focus_rect(
    mut query: Query<(Entity, Has<Outline>), With<DemoButton>>,
    focus: Res<InputFocus>,
    focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    for (button, has_focus) in query.iter_mut() {
        let needs_focus = Some(button) == focus.0 && focus_visible.0;
        if needs_focus != has_focus {
            if needs_focus {
                commands.entity(button).insert(Outline {
                    color: colors::FOCUS.into(),
                    width: ui::Val::Px(2.0),
                    offset: ui::Val::Px(1.0),
                });
            } else {
                commands.entity(button).remove::<Outline>();
            }
        }
    }
}

/// Create a column of demo checkboxes
fn checkbox_demo() -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Column,
            align_items: ui::AlignItems::Start,
            align_content: ui::AlignContent::Start,
            padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            row_gap: ui::Val::Px(6.0),
            ..default()
        },
        Children::spawn((
            Spawn(checkbox("Show Tutorial", true, None)),
            Spawn(checkbox("Just Kidding", false, None)),
        )),
    )
}

#[derive(Component, Default)]
struct DemoCheckbox;

/// Create a demo checkbox
fn checkbox(caption: &str, checked: bool, on_change: Option<SystemId<In<bool>>>) -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            justify_content: ui::JustifyContent::FlexStart,
            align_items: ui::AlignItems::Center,
            align_content: ui::AlignContent::Center,
            column_gap: ui::Val::Px(4.0),
            ..default()
        },
        Name::new("Checkbox"),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoCheckbox,
        CoreCheckbox { on_change, checked },
        TabIndex(0),
        Children::spawn((
            Spawn((
                // Checkbox outer
                Node {
                    display: ui::Display::Flex,
                    width: ui::Val::Px(16.0),
                    height: ui::Val::Px(16.0),
                    border: ui::UiRect::all(ui::Val::Px(2.0)),
                    ..default()
                },
                BorderColor(colors::U4.into()), // Border color for the checkbox
                BorderRadius::all(ui::Val::Px(3.0)),
                children![
                    // Checkbox inner
                    (
                        Node {
                            display: ui::Display::Flex,
                            width: ui::Val::Px(8.0),
                            height: ui::Val::Px(8.0),
                            position_type: ui::PositionType::Absolute,
                            left: ui::Val::Px(2.0),
                            top: ui::Val::Px(2.0),
                            ..default()
                        },
                        BackgroundColor(colors::PRIMARY.into()),
                    ),
                ],
            )),
            Spawn((
                Text::new(caption),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            )),
        )),
    )
}

// Update the checkbox's background color.
#[allow(clippy::type_complexity)]
fn update_checkbox_colors(
    mut q_checkbox: Query<
        (
            &CoreCheckbox,
            &Hovering,
            Has<InteractionDisabled>,
            &Children,
        ),
        (
            With<DemoCheckbox>,
            Or<(
                Added<DemoCheckbox>,
                Changed<Hovering>,
                Changed<CoreCheckbox>,
            )>,
        ),
    >,
    mut q_border_color: Query<(&mut BorderColor, &mut Children), Without<DemoCheckbox>>,
    mut q_bg_color: Query<&mut BackgroundColor, (Without<DemoCheckbox>, Without<Children>)>,
) {
    for (checkbox_state, Hovering(is_hovering), is_disabled, children) in q_checkbox.iter_mut() {
        let color: Color = if is_disabled {
            // If the checkbox is disabled, use a lighter color
            colors::U4.with_alpha(0.2)
        } else if *is_hovering {
            // If hovering, use a lighter color
            colors::U5
        } else {
            // Default color for the checkbox
            colors::U4
        }
        .into();

        let Some(border_id) = children.first() else {
            continue;
        };

        let Ok((mut border_color, border_children)) = q_border_color.get_mut(*border_id) else {
            continue;
        };

        if border_color.0 != color {
            // Update the background color of the check mark
            border_color.0 = color;
        }

        let Some(mark_id) = border_children.first() else {
            warn!("Checkbox does not have a mark entity.");
            continue;
        };

        let Ok(mut mark_bg) = q_bg_color.get_mut(*mark_id) else {
            warn!("Checkbox mark entity lacking a background color.");
            continue;
        };

        let mark_color: Color = match (is_disabled, checkbox_state.checked) {
            (true, true) => colors::PRIMARY.with_alpha(0.5),
            (false, true) => colors::PRIMARY,
            (_, false) => Srgba::NONE,
        }
        .into();

        if mark_bg.0 != mark_color {
            // Update the color of the check mark
            mark_bg.0 = mark_color;
        }
    }
}

// Update the checkbox's focus rectangle.
#[allow(clippy::type_complexity)]
fn update_checkbox_focus_rect(
    mut query: Query<(Entity, Has<Outline>), With<DemoCheckbox>>,
    focus: Res<InputFocus>,
    focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    for (checkbox, has_focus) in query.iter_mut() {
        let needs_focus = Some(checkbox) == focus.0 && focus_visible.0;
        if needs_focus != has_focus {
            if needs_focus {
                commands.entity(checkbox).insert(Outline {
                    color: colors::FOCUS.into(),
                    width: ui::Val::Px(2.0),
                    offset: ui::Val::Px(1.0),
                });
            } else {
                commands.entity(checkbox).remove::<Outline>();
            }
        }
    }
}

/// Create a column of demo radio buttons
fn radio_demo() -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Column,
            align_items: ui::AlignItems::Start,
            align_content: ui::AlignContent::Start,
            padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            row_gap: ui::Val::Px(6.0),
            ..default()
        },
        TabIndex(0),
        CoreRadioGroup,
        Children::spawn((
            Spawn(radio("WKRP", true, None)),
            Spawn(radio("WPIG", false, None)),
            Spawn(radio("Galaxy News Radio", false, None)),
            Spawn(radio("KBBL-FM", false, None)),
            Spawn(radio("Radio Rock", false, None)),
        )),
    )
}

#[derive(Component, Default)]
struct DemoRadio;

/// Create a demo radio button
fn radio(caption: &str, checked: bool, on_click: Option<SystemId>) -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            justify_content: ui::JustifyContent::FlexStart,
            align_items: ui::AlignItems::Center,
            align_content: ui::AlignContent::Center,
            column_gap: ui::Val::Px(4.0),
            ..default()
        },
        Name::new("Radio"),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoRadio,
        CoreRadio { on_click, checked },
        Children::spawn((
            Spawn((
                // Radio outer
                Node {
                    display: ui::Display::Flex,
                    width: ui::Val::Px(16.0),
                    height: ui::Val::Px(16.0),
                    border: ui::UiRect::all(ui::Val::Px(2.0)),
                    ..default()
                },
                BorderColor(colors::U4.into()), // Border color for the radio
                BorderRadius::all(ui::Val::Percent(50.0)),
                children![
                    // Radio inner
                    (
                        Node {
                            display: ui::Display::Flex,
                            width: ui::Val::Px(8.0),
                            height: ui::Val::Px(8.0),
                            position_type: ui::PositionType::Absolute,
                            left: ui::Val::Px(2.0),
                            top: ui::Val::Px(2.0),
                            ..default()
                        },
                        BackgroundColor(colors::PRIMARY.into()),
                        BorderRadius::all(ui::Val::Percent(50.0)),
                    ),
                ],
            )),
            Spawn((
                Text::new(caption),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            )),
        )),
    )
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
fn update_radio_colors(
    mut q_radio: Query<
        (&CoreRadio, &Hovering, Has<InteractionDisabled>, &Children),
        (
            With<DemoRadio>,
            Or<(Added<DemoRadio>, Changed<Hovering>, Changed<CoreRadio>)>,
        ),
    >,
    mut q_border_color: Query<(&mut BorderColor, &mut Children), Without<DemoRadio>>,
    mut q_bg_color: Query<&mut BackgroundColor, (Without<DemoRadio>, Without<Children>)>,
) {
    for (radio_state, Hovering(is_hovering), is_disabled, children) in q_radio.iter_mut() {
        let color: Color = if is_disabled {
            // If the radio is disabled, use a lighter color
            colors::U4.with_alpha(0.2)
        } else if *is_hovering {
            // If hovering, use a lighter color
            colors::U5
        } else {
            // Default color for the radio
            colors::U4
        }
        .into();

        let Some(border_id) = children.first() else {
            continue;
        };

        let Ok((mut border_color, border_children)) = q_border_color.get_mut(*border_id) else {
            continue;
        };

        if border_color.0 != color {
            // Update the background color of the check mark
            border_color.0 = color;
        }

        let Some(mark_id) = border_children.first() else {
            warn!("Radio does not have a mark entity.");
            continue;
        };

        let Ok(mut mark_bg) = q_bg_color.get_mut(*mark_id) else {
            warn!("Radio mark entity lacking a background color.");
            continue;
        };

        let mark_color: Color = match (is_disabled, radio_state.checked) {
            (true, true) => colors::PRIMARY.with_alpha(0.5),
            (false, true) => colors::PRIMARY,
            (_, false) => Srgba::NONE,
        }
        .into();

        if mark_bg.0 != mark_color {
            // Update the color of the check mark
            mark_bg.0 = mark_color;
        }
    }
}

// Update the button's focus rectangle.
#[allow(clippy::type_complexity)]
fn update_radio_group_focus_rect(
    mut query: Query<(Entity, Has<Outline>), With<CoreRadioGroup>>,
    focus: Res<InputFocus>,
    focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    for (radio, has_focus) in query.iter_mut() {
        let needs_focus = Some(radio) == focus.0 && focus_visible.0;
        if needs_focus != has_focus {
            if needs_focus {
                commands.entity(radio).insert(Outline {
                    color: colors::FOCUS.into(),
                    width: ui::Val::Px(2.0),
                    offset: ui::Val::Px(1.0),
                });
            } else {
                commands.entity(radio).remove::<Outline>();
            }
        }
    }
}

mod colors {
    use bevy::color::Srgba;

    // pub const U1: Srgba = Srgba::new(0.094, 0.094, 0.102, 1.0);
    // pub const U2: Srgba = Srgba::new(0.137, 0.137, 0.149, 1.0);
    pub const U3: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
    pub const U4: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
    pub const U5: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
    // pub const FOREGROUND: Srgba = Srgba::new(0.925, 0.925, 0.925, 1.0);
    pub const PRIMARY: Srgba = Srgba::new(0.341, 0.435, 0.525, 1.0);
    pub const DESTRUCTIVE: Srgba = Srgba::new(0.525, 0.341, 0.404, 1.0);
    pub const FOCUS: Srgba = Srgba::new(0.055, 0.647, 0.914, 0.15);
}
