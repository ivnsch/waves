use bevy::prelude::*;
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

#[derive(Resource)]
pub struct GuiInputEntities {
    pub amplitude: Entity,
    pub wave_length: Entity,
}

/// marker component for amplitude text input
/// needs to be public to add the component in main, maybe I restructure this later
#[derive(Component, Default)]
pub struct AmplitudeInputMarker;
#[derive(Component, Default)]
pub struct WaveLengthInputMarker;

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

    let root_id = root.id();

    let amplitude_input = generate_input(
        &font,
        root_id,
        &mut commands,
        "Amplitude",
        AmplitudeInputMarker,
    );
    let wave_length_input = generate_input(
        &font,
        root_id,
        &mut commands,
        "Wave length",
        WaveLengthInputMarker,
    );

    commands.insert_resource(GuiInputEntities {
        amplitude: amplitude_input,
        wave_length: wave_length_input,
    });
}

fn generate_input<T>(
    font: &Handle<Font>,
    root_id: Entity,
    commands: &mut Commands,
    label: &str,
    marker: T,
) -> Entity
where
    T: Component,
{
    let label = generate_input_label(font, label);
    let wrapper = generate_wrapper();
    let text_input_bundle = generate_text_input();

    let spawned_label = commands.spawn(label).id();
    commands.entity(root_id).push_children(&[spawned_label]);

    let spawned_wrapper = commands.spawn(wrapper).id();
    commands.entity(root_id).push_children(&[spawned_wrapper]);

    let spawned_text_input_bundle = commands.spawn((marker, text_input_bundle)).id();
    commands
        .entity(root_id)
        .push_children(&[spawned_text_input_bundle]);

    spawned_text_input_bundle
}

fn generate_input_label(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
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
    }
}

fn generate_wrapper() -> NodeBundle {
    NodeBundle {
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
    }
}

fn generate_text_input() -> (NodeBundle, TextInputBundle) {
    let input = TextStyle {
        font_size: 40.,
        color: Color::WHITE,
        ..default()
    };

    (
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
        TextInputBundle::default().with_text_style(input),
    )
}

pub fn text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut inputs: ResMut<GuiInputs>,
    input_entities: Res<GuiInputEntities>,
) {
    for event in events.read() {
        if event.entity == input_entities.amplitude {
            info!("submitted amplitude: {}", event.value);
            inputs.amplitude = event.value.clone();
        } else if event.entity == input_entities.wave_length {
            println!("submitted wave length: {:?}", event.entity);
            inputs.wave_length = event.value.clone();
        } else {
            println!("unknown entity: {:?}", event.entity);
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
