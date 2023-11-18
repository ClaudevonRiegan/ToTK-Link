use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
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

pub unsafe fn set_boomerang_fuse_params(module_accessor: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE,*ITEM_KIND_ASSIST,*ITEM_KIND_LINKARROW].contains(&item_kind)
    && ![*ITEM_TRAIT_FLAG_NONE,*ITEM_TRAIT_FLAG_SHOOT,*ITEM_TRAIT_FLAG_SWING].contains(&trait_type))
    || [*ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        WorkModule::set_int(module_accessor,item_kind,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        let owner_kind = utility::get_kind(&mut *owner_module_accessor);
        if fuse_kind == FuseKind::FUSE {
            WorkModule::set_int(owner_module_accessor,item_kind,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            let item_id = ItemModule::get_have_item_id(owner_module_accessor,0) as i32;
            WorkModule::set_int(module_accessor,item_id,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_module_accessor,item_id,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let mut params = CreateItemParam {
                founder_pos: Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor), w: 0.0},
                item_pos: Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor), w: 0.0},
                item_kind: ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(module_accessor),
                owner_id: (*(module_accessor)).battle_object_id,
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
            let item_boma = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL,*ITEM_KIND_CHEWING,*ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_HAVE,false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_boma,1.3,false);
            }
            let item_id = (*(item_boma)).battle_object_id as i32;
            WorkModule::set_int(module_accessor,item_id,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_module_accessor,item_id,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
            if owner_kind == *FIGHTER_KIND_MURABITO
            || owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::set_int(owner_module_accessor,*ITEM_KIND_NONE,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(module_accessor,*ITEM_BOMBER_STATUS_KIND_BORN2,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_LOST,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_BORN,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_THROW,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

#[status_script(agent = "link_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_HAVED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn boomerang_haved_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = utility::get_kind(&mut *owner_module_accessor);
    WorkModule::set_int(weapon.module_accessor,-1,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
    if StatusModule::status_kind(owner_module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if ItemModule::is_have_item(owner_module_accessor,0) {
            let (kind,fuse_type,trait_type) = (ItemModule::get_have_item_kind(owner_module_accessor,0),FuseKind::FUSE,ItemModule::get_have_item_trait(owner_module_accessor,0) as i32);
            set_boomerang_fuse_params(weapon.module_accessor,kind,fuse_type,trait_type);
        }
        else if owner_kind == *FIGHTER_KIND_MURABITO
        || owner_kind == *FIGHTER_KIND_SHIZUE {
            if WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM) != *ITEM_KIND_NONE {
                let (kind,fuse_type,trait_type) = (WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM),FuseKind::REFUSE,i32::MAX);
                set_boomerang_fuse_params(weapon.module_accessor,kind,fuse_type,trait_type);
            }
        }
        else {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_module_accessor,*ITEM_KIND_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
        }
        let is_fused = WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
        if is_fused {
            let item_id = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink(item_boma,*ITEM_LINK_NO_HAVE);
            }
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
                VisibilityModule::set_whole(item_boma,true);
                LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(weapon.module_accessor)).battle_object_id);
                LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("have"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32,true);
                let offset_pos = Vector3f{x: 0.0, y: 9.5, z: 0.0};
                LinkModule::set_constraint_translate_offset(item_boma,&offset_pos);
            }
            if owner_kind == *FIGHTER_KIND_LINK {
                let navi_id = ArticleModule::get_active_num(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY);
                WorkModule::set_int(weapon.module_accessor,navi_id,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
                ArticleModule::generate_article(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,false,-1);
                let navi = ArticleModule::get_article_from_no(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
                let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                LinkModule::remove_model_constraint(navi_boma,true);
                if LinkModule::is_link(navi_boma,*WEAPON_LINK_NO_CONSTRAINT) {
                    LinkModule::unlink(navi_boma,*WEAPON_LINK_NO_CONSTRAINT);
                }
                if LinkModule::is_link(navi_boma,*WEAPON_LINK_NO_CONSTRAINT) == false {
                    VisibilityModule::set_whole(navi_boma,true);
                    LinkModule::link(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,(*(weapon.module_accessor)).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("have"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32,true);
                    let offset_pos = Vector3f{x: 0.0, y: 9.5, z: 0.0};
                    LinkModule::set_constraint_translate_offset(navi_boma,&offset_pos);
                    PLAY_SE(weapon,Hash40::new("se_link_scene_slow"));
                    ModelModule::set_mesh_visibility(navi_boma,Hash40::new("link_ken"),false);
                    MotionModule::change_motion(navi_boma,Hash40::new("effect"),0.0,1.0,false,0.0,false,false);
                }
            }
        }
        MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved"),0.0,1.0,false,0.0,false,false);
        weapon.fastshift(L2CValue::Ptr(boomerang_haved_main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved"),0.0,1.0,false,0.0,false,false);
        weapon.fastshift(L2CValue::Ptr(boomerang_haved_main_loop as *const () as _))
    }
}

unsafe fn boomerang_haved_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REMOVE_SELF) {
        smash_script::notify_event_msc_cmd!(weapon,Hash40::new_raw(0x199c462b5du64));
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,false);
        let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
        let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        VisibilityModule::set_whole(navi_boma,false);
    }
    original!(fighter)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,true);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
        let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
        let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        VisibilityModule::set_whole(navi_boma,true);
    }
    original!(fighter)
}

#[acmd_script(agent = "link", scripts = ["game_specials1","game_specialairs1"], category = ACMD_GAME)]
pub unsafe fn special_s1(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, false, -1);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_glow"),0.0,2.0,false,false,0.0,false,false,false);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_loop"),0.0,1.0,false,false,0.0,false,false,false);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    sv_animcmd::frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,true);
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),false);
            let navi_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
            let navi = ArticleModule::get_article_from_no(agent.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
            let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            VisibilityModule::set_whole(navi_boma,true);
        }
    }
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,true);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
        let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
        let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        VisibilityModule::set_whole(navi_boma,true);
    }
    original!(fighter)
}

#[acmd_script(agent = "link", scripts = ["game_specials2","game_specialairs2"], category = ACMD_GAME)]
pub unsafe fn special_s2(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
            let item_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = sv_battle_object::module_accessor(item_id);
            VisibilityModule::set_whole(item_boma,true);
            ModelModule::set_mesh_visibility(agent.module_accessor,Hash40::new("link_ken"),false);
            let navi_id = WorkModule::get_int(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
            let navi = ArticleModule::get_article_from_no(agent.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
            let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
            VisibilityModule::set_whole(navi_boma,true);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) == false {
        let item_id = ArticleModule::get_int(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_FALL,false);
        }
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) == false {
            MotionModule::remove_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT);
        }
        ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG,ArticleOperationTarget(0));
    }
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VisibilityModule::set_whole(item_boma,true);
        ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("link_ken"),false);
        let navi_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
        let navi = ArticleModule::get_article_from_no(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        VisibilityModule::set_whole(navi_boma,true);
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn boomerang_fly_end(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
    && WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) == false
    && (AttackModule::is_infliction_status(weapon.module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_SHIELD)) {
        let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_boma,status,false);
        }
    }
    else if StatusModule::status_kind_next(weapon.module_accessor) != *WN_LINK_BOOMERANG_STATUS_KIND_TURN
    && StatusModule::status_kind_next(weapon.module_accessor) != *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED
    && WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_FALL,false);
    }
    return L2CValue::I32(0)
}

#[smashline::weapon_frame(agent = WEAPON_KIND_LINK_BOOMERANG, main)]
pub fn link_boomerang(weapon: &mut L2CFighterBase) {
    unsafe {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)
        && StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOOMERANG_STATUS_KIND_FLY
        && WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            WorkModule::on_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
            let item_id = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
            let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
            TeamModule::set_team(item_boma,team_no,true);
            TeamModule::set_team_owner_id(item_boma,team_owner_id);
        }
        if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_SHIELD))
        && (StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOOMERANG_STATUS_KIND_TURN
        || StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)
        && WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                let team_no = WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
                TeamModule::set_team(weapon.module_accessor,team_no,true);
                TeamModule::set_team_owner_id(weapon.module_accessor,(*(owner_module_accessor)).battle_object_id);
                WorkModule::off_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
            }
            let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma,status,false);
            }
        }
        if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let navi_id = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
            if navi_id > -1 {
                let navi = ArticleModule::get_article_from_no(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
                let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                if MotionModule::is_end(navi_boma) {
                    ArticleModule::remove_exist_object_id(owner_module_accessor,(*(navi_boma)).battle_object_id);
                    WorkModule::set_int(weapon.module_accessor,-1,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
                }
            }
        }
    }
}

#[status_script(agent = "link_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn boomerang_swallowed_pre(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id);
    }
    original!(weapon)
}

#[status_script(agent = "link_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn boomerang_swallowed_end(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        if sv_battle_object::is_active(item_id) {
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma,status,false);
            }
        }
    }
    original!(weapon)
}

#[status_script(agent = "link_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_HAVED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn boomerang_haved_end(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if StatusModule::status_kind(owner_module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2
    && WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_FALL,false);
        if ItemModule::is_have_item(owner_module_accessor,0) == false {
            WorkModule::on_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
        }
    }
    return L2CValue::I32(0)
}