use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_simple_text_input::{TextInputBundle, TextInputSubmitEvent};

#[derive(Resource)]
pub struct GuiInputs {
    pub amplitude: String,
    pub wave_length: String,
}

#[derive(Event, Default, Debug)]
pub struct GuiInputsEvent {
    pub amplitude: String,
    pub wave_length: String,
}

/// marker component for amplitude text input
/// needs to be public to add the component in main, maybe I restructure this later
#[derive(Component, Default)]
pub struct AmplitudeInputMarker;
#[derive(Component, Default)]
pub struct WaveLengthInputMarker;

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

    add_input(&font, &mut root, "Amplitude", AmplitudeInputMarker);
    add_input(&font, &mut root, "Wave length", WaveLengthInputMarker);
}

pub fn add_input<T>(font: &Handle<Font>, root: &mut EntityCommands, label: &str, marker: T)
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
        // background_color: BackgroundColor(Color::GRAY),
        ..default()
    };

    let text_input_bundle = (
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: Color::GRAY.into(),
            background_color: Color::GRAY.into(),
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

pub fn text_listener(mut events: EventReader<TextInputSubmitEvent>, mut inputs: ResMut<GuiInputs>) {
    for event in events.read() {
        let index = event.entity.index();
        match index {
            5 => {
                info!("{:?} submitted amplitude: {}", index, event.value);
                inputs.amplitude = event.value.clone();
            }
            8 => {
                info!("{:?} submitted wave length: {}", index, event.value);
                inputs.wave_length = event.value.clone();
            }
            _ => {
                println!("unknown index: {}", index);
            }
        }
    }
}

pub fn form_state_notifier_system(
    form_state: Res<GuiInputs>,
    mut my_events: EventWriter<GuiInputsEvent>,
) {
    // println!(
    //     "amplitude: {}, wave length: {}",
    //     form_state.amplitude, form_state.wave_length
    // );
    my_events.send(GuiInputsEvent {
        amplitude: form_state.amplitude.clone(),
        wave_length: form_state.wave_length.clone(),
    });
}
