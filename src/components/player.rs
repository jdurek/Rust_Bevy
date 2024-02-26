/* Player.rs
 *  contains basic components a player entity would have
 *  Due to the design, this would be reused for everything that could fight
 *      Might be better to rename away from player to something else? 
 */


// List of components to implement for the player entity - 
// Health system (HP, MP, and other values)
// Skill/ability system
// Rough AI for combat (How they use skills)
// Equipment (Has a set of inventory slots with specific locks on them)
// Location (Player and NPCs all need a location)
//     Player will usually defer to the party location, so this may be NPC specific

// List of components for NPC entities - 
// Combat sprite
// Exp provided
// Drop Pool


// Brainstorming for handling skills - 
// For class specific, run a query and return what's available for our level?
//  This wouldn't work for a skill-tree where we'd need to record what's been taken...
//  Could define a JSON with skill details (derive from skill struct)
// For char specific, we'd need to trawl a file to figure that out...  

use crate::components::*;

 #[derive(Component, Serialize, Deserialize, Copy, Clone)]
 pub struct Player {
    // health: Health,
    // blue: Mana,
    // skills: - TODO, figure out how to handle these
    
 }