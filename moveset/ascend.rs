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

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,false);
        ArticleModule::set_visibility_whole(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,false,ArticleOperationTarget(0));
    }
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_kenb"),true);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_tate"),false);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_tateb"),true);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ASCEND_FAIL);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_hi"),0.0,1.0,false,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_hi"),0.0,1.0,false,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT) {
        let heights = [60.0,80.0,100.0];
        for height in heights {
            let pos = Vector3f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor) + height, z: PostureModule::pos_z(fighter.module_accessor)};
            let pos2f = Vector2f{x: pos.x, y: pos.y};
            let check_pos = Vector2f{x: 0.0, y: height * -1.0};
            let mut hit_pos = Vector2f{x: 0.0, y: 0.0};
            if GroundModule::ray_check_hit_pos(fighter.module_accessor,&pos2f,&check_pos,&mut hit_pos,true) {
                let floor = GroundModule::get_distance_to_floor(fighter.module_accessor,&pos,pos.y + height,true);
                WorkModule::set_float(fighter.module_accessor,PostureModule::pos_y(fighter.module_accessor),FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_HEIGHT);
                WorkModule::set_float(fighter.module_accessor,height - floor,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_HEIGHT);
                fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
                break;
            }
        }
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT);
    }
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ASCEND_FAIL)
    && WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE) == false {
        let mut params = CreateItemParam {
            founder_pos: Vector4f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor), w: 0.0},
            item_pos: Vector4f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor) + 50.0, z: PostureModule::pos_z(fighter.module_accessor), w: 0.0},
            item_kind: ItemKind(*ITEM_KIND_CARRIERBOX),
            another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
            variation_kind: *ITEM_VARIATION_CARRIERBOX_WOOD,
            lr_dir: PostureModule::lr(fighter.module_accessor),
            owner_id: (*(fighter.module_accessor)).battle_object_id,
            unk_20: 20,
            pokeball_or_assist_kind: *ITEM_KIND_NONE,
            unk_0: 0,
            weird_flag: 0x633F800000,
            unk_1_weird: 1,
            unk_approx_0: 0.0,
            unk_02: 0.0
        };
        let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
        let battle_object = create_item(item_manager,&mut params,false,false,false);
        let hover_stone = (*battle_object).module_accessor;
        let hover_id = (*hover_stone).battle_object_id;
        WorkModule::set_int(fighter.module_accessor,hover_id as i32,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_ID);
        StatusModule::change_status_request(hover_stone,*ITEM_STATUS_KIND_HAVE,false);
        KineticModule::unable_energy_all(hover_stone);
        WorkModule::set_int(fighter.module_accessor,60,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_REMOVE_TIMER);
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE);
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ASCEND_FAIL);
        WorkModule::set_float(fighter.module_accessor,PostureModule::pos_y(fighter.module_accessor),FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_HEIGHT);
        WorkModule::set_float(fighter.module_accessor,60.0,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_HEIGHT);
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x17].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_hi"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,Hash40::new("special_air_hi"),-1.0,1.0,0.0,false,false);
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
        LANDING_EFFECT(fighter,Hash40::new("sys_landing_smoke_s"),Hash40::new("top"),-1,0,0,0,0,0,1,0,0,0,0,0,0,false);
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

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD {
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) == false {
            MotionModule::remove_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
        }
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,true);
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
            ArticleModule::set_visibility_whole(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,true,ArticleOperationTarget(0));
        }
        else {
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),true);
        }
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_kenb"),false);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_tate"),true);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_tateb"),false);
    }
    original!(fighter)
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WHOLE_HIT(fighter,*HIT_STATUS_OFF);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT);
    GroundModule::set_correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_hi_ascend"),0.0,1.0,false,0.0,false,false);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,4.5);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,2.5);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    PLAY_SE(fighter,Hash40::new("se_link_stasis_end"));
    let mut params = CreateItemParam {
        founder_pos: Vector4f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor), w: 0.0},
        item_pos: Vector4f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor), w: 0.0},
        item_kind: ItemKind(*ITEM_KIND_CARRIERBOX),
        another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
        variation_kind: *ITEM_VARIATION_CARRIERBOX_FUTURE,
        lr_dir: PostureModule::lr(fighter.module_accessor),
        owner_id: (*(fighter.module_accessor)).battle_object_id,
        unk_20: 20,
        pokeball_or_assist_kind: *ITEM_KIND_NONE,
        unk_0: 0,
        weird_flag: 0x633F800000,
        unk_1_weird: 1,
        unk_approx_0: 0.0,
        unk_02: 0.0
    };
    let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
    let battle_object = create_item(item_manager,&mut params,false,false,false);
    let hover_stone = (*battle_object).module_accessor;
    let hover_id = (*hover_stone).battle_object_id;
    WorkModule::set_int(fighter.module_accessor,hover_id as i32,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE2_ID);
    StatusModule::change_status_request(hover_stone,*ITEM_STATUS_KIND_HAVE,false);
    KineticModule::unable_energy_all(hover_stone);
    MotionModule::remove_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
    MotionModule::add_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_loop"),0.0,1.0,false,false,0.0,false,false,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_hold_main_loop as *const () as _))
}

unsafe fn special_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    let height = WorkModule::get_float(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_HEIGHT).ceil();
    let start_height = WorkModule::get_float(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_HEIGHT).ceil();
    if PostureModule::pos_y(fighter.module_accessor) >= start_height + height {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.0);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE2_ID);
    let hover_stone = sv_battle_object::module_accessor(id as u32);
    MotionModule::set_rate(hover_stone,0.0);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_TIME_DEC);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_hi_end"),0.0,1.0,false,0.0,false,false);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    PLAY_SE(fighter,Hash40::new("se_link_stasis_hit"));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}

unsafe fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
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

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,true);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
        ArticleModule::set_visibility_whole(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,true,ArticleOperationTarget(0));
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),true);
    }
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_kenb"),false);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_tate"),true);
    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_tateb"),false);
    let hover_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE2_ID) as u32;
    let hover_stone = sv_battle_object::module_accessor(hover_id);
    StatusModule::change_status_request(hover_stone,*ITEM_STATUS_KIND_LOST,false);
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) == false {
        MotionModule::remove_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
    }
    original!(fighter)
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn special_hi_end_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}

#[acmd_script(agent = "link", scripts = ["game_specialhi","game_specialairhi"], category = ACMD_GAME)]
pub unsafe fn special_hi(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_stasis_start"));
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_glow"),0.0,2.0,false,false,0.0,false,false,false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT);
    }
    sv_animcmd::frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_loop"),0.0,1.0,false,false,0.0,false,false,false);
        WorkModule::off_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ASCEND_FAIL);
    }
    sv_animcmd::frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,true);
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),false);
            ArticleModule::set_visibility_whole(agent.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,true,ArticleOperationTarget(0));
        }
        else {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),true);
        }
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_kenb"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_tate"),true);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_tateb"),false);
        MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
    }
}

#[acmd_script(agent = "link", script = "game_specialhiend", category = ACMD_GAME)]
pub unsafe fn special_hi_end(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WHOLE_HIT(agent,*HIT_STATUS_OFF);
    }
    sv_animcmd::frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_rune_filter"));
    }
    sv_animcmd::frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        WHOLE_HIT(agent,*HIT_STATUS_NORMAL);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_TIME_DEC);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,true);
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),false);
            ArticleModule::set_visibility_whole(agent.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,true,ArticleOperationTarget(0));
        }
        else {
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),true);
        }
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_kenb"),false);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_tate"),true);
        ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_tateb"),false);
        MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
        MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        WorkModule::on_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
    }
}

pub unsafe fn remove_hover_stone(module_accessor: &mut BattleObjectModuleAccessor) {
    let item_id = WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_ID);
    let item_boma = sv_battle_object::module_accessor(item_id as u32);
    StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_LOST,false);
    WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_TIME_DEC);
    WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_DESPAWN);
}

pub unsafe fn remove_hover_stone2(module_accessor: &mut BattleObjectModuleAccessor) {
    let item_id = WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE2_ID);
    let item_boma = sv_battle_object::module_accessor(item_id as u32);
    StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_LOST,false);
}

pub static mut HOVER_LOST_PRE: usize = 0x618330;

#[skyline::hook(replace = HOVER_LOST_PRE)]
pub unsafe fn hover_lost_pre(item: &mut L2CAgentBase) -> L2CValue {
    let lost_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lost_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_LOST),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),lost_coroutine_func);
    let lost_status_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(lost_status as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_LOST),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS),lost_status_func);
    original!()(item)
}

unsafe fn lost_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    if item::variation(item.lua_state_agent) == *ITEM_VARIATION_CARRIERBOX_FUTURE {
        MotionModule::change_motion(item.module_accessor,Hash40::new("lost2"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(item.module_accessor,Hash40::new("lost"),0.0,1.0,false,0.0,false,false);
    }
    item_collision_manager::disable_ground_collision(item.lua_state_agent);
    item_collision_manager::remove_ground_collision(item.lua_state_agent);
    return L2CValue::I32(0)
}

unsafe fn lost_status(item: &mut L2CAgentBase) -> L2CValue {
    if MotionModule::is_end(item.module_accessor) {
        item::request_remove(item.lua_state_agent);
    }
    return L2CValue::I32(0)
}

pub static mut HOVER_HAVE_PRE: usize = 0x616e20;

#[skyline::hook(replace = HOVER_HAVE_PRE)]
pub unsafe fn hover_have_pre(item: &mut L2CAgentBase) -> L2CValue {
    let have_coroutine_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(have_coroutine as *const () as _).get_ptr());
    item.sv_set_status_func(L2CValue::I32(*ITEM_STATUS_KIND_HAVE),L2CValue::I32(*ITEM_LUA_SCRIPT_STATUS_FUNC_STATUS_COROUTINE),have_coroutine_func);
    original!()(item)
}

unsafe fn have_coroutine(item: &mut L2CAgentBase) -> L2CValue {
    if item::variation(item.lua_state_agent) == *ITEM_VARIATION_CARRIERBOX_FUTURE {
        MotionModule::change_motion(item.module_accessor,Hash40::new("fall2"),0.0,1.7,false,0.0,false,false);
        item_collision_manager::disable_ground_collision(item.lua_state_agent);
        item_collision_manager::remove_ground_collision(item.lua_state_agent);
    }
    else {
        MotionModule::change_motion(item.module_accessor,Hash40::new("fall"),0.0,1.0,false,0.0,false,false);
    }
    HitModule::set_whole(item.module_accessor,HitStatus(*HIT_STATUS_XLU),0);
    return L2CValue::I32(0)
}