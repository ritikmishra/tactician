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

pub struct Materials {
    pub ship_img: Handle<ColorMaterial>,
    pub planet_img: Handle<ColorMaterial>,
    pub missile_img: Handle<ColorMaterial>,
    pub explosion_frames: Handle<TextureAtlas>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Unable to get AssetServer when initializing Materials resource");

        let planet_handle = asset_server.load("images/planet.png");
        let missile_handle = asset_server.load("images/missile.png");
        let ship_handle = asset_server.load("images/ship.png");
        let explosion_frames_handle = asset_server.load("images/explosion_spritesheet.png");

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().expect(
            "Unable to (mutably) get Assets<ColorMaterial> when initializing Materials resource",
        );

        let planet_material = materials.add(planet_handle.into());
        let missile_material = materials.add(missile_handle.into());
        let ship_material = materials.add(ship_handle.into());

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().expect(
            "Unable to (mutably) get Assets<TextureAtlas> when initializing Materials resource",
        );

        let number_frames = 32;
        let texture_atlas =
            TextureAtlas::from_grid(explosion_frames_handle, Vec2::splat(300.), 1, number_frames);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Materials {
            ship_img: ship_material,
            planet_img: planet_material,
            missile_img: missile_material,
            explosion_frames: texture_atlas_handle,
        }
    }
}
