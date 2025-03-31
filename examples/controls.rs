//! Example of a simple UI layout

use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
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
    hover::Hovering, CoreButton, CoreButtonPressed, CoreCheckbox, CoreWidgetsPlugin,
    InteractionDisabled,
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
                close_on_esc,
            ),
        )
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

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
            Spawn(buttons_demo()),
            Spawn(checkbox_demo()),
            // (
            //     Spawn((Text::new("Swatch"), UseInheritedTextStyles)),
            //     Spawn((
            //         Node::default(),
            //         Styles(style_row),
            //         Children::spawn((
            //             Invoke(Swatch::new(palettes::css::GOLDENROD)),
            //             Invoke(Swatch::new(palettes::css::LIME)),
            //             Invoke(Swatch::new(palettes::css::RED)),
            //             Invoke(Swatch::new(Srgba::NONE)),
            //             Invoke(Swatch::new(palettes::css::BLUE).selected(true)),
            //         )),
            //     )),
            // ),
            // (
            //     Spawn((Text::new("SwatchGrid"), UseInheritedTextStyles)),
            //     Spawn((
            //         Node::default(),
            //         Styles(style_row),
            //         Children::spawn(Invoke(SwatchGridDemo)),
            //     )),
            // ),
            // (
            //     Spawn((Text::new("Checkbox"), UseInheritedTextStyles)),
            //     Spawn((
            //         Node::default(),
            //         Styles(style_column),
            //         Children::spawn(Invoke(CheckboxDemo)),
            //     )),
            // ),
            // (
            //     Spawn((Text::new("Slider"), UseInheritedTextStyles)),
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
            // (
            //     Spawn((Text::new("GradientSlider"), UseInheritedTextStyles)),
            //     Spawn((
            //         Node::default(),
            //         Styles((style_column, |ec: &mut EntityCommands| {
            //             ec.entry::<Node>().and_modify(|mut node| {
            //                 node.align_items = ui::AlignItems::Stretch;
            //             });
            //         })),
            //         Children::spawn(Invoke(GradientSliderDemo)),
            //     )),
            // ),
            // (
            //     Spawn((Text::new("SpinBox"), UseInheritedTextStyles)),
            //     Spawn((
            //         Node::default(),
            //         Styles((style_column, |ec: &mut EntityCommands| {
            //             ec.entry::<Node>().and_modify(|mut node| {
            //                 node.align_items = ui::AlignItems::Stretch;
            //             });
            //         })),
            //         Children::spawn(Invoke(SpinBoxDemo)),
            //     )),
            // ),
            // Spawn((Text::new("DisclosureToggle"), UseInheritedTextStyles)),
            // Invoke(DisclosureToggleDemo),
        )),
    ));
}

// struct SwatchGridDemo;

// impl Template for SwatchGridDemo {
//     fn build(&self, tc: &mut TemplateContext) {
//         let selected_color = tc.create_mutable::<Srgba>(palettes::css::BLUE);
//         let on_change_color =
//             tc.create_callback_arg(move |color: In<Srgba>, mut world: DeferredWorld| {
//                 selected_color.set(&mut world, *color);
//             });
//         tc.invoke(
//             SwatchGrid::new(vec![
//                 palettes::css::BLUE,
//                 palettes::css::RED,
//                 palettes::css::GREEN,
//                 palettes::css::REBECCA_PURPLE,
//             ])
//             .grid_size(UVec2::new(12, 4))
//             .selected(selected_color.signal())
//             .on_change(on_change_color),
//         );
//     }
// }

// struct CheckboxDemo;

// impl Template for CheckboxDemo {
//     fn build(&self, tc: &mut TemplateContext) {
//         let checked_1 = tc.create_mutable(true);
//         let checked_2 = tc.create_mutable(false);
//         let on_change_1 =
//             tc.create_callback_arg(move |value: In<bool>, mut world: DeferredWorld| {
//                 checked_1.set(&mut world, *value);
//             });
//         let on_change_2 =
//             tc.create_callback_arg(move |value: In<bool>, mut world: DeferredWorld| {
//                 checked_2.set(&mut world, *value);
//             });
//         tc.invoke(
//             Checkbox::new()
//                 .labeled("Checked")
//                 .aria_label("Alpha")
//                 .checked(checked_1)
//                 .on_change(on_change_1),
//         )
//         .invoke(
//             Checkbox::new()
//                 .labeled("Checked (disabled)")
//                 .aria_label("Beta")
//                 .checked(checked_1)
//                 .on_change(on_change_1)
//                 .disabled(true),
//         )
//         .invoke(
//             Checkbox::new()
//                 .labeled("Unchecked")
//                 .aria_label("Gamma")
//                 .checked(checked_2)
//                 .on_change(on_change_2),
//         )
//         .invoke(
//             Checkbox::new()
//                 .labeled("Unchecked (disabled)")
//                 .aria_label("Delta")
//                 .checked(checked_2)
//                 .on_change(on_change_2)
//                 .disabled(true),
//         );
//     }
// }

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

// struct GradientSliderDemo;

// impl Template for GradientSliderDemo {
//     fn build(&self, tc: &mut TemplateContext) {
//         tc.spawn((Text::new("GradientSlider"), UseInheritedTextStyles));
//         let red = tc.create_mutable::<f32>(128.);
//         let on_change_red =
//             tc.create_callback_arg(move |new_value: In<f32>, mut world: DeferredWorld| {
//                 red.set(&mut world, *new_value);
//             });
//         tc.invoke(
//             GradientSlider::new()
//                 .gradient(Signal::Constant(ColorGradient::new(&[
//                     Srgba::new(0.0, 0.0, 0.0, 1.0),
//                     Srgba::new(1.0, 0.0, 0.0, 1.0),
//                 ])))
//                 .min(0.)
//                 .max(255.)
//                 .value(red)
//                 // .style(style_slider)
//                 .precision(1)
//                 .on_change(on_change_red),
//         );
//     }
// }

// struct SpinBoxDemo;

// impl Template for SpinBoxDemo {
//     fn build(&self, tc: &mut TemplateContext) {
//         let spinbox_value = tc.create_mutable::<f32>(50.);
//         let on_change_spinbox =
//             tc.create_callback_arg(move |new_value: In<f32>, mut world: DeferredWorld| {
//                 spinbox_value.set(&mut world, *new_value);
//             });
//         tc.invoke(
//             SpinBox::new()
//                 .min(0.)
//                 .max(100.)
//                 .value(spinbox_value)
//                 .on_change(on_change_spinbox),
//         );
//     }
// }

// struct DisclosureToggleDemo;

// impl Template for DisclosureToggleDemo {
//     fn build(&self, tc: &mut TemplateContext) {
//         let expanded = tc.create_mutable(false);
//         let on_change = tc.create_callback_arg(move |value: In<bool>, mut world: DeferredWorld| {
//             expanded.set(&mut world, *value);
//         });

//         tc.spawn((
//             Node::default(),
//             Children::spawn(Invoke(
//                 DisclosureToggle::new()
//                     .expanded(expanded)
//                     .on_change(on_change),
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

fn buttons_demo() -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Row,
            justify_content: ui::JustifyContent::Start,
            align_items: ui::AlignItems::Center,
            align_content: ui::AlignContent::Center,
            // padding: ui::UiRect::axes(ui::Val::Px(12.0), ui::Val::Px(0.0)),
            column_gap: ui::Val::Px(6.0),
            ..default()
        },
        Children::spawn((
            Spawn(button("Open...", ButtonVariant::Default, None)),
            Spawn(button("Save", ButtonVariant::Default, None)),
            Spawn(button("Create", ButtonVariant::Primary, None)),
        )),
    )
}

#[derive(Component, Default)]
struct DemoButton {
    variant: ButtonVariant,
}

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

fn checkbox_demo() -> impl Bundle {
    (
        Node {
            display: ui::Display::Flex,
            flex_direction: ui::FlexDirection::Column,
            align_items: ui::AlignItems::Start,
            align_content: ui::AlignContent::Start,
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

// Update the button's background color.
#[allow(clippy::type_complexity)]
fn update_checkbox_colors(
    mut q_checkbox: Query<
        (
            Entity,
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
    mut q_bg_color: Query<&mut BorderColor, (Without<DemoCheckbox>, Without<Children>)>,
) {
    for (checkbox_id, checkbox_state, Hovering(is_hovering), is_disabled, children) in
        q_checkbox.iter_mut()
    {
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

        let Ok((mut border_color, children)) = q_border_color.get_mut(*border_id) else {
            continue;
        };

        if border_color.0 != color {
            // Update the background color of the checkbox
            border_color.0 = color;
        }

        let Some(mark_id) = children.first() else {
            continue;
        };

        let Ok(mut bg_color) = q_bg_color.get_mut(*mark_id) else {
            continue;
        };

        let color: Color = match (is_disabled, checkbox_state.checked) {
            (true, true) => colors::PRIMARY.with_alpha(0.5),
            (false, true) => colors::PRIMARY,
            (_, false) => Srgba::NONE,
        }
        .into();

        if bg_color.0 != color {
            // Update the background color of the checkbox
            bg_color.0 = color;
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
    // pub const BACKGROUND: Srgba = Srgba::new(0.118, 0.118, 0.133, 1.0);
    // pub const FOREGROUND: Srgba = Srgba::new(0.925, 0.925, 0.925, 1.0);
    // pub const DIM: Srgba = Srgba::new(0.7, 0.7, 0.7, 1.0);
    pub const PRIMARY: Srgba = Srgba::new(0.341, 0.435, 0.525, 1.0);
    pub const DESTRUCTIVE: Srgba = Srgba::new(0.525, 0.341, 0.404, 1.0);
    pub const FOCUS: Srgba = Srgba::new(0.055, 0.647, 0.914, 0.15);
}
