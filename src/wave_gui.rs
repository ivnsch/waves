use bevy::{ecs::system::EntityCommands, prelude::*};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// text input confirmed via add button
#[derive(Event, Default)]
pub struct WaveGuiInput {
    pub text: String,
}

/// marker component for text input
/// needs to be public to add the component in main, maybe I restructure this later
#[derive(Component, Default)]
pub struct TextInput;

pub fn setup_wave_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let mut root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(300.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });

    let label = TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
            ..default()
        },
        text: Text::from_section(
            "Amplitude:".to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        ..default()
    };

    let text_input = TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            ..default()
        },
        text: Text::from_section(
            "".to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..default()
    };

    root.with_children(|parent| {
        parent.spawn(label);
    });

    root.with_children(|parent| {
        parent.spawn((TextInput, text_input));
    });

    add_add_button(root, &font);
}

pub fn button_system(
    mut my_events: EventWriter<WaveGuiInput>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    edit_text: Query<&mut Text, With<TextInput>>,
) {
    match interaction_query.get_single_mut() {
        Ok((interaction, mut color, mut border_color, _)) => match *interaction {
            Interaction::Pressed => {
                println!("pressed add!");
                match edit_text.get_single() {
                    Ok(text) => {
                        my_events.send(WaveGuiInput {
                            text: text.sections[0].value.clone(),
                        });
                    }
                    Err(err) => panic!("error: {}", err),
                }
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        },
        Err(_) => {
            // TODO use iter_mut()
        }
    }
}

pub fn listen_received_character_events(
    mut events: EventReader<ReceivedCharacter>,
    mut edit_text: Query<&mut Text, With<TextInput>>,
) {
    for event in events.read() {
        println!("received text: {:?}", event);
        let s = &mut edit_text.single_mut().sections[0].value;

        if event.char == "\u{8}" {
            if !s.is_empty() {
                println!("deleting..");
                s.remove(s.len() - 1);
            }
        } else {
            s.push_str(&event.char);
        }
    }
}

fn add_add_button(mut gui_root: EntityCommands, font: &Handle<Font>) {
    let button_node = NodeBundle {
        style: Style {
            width: Val::Px(100.0),
            height: Val::Px(50.0),
            // align_items: AlignItems::Center,
            // justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let button = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

    let label = TextBundle::from_section(
        "Add",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    gui_root.with_children(|parent| {
        parent.spawn(button_node).with_children(|parent| {
            parent.spawn(button).with_children(|parent| {
                parent.spawn(label);
            });
        });
    });
}
