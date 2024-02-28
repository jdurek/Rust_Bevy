// NOTE - this is a placeholder 'party' component
// I'm just using this as the 'object' to move around on the minimap

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::*;
use crate::minimap::*;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Party {
    pub name: String,

}

// Couple of functions common to all parties (Player/Enemy)
impl Party {
    pub fn new(name: &str) -> Party {
        Party{name: name.to_string()}
    }

    
    // pub fn add_member();
    // pub fn remove_member(index: usize);

}

// Creates a party Entity for us to use, along with a placeholder sprite
pub fn party_setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle{
            sprite: Sprite { color: Color::BLACK, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                visibility: Visibility::Visible,
                transform: Transform {
                    translation: Vec2::new(0., 0.).extend(10.0),
                    scale: Vec3::new(7., 7., 0.),
                    ..default()
                },
                ..Default::default()
        },
        Party{
            name: "Demo".to_string(),
        },
        Position{
            x: 0, y: 0, z: 0
        }
    ));
}


const ZOOM_LEVEL: f32 = 16.0; // Number of pixels a tile occupies
const ZL: f32 = ZOOM_LEVEL;
// TODO - move this into a plugin to bundle it up neatly
pub fn party_movement_minimap(
    // TODO - modify query with component that denotes our party specifically.
    mut party: Query<(&Party, &mut Position, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    mg: Res<MapGrid>,
){
    let (party, mut pos, mut transform) = party.get_single_mut().expect("More than 1 party matched");
    
    // TODO - rewrite so we use a nice match statement?
    if input.any_pressed([KeyCode::W, KeyCode::Up]){
        // Up direction was pressed - validate and move if passed
        if mg.validate_move(&pos, 8).unwrap() {
            pos.y = pos.y + 1;
            transform.translation.y += ZOOM_LEVEL;
        }
    }
    if input.any_pressed([KeyCode::A, KeyCode::Left]){
        // Left direction was pressed
        if mg.validate_move(&pos, 4).unwrap() {
            pos.x = pos.x - 1;
            transform.translation.x -= ZOOM_LEVEL;
        }
    }
    if input.any_pressed([KeyCode::S, KeyCode::Down]){
        // Down direction was pressed
        if mg.validate_move(&pos, 2).unwrap() {
            pos.y = pos.y - 1;
            transform.translation.y -= ZOOM_LEVEL;
        }
    }
    if input.any_pressed([KeyCode::D, KeyCode::Right]){
        // Right direction was pressed
        if mg.validate_move(&pos, 6).unwrap() {
            pos.x = pos.x + 1;
            transform.translation.x += ZOOM_LEVEL;
        }
    }
}