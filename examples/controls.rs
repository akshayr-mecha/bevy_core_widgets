//! Demonstrations of the various core widgets.
//!
//! Note that this example should not be used as a basis for a real application. A real application
//! would likely use a more sophisticated UI framework or library that includes composable styles,
//! templates, reactive signals, and other techniques that improve the developer experience. This
//! example has been written in a very brute-force, low-level style so as to demonstrate the
//! functionality of the core widgets with minimal dependencies.

use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, system::SystemId, world::DeferredWorld},
    input_focus::{
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
        InputDispatchPlugin, InputFocus, InputFocusVisible,
    },
    prelude::*,
    ui,
    window::SystemCursorIcon,
    winit::{cursor::CursorIcon, WinitSettings},
};
use bevy_core_widgets::{
    hover::Hovering, ButtonClicked, ButtonPressed, Checked, CoreButton, CoreCheckbox, CoreRadio,
    CoreRadioGroup, CoreSlider, CoreWidgetsPlugin, InteractionDisabled, SliderDragState,
    ValueChange,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CoreWidgetsPlugin,
            InputDispatchPlugin,
            TabNavigationPlugin,
        ))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_view_root)
        .add_systems(
            Update,
            (
                update_button_bg_colors,
                update_focus_rect,
                update_checkbox_colors,
                update_radio_colors,
                update_slider_thumb,
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
            Spawn(slider_demo()),
            // Spawn(Text::new("SpinBox")),
            // Spawn(Text::new("DisclosureToggle")),
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
        |mut trigger: Trigger<ValueChange<bool>>,
         q_checkbox: Query<&CoreCheckbox>,
         mut commands: Commands| {
            trigger.propagate(false);
            if q_checkbox.contains(trigger.target()) {
                // Update checkbox state from event.
                let is_checked = trigger.event().0;
                commands
                    .entity(trigger.target())
                    .insert(Checked(is_checked));
                info!("New checkbox state: {:?}", is_checked);
            }
        },
    );

    // Observer for radio buttons.
    commands.add_observer(
        |mut trigger: Trigger<ValueChange<Entity>>,
         q_radio_group: Query<&Children, With<CoreRadioGroup>>,
         q_radio: Query<(&ChildOf, &RadioValue), With<CoreRadio>>,
         mut commands: Commands| {
            trigger.propagate(false);
            if q_radio_group.contains(trigger.target()) {
                // Update checkbox state from event.
                let selected_entity = trigger.event().0;
                let (child_of, radio_value) = q_radio.get(selected_entity).unwrap();
                // Mutual exclusion logic
                let group_children = q_radio_group.get(child_of.parent).unwrap();
                for radio_child in group_children.iter() {
                    if let Ok((_, value)) = q_radio.get(radio_child) {
                        commands
                            .entity(radio_child)
                            .insert(Checked(value.0 == radio_value.0));
                    }
                }
                info!("Radio Value: {}", radio_value.0);
            }
        },
    );

    // Observer for sliders that don't have an on_change handler.
    commands.add_observer(
        |mut trigger: Trigger<ValueChange<f32>>, mut q_slider: Query<&mut CoreSlider>| {
            trigger.propagate(false);
            if let Ok(mut slider) = q_slider.get_mut(trigger.target()) {
                // Update slider state from event.
                slider.set_value(trigger.event().0);
                info!("New slider state: {:?}", slider.value());
            }
        },
    );
}

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

// Places an outline around the currently focused widget. This is a generic implementation for demo
// purposes; in a real widget library the focus rectangle would be customized to the shape of
// the individual widgets.
#[allow(clippy::type_complexity)]
fn update_focus_rect(
    mut query: Query<(Entity, Has<Outline>), With<TabIndex>>,
    focus: Res<InputFocus>,
    focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    for (control, has_focus) in query.iter_mut() {
        let needs_focus = Some(control) == focus.0 && focus_visible.0;
        if needs_focus != has_focus {
            if needs_focus {
                commands.entity(control).insert(Outline {
                    color: colors::FOCUS.into(),
                    width: ui::Val::Px(2.0),
                    offset: ui::Val::Px(1.0),
                });
            } else {
                commands.entity(control).remove::<Outline>();
            }
        }
    }
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
        AccessibleName(caption.to_string()),
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
            &ButtonPressed,
            Has<InteractionDisabled>,
        ),
        Or<(Added<DemoButton>, Changed<Hovering>, Changed<ButtonPressed>)>,
    >,
) {
    for (button, mut bg_color, Hovering(is_hovering), ButtonPressed(is_pressed), is_disabled) in
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
        AccessibleName(caption.to_string()),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoCheckbox,
        CoreCheckbox { on_change },
        Checked(checked),
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
        (&Checked, &Hovering, Has<InteractionDisabled>, &Children),
        (
            With<DemoCheckbox>,
            Or<(Added<DemoCheckbox>, Changed<Hovering>, Changed<Checked>)>,
        ),
    >,
    mut q_border_color: Query<(&mut BorderColor, &mut Children), Without<DemoCheckbox>>,
    mut q_bg_color: Query<&mut BackgroundColor, (Without<DemoCheckbox>, Without<Children>)>,
) {
    for (Checked(checked), Hovering(is_hovering), is_disabled, children) in q_checkbox.iter_mut() {
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

        let mark_color: Color = match (is_disabled, *checked) {
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
        CoreRadioGroup { on_change: None },
        Children::spawn((
            Spawn(radio("WKRP", true)),
            Spawn(radio("WPIG", false)),
            Spawn(radio("Galaxy News Radio", false)),
            Spawn(radio("KBBL-FM", false)),
            Spawn(radio("Radio Rock", false)),
        )),
    )
}

#[derive(Component, Default)]
struct DemoRadio;

#[derive(Component, Default)]
struct RadioValue(String);

/// Create a demo radio button
fn radio(caption: &str, checked: bool) -> impl Bundle {
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
        AccessibleName(caption.to_string()),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoRadio,
        CoreRadio,
        Checked(checked),
        RadioValue(caption.to_string()),
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
        (&Checked, &Hovering, Has<InteractionDisabled>, &Children),
        (
            With<DemoRadio>,
            Or<(Added<DemoRadio>, Changed<Hovering>, Changed<Checked>)>,
        ),
    >,
    mut q_border_color: Query<(&mut BorderColor, &mut Children), Without<DemoRadio>>,
    mut q_bg_color: Query<&mut BackgroundColor, (Without<DemoRadio>, Without<Children>)>,
) {
    for (Checked(checked), Hovering(is_hovering), is_disabled, children) in q_radio.iter_mut() {
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

        let mark_color: Color = match (is_disabled, *checked) {
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

/// Create a column of demo checkboxes
fn slider_demo() -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Column,
            align_items: ui::AlignItems::Start,
            align_content: ui::AlignContent::Start,
            width: ui::Val::Px(200.0),
            padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            row_gap: ui::Val::Px(6.0),
            ..default()
        },
        Children::spawn((
            Spawn(slider("Volume", 0.0, 100.0, 0.0, None)),
            Spawn(slider("Difficulty", 0.0, 10.0, 5.0, None)),
        )),
    )
}

#[derive(Component, Default)]
struct DemoSlider;

/// Create a demo slider
fn slider(
    label: &str,
    min: f32,
    max: f32,
    value: f32,
    on_change: Option<SystemId<In<f32>>>,
) -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Column,
            justify_content: ui::JustifyContent::Center,
            align_self: ui::AlignSelf::Stretch,
            align_items: ui::AlignItems::Stretch,
            justify_items: ui::JustifyItems::Center,
            column_gap: ui::Val::Px(4.0),
            height: ui::Val::Px(12.0),
            ..default()
        },
        Name::new("Slider"),
        AccessibleName(label.to_string()),
        Hovering::default(),
        CursorIcon::System(SystemCursorIcon::Pointer),
        DemoSlider,
        CoreSlider {
            max,
            min,
            value,
            on_change,
            thumb_size: 12.0,
            ..default()
        },
        TabIndex(0),
        Children::spawn((
            // Slider background rail
            Spawn((
                Node {
                    height: ui::Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(colors::U3.into()), // Border color for the checkbox
                BorderRadius::all(ui::Val::Px(3.0)),
            )),
            // Invisible track to allow absolute placement of thumb entity. This is narrower than
            // the actual slider, which allows us to position the thumb entity using simple
            // percentages, without having to measure the actual width of the slider thumb.
            Spawn((
                Node {
                    display: ui::Display::Flex,
                    position_type: ui::PositionType::Absolute,
                    left: ui::Val::Px(0.0),
                    right: ui::Val::Px(12.0), // Track is short by 12px to accommodate the thumb
                    top: ui::Val::Px(0.0),
                    bottom: ui::Val::Px(0.0),
                    ..default()
                },
                children![(
                    // Thumb
                    Node {
                        display: ui::Display::Flex,
                        width: ui::Val::Px(12.0),
                        height: ui::Val::Px(12.0),
                        position_type: ui::PositionType::Absolute,
                        left: ui::Val::Percent(50.0), // This will be updated by the slider's value
                        ..default()
                    },
                    BorderRadius::all(ui::Val::Px(6.0)),
                    BackgroundColor(colors::PRIMARY.into()),
                )],
            )),
        )),
    )
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
fn update_slider_thumb(
    mut q_radio: Query<
        (
            &CoreSlider,
            &SliderDragState,
            &Hovering,
            Has<InteractionDisabled>,
            &Children,
        ),
        (
            With<DemoSlider>,
            Or<(Added<DemoSlider>, Changed<Hovering>, Changed<CoreSlider>)>,
        ),
    >,
    mut q_track: Query<&mut Children, Without<DemoSlider>>,
    mut q_thumb: Query<(&mut BackgroundColor, &mut Node), (Without<DemoSlider>, Without<Children>)>,
) {
    for (slider_state, drag_state, Hovering(is_hovering), is_disabled, children) in
        q_radio.iter_mut()
    {
        let color: Color = if is_disabled {
            // If the slider is disabled, use a lighter color
            colors::U4.with_alpha(0.2)
        } else if *is_hovering || drag_state.dragging {
            // If hovering, use a lighter color
            colors::U5
        } else {
            // Default color for the slider
            colors::U4
        }
        .into();

        let Some(track_id) = children.last() else {
            warn!("Slider does not have a track entity.");
            continue;
        };

        let Ok(track_children) = q_track.get_mut(*track_id) else {
            continue;
        };

        let Some(mark_id) = track_children.first() else {
            warn!("Slider does not have a thumb entity.");
            continue;
        };

        let Ok((mut thumb_bg, mut node)) = q_thumb.get_mut(*mark_id) else {
            warn!("Slider thumb lacking a background color or node.");
            continue;
        };

        if thumb_bg.0 != color {
            // Update the color of the thumb
            thumb_bg.0 = color;
        }

        let thumb_position = ui::Val::Percent(slider_state.thumb_position() * 100.0);
        if node.left != thumb_position {
            node.left = thumb_position;
        }
    }
}

#[derive(Component, Default)]
#[component(immutable, on_add = on_set_label, on_replace = on_set_label)]
struct AccessibleName(String);

// Hook to set the a11y "checked" state when the checkbox is added.
fn on_set_label(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let name = entt.get::<AccessibleName>().unwrap().0.clone();
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_label(name.as_str());
    }
}

mod colors {
    use bevy::color::Srgba;

    pub const U3: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
    pub const U4: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
    pub const U5: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
    pub const PRIMARY: Srgba = Srgba::new(0.341, 0.435, 0.525, 1.0);
    pub const DESTRUCTIVE: Srgba = Srgba::new(0.525, 0.341, 0.404, 1.0);
    pub const FOCUS: Srgba = Srgba::new(0.055, 0.647, 0.914, 0.15);
}
