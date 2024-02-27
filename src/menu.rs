use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use belly::prelude::*;
use std::{io, fs};
use std::path::Path;

pub fn spawn_menu_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/built_in/Main_Menu.ldtk"),
        ..default()
    });
}

pub fn find_world_files (
    mut commands: Commands,
) {
    let level_path = "./assets/levels/built_in/";

    for entry in fs::read_dir(Path::new(level_path)).expect("for loop entry error") {
        let entry  = entry.expect("entry error");
        println!("{:?}", entry.path());
    }
}

fn toggle_play_menu(
    ctx: &mut EventContext<impl Event>,
) {
    ctx.select(".main-wrapper").toggle_class("hidden");
    ctx.select(".play-wrapper").toggle_class("hidden");
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
            <img {logo} c:logo src="textures/Bass-Master-Logo.png" mode="fit"/>
            <div c:main-wrapper>
                <button on:press=toggle_play_menu c:control>
                    <img {play} c:image src="textures/Play-Logo.png" mode="fit"/>
                </button>
                <button c:control>
                    <img {settings} c:image src="textures/Settings-Logo.png" mode="fit"/>
                </button>
                <button c:control>
                    <img {quit} c:image src="textures/Quit-Logo.png" mode="fit"/>
                </button>
            </div>
            <div class="play-wrapper hidden">
                <span>"Penis"</span>
            </div>
        </body>
    });
}
