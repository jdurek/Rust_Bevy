/* Party.rs
 * Generic implementation of the player's current 'party' - Also serves to centralize components
 * EG - party will be made up of X players, have party-inventory, and the location component
 * ECS makes this a bit hard to wrap my head around, but the party itself is what's moving
 * 
 *  Components of a party - 
 *      Location (coordinate)
 *      
 * 
 * Storing items is going to be the tricky part I guess...
 *      Could split between ECS for certain things?
 *      Consumables and equipables are separate (Stackable component, etc...)
 */


// TODO - figure out how to store a list of entities - 
// I specifically want to associate some arbitrary number of "party members",
// Or to keep this simpler, items with Player component. 

use crate::components::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Party {
    pub loc: Position,
    pub name: String,
    pub reknown: i32,
    pub members: Vec<Player>,
}

// Initialize party with no members
pub fn create_party(mut commands: Commands) {

}

// Initialize enemy party for fight
pub fn create_enemy_party(
    mut commands: Commands,
    // Query the map/region to figure out which groups we can generate
){
    // Create Party struct - Only thing that's necessary is the members (List of enemies)
    // Append the 'In combat' state for now (We only make an enemy party if we're fighting them)
    
}

