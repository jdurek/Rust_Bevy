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


/* List of common components I expect to use in conjunction with a Party
    Position
    InCombat
    DisplaySprite (For showing on the map/game)
*/
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Party {
    pub name: String,
    pub reknown: i32,
    pub members: Vec<Option<Entity>>,   // Contains party members - can iterate over this to fetch entities instead of querying in some calls
}

// General idea is if a party is InCombat, we can query to find the 2 parties with InCombat, and fetch all entities involved from these 2 parties.


// Initialize party with no members
pub fn create_party(mut commands: Commands) {

}

pub fn add_member(
    mut commands: Commands, 
    // Query our specific player entity somehow- how do we identify it?
    // How would that entity get the component we're querying on? 
){

}

// NPCs would have a lot in common with party - needs access to dialogue tree
// Some NPCs could join the party as guests, so they'd need their own stats
pub struct NPC {
    pub party: Party,
    // pub dialogue_tree: Dialogue,
}