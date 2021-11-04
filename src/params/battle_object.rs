#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectParamsStruct0 {
    pub x02bf396750: u64,
    pub x05a4fa7c89: *mut [f32; 8],
    pad_00: [u64; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectParamsStruct1 {
    pub x05a4fa7c89: *mut [BattleObjectParamsStruct2; 28],
    pad_00: [u64; 2],
    pub x043bc4bcd9: u64,
    pub x0836a37cfd: f32,
    pub x0382d773ca: i32,
    pub x0467ab645b: bool,
    pub x08bc91f416: u64,
    pub x075da53379: bool,
    pub x0bd3619f27: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectParamsStruct2 {
    pub x018cdc1683: f32,
    pub x01fbdb261: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectParamsStruct3 {
    pub x192e58d48b: *mut [BattleObjectParamsStruct4; 76],
    pad_00: [u64; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectParamsStruct4 {
    pub x05b5f83ccd: f32,
    pub x051d775834: f32,
    pub x0b3f061b61: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BattleObjectParams {
    pub params: u64,
    pub stick_clamp_max: f32,
    pub stick_neutral: f32,
    pub clatter_double_invalid_frame: i32,
    pub x21af4b1219: i32,
    pub clatter_button_dec_frame_mul: f32,
    pub reaction_fix_storage: f32,
    pub reaction_damage_storage: f32,
    pub reaction_attack: f32,
    pub reaction_damage_mul: f32,
    pub reaction_damage_add: f32,
    pub reaction_effect_mul: f32,
    pub power_up_reaction_effective_rate: f32,
    pub power_up_reaction_effective_max_mag: f32,
    pub power_up_reaction_effective_rate1: f32,
    pub power_up_reaction_effective_rate2: f32,
    pub power_up_reaction_effective_rate3: f32,
    pub power_up_reaction_effective_rate4: f32,
    pub reaction_max: f32,
    pub damage_max: f32,
    pub damage_frame_mul: f32,
    pub damage_frame_min: f32,
    pub damage_level1: f32,
    pub damage_level2: f32,
    pub damage_level3: f32,
    pub x1c67f223e1: i32,
    pub x1a53290f14: f32,
    pub change_damage_reaction_frame: f32,
    pub x294d96308d: f32,
    pub x2750119c3f: f32,
    pub x2657636c00: f32,
    pub damage_speed_mul: f32,
    pub damage_speed_limit: f32,
    pub meteor_vector_min: i32,
    pub meteor_vector_max: i32,
    pub damage_angle_ground_zero_speed_mul: f32,
    pub damage_angle_ground_reaction_min: f32,
    pub damage_angle_ground_reaction_max: f32,
    pub damage_angle_ground_max: f32,
    pub damage_angle_zero_reaction_min: f32,
    pub damage_angle_zero_damage_angle_max: f32,
    pub damage_sync_speed_mul: f32,
    pub damage_pull_speed_mul: f32,
    pub damage_pull_speed_down_mul: f32,
    pub damage_pull_attack_speed_max: f32,
    pub damage_pull_ground_angle: f32,
    pub damage_down_on_ground_y_speed_mul: f32,
    pub damage_turn_speed_ground_mul: f32,
    pub damage_turn_speed_air_mul: f32,
    pub down_damage_reaction: f32,
    pub down_damage_s_reaction: f32,
    pub slip_damage_reaction: f32,
    pub capture_cut_attack_power: f32,
    pub capture_cut_damage: f32,
    pub damage_fly_quake_m: f32,
    pub damage_fly_quake_l: f32,
    pub damage_fly_speed_up_reaction_frame_min: i32,
    pub damage_fly_speed_up_reaction_frame_max: i32,
    pub damage_fly_speed_up_end_rate: i32,
    pub damage_fly_speed_up_max_mag: i32,
    pub damage_fly_speed_up_angle_base: f32,
    pub damage_fly_speed_up_min_max_angle: f32,
    pub damage_fly_speed_up_angle_rate: f32,
    pub damage_target_pos_reaction_frame: f32,
    pub damage_target_pos_speed_limit: f32,
    pub hitstop_frame_max: f32,
    pub hitstop_frame_add: f32,
    pub hitstop_frame_mul: f32,
    pub hitstop_elec_mul: f32,
    pub hitstop_frame_paralyze_max: f32,
    pub hitstop_frame_paralyze_add: f32,
    pub hitstop_frame_paralyze_mul: f32,
    pub x1b63c43ae1: i32,
    pub just_shield_hitstop_frame_add: i32,
    pub just_shield_hitstop_hit_xlu_extent_grow: i32,
    pub x28492c704c: i32,
    pub x323a6f8f63: i32,
    pub x2bef82b907: i32,
    pub just_shield_hitstop_frame_max: i32,
    pub damage_shake_add_value: f32,
    pub damage_shake_mul_value: f32,
    pub x22a00d63c4: i32,
    pub x1c2fab2856: f32,
    pub capture_damage_shake_mul_value: f32,
    pub x1cfa62cbb2: i32,
    pub x1ebdc5f94a: f32,
    pub x27308926c5: f32,
    pub x2578843261: i32,
    pub x27d3b1a7c8: f32,
    pub x30125f75ea: f32,
    pub x23d2d966eb: i32,
    pub rebound_reaction_mul: f32,
    pub rebound_reaction_add: f32,
    pub rebound_frame: i32,
    pub rebound_reaction_max: i32,
    pub rebound_power_diff: f32,
    pub rebound_speed_add: f32,
    pub rebound_speed_mul: f32,
    pub reflect_count_max: i32,
    pub just_shield_reflect_attack_mul: f32,
    pub just_shield_reflect_speed_mul: f32,
    pub just_shield_reflect_life_mul: f32,
    pub just_shield_reflect_count_max: i32,
    pub x117e9b3582: f32,
    pub x112a8367c4: f32,
    pub x1176e209f3: f32,
    pub x114aef36aa: f32,
    pub x135d2f04a3: f32,
    pub x139ca1db63: f32,
    pub x16a568e4a4: f32,
    pub x16f170b6e2: f32,
    pub x16ad11d8d5: f32,
    pub x16911ce78c: f32,
    pub x186f25be36: f32,
    pub x18aeab61f6: f32,
    pub x2a9e6d7725: f32,
    pub hit_effect_rand_ofs_x: f32,
    pub hit_effect_rand_ofs_y: f32,
    pub hit_effect_rand_ofs_z: f32,
    pub rush_hit_effect_rand_ofs_x: f32,
    pub rush_hit_effect_rand_ofs_y: f32,
    pub rush_hit_effect_rand_ofs_z: f32,
    pub x239cfe6de5: f32,
    pub attack_direction_max_down_angle: f32,
    pub attack_direction_max_down_angle_sting: f32,
    pub jostle_area_outside_rate: f32,
    pub x1cbc105376: f32,
    pub x1c0536c807: f32,
    pub x2390f6ee99: f32,
    pub x24751bf94c: f32,
    pub jostle_weight_max: f32,
    pub jostle_weight_min: f32,
    pub jostle_weight_max_speed: f32,
    pub jostle_weight_mid_speed: f32,
    pub jostle_weight_min_speed: f32,
    pub jostle_weight_mul_ga: f32,
    pub jostle_weight_mul_aa: f32,
    pub jostle_weight_mul_ag: f32,
    pub jostle_pushed_max_speed: f32,
    pub x1dd35f2e72: f32,
    pub x1d850589f4: f32,
    pub x1d6c662cc1: f32,
    pub jostle_team_overlap_rate: f32,
    pub turn_param_damage_turn_angle: *mut [f32; 8],
    pad_00: [u64; 2],
    pub x0a0ce05bed: *mut [BattleObjectParamsStruct0; 8],
    pad_01: [u64; 2],
    pub x103d3224e2: *mut [BattleObjectParamsStruct1; 21],
    pad_02: [u64; 2],
    pub x121df2b78a: BattleObjectParamsStruct3,
    pub reaction_pattern_attack_rate: f32,
    pub damage_angle_air: f32,
    pub damage_pull_attack_speed_y_max: f32,
    pub damage_down_on_ground_angle: f32,
    pub fly_top_angle_lw: f32,
    pub fly_top_angle_hi: f32,
    pub shield_attack_range_mul: f32,
    pub shield_attack_range_mul_min: f32,
    pub x163cc39d64: f32,
    pub x1600cea23d: f32,
    pub x1b414bf630: f32,
    pub x1ba72b59d1: f32,
    pub fly_roll_damage: i32,
    pub fly_roll_rate: f32,
    pub back_damage_fly_roll_rate: f32,
    pub clatter_input_count_frame: i32,
    pub x192e58d48b: *mut [f32; 7],
    pad_03: [u64; 2],
    pub x1af80fd893: *mut [f32; 7],
    pad_04: [u64; 2],
    pub max_ink_value: f32,
    pub start_decrease_ink_frame: i32,
    pub decrease_ink_value: f32,
    pub max_ink_for_damage_mul: f32,
    pub max_damage_mul_for_ink: f32,
    pub ink_interpolate_frame: i32,
    pub start_decrease_oil_frame: i32,
    pub decrease_oil_value: f32,
    pub spirits_critical_attack_color: BattleObjectRGBColor,
    pub spirits_critical_defense_color: BattleObjectRGBColor,
    pub spirits_critical_throw_color: BattleObjectRGBColor,
}

impl super::Filepath for BattleObjectParams {
    fn filepath() -> &'static str {
        "fighter/common/param/battle_object.prc"
    }
}