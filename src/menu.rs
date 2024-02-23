use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use belly::prelude::*;

pub fn spawn_menu_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/built_in/Main_Menu.ldtk"),
        ..default()
    });
}

pub fn draw_menu_ui(
    mut commands: Commands
) {
    // image spawns
    let logo = commands.spawn_empty().id();
    let play = commands.spawn_empty().id();
    let settings = commands.spawn_empty().id();
    let quit = commands.spawn_empty().id();

    commands.add(StyleSheet::load("stylesheets/Bass.ess"));
    commands.add(eml! {
        <body>
            <div c:wrapper>
                <img {logo} c:logo src="textures/Bass-Master-Logo.png" mode="fit"/>
                <button c:control>
                    <img {play} c:image src="textures/Play-Logo.png" mode="fit"/>
                </button>
                <button c:control>
                    <img {settings} c:image src="textures/Settings-Logo.png" mode="fit"/>
                </button>
                <button c:control>
                    <img {quit} c:image src="textures/Quit-Logo.png" mode="fit"/>
                </button>
            </div>
        </body>
    });
}
