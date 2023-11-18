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
use crate::ITEM_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
use std::arch::asm;
use skyline::patching::Patch;

pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SWORD_FUSED: i32 = 0x200000ea;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_KILLSWORD: i32 = 0x200000eb;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_CLUB: i32 = 0x200000ec;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_FUSED_HOMERUNBAT: i32 = 0x200000ed;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_IS_SWORD_ATTACK: i32 = 0x200000ee;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED: i32 = 0x200000ef;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RECORD_ASCEND_HEIGHT: i32 = 0x200000f0;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ASCEND_FAIL: i32 = 0x200000f1;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE: i32 = 0x200000f2;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_TIME_DEC: i32 = 0x200000f3;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_HOVER_STONE_DESPAWN: i32 = 0x200000f4;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM: i32 = 0x200000f5;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FUSE_EFFECT: i32 = 0x200000f6;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID: i32 = 0x100000c6;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_EFFECT: i32 = 0x100000c7;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_SOUND: i32 = 0x100000c8;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSED_SWORD_USAGE: i32 = 0x100000c9;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_ID: i32 = 0x100000ca;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE_REMOVE_TIMER: i32 = 0x100000cb;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_NEW_WAIT: i32 = 0x100000cc;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_HOVER_STONE2_ID: i32 = 0x100000cd;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO: i32 = 0x100000ce;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE: i32 = 0x100000cf;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE: i32 = 0x100000d0;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID: i32 = 0x100000d1;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID: i32 = 0x100000d2;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_FUSED_SWORD_LENGTH: i32 = 0x59;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_HEIGHT: i32 = 0x5a;
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_HEIGHT: i32 = 0x5b;

pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND: i32 = 0x1000000e;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG: i32 = 0x1000000f;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS: i32 = 0x10000010;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID: i32 = 0x10000011;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID: i32 = 0x10000012;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED: i32 = 0x20000009;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT: i32 = 0x2000000a;
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW: i32 = 0x2000000b;

pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND: i32 = 0x1000000c;
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS: i32 = 0x1000000d;
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID: i32 = 0x1000000e;
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_NAVY_ID: i32 = 0x1000000f;
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED: i32 = 0x2000000e;
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT: i32 = 0x2000000f;

pub const ITEM_KILLSWORD_INSTANCE_WORK_FLAG_IS_CRITICAL_ATTACK: i32 = 0x20000000;

pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_FUSE: i32 = 0x1e8;

pub const FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK: i32 = 0x200000e6;
pub const FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM: i32 = 0x100000c5;

pub const FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM: i32 = 0x100000d1;
pub const FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM: i32 = 0x100000d2;
pub const FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID: i32 = 0x100000d3;
pub const FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO: i32 = 0x100000d4;

pub const FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE: i32 = 0x1000010b;
pub const FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO: i32 = 0x1000010c;

#[skyline::hook(replace=FighterUtil::is_valid_auto_catch_item)]
pub unsafe fn is_valid_auto_catch_item_hook(module_accessor: &mut BattleObjectModuleAccessor, is_possible: bool) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
            return true;
        }
        else {
            original!()(module_accessor,is_possible)
        }
    }
    else {
        original!()(module_accessor,is_possible)
    }
}

pub struct FuseKind(i32);

impl FuseKind {
    pub const FUSE: i32 = 0;
    pub const REFUSE: i32 = 1;
}

pub mod arrow_fuse;
use arrow_fuse::*;
pub mod sword_fuse;
use sword_fuse::*;
pub mod fuse_attacks;
use fuse_attacks::*;
pub mod ascend;
use ascend::*;
pub mod boomerang_fuse;
use boomerang_fuse::*;

#[fighter_init]
pub fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_LINK {
            fighter.global_table[0x26].assign(&L2CValue::Ptr(enable_disable_special_lw as *const () as _));
            fighter.global_table[0x27].assign(&L2CValue::Ptr(enable_disable_special_lw as *const () as _));
            fighter.global_table[0x28].assign(&L2CValue::Ptr(check_throw_or_fuse as *const () as _));
            fighter.global_table[0x2d].assign(&L2CValue::Ptr(check_throw_or_fuse as *const () as _));
            let special_lw_fuse_pre_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(special_lw_fuse_pre as *const () as _).get_ptr());
            fighter.sv_set_status_func(L2CValue::I32(FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_FUSE),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_PRE),special_lw_fuse_pre_func);
            let special_lw_fuse_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(special_lw_fuse as *const () as _).get_ptr());
            fighter.sv_set_status_func(L2CValue::I32(FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_FUSE),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN),special_lw_fuse_func);
            let special_lw_fuse_end_func: &mut skyline::libc::c_void = std::mem::transmute(L2CValue::Ptr(special_lw_fuse_end as *const () as _).get_ptr());
            fighter.sv_set_status_func(L2CValue::I32(FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_FUSE),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_END),special_lw_fuse_end_func);
            fighter.sv_set_status_func(L2CValue::I32(FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_FUSE),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS),special_lw_fuse_end_func);
            let team_no = TeamModule::team_no(fighter.module_accessor) as i32;
            WorkModule::set_int(fighter.module_accessor,team_no,FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
            WorkModule::set_int(fighter.module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_NAVI_ID);
        }
        if fighter_kind == *FIGHTER_KIND_DEDEDE {
            WorkModule::set_int(fighter.module_accessor,*ITEM_KIND_NONE,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        }
        if fighter_kind == *FIGHTER_KIND_MURABITO
        || fighter_kind == *FIGHTER_KIND_SHIZUE {
            let team_no = TeamModule::team_no(fighter.module_accessor) as i32;
            WorkModule::set_int(fighter.module_accessor,team_no,FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO);
            WorkModule::set_int(fighter.module_accessor,*ITEM_KIND_NONE,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        }
        if fighter_kind == *FIGHTER_KIND_KIRBY {
            let team_no = TeamModule::team_no(fighter.module_accessor) as i32;
            WorkModule::set_int(fighter.module_accessor,team_no,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO);
        }
    }
}

pub fn nro_hook(info: &skyline::nro::NroInfo) {
    match info.name {
        "item" => {
            unsafe {
                HOVER_LOST_PRE += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(hover_lost_pre);
                HOVER_HAVE_PRE += (*info.module.ModuleObject).module_base as usize;
                skyline::install_hook!(hover_have_pre);
            }
        },
        _ => {}
    }
}

#[repr(C)]
pub struct CreateItemParam {
    founder_pos: Vector4f,
    item_pos: Vector4f,
    item_kind: ItemKind,
    another_battle_object_id: u32,
    variation_kind: i32,
    lr_dir: f32,
    owner_id: u32,
    unk_20: u32,
    pokeball_or_assist_kind: i32,
    unk_0: u64,
    weird_flag: u64,
    unk_1_weird: u64,
    unk_approx_0: f32,
    unk_02: f32
}

#[skyline::hook(offset = 0x15daea0)]
pub unsafe fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager,create_item_param,unk,unk2,unk3)
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        LookupSymbol(
            &mut ITEM_MANAGER,
            "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
    install_acmd_scripts!(
        arrow,
        attack_11,
        attack_12,
        attack_13,
        attack_s3,
        attack_hi3,
        attack_lw3,
        attack_dash,
        attack_s4,
        attack_s4_s2,
        attack_hi4,
        attack_lw4,
        attack_air_f,
        attack_air_hi,
        attack_air_lw,
        cliff_attack,
        down_attack_d,
        down_attack_u,
        slip_attack,
        special_hi,
        special_hi_end,
        game_have,
        special_lw,
        special_n_start,
        special_n_end,
        special_s1,
        special_s2,
        special_lw_fuse
    );
    install_status_scripts!(
        arrow_haved_main,
        special_n_pre,
        special_n_end,
        special_n_exit,
        kirby_special_n_exit,
        arrow_fly_end,
        arrow_fly_init,
        arrow_stick_end,
        arrow_hit_stick_end,
        special_lw_pre,
        special_lw_end,
        special_lw_main,
        special_hi_main,
        special_hi_hold_main,
        special_hi_end_main,
        special_hi_end_init,
        boomerang_haved_main,
        boomerang_fly_end,
        boomerang_swallowed_pre,
        boomerang_swallowed_end,
        boomerang_haved_end,
        special_s_pre,
        special_s_end,
        special_s_exit,
        special_s2_end,
        special_hi_end,
        special_hi_end_end,
        rebirth_pre
    );
    install_agent_frames!(
        link,
        link_bowarrow,
        link_boomerang,
        dedede_fix,
        villager_fix,
        isabelle_fix,
        kirby_fix,
        rosalina_fix
    );
    install_agent_init_callback!(agent_init);
    skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        create_item,
        is_valid_auto_catch_item_hook
    );
    //bomb
    Patch::in_text(0x4fd6664).data(5).expect("Rip bozo");
    //navy
    Patch::in_text(0x4fd65c4).data(2).expect("Rip bozo");
}