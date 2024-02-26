// use crate::prelude::*;
use bevy::prelude::*;

/* Defines global resources, particularly game states */

// Enter dungeon, main loop (Wait for input, menu | player turn | dialogue | enter combat | exit dungeon, enemy turn | enter combat, other turn, back to waiting on input)
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

// A quick way for us to 'disable' exploration-based ticks if it's not tied to the grid movement, or just prevent inputs from registering
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameplayState {
    #[default]
    MainMenu,
    Menu,
    Exploration,
    Dialogue,
    Cutscene,
}

// Simple states for the map_builder loop
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MapBuildState {
    #[default]
    RenderMap,
    LoadingMap,
    SavingMap,
    Drawing,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CombatState {
    #[default]
    EnteredCombat,
    Planning,
    Computing,
    Executing,
    ExitingCombat,  // Includes Fleeing and winning
}