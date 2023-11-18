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

pub struct FuseType(i32);

impl FuseType {
    pub const NORMAL: i32 = 0;
    pub const POWER: i32 = 1;
    pub const ELEMENTAL: i32 = 2;
}

#[acmd_script(agent = "link_ancientbowarrow", script = "game_have", category = ACMD_GAME)]
pub unsafe fn game_have(agent: &mut L2CAgentBase) {
    sv_animcmd::frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let navi_id = ArticleModule::get_active_num(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY);
        ArticleModule::generate_article(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,false,-1);
        let navi = ArticleModule::get_article_from_no(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        LinkModule::remove_model_constraint(navi_boma,true);
        if LinkModule::is_link(navi_boma,*WEAPON_LINK_NO_CONSTRAINT) {
            LinkModule::unlink(navi_boma,*WEAPON_LINK_NO_CONSTRAINT);
        }
        if LinkModule::is_link(navi_boma,*WEAPON_LINK_NO_CONSTRAINT) == false {
            VisibilityModule::set_whole(navi_boma,true);
            LinkModule::link(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,(*(agent.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("trigger"),*CONSTRAINT_FLAG_POSITION as u32,true);
            PLAY_SE(agent,Hash40::new("se_link_scene_slow"));
            ModelModule::set_mesh_visibility(navi_boma,Hash40::new("link_ken"),false);
            MotionModule::change_motion(navi_boma,Hash40::new("effect"),0.0,1.0,false,0.0,false,false);
        }
        PLAY_SE(agent, Hash40::new("se_link_scene_slow"));
        PLAY_SE(agent, Hash40::new("se_link_final01"));
    }
    sv_animcmd::frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let navi_id = ArticleModule::get_active_num(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY) - 1;
        let navi = ArticleModule::get_article_from_no(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
        let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
        ArticleModule::remove_exist_object_id(owner_module_accessor,(*(navi_boma)).battle_object_id);
    }
}

unsafe fn set_arrow_fuse_params(module_accessor: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE,*ITEM_KIND_ASSIST,*ITEM_KIND_LINKARROW].contains(&item_kind)
    && ![*ITEM_TRAIT_FLAG_NONE,*ITEM_TRAIT_FLAG_SHOOT,*ITEM_TRAIT_FLAG_SWING].contains(&trait_type))
    || [*ITEM_KIND_BANANAGUN,*ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        WorkModule::set_int(module_accessor,item_kind,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        let owner_kind = utility::get_kind(&mut *owner_module_accessor);
        if fuse_kind == FuseKind::FUSE {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_module_accessor,item_kind,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::set_int(owner_module_accessor,item_kind,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            let item_id = ItemModule::get_have_item_id(owner_module_accessor,0) as i32;
            WorkModule::set_int(module_accessor,item_id,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
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
            WorkModule::set_int(module_accessor,item_id,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            if owner_kind == *FIGHTER_KIND_MURABITO
            || owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::set_int(owner_module_accessor,*ITEM_KIND_NONE,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(module_accessor,FuseType::NORMAL,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor,*ITEM_BOMBER_STATUS_KIND_BORN2,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if [*ITEM_KIND_KILLER,*ITEM_KIND_BANANAGUN,*ITEM_KIND_DOLPHINBOMB].contains(&item_kind) {
            WorkModule::set_int(module_accessor,FuseType::POWER,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_THROW,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(module_accessor,FuseType::ELEMENTAL,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_LOST,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(module_accessor,FuseType::NORMAL,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_BORN,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(module_accessor,FuseType::NORMAL,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor,*ITEM_STATUS_KIND_THROW,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

unsafe fn set_elemental_fuse(weapon: &mut L2CFighterBase, element: i32, fuse_type: i32, end_status: i32) {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = utility::get_kind(&mut *owner_module_accessor);
    if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_LINK {
        WorkModule::set_int(owner_module_accessor,element,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    else {
        WorkModule::set_int(owner_module_accessor,element,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    let mut params = CreateItemParam {
        founder_pos: Vector4f{x: PostureModule::pos_x(owner_module_accessor), y: PostureModule::pos_y(owner_module_accessor), z: PostureModule::pos_z(owner_module_accessor), w: 0.0},
        item_pos: Vector4f{x: PostureModule::pos_x(owner_module_accessor), y: PostureModule::pos_y(owner_module_accessor), z: PostureModule::pos_z(owner_module_accessor), w: 0.0},
        item_kind: ItemKind(element),
        another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
        variation_kind: *ITEM_VARIATION_NONE,
        lr_dir: PostureModule::lr(owner_module_accessor),
        owner_id: (*(owner_module_accessor)).battle_object_id,
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
    WorkModule::set_int(weapon.module_accessor,element,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(weapon.module_accessor,fuse_type,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
    WorkModule::set_int(weapon.module_accessor,end_status,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    let item_boma = (*battle_object).module_accessor;
    let item_id = (*item_boma).battle_object_id;
    WorkModule::set_int64(weapon.module_accessor,item_id as i64,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_THROW,false);
    LinkModule::remove_model_constraint(item_boma,true);
    if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
        LinkModule::unlink(item_boma,*ITEM_LINK_NO_HAVE);
    }
    if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
        VisibilityModule::set_whole(item_boma,true);
        LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(weapon.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
    }
    if owner_kind == *FIGHTER_KIND_LINK {
        let navi_id = ArticleModule::get_active_num(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY);
        WorkModule::set_int(weapon.module_accessor,navi_id,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
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
            LinkModule::set_model_constraint_pos_ort(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
            PLAY_SE(weapon,Hash40::new("se_link_scene_slow"));
            ModelModule::set_mesh_visibility(navi_boma,Hash40::new("link_ken"),false);
            MotionModule::change_motion(navi_boma,Hash40::new("effect"),0.0,1.0,false,0.0,false,false);
        }
    }
    WorkModule::on_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_HAVED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn arrow_haved_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = utility::get_kind(&mut *owner_module_accessor);
    WorkModule::set_int(weapon.module_accessor,-1,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
    if ItemModule::is_have_item(owner_module_accessor,0) {
        let (kind,fuse_kind,trait_type) = (ItemModule::get_have_item_kind(owner_module_accessor,0),FuseKind::FUSE,ItemModule::get_have_item_trait(owner_module_accessor,0) as i32);
        set_arrow_fuse_params(weapon.module_accessor,kind,fuse_kind,trait_type);
    }
    else if owner_kind == *FIGHTER_KIND_MURABITO
    || owner_kind == *FIGHTER_KIND_SHIZUE {
        if WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM) != *ITEM_KIND_NONE {
            let (kind,fuse_kind,trait_type) = (WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM),FuseKind::REFUSE,i32::MAX);
            set_arrow_fuse_params(weapon.module_accessor,kind,fuse_kind,trait_type);
        }
    }
    else {
        if owner_kind == *FIGHTER_KIND_LINK {
            WorkModule::set_int(owner_module_accessor,*ITEM_KIND_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
        else if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::set_int(owner_module_accessor,*ITEM_KIND_NONE,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
    }
    let is_fused = WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    if is_fused {
        let item_id = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(item_boma,*ITEM_LINK_NO_HAVE);
        }
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
            VisibilityModule::set_whole(item_boma,true);
            LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(weapon.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
        }
        if owner_kind == *FIGHTER_KIND_LINK {
            let navi_id = ArticleModule::get_active_num(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY);
            WorkModule::set_int(weapon.module_accessor,navi_id,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
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
                LinkModule::set_model_constraint_pos_ort(navi_boma,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
                PLAY_SE(weapon,Hash40::new("se_link_scene_slow"));
                ModelModule::set_mesh_visibility(navi_boma,Hash40::new("link_ken"),false);
                MotionModule::change_motion(navi_boma,Hash40::new("effect"),0.0,1.0,false,0.0,false,false);
            }
        }
    }
    MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved"),0.0,1.0,false,0.0,false,false);
    weapon.fastshift(L2CValue::Ptr(arrow_haved_main_loop as *const () as _))
}

unsafe fn arrow_haved_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) == false {
        if ControlModule::check_button_trigger(owner_module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R)
        || ControlModule::check_button_trigger(owner_module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
            set_elemental_fuse(weapon,*ITEM_KIND_FREEZER,FuseType::ELEMENTAL,*ITEM_STATUS_KIND_THROW);
        }
        else if ControlModule::check_button_trigger(owner_module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
            set_elemental_fuse(weapon,*ITEM_KIND_FIREFLOWER,FuseType::ELEMENTAL,*ITEM_STATUS_KIND_LOST);
        }
        else if ControlModule::check_button_trigger(owner_module_accessor,*CONTROL_PAD_BUTTON_APPEAL_LW) {
            set_elemental_fuse(weapon,*ITEM_KIND_THUNDER,FuseType::ELEMENTAL,*ITEM_STATUS_KIND_LOST);
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[acmd_script(agent = "link", scripts = ["game_specialnstart","game_specialairnstart"], category = ACMD_GAME)]
pub unsafe fn special_n_start(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, -1);
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_glow"),0.0,2.0,false,false,0.0,false,false,false);
        }
    }
    FT_MOTION_RATE(agent, 0.8);
    sv_animcmd::frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_loop"),0.0,1.0,false,false,0.0,false,false,false);
        }
    }
    FT_MOTION_RATE(agent, 0.8);
    sv_animcmd::frame(agent.lua_state_agent, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    }
    sv_animcmd::frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT) {
            MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        }
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[acmd_script(agent = "link", scripts = ["game_specialnend","game_specialairnend"], category = ACMD_GAME)]
pub unsafe fn special_n_end(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT)
        && MotionModule::motion_kind_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY) == hash40("arm_loop") {
            MotionModule::remove_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,true);
            MotionModule::add_motion_partial(agent.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,Hash40::new("arm_fade"),0.0,1.0,false,false,0.0,false,false,false);
        }
    }
    sv_animcmd::frame(agent.lua_state_agent, 21.0);
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

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor,bow_id,*WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
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
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,ArticleOperationTarget(0));
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

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn kirby_special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor,bow_id,*WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_FALL,false);
            }
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,ArticleOperationTarget(0));
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_fly_end(weapon: &mut L2CFighterBase) -> L2CValue {
    if StatusModule::status_kind_next(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_STICK
    || StatusModule::status_kind_next(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK {
        if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma,status,false);
            }
        }
    }
    else {
        if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int64(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
    EffectModule::detach_all(weapon.module_accessor,5);
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn arrow_fly_init(weapon: &mut L2CFighterBase) -> L2CValue {
    original!(weapon);
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        if WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::POWER {
            let lr = PostureModule::lr(weapon.module_accessor);
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,12.0*lr,0.0);
            sv_kinetic_energy::set_speed(weapon.lua_state_agent);
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,0.0,0.0);
            sv_kinetic_energy::set_accel(weapon.lua_state_agent);
            KineticModule::enable_energy(weapon.module_accessor,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            AttackModule::set_power_mul(weapon.module_accessor,2.5);
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_STICK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_stick_end(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let team_no = if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_KIRBY {
            WorkModule::get_int(owner_module_accessor,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO
        || utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_SHIZUE {
            WorkModule::get_int(owner_module_accessor,FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else {
            WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO)
        };
        TeamModule::set_team(weapon.module_accessor,team_no,true);
        TeamModule::set_team_owner_id(weapon.module_accessor,(*(owner_module_accessor)).battle_object_id);
        WorkModule::off_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_hit_stick_end(weapon: &mut L2CFighterBase) -> L2CValue {
    arrow_stick_end(weapon)
}

#[acmd_script(agent = "link_bowarrow", script = "game_fly", category = ACMD_GAME)]
pub unsafe fn arrow(weapon: &mut L2CAgentBase) {
    if WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::ELEMENTAL {
        if WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND) == *ITEM_KIND_FIREFLOWER {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(weapon.module_accessor,1.15);
                AttackModule::enable_safe_pos(weapon.module_accessor);
            }
        }
        else if WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND) == *ITEM_KIND_FREEZER {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(weapon.module_accessor,0.4);
                AttackModule::enable_safe_pos(weapon.module_accessor);
            }
        }
        else {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.75,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(weapon.module_accessor,1.5);
                AttackModule::enable_safe_pos(weapon.module_accessor);
                QUAKE(weapon,*CAMERA_QUAKE_KIND_S);
            }
        }
    }
    else {
        if WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_SHOOT_NUM) <= 0 {
            if is_excute(weapon) {
                ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        }
        else {
            if is_excute(weapon) {
                ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 110, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(weapon.module_accessor);
            }
        }
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_LINK_BOWARROW, main)]
pub fn link_bowarrow(weapon: &mut L2CFighterBase) {
    unsafe {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)
        && StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_FLY
        && WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            WorkModule::on_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
            let item_id = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
            let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
            TeamModule::set_team(item_boma,team_no,true);
            TeamModule::set_team_owner_id(item_boma,team_owner_id);
        }
        if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let navi_id = WorkModule::get_int(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
            if navi_id > -1 {
                let navi = ArticleModule::get_article_from_no(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,navi_id);
                let navi_boma = *(navi as *mut smash::app::Article as *mut u64).add(0x8/8) as *mut BattleObjectModuleAccessor;
                if MotionModule::is_end(navi_boma) {
                    ArticleModule::remove_exist_object_id(owner_module_accessor,(*(navi_boma)).battle_object_id);
                    WorkModule::set_int(weapon.module_accessor,-1,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID);
                }
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_DEDEDE, main)]
pub fn dedede_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                let owner_kind = utility::get_kind(&mut *owner_module_accessor);
                let fused_item = if owner_kind == *FIGHTER_KIND_MURABITO
                || owner_kind == *FIGHTER_KIND_SHIZUE {
                    WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM)
                }
                else if owner_kind == *FIGHTER_KIND_KIRBY {
                    WorkModule::get_int(owner_module_accessor,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
                }
                else {
                    WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
                };
                WorkModule::set_int(fighter.module_accessor,fused_item,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
            else if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOOMERANG {
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                let owner_kind = utility::get_kind(&mut *owner_module_accessor);
                let fused_item = if owner_kind == *FIGHTER_KIND_MURABITO
                || owner_kind == *FIGHTER_KIND_SHIZUE {
                    WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM)
                }
                else {
                    WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE)
                };
                WorkModule::set_int(fighter.module_accessor,fused_item,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SHOT_OBJECT_SHOOT)
            && WorkModule::get_int(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM) != *ITEM_KIND_NONE {
                WorkModule::on_flag(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
            }
            if WorkModule::is_flag(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK)
            && MotionModule::frame(fighter.module_accessor) >= 7.0 {
                let item = WorkModule::get_int(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
                if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                    set_arrow_fuse_params(obj_boma,item,FuseKind::REFUSE,i32::MAX);
                }
                else {
                    WorkModule::on_flag(obj_boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
                    set_boomerang_fuse_params(obj_boma,item,FuseKind::REFUSE,i32::MAX);
                }
                let item_id = WorkModule::get_int(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
                LinkModule::remove_model_constraint(item_boma,true);
                if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
                    VisibilityModule::set_whole(item_boma,true);
                    LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,obj_id);
                    if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                        LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
                    }
                    else {
                        LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("have"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32,true);
                        let offset_pos = Vector3f{x: 0.0, y: 9.5, z: 0.0};
                        LinkModule::set_constraint_translate_offset(item_boma,&offset_pos);
                    }
                }
                WorkModule::set_int(fighter.module_accessor,*ITEM_KIND_NONE,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
                WorkModule::on_flag(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW);
                WorkModule::off_flag(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
            }
        }
    }
}

unsafe fn villager_shizue_common(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        let obj_boma = sv_battle_object::module_accessor(obj_id);
        if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
            let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            let owner_kind = utility::get_kind(&mut *owner_module_accessor);
            let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
            let fused_item = if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::get_int(owner_module_accessor,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            }
            else if owner_kind == *FIGHTER_KIND_SHIZUE
            || owner_kind == *FIGHTER_KIND_MURABITO {
                WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM)
            }
            else if WorkModule::is_flag(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW) {
                let item_id = WorkModule::get_int64(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
                utility::get_kind(&mut *item_boma)
            }
            else {
                WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            };
            WorkModule::set_int(fighter.module_accessor,fused_item,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            let item_id = WorkModule::get_int64(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id);
        }
        else if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOOMERANG {
            let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            let owner_kind = utility::get_kind(&mut *owner_module_accessor);
            let fused_item = if StatusModule::status_kind(obj_boma) == *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED {
                let item_id = WorkModule::get_int(obj_boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
                utility::get_kind(&mut *item_boma)
            }
            else if owner_kind == *FIGHTER_KIND_MURABITO ||
            owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::get_int(owner_module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM)
            }
            else {
                WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE)
            };
            WorkModule::set_int(fighter.module_accessor,fused_item,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            let item_id = WorkModule::get_int64(obj_boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) == false {
        let boomerang_fuse_item_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(boomerang_fuse_item_id);
        if sv_battle_object::is_active(boomerang_fuse_item_id)
        && StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,boomerang_fuse_item_id);
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_MURABITO, main)]
pub fn villager_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        villager_shizue_common(fighter);
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_SHIZUE, main)]
pub fn isabelle_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        villager_shizue_common(fighter);
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_KIRBY, main)]
pub fn kirby_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_INHALE_OBJECT_ID) as u32;
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let item_id = if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                WorkModule::get_int64(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
            }
            else if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOOMERANG {
                WorkModule::get_int64(obj_boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
            }
            else {
                *BATTLE_OBJECT_ID_INVALID as u32
            };
            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_ROSETTA, main)]
pub fn rosalina_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let item_id = if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                WorkModule::get_int64(obj_boma,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
            }
            else if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOOMERANG {
                WorkModule::get_int64(obj_boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
            }
            else {
                *BATTLE_OBJECT_ID_INVALID as u32
            };
            let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
}