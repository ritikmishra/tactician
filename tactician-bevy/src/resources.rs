use bevy::prelude::*;

pub struct Typography {
    pub default_font: Handle<Font>,
    pub heading: TextStyle,
    pub body: TextStyle,
}

impl Typography {
    const DEFAULT_FONT: &'static str = "fonts/FiraMono-Medium.ttf";
}

impl FromWorld for Typography {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Unable to get AssetServer when initializing Typography resource");
        let default_font: Handle<Font> = asset_server.load(Typography::DEFAULT_FONT);

        Typography {
            heading: TextStyle {
                font: default_font.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
            body: TextStyle {
                font: default_font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            },
            default_font,
        }
    }
}
