use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

mod marching_squares;

#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Marching Squares".to_string(),
				width: 1200.0,
				height: 800.0,
				..Default::default()
			},
			..Default::default()
		}))
        .add_startup_system(setup)
		.add_plugin(PanCamPlugin::default())
        .add_plugin(marching_squares::MarchingSquaresPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {
	//camera
	commands.spawn(Camera2dBundle::default())
    .insert(PanCam {
        grab_buttons: vec![MouseButton::Middle], // which buttons should drag the camera
        enabled: true, // when false, controls are disabled. See toggle example.
        zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
        min_scale: 0.3, // prevent the camera from zooming too far in
        max_scale: Some(40.), // prevent the camera from zooming too far out
		..default()
    })
	.insert(MainCamera);

	//window resource
    let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

    let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);
}