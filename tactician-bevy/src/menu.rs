use crate::misc::AppState;
use bevy::prelude::*;

// struct MenuData {
//     button_entity: Entity,
// }

pub fn init_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: color_materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Tactician",
                    TextStyle {
                        font_size: 60.0,
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        color: Color::WHITE
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(25.0), Val::Px(60.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: color_materials.add(Color::GRAY.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });

    // commands.insert_resource(MenuData {
    //     button_entity: button_id,
    // });
}

pub fn update_menu(
    mut state: ResMut<State<AppState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            let result = state.set(AppState::Game);
            if let Err(e) = result {
                warn!("issue changing state into AppState::Game??\n{}", e);
            }
        }
    }
}
