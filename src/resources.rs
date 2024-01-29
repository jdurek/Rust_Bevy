use crate::prelude::*;

/* Defines global resources, particularly game states */


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    EnterDungeon,   // This also applies to traversal between floors/areas
    AwaitingInput,
    InMenus,
    PlayerTurn,     // May rename this due to how I wanted to handle "Unlimited movement within a tile" down the line, but it is their turn still
    EnemyTurn,
    OtherTurn, // Terrain changes, etc...
    EnterCombat,
    EnterDialogue,  // This could be a variant of InMenus, 
    ExitDungeon,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    UpperView,
    AutoHeal,
    Party,
    Equipment,
    Skills,
    Map,
    Questbook, // May rename this since it won't just be quests but other things? 
    Options,
    SaveLoad,
}