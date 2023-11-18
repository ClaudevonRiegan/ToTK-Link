use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase,L2CFighterAnimcmdGameCommon,L2CFighterAnimcmdEffectCommon};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use crate::FIGHTER_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
use std::arch::asm;

use crate::moveset::*;

unsafe fn set_fuse_params(fighter: &mut L2CFighterCommon) {
    let kind = ItemModule::get_have_item_kind(fighter.module_accessor,0);
    if kind == *ITEM_KIND_STARROD {
        WorkModule::set_float(fighter.module_accessor,10.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_magic") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_MAGIC,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_KILLSWORD {
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_KILLSWORD);
        WorkModule::set_float(fighter.module_accessor,18.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_cutup") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_CUTUP,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_BEAMSWORD {
        WorkModule::set_float(fighter.module_accessor,8.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_cutup") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_CUTUP,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_HOMERUNBAT {
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_HOMERUNBAT);
        WorkModule::set_float(fighter.module_accessor,12.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_normal") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_BAT,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_RIPSTICK {
        WorkModule::set_float(fighter.module_accessor,6.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_flower") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_MAGIC,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_FIREBAR {
        WorkModule::set_float(fighter.module_accessor,25.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_fire") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_FIRE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_CLUB {
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_CLUB);
        WorkModule::set_float(fighter.module_accessor,22.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_normal") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_KICK,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_DEATHSCYTHE {
        WorkModule::set_float(fighter.module_accessor,18.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_deathscythe") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_CUTUP,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else if kind == *ITEM_KIND_LINKBOMB {
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
        WorkModule::set_float(fighter.module_accessor,0.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_cutup") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_CUTUP,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    else {
        WorkModule::set_float(fighter.module_accessor,0.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH);
        WorkModule::set_int64(fighter.module_accessor,hash40("collision_attr_cutup") as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT);
        WorkModule::set_int(fighter.module_accessor,*COLLISION_SOUND_ATTR_CUTUP,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND);
    }
    let uses = 4 + sv_math::rand(hash40("fighter"),5);
    WorkModule::set_int(fighter.module_accessor,uses,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_USAGE);
    if kind == *ITEM_KIND_LINKBOMB {
        WorkModule::set_int(fighter.module_accessor,1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_USAGE);
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,0,0,0,0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_SHOOT as u64 | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,0,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,0);
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw"),0.0,1.0,false,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_lw"),0.0,1.0,false,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x17].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_lw"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_lw"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB) {
        ArticleModule::generate_article_have_item(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB,*FIGHTER_HAVE_ITEM_WORK_MAIN,Hash40::new("invalid"));
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,true);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
        ArticleModule::set_visibility_whole(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,true,ArticleOperationTarget(0));
    }
    original!(fighter)
}

#[acmd_script(agent = "link", scripts = ["game_speciallw","game_specialairlw"], category = ACMD_GAME)]
pub unsafe fn special_lw(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,false);
            ArticleModule::set_visibility_whole(agent.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,false,ArticleOperationTarget(0));
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
    }
    sv_animcmd::frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,true);
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),false);
            ArticleModule::set_visibility_whole(agent.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,true,ArticleOperationTarget(0));
        }
    }
}

pub unsafe fn special_lw_fuse_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,0,0,0,0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_SHOOT as u64 | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,0,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,0);
    return L2CValue::I32(0)
}

pub unsafe fn special_lw_fuse(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_lw_fuse"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw_fuse"),0.0,1.0,false,0.0,false,false);
    }
    let navi_id = ArticleModule::get_active_num(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY);
    WorkModule::set_int(fighter.module_accessor,navi_id,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
    ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,false,-1);
    let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
    let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
    LinkModule::remove_model_constraint(navi_boma,true);
    LinkModule::set_model_constraint_pos_ort(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("sword2"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32 | *CONSTRAINT_FLAG_OFFSET_ROT as u32,true);
    let offset_rot = Vector3f{x: 0.0, y: 0.0, z: -90.0};
    LinkModule::set_constraint_rot_offset(navi_boma,&offset_rot);
    let offset_pos = Vector3f{x: 0.0, y: 12.0, z: 0.0};
    LinkModule::set_constraint_translate_offset(navi_boma,&offset_pos);
    ModelModule::set_mesh_visibility(navi_boma,Hash40::new("link_ken"),true);
    MotionModule::change_motion(navi_boma,Hash40::new("effect"),0.0,1.0,false,0.0,false,false);
    let item_id = ItemModule::get_have_item_id(fighter.module_accessor,0);
    WorkModule::set_int64(fighter.module_accessor,item_id as i64,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id as u32);
    set_fuse_params(fighter);
    LinkModule::remove_model_constraint(item_boma,true);
    if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
        LinkModule::unlink(item_boma,*ITEM_LINK_NO_HAVE);
    }
    if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
        VisibilityModule::set_whole(item_boma,true);
        LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(navi_boma)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("have"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
        PLAY_SE(fighter,Hash40::new("se_link_scene_slow"));
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.4 * speed_x,0.2 * speed_y);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_fuse_loop as *const () as _))
}

unsafe fn special_lw_fuse_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x17].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_lw_fuse"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_lw_fuse"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
        LANDING_EFFECT(fighter,Hash40::new("sys_landing_smoke_s"),Hash40::new("top"),-1,0,0,0,0,0,1.5,0,0,0,0,0,0,false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn special_lw_fuse_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
    let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
    let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
    MotionModule::change_motion(navi_boma,Hash40::new("effect_hide"),0.0,1.0,false,0.0,false,false);
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) == false {
        MotionModule::remove_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
    }
    return L2CValue::I32(0)
}

#[acmd_script(agent = "link", scripts = ["game_speciallwfuse", "game_specialairlwfuse"], category = ACMD_GAME)]
pub unsafe fn special_lw_fuse(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_glow"),0.0,2.0,false,false,0.0,false,false,false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_loop"),0.0,1.0,false,false,0.0,false,false,false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_LINK)]
pub fn link(fighter: &mut L2CFighterCommon) {
    unsafe {
        //fuse
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK) {
                if AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction(fighter.module_accessor,*COLLISION_KIND_MASK_SHIELD) {
                    WorkModule::dec_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_USAGE);
                    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
                }
            }
            if WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_USAGE) <= 0 {
                if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
                    check_bomb_fuse(fighter);
                    disable_fused_item(fighter);
                    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),true);
                    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
                    let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
                    if navi_id > -1 {
                        let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
                        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                        ArticleModule::remove_exist_object_id(fighter.module_accessor,(*(navi_boma)).battle_object_id);
                        WorkModule::set_int(fighter.module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
                    }
                }
                else {
                    disable_fused_item(fighter);
                    remove_fused_item(fighter);
                    PLAY_SE(fighter,Hash40::new("se_link_mipha_grace"));
                }
            }
        }
        //bomb
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
            let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            if sv_battle_object::is_active(item_id) == false {
                disable_fused_item(fighter);
                let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
                if navi_id > -1 {
                    let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
                    let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                    ArticleModule::remove_exist_object_id(fighter.module_accessor,(*(navi_boma)).battle_object_id);
                    WorkModule::set_int(fighter.module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
                }
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),true);
                WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
            }
        }
        //hover stone
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE) {
            if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_TIME_DEC) {
                let id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_ID);
                let hover_stone = sv_battle_object::module_accessor(id as u32);
                HitModule::set_whole(hover_stone,HitStatus(*HIT_STATUS_XLU),0);
                WorkModule::dec_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_REMOVE_TIMER);
                if WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_REMOVE_TIMER) <= 0 {
                    PLAY_SE(fighter,Hash40::new("se_link_parry"));
                    remove_hover_stone(&mut *fighter.module_accessor);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_DESPAWN) {
            WorkModule::inc_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_NEW_WAIT);
            if WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_NEW_WAIT) >= 10 {
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_DESPAWN);
                    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE);
                    WorkModule::set_int(fighter.module_accessor,0,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_NEW_WAIT);
                }
            }
        }
        //eff
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            if MotionModule::motion_kind_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY) == hash40("arm_fade")
            && MotionModule::frame_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY) == 30.0 {
                WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
                MotionModule::remove_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            }
        }
        //demo
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        if smash::app::lua_bind::FighterManager::is_result_mode(fighter_manager) {
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),true);
        }
        //boomerang
        if ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) == false {
            if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
                WorkModule::set_int(fighter.module_accessor,*BATTLE_OBJECT_ID_INVALID,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                WorkModule::set_int(fighter.module_accessor,*ITEM_KIND_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
            else {
                let boomerang_fuse_item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID) as u32;
                if sv_battle_object::is_active(boomerang_fuse_item_id) {
                    let item_boma = sv_battle_object::module_accessor(boomerang_fuse_item_id);
                    if StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
                        if utility::get_kind(&mut *item_boma) != *ITEM_KIND_LINKBOMB {
                            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
                            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,boomerang_fuse_item_id);
                        }
                        else {
                            StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_THROW,false);
                            WorkModule::set_int(fighter.module_accessor,*BATTLE_OBJECT_ID_INVALID,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                            WorkModule::set_int(fighter.module_accessor,*ITEM_KIND_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
                        }
                    }
                }
            }
            if ItemModule::is_have_item(fighter.module_accessor,0) {
                WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
            }
        }
    }
}

unsafe fn check_bomb_fuse(fighter: &mut L2CFighterCommon) {
    let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let bomb = sv_battle_object::module_accessor(item_id);
    LinkModule::remove_model_constraint(bomb,true);
    if LinkModule::is_link(bomb,*ITEM_LINK_NO_HAVE) {
        LinkModule::unlink_all(bomb);
    }
    StatusModule::change_status_request(bomb,*ITEM_STATUS_KIND_BORN,false);
    WorkModule::set_int(fighter.module_accessor,*BATTLE_OBJECT_ID_INVALID,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
}

unsafe fn disable_fused_item(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_CLUB);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_HOMERUNBAT);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_KILLSWORD);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK);
}

unsafe fn remove_fused_item(fighter: &mut L2CFighterCommon) {
    let item_id = WorkModule::get_int64(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id as u32);
    LinkModule::remove_model_constraint(item_boma,true);
    if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
        LinkModule::unlink_all(item_boma);
    }
    let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
    if navi_id > -1 {
        let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        ArticleModule::remove_exist_object_id(fighter.module_accessor,(*(navi_boma)).battle_object_id);
        WorkModule::set_int(fighter.module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
    }
    WorkModule::set_int(fighter.module_accessor,*BATTLE_OBJECT_ID_INVALID,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id as u32);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),true);
}

pub unsafe fn check_throw_or_fuse(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.global_table[0x1f].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) != 0 {
        let stick_y = fighter.global_table[0x1b].get_f32() * -1.0;
        if WorkModule::get_param_float(fighter.module_accessor,hash40("common"),hash40("special_stick_y")) <= stick_y {
            if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) == false {
                let trait_flag = ItemModule::get_have_item_trait(fighter.module_accessor,0);
                if trait_flag == *ITEM_TRAIT_FLAG_SWING as u64
                || trait_flag == *ITEM_TRAIT_FLAG_SWING_NO_THROW as u64
                || trait_flag == *ITEM_TRAIT_FLAG_SWING_NO_BOUND_REMOVE as u64
                || ItemModule::get_have_item_kind(fighter.module_accessor,0) == *ITEM_KIND_LINKBOMB {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
                        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED);
                        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_FUSE.into(),false.into());
                        return L2CValue::I32(1)
                    }
                }
            }
            else {
                if ItemModule::is_have_item(fighter.module_accessor,0) {
                    if ItemModule::get_have_item_kind(fighter.module_accessor,0) == *ITEM_KIND_LINKBOMB {
                        if WorkModule::is_enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
                            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(),false.into());
                            return L2CValue::I32(1)
                        }
                    }
                }
            }
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn enable_disable_special_lw(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor,0) {
        WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        disable_fused_item(fighter);
        remove_fused_item(fighter);
        remove_hover_stone(&mut *fighter.module_accessor);
        remove_hover_stone2(&mut *fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_DESPAWN);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE);
        WorkModule::set_int(fighter.module_accessor,*BATTLE_OBJECT_ID_INVALID,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor,*FIGHTER_STATUS_KIND_REBIRTH);
        return L2CValue::I32(1)
    }
    original!(fighter)
}