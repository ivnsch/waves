use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_simple_text_input::{TextInputBundle, TextInputSubmitEvent};

/// form inputs
#[derive(Event, Default)]
pub struct WaveGuiInputs {
    pub amplitude: String,
}

/// marker component for amplitude text input
/// needs to be public to add the component in main, maybe I restructure this later
#[derive(Component, Default)]
pub struct AmplitudeInput;

pub fn setup_wave_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let root = commands.spawn(NodeBundle {
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

    add_input(&font, root, "Amplitude", AmplitudeInputMarker);
}

pub fn add_input<T>(font: &Handle<Font>, mut root: EntityCommands, label: &str, marker: T)
where
    T: Component,
{
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
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        ..default()
    };

    let wrapper = NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(50.0),

            ..default()
        },
        background_color: BackgroundColor(Color::GRAY),
        ..default()
    };

    let text_input_bundle = (
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            // border_color: BORDER_COLOR_ACTIVE.into(),
            // background_color: BACKGROUND_COLOR.into(),
            ..default()
        },
        TextInputBundle::default().with_text_style(TextStyle {
            font_size: 40.,
            color: Color::WHITE,
            ..default()
        }),
    );

    root.with_children(|parent| {
        parent.spawn(label);
    });

    root.with_children(|parent| {
        parent.spawn(wrapper).with_children(|parent| {
            parent.spawn((marker, text_input_bundle));
        });
    });
}

pub fn text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut my_events: EventWriter<WaveGuiInputs>,
) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
        my_events.send(WaveGuiInputs {
            amplitude: event.value.clone(),
        });
    }
}
