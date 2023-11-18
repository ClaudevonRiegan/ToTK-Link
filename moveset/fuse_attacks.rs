use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use std::arch::asm;

use crate::moveset::*;

unsafe fn get_params(agent: &mut L2CAgentBase) -> (f32,u64,i32,f32) {
    WorkModule::on_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    let item_id = WorkModule::get_int64(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id as u32);
    let length = WorkModule::get_float(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
    let effect = WorkModule::get_int64(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
    let sound = WorkModule::get_int(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    let power = if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_KILLSWORD)
    && WorkModule::is_flag(item_boma, ITEM_KILLSWORD_INSTANCE_WORK_FLAG_IS_CRITICAL_ATTACK) {
        2.0
    }
    else if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_CLUB) {
        1.5
    }
    else if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_HOMERUNBAT) {
        2.0
    }
    else {
        1.0
    };
    return (length,effect as u64,sound,power);
}

#[acmd_script(agent = "link", script = "game_attack11", category = ACMD_GAME)]
pub unsafe fn attack_11(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    sv_animcmd::frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_animcmd::frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 25, 0, 25, 1.8, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(7.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 25, 0, 25, 1.8, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(7.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 180, 15, 0, 20, 1.8, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(7.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 15, 0, 20, 1.8, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(7.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("top"), 3.0 * power, 361, 15, 0, 20, 1.8, 0.0, 8.0, 22.0 + length, Some(0.0), Some(8.0), Some(7.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script(agent = "link", script = "game_attack12", category = ACMD_GAME)]
pub unsafe fn attack_12(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 25, 0, 25, 2.8, 0.0, 8.5, 9.5, Some(0.0), Some(8.5), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 25, 0, 25, 3.5, 0.0, 8.5, 14.5, Some(0.0), Some(8.5), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 180, 15, 0, 20, 3.5, 0.0, 8.5, 18.5, Some(0.0), Some(8.5), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 15, 0, 20, 3.5, 0.0, 8.5, 18.5, Some(0.0), Some(8.5), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("top"), 3.0 * power, 361, 15, 0, 20, 3.5, 0.0, 8.0, 18.5 + length, Some(0.0), Some(8.0), Some(8.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script(agent = "link", script = "game_attack13", category = ACMD_GAME)]
pub unsafe fn attack_13(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 30, 70, 0, 70, 4.5, 0.0, 8.0, 10.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 30, 70, 0, 70, 4.5, 0.0, 8.0, 15.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 4.0, 30, 70, 0, 70, 3.8, 11.0, 0.0, -1.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 3, 0, Hash40::new("sword1"), 4.0 * power, 30, 70, 0, 70, 3.8, 14.0, 0.0, -1.5, Some(14.0 + length), Some(0.0), Some(-1.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword1"), 4.0 * power, 30, 70, 0, 70, 3.8, 14.0, 0.0, -6.0, Some(14.0 + length), Some(0.0), Some(-6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attacks3", category = ACMD_GAME)]
pub unsafe fn attack_s3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 361, 82, 0, 55, 4.0, 1.8, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 361, 82, 0, 55, 2.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("colonells"), 13.0, 361, 82, 0, 55, 2.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 13.0, 361, 82, 0, 55, 3.5, 7.8, 0.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 13.0 * power, 361, 82, 0, 55, 3.5, 11.8, 0.0, -1.0, Some(11.8 + length), Some(0.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 13.0 * power, 361, 82, 0, 55, 3.5, 11.8, 0.0, -5.0, Some(11.8 + length), Some(0.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attackhi3", category = ACMD_GAME)]
pub unsafe fn attack_hi3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 11.0, 95, 103, 0, 30, 3.4, 8.4, 0.0, -2.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 11.0, 85, 111, 0, 30, 4.3, 2.0, 0.0, -3.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 11.0, 85, 105, 0, 30, 2.7, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("colonells"), 11.0, 85, 104, 0, 30, 2.16, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 11.0 * power, 95, 103, 0, 30, 3.4, 12.4, 0.0, -1.0, Some(12.4 + length), Some(0.0), Some(-1.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 11.0 * power, 85, 111, 0, 30, 4.3, 12.4, 0.0, -5.0, Some(12.4 + length), Some(0.0), Some(-5.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attacklw3", category = ACMD_GAME)]
pub unsafe fn attack_lw3(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.7);
    sv_animcmd::frame(agent.lua_state_agent, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 85, 30, 0, 90, 2.0, 0.0, 1.3, 17.5, Some(0.0), Some(5.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0 * power, 85, 30, 0, 90, 2.0, 0.0, 1.3, 17.5 + length, Some(0.0), Some(5.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attackdash", category = ACMD_GAME)]
pub unsafe fn attack_dash(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 14.0, 45, 85, 0, 70, 3.2, 8.5, 0.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 50, 85, 0, 85, 3.5, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 55, 77, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.4);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 3, 0, Hash40::new("sword2"), 14.0 * power, 45, 85, 0, 70, 3.2, 12.5, 0.0, -1.0, Some(12.5 + length), Some(0.0), Some(-1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 14.0 * power, 45, 85, 0, 70, 3.2, 12.5, 0.0, -4.0, Some(12.5 + length), Some(0.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attacks4", category = ACMD_GAME)]
pub unsafe fn attack_s4(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 1.5);
    sv_animcmd::frame(agent.lua_state_agent, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_animcmd::frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, 69, 12, 0, 45, 4.3, 2.7, -2.0, -2.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 14.0, 361, 100, 0, 30, 3.5, 8.5, -2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 60, 12, 0, 45, 3.2, 0.0, 8.5, 6.0, Some(0.0), Some(8.5), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_SWORD_BEAM, false, -1);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 3, 0, Hash40::new("sword2"), 14.0 * power, 361, 100, 0, 30, 3.5, 12.0, -2.0, -1.0, Some(12.0 + length), Some(-2.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 14.0 * power, 361, 100, 0, 30, 3.5, 12.0, -2.0, -4.0, Some(12.0 + length), Some(-2.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
    sv_animcmd::wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script(agent = "link", script = "game_attacks4s2", category = ACMD_GAME)]
pub unsafe fn attack_s4_s2(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 13.0, 48, 89, 0, 85, 4.5, 7.0, 2.3, -2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 48, 89, 0, 85, 4.5, 1.5, 2.3, -2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 48, 89, 0, 85, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 48, 89, 0, 85, 3.5, 0.0, 9.0, 3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 13.0 * power, 48, 89, 0, 85, 4.5, 12.0, 2.3, -1.5, Some(12.0 + length), Some(2.3), Some(-1.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 13.0 * power, 48, 89, 0, 85, 4.5, 12.0, 2.3, -4.5, Some(12.0 + length), Some(2.3), Some(-4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attackhi4", category = ACMD_GAME)]
pub unsafe fn attack_hi4(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("colonells"), 4.0, 105, 100, 55, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 4.0, 115, 100, 38, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"), 4.0, 107, 100, 28, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 4.0, 115, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword2"), 4.0, 107, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 3.0, false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 4.0 * power, 115, 100, 28, 0, 3.0, 12.0, 0.0, -1.2, Some(12.0 + length), Some(0.0), Some(-1.2), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 6, 0, Hash40::new("sword2"), 4.0 * power, 115, 100, 28, 0, 3.0, 12.0, 0.0, -5.5, Some(12.0 + length), Some(0.0), Some(-5.5), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 4.0, 135, 100, 45, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
    sv_animcmd::frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("colonells"), 3.0, 105, 100, 55, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 3.0, 115, 100, 38, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"), 3.0, 107, 100, 28, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 3.0, 115, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("sword2"), 3.0, 107, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 3.0, false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 3.0 * power, 115, 100, 28, 0, 3.0, 12.0, 0.0, -1.2, Some(12.0 + length), Some(0.0), Some(-1.2), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 6, 0, Hash40::new("sword2"), 3.0 * power, 115, 100, 28, 0, 3.0, 12.0, 0.0, -5.5, Some(12.0 + length), Some(0.0), Some(-5.5), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_S, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("sword2"), 3.0, 135, 100, 40, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
    sv_animcmd::frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 11.0, 80, 101, 0, 60, 4.3, 7.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 11.0, 90, 101, 0, 60, 4.3, 1.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 90, 101, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 90, 101, 0, 60, 5.2, 0.0, 11.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 11.0 * power, 80, 101, 0, 60, 4.3, 11.0, 0.0, -1.5, Some(11.0 + length), Some(0.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 11.0 * power, 80, 101, 0, 60, 4.3, 11.0, 0.0, -6.0, Some(11.0 + length), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attacklw4", category = ACMD_GAME)]
pub unsafe fn attack_lw4(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 2.0);
    sv_animcmd::frame(agent.lua_state_agent, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_animcmd::frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 16.0, 78, 88, 0, 40, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 17.0, 78, 88, 0, 40, 3.6, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 16.0, 78, 88, 0, 40, 3.4, 1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("colonells"), 14.0, 78, 88, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("top"), 17.0 * power, 78, 88, 0, 40, 3.6, 0.0, 3.0, 17.5, Some(0.0), Some(3.0), Some(17.5 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
    sv_animcmd::frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 12.0, 30, 63, 0, 80, 3.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 12.0, 30, 63, 0, 80, 3.6, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 11.0, 30, 63, 0, 80, 3.4, 1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("colonells"), 10.0, 30, 63, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 4, 0, Hash40::new("sword2"), 12.0 * power, 30, 63, 0, 80, 3.6, 12.0, 0.0, 0.0, Some(12.0 + length), Some(8.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("sword2"), 12.0 * power, 30, 63, 0, 80, 3.6, 12.0, 0.0, -3.0, Some(12.0 + length), Some(8.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_attackairf", category = ACMD_GAME)]
pub unsafe fn attack_air_f(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(agent.lua_state_agent, 2.0);
    FT_MOTION_RATE(agent, 1.2);
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    sv_animcmd::frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 43, 28, 0, 31, 4.0, 0.0, 11.5, 18.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 367, 30, 0, 25, 4.0, 0.0, 11.5, 18.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 47, 30, 0, 33, 6.5, 0.0, 11.5, 11.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 367, 30, 0, 25, 6.5, 0.0, 11.5, 11.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 8.0, 47, 28, 0, 33, 7.0, 0.0, 11.5, 6.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 5, 0, Hash40::new("top"), 8.0, 38, 26, 0, 26, 7.0, 0.0, 11.5, 6.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 6, 0, Hash40::new("top"), 8.0 * power, 47, 28, 0, 33, 7.0, 0.0, 11.5, 16.0, Some(0.0), Some(11.5), Some(16.0 + length), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 7, 0, Hash40::new("top"), 8.0 * power, 38, 26, 0, 26, 7.0, 0.0, 11.5, 16.0, Some(0.0), Some(11.5), Some(16.0 + length), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 44, 120, 0, 43, 4.5, 0.0, 11.0, 17.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 44, 120, 0, 43, 6.5, 0.0, 11.0, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 44, 120, 0, 43, 6.8, 0.0, 11.0, 6.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 3, 0, Hash40::new("top"), 10.0 * power, 44, 120, 0, 43, 4.5, 0.0, 11.0, 19.0, Some(0.0), Some(6.0), Some(19.0 + length), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
    sv_animcmd::frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(agent = "link", script = "game_attackairhi", category = ACMD_GAME)]
pub unsafe fn attack_air_hi(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 15.0, 85, 93, 0, 23, 4.5, 6.9, 0.0, 0.0, Some(-1.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("sword2"), 15.0 * power, 85, 93, 0, 23, 4.5, 11.0, 0.0, 0.0, Some(11.0 + length), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 13.0, 85, 88, 0, 18, 4.5, 6.9, 0.0, 0.0, Some(-1.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_POWER(agent, 0, 13);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0 * power, 85, 88, 0, 18, 4.5, 11.0, 0.0, 0.0, Some(11.0 + length), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(agent = "link", script = "game_attackairlw", category = ACMD_GAME)]
pub unsafe fn attack_air_lw(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 3.0, 7.0, 3.0);
    }
    sv_animcmd::frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 270, 80, 0, 30, 4.0, 1.5, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 60, 78, 0, 30, 4.0, 1.5, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 2, 0, Hash40::new("top"), 18.0 * power, 270, 80, 0, 30, 4.0, 1.5, -3.0, 0.0, Some(1.5), Some(-3.0 - length), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 18.0 * power, 60, 78, 0, 30, 4.0, 1.5, -3.0, 0.0, Some(1.5), Some(-3.0 - length), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 60, 80, 0, 30, 4.2, 1.0, 1.5, 0.0, Some(1.0), Some(4.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 2, 0, Hash40::new("top"), 15.0 * power, 60, 80, 0, 30, 4.2, 1.0, -4.0, 0.0, Some(1.0), Some(-4.0 - length), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_L, sound, *ATTACK_REGION_SWORD);
            AttackModule::clear(agent.module_accessor, 3, false);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(agent = "link", script = "game_cliffattack", category = ACMD_GAME)]
pub unsafe fn cliff_attack(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 15.5, Some(0.0), Some(5.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0 * power, 45, 20, 0, 90, 5.0, 0.0, 5.0, 15.5, Some(0.0), Some(5.0), Some(15.5 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_downattackd", category = ACMD_GAME)]
pub unsafe fn down_attack_d(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, -15.0, Some(0.0), Some(5.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 * power, 48, 48, 0, 80, 5.0, 0.0, 5.0, -4.0, Some(0.0), Some(5.0), Some(-4.0 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 * power, 48, 48, 0, 80, 5.0, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(17.0 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_downattacku", category = ACMD_GAME)]
pub unsafe fn down_attack_u(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, -17.0, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 * power, 48, 48, 0, 80, 5.0, 0.0, 5.0, -5.0, Some(0.0), Some(5.0), Some(-5.0 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0 * power, 48, 48, 0, 80, 5.0, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(19.0 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}

#[acmd_script(agent = "link", script = "game_slipattack", category = ACMD_GAME)]
pub unsafe fn slip_attack(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, 24.0, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0 * power, 361, 50, 0, 60, 5.0, 0.0, 5.0, 24.0, Some(0.0), Some(5.0), Some(24.0 + length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    sv_animcmd::frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, -19.5, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        if is_excute(agent) {
            let (length,effect,sound,power) = get_params(agent);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0 * power, 361, 50, 0, 60, 5.0, 0.0, 5.0, -19.5, Some(0.0), Some(5.0), Some(-19.5 - length), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(effect), *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
    }
}