use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "dedede", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 
        0, 
        0, 
        Hash40::new("hammer2"), /*bone*/
        15.0, /*damage*/
        361, /*angle*/
        95, 0, 15, /*kbg, fkb, bkb*/
        7.2, /*size*/
        0.0, 0.0, 0.0, /*postion in x,y,z*/
        None, None, None, /*x2, y2, z2*/
        1.0, /*hitlag*/
        1.0, /*sdi*/
        *ATTACK_SETOFF_KIND_ON, 
        *ATTACK_LR_CHECK_F, 
        false, 
        0, /*shield dmg*/
        0.0, /*trip*/
        0, /*rehit*/
        false, 
        false, 
        false, 
        false, 
        true, /*reflect, absorb, flinchless, disable hitlag, direct hitbox*/
        *COLLISION_SITUATION_MASK_GA,
        *COLLISION_CATEGORY_MASK_ALL, 
        *COLLISION_PART_MASK_ALL, 
        true, /*friendly fire*/
        Hash40::new("collision_attr_dedede_hammer"), /*effect*/
        *ATTACK_SOUND_LEVEL_L, /*sfx level*/
        *COLLISION_SOUND_ATTR_DEDEDE, /*sfx type*/
        *ATTACK_REGION_HAMMER); /*type*/

        macros::ATTACK(agent, 1, 0, Hash40::new("hammer2"), 
        12.0, 
        361, 
        95, 0, 15, 
        3.0, 
        -11.0, 0.0, 0.0, 
        Some(-13.0), None, None, 
        1.0, 
        1.0, 
        *ATTACK_SETOFF_KIND_ON, 
        *ATTACK_LR_CHECK_POS, 
        false, 
        0, 
        0.0, 
        0, 
        false, false, false, false, true, 
        *COLLISION_SITUATION_MASK_GA, 
        *COLLISION_CATEGORY_MASK_ALL, 
        *COLLISION_PART_MASK_ALL, 
        true, 
        Hash40::new("collision_attr_dedede_hammer"), 
        *ATTACK_SOUND_LEVEL_L, 
        *COLLISION_SOUND_ATTR_DEDEDE, 
        *ATTACK_REGION_HAMMER);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


#[acmd_script( agent = "dedede", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dedede_hammer_body"), Hash40::new("hammer2"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dedede_hammer_arc_wind_c"), Hash40::new("top"), 0, 15, 0, 0, -62, 75, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2.3);

    
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("dedede_hammer_arc_wind_c"), Hash40::new("top"), 0, 15, 0, 0, 0, 75, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.9);
    }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("dedede_hammer_body"), false, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("dedede_hammer_arc_wind"), -1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("dedede_hammer_arc_wind"), true, true);
    }
    
}

pub fn install() {
   
    smashline::install_acmd_scripts!(
        game_attackairf,
        effect_attackairf
    );
}
