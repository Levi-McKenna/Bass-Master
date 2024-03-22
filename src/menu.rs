use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::app::AppExit;
use belly::prelude::*;
use std::fs;
use std::error::Error;
use std::path::Path;
use crate::{GameState};

// QuitEvent used in later function, close_event.
#[derive(Event)]
pub struct QuitEvent;

// ExitLevelEvent used in later function, exit_level_event
#[derive(Event)]
pub struct ExitLevelEvent;

#[derive(Event)]
pub struct PlayEvent;

// WorldEvent used to keep track of what world the player selects and is used in function
// insert_world_dir in src/levels.rs.
#[derive(Event)]
pub struct WorldEvent(pub String);

// Find all world files and strip them of their ./assets prefix and return with a Result.
fn find_world_files() -> Result<Vec<String>, Box<dyn Error>> {
    let level_path = "./assets/levels/built_in/";
    let mut levels: Vec<String> = Vec::new();

    for entry in fs::read_dir(Path::new(level_path))? {
        let entry = entry?.path();

        // strip /assets for easier file finding
        let entry = entry.as_path();
        if entry.extension().is_some_and(|extension| extension == "ldtk") {
            let entry = entry.strip_prefix("./assets")?;
            levels.push(entry.to_str().unwrap().to_string());
        }
    }

    Ok(levels)
}

pub fn load_main_menu(
    mut change_game_state: ResMut<NextState<GameState>>,
) {
    change_game_state.set(GameState::MenuAssetLoading);
}

// Toggle between presenting options to levels.
fn toggle_play_menu(
    ctx: &mut EventContext<impl Event>,
) {
    ctx.select(".main-wrapper").toggle_class("hidden");
    ctx.select(".play-wrapper").toggle_class("hidden");
}

// If an event from UI is sent to quit the game, then send the event for bevy to read and quit the
// game.
pub fn close_event(
    mut exit: EventWriter<AppExit>,
    mut quit_event: EventReader<QuitEvent>,
) {
    for quit in quit_event.iter() {
        exit.send(AppExit);
    }
}

// If an event from UI is sent to exit the level, then change the game state
pub fn exit_level_event(
    mut exit_event: EventReader<ExitLevelEvent>,
    mut change_game_state: ResMut<NextState<GameState>>, 
) {
    for exit in exit_event.iter() {
        change_game_state.set(GameState::Ending);
    }
}

pub fn play_event(
    mut play_event: EventReader<PlayEvent>,
    mut change_game_state: ResMut<NextState<GameState>>,
) {
    for play in play_event.iter() {
        change_game_state.set(GameState::InGame);
    }
}

// Despawn the body from the struct, Elements, that Belly provides.
pub fn despawn_ui(
    mut elements: Elements,
) {
    elements.select("body").remove();
}

// Using Belly, create a main menu ui.
pub fn draw_main_menu_ui(
    mut commands: Commands
) {
    // image spawns
    let logo = commands.spawn_empty().id();
    let play = commands.spawn_empty().id();
    let settings = commands.spawn_empty().id();
    let quit = commands.spawn_empty().id();
    let exit = commands.spawn_empty().id();

    // Fetch worlds
    let worlds = find_world_files().unwrap();

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
                <button c:control on:press=|ctx| ctx.send_event(QuitEvent)>
                    <img {quit} c:image src="textures/Quit-Logo.png" mode="fit"/>
                </button>
            </div>
            <div class="play-wrapper hidden">
                <button on:press=toggle_play_menu c:control><img {exit} src="textures/Back-Logo.png" mode="fit" c:image/></button>
                <for world in=worlds>
                    <button c:control on:press=move |ctx| ctx.send_event(WorldEvent(world.clone()))><strong>{world.clone()}</strong></button>
                </for>
            </div>
        </body>
    });
}

pub fn draw_game_menu_ui(
    mut commands: Commands,
) {
    let logo = commands.spawn_empty().id();
    let play = commands.spawn_empty().id();
    let settings = commands.spawn_empty().id();
    let quit = commands.spawn_empty().id();
    let exit = commands.spawn_empty().id();

    commands.add(StyleSheet::load("stylesheets/Bass.ess"));
    commands.add(eml! {
        <body>
/*             <img {logo} c:logo src="textures/Bass-Master-Logo.png" mode="fit"/> */
            <div c:game-wrapper>
                <button c:control on:press=|ctx| ctx.send_event(PlayEvent)>
                    <img {play} c:image src="textures/Play-Logo.png" mode="fit"/>
                </button>
                <button c:control>
                    <img {settings} c:image src="textures/Settings-Logo.png" mode="fit"/>
                </button>
                <button c:control on:press=|ctx| ctx.send_event(ExitLevelEvent)>
                    <img {exit} c:image src="textures/Quit-Logo.png" mode="fit"/>
                </button>
            </div>
        </body>
    });
    
}
