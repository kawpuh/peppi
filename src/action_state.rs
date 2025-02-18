use std::fmt;

use ::arrow::datatypes;

use super::character::Internal;

macro_rules! state {
	($name: ident {
		$unknown: ident,
		$common: ident ( $common_type: ident ),
		$( $variant: ident ( $variant_type: ident ) => $internal: ident ),* $(,)?
	}) => {
		#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize)]
		#[serde(untagged)]
		pub enum $name {
			$common($common_type),
			$unknown(u16),
			$( $variant($variant_type), )*
		}

		impl $name {
			pub fn from(value: u16, character: Internal) -> $name {
				if value <= 340 {
					$name::$common($common_type(value))
				} else {
					match character {
						$( Internal::$internal => $name::$variant($variant_type(value)), )*
						_ => $name::$unknown(value),
					}
				}
			}
		}

		impl From<$name> for u16 {
			fn from(state: $name) -> u16 {
				match state {
					$name::$unknown(s) => s,
					$name::$common(s) => s.0,
					$( $name::$variant(s) => s.0, )*
				}
			}
		}

		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				 match *self {
					$name::$unknown(s) => write!(f, "{:?}", s),
					$name::$common(s) => write!(f, "{:?}", s),
					$( $name::$variant(s) => write!(f, "{:?}", s), )*
				}
			}
		}

		impl super::arrow::ArrowPrimitive for State {
			type ArrowNativeType = u16;
			type ArrowType = datatypes::UInt16Type;
			const ARROW_DATA_TYPE: datatypes::DataType = datatypes::DataType::UInt16;
			fn into_arrow_native(self) -> Self::ArrowNativeType {
				u16::from(self)
			}
		}
	}
}

state!(State {
    Unknown,
    Common(Common),
    Bowser(Bowser) => BOWSER,
    CaptainFalcon(CaptainFalcon) => CAPTAIN_FALCON,
    DonkeyKong(DonkeyKong) => DONKEY_KONG,
    DrMario(DrMario) => DR_MARIO,
    Falco(Falco) => FALCO,
    Fox(Fox) => FOX,
    GameAndWatch(GameAndWatch) => GAME_AND_WATCH,
    Ganondorf(Ganondorf) => GANONDORF,
    Jigglypuff(Jigglypuff) => JIGGLYPUFF,
    Kirby(Kirby) => KIRBY,
    Link(Link) => LINK,
    Luigi(Luigi) => LUIGI,
    Mario(Mario) => MARIO,
    Marth(Marth) => MARTH,
    Mewtwo(Mewtwo) => MEWTWO,
    Nana(Nana) => NANA,
    Ness(Ness) => NESS,
    Peach(Peach) => PEACH,
    Pichu(Pichu) => PICHU,
    Pikachu(Pikachu) => PIKACHU,
    Popo(Popo) => POPO,
    Roy(Roy) => ROY,
    Samus(Samus) => SAMUS,
    Sheik(Sheik) => SHEIK,
    Yoshi(Yoshi) => YOSHI,
    YoungLink(YoungLink) => YOUNG_LINK,
    Zelda(Zelda) => ZELDA,
});

pseudo_enum!(Common: u16 {
    000 => DEAD_DOWN,
    001 => DEAD_LEFT,
    002 => DEAD_RIGHT,
    003 => DEAD_UP,
    004 => DEAD_UP_STAR,
    005 => DEAD_UP_STAR_ICE,
    006 => DEAD_UP_FALL,
    007 => DEAD_UP_FALL_HIT_CAMERA,
    008 => DEAD_UP_FALL_HIT_CAMERA_FLAT,
    009 => DEAD_UP_FALL_ICE,
    010 => DEAD_UP_FALL_HIT_CAMERA_ICE,
    011 => SLEEP,
    012 => REBIRTH,
    013 => REBIRTH_WAIT,
    014 => WAIT,
    015 => WALK_SLOW,
    016 => WALK_MIDDLE,
    017 => WALK_FAST,
    018 => TURN,
    019 => TURN_RUN,
    020 => DASH,
    021 => RUN,
    022 => RUN_DIRECT,
    023 => RUN_BRAKE,
    024 => KNEE_BEND,
    025 => JUMP_F,
    026 => JUMP_B,
    027 => JUMP_AERIAL_F,
    028 => JUMP_AERIAL_B,
    029 => FALL,
    030 => FALL_F,
    031 => FALL_B,
    032 => FALL_AERIAL,
    033 => FALL_AERIAL_F,
    034 => FALL_AERIAL_B,
    035 => FALL_SPECIAL,
    036 => FALL_SPECIAL_F,
    037 => FALL_SPECIAL_B,
    038 => DAMAGE_FALL,
    039 => SQUAT,
    040 => SQUAT_WAIT,
    041 => SQUAT_RV,
    042 => LANDING,
    043 => LANDING_FALL_SPECIAL,
    044 => ATTACK_11,
    045 => ATTACK_12,
    046 => ATTACK_13,
    047 => ATTACK_100_START,
    048 => ATTACK_100_LOOP,
    049 => ATTACK_100_END,
    050 => ATTACK_DASH,
    051 => ATTACK_S_3_HI,
    052 => ATTACK_S_3_HI_S,
    053 => ATTACK_S_3_S,
    054 => ATTACK_S_3_LW_S,
    055 => ATTACK_S_3_LW,
    056 => ATTACK_HI_3,
    057 => ATTACK_LW_3,
    058 => ATTACK_S_4_HI,
    059 => ATTACK_S_4_HI_S,
    060 => ATTACK_S_4_S,
    061 => ATTACK_S_4_LW_S,
    062 => ATTACK_S_4_LW,
    063 => ATTACK_HI_4,
    064 => ATTACK_LW_4,
    065 => ATTACK_AIR_N,
    066 => ATTACK_AIR_F,
    067 => ATTACK_AIR_B,
    068 => ATTACK_AIR_HI,
    069 => ATTACK_AIR_LW,
    070 => LANDING_AIR_N,
    071 => LANDING_AIR_F,
    072 => LANDING_AIR_B,
    073 => LANDING_AIR_HI,
    074 => LANDING_AIR_LW,
    075 => DAMAGE_HI_1,
    076 => DAMAGE_HI_2,
    077 => DAMAGE_HI_3,
    078 => DAMAGE_N_1,
    079 => DAMAGE_N_2,
    080 => DAMAGE_N_3,
    081 => DAMAGE_LW_1,
    082 => DAMAGE_LW_2,
    083 => DAMAGE_LW_3,
    084 => DAMAGE_AIR_1,
    085 => DAMAGE_AIR_2,
    086 => DAMAGE_AIR_3,
    087 => DAMAGE_FLY_HI,
    088 => DAMAGE_FLY_N,
    089 => DAMAGE_FLY_LW,
    090 => DAMAGE_FLY_TOP,
    091 => DAMAGE_FLY_ROLL,
    092 => LIGHT_GET,
    093 => HEAVY_GET,
    094 => LIGHT_THROW_F,
    095 => LIGHT_THROW_B,
    096 => LIGHT_THROW_HI,
    097 => LIGHT_THROW_LW,
    098 => LIGHT_THROW_DASH,
    099 => LIGHT_THROW_DROP,
    100 => LIGHT_THROW_AIR_F,
    101 => LIGHT_THROW_AIR_B,
    102 => LIGHT_THROW_AIR_HI,
    103 => LIGHT_THROW_AIR_LW,
    104 => HEAVY_THROW_F,
    105 => HEAVY_THROW_B,
    106 => HEAVY_THROW_HI,
    107 => HEAVY_THROW_LW,
    108 => LIGHT_THROW_F_4,
    109 => LIGHT_THROW_B_4,
    110 => LIGHT_THROW_HI_4,
    111 => LIGHT_THROW_LW_4,
    112 => LIGHT_THROW_AIR_F_4,
    113 => LIGHT_THROW_AIR_B_4,
    114 => LIGHT_THROW_AIR_HI_4,
    115 => LIGHT_THROW_AIR_LW_4,
    116 => HEAVY_THROW_F_4,
    117 => HEAVY_THROW_B_4,
    118 => HEAVY_THROW_HI_4,
    119 => HEAVY_THROW_LW_4,
    120 => SWORD_SWING_1,
    121 => SWORD_SWING_3,
    122 => SWORD_SWING_4,
    123 => SWORD_SWING_DASH,
    124 => BAT_SWING_1,
    125 => BAT_SWING_3,
    126 => BAT_SWING_4,
    127 => BAT_SWING_DASH,
    128 => PARASOL_SWING_1,
    129 => PARASOL_SWING_3,
    130 => PARASOL_SWING_4,
    131 => PARASOL_SWING_DASH,
    132 => HARISEN_SWING_1,
    133 => HARISEN_SWING_3,
    134 => HARISEN_SWING_4,
    135 => HARISEN_SWING_DASH,
    136 => STAR_ROD_SWING_1,
    137 => STAR_ROD_SWING_3,
    138 => STAR_ROD_SWING_4,
    139 => STAR_ROD_SWING_DASH,
    140 => LIP_STICK_SWING_1,
    141 => LIP_STICK_SWING_3,
    142 => LIP_STICK_SWING_4,
    143 => LIP_STICK_SWING_DASH,
    144 => ITEM_PARASOL_OPEN,
    145 => ITEM_PARASOL_FALL,
    146 => ITEM_PARASOL_FALL_SPECIAL,
    147 => ITEM_PARASOL_DAMAGE_FALL,
    148 => L_GUN_SHOOT,
    149 => L_GUN_SHOOT_AIR,
    150 => L_GUN_SHOOT_EMPTY,
    151 => L_GUN_SHOOT_AIR_EMPTY,
    152 => FIRE_FLOWER_SHOOT,
    153 => FIRE_FLOWER_SHOOT_AIR,
    154 => ITEM_SCREW,
    155 => ITEM_SCREW_AIR,
    156 => DAMAGE_SCREW,
    157 => DAMAGE_SCREW_AIR,
    158 => ITEM_SCOPE_START,
    159 => ITEM_SCOPE_RAPID,
    160 => ITEM_SCOPE_FIRE,
    161 => ITEM_SCOPE_END,
    162 => ITEM_SCOPE_AIR_START,
    163 => ITEM_SCOPE_AIR_RAPID,
    164 => ITEM_SCOPE_AIR_FIRE,
    165 => ITEM_SCOPE_AIR_END,
    166 => ITEM_SCOPE_START_EMPTY,
    167 => ITEM_SCOPE_RAPID_EMPTY,
    168 => ITEM_SCOPE_FIRE_EMPTY,
    169 => ITEM_SCOPE_END_EMPTY,
    170 => ITEM_SCOPE_AIR_START_EMPTY,
    171 => ITEM_SCOPE_AIR_RAPID_EMPTY,
    172 => ITEM_SCOPE_AIR_FIRE_EMPTY,
    173 => ITEM_SCOPE_AIR_END_EMPTY,
    174 => LIFT_WAIT,
    175 => LIFT_WALK_1,
    176 => LIFT_WALK_2,
    177 => LIFT_TURN,
    178 => GUARD_ON,
    179 => GUARD,
    180 => GUARD_OFF,
    181 => GUARD_SET_OFF,
    182 => GUARD_REFLECT,
    183 => DOWN_BOUND_U,
    184 => DOWN_WAIT_U,
    185 => DOWN_DAMAGE_U,
    186 => DOWN_STAND_U,
    187 => DOWN_ATTACK_U,
    188 => DOWN_FOWARD_U,
    189 => DOWN_BACK_U,
    190 => DOWN_SPOT_U,
    191 => DOWN_BOUND_D,
    192 => DOWN_WAIT_D,
    193 => DOWN_DAMAGE_D,
    194 => DOWN_STAND_D,
    195 => DOWN_ATTACK_D,
    196 => DOWN_FOWARD_D,
    197 => DOWN_BACK_D,
    198 => DOWN_SPOT_D,
    199 => PASSIVE,
    200 => PASSIVE_STAND_F,
    201 => PASSIVE_STAND_B,
    202 => PASSIVE_WALL,
    203 => PASSIVE_WALL_JUMP,
    204 => PASSIVE_CEIL,
    205 => SHIELD_BREAK_FLY,
    206 => SHIELD_BREAK_FALL,
    207 => SHIELD_BREAK_DOWN_U,
    208 => SHIELD_BREAK_DOWN_D,
    209 => SHIELD_BREAK_STAND_U,
    210 => SHIELD_BREAK_STAND_D,
    211 => FURA_FURA,
    212 => CATCH,
    213 => CATCH_PULL,
    214 => CATCH_DASH,
    215 => CATCH_DASH_PULL,
    216 => CATCH_WAIT,
    217 => CATCH_ATTACK,
    218 => CATCH_CUT,
    219 => THROW_F,
    220 => THROW_B,
    221 => THROW_HI,
    222 => THROW_LW,
    223 => CAPTURE_PULLED_HI,
    224 => CAPTURE_WAIT_HI,
    225 => CAPTURE_DAMAGE_HI,
    226 => CAPTURE_PULLED_LW,
    227 => CAPTURE_WAIT_LW,
    228 => CAPTURE_DAMAGE_LW,
    229 => CAPTURE_CUT,
    230 => CAPTURE_JUMP,
    231 => CAPTURE_NECK,
    232 => CAPTURE_FOOT,
    233 => ESCAPE_F,
    234 => ESCAPE_B,
    235 => ESCAPE,
    236 => ESCAPE_AIR,
    237 => REBOUND_STOP,
    238 => REBOUND,
    239 => THROWN_F,
    240 => THROWN_B,
    241 => THROWN_HI,
    242 => THROWN_LW,
    243 => THROWN_LW_WOMEN,
    244 => PASS,
    245 => OTTOTTO,
    246 => OTTOTTO_WAIT,
    247 => FLY_REFLECT_WALL,
    248 => FLY_REFLECT_CEIL,
    249 => STOP_WALL,
    250 => STOP_CEIL,
    251 => MISS_FOOT,
    252 => CLIFF_CATCH,
    253 => CLIFF_WAIT,
    254 => CLIFF_CLIMB_SLOW,
    255 => CLIFF_CLIMB_QUICK,
    256 => CLIFF_ATTACK_SLOW,
    257 => CLIFF_ATTACK_QUICK,
    258 => CLIFF_ESCAPE_SLOW,
    259 => CLIFF_ESCAPE_QUICK,
    260 => CLIFF_JUMP_SLOW_1,
    261 => CLIFF_JUMP_SLOW_2,
    262 => CLIFF_JUMP_QUICK_1,
    263 => CLIFF_JUMP_QUICK_2,
    264 => APPEAL_R,
    265 => APPEAL_L,
    266 => SHOULDERED_WAIT,
    267 => SHOULDERED_WALK_SLOW,
    268 => SHOULDERED_WALK_MIDDLE,
    269 => SHOULDERED_WALK_FAST,
    270 => SHOULDERED_TURN,
    271 => THROWN_F_F,
    272 => THROWN_F_B,
    273 => THROWN_F_HI,
    274 => THROWN_F_LW,
    275 => CAPTURE_CAPTAIN,
    276 => CAPTURE_YOSHI,
    277 => YOSHI_EGG,
    278 => CAPTURE_KOOPA,
    279 => CAPTURE_DAMAGE_KOOPA,
    280 => CAPTURE_WAIT_KOOPA,
    281 => THROWN_KOOPA_F,
    282 => THROWN_KOOPA_B,
    283 => CAPTURE_KOOPA_AIR,
    284 => CAPTURE_DAMAGE_KOOPA_AIR,
    285 => CAPTURE_WAIT_KOOPA_AIR,
    286 => THROWN_KOOPA_AIR_F,
    287 => THROWN_KOOPA_AIR_B,
    288 => CAPTURE_KIRBY,
    289 => CAPTURE_WAIT_KIRBY,
    290 => THROWN_KIRBY_STAR,
    291 => THROWN_COPY_STAR,
    292 => THROWN_KIRBY,
    293 => BARREL_WAIT,
    294 => BURY,
    295 => BURY_WAIT,
    296 => BURY_JUMP,
    297 => DAMAGE_SONG,
    298 => DAMAGE_SONG_WAIT,
    299 => DAMAGE_SONG_RV,
    300 => DAMAGE_BIND,
    301 => CAPTURE_MEWTWO,
    302 => CAPTURE_MEWTWO_AIR,
    303 => THROWN_MEWTWO,
    304 => THROWN_MEWTWO_AIR,
    305 => WARP_STAR_JUMP,
    306 => WARP_STAR_FALL,
    307 => HAMMER_WAIT,
    308 => HAMMER_WALK,
    309 => HAMMER_TURN,
    310 => HAMMER_KNEE_BEND,
    311 => HAMMER_FALL,
    312 => HAMMER_JUMP,
    313 => HAMMER_LANDING,
    314 => KINOKO_GIANT_START,
    315 => KINOKO_GIANT_START_AIR,
    316 => KINOKO_GIANT_END,
    317 => KINOKO_GIANT_END_AIR,
    318 => KINOKO_SMALL_START,
    319 => KINOKO_SMALL_START_AIR,
    320 => KINOKO_SMALL_END,
    321 => KINOKO_SMALL_END_AIR,
    322 => ENTRY,
    323 => ENTRY_START,
    324 => ENTRY_END,
    325 => DAMAGE_ICE,
    326 => DAMAGE_ICE_JUMP,
    327 => CAPTURE_MASTER_HAND,
    328 => CAPTURE_DAMAGE_MASTER_HAND,
    329 => CAPTURE_WAIT_MASTER_HAND,
    330 => THROWN_MASTER_HAND,
    331 => CAPTURE_KIRBY_YOSHI,
    332 => KIRBY_YOSHI_EGG,
    333 => CAPTURE_REDEAD,
    334 => CAPTURE_LIKE_LIKE,
    335 => DOWN_REFLECT,
    336 => CAPTURE_CRAZY_HAND,
    337 => CAPTURE_DAMAGE_CRAZY_HAND,
    338 => CAPTURE_WAIT_CRAZY_HAND,
    339 => THROWN_CRAZY_HAND,
    340 => BARREL_CANNON_WAIT,
});

pseudo_enum!(Bowser: u16 {
    341 => FIRE_BREATH_GROUND_STARTUP,
    342 => FIRE_BREATH_GROUND_LOOP,
    343 => FIRE_BREATH_GROUND_END,
    344 => FIRE_BREATH_AIR_STARTUP,
    345 => FIRE_BREATH_AIR_LOOP,
    346 => FIRE_BREATH_AIR_END,
    347 => KOOPA_KLAW_GROUND,
    348 => KOOPA_KLAW_GROUND_GRAB,
    349 => KOOPA_KLAW_GROUND_PUMMEL,
    350 => KOOPA_KLAW_GROUND_WAIT,
    351 => KOOPA_KLAW_GROUND_THROW_F,
    352 => KOOPA_KLAW_GROUND_THROW_B,
    353 => KOOPA_KLAW_AIR,
    354 => KOOPA_KLAW_AIR_GRAB,
    355 => KOOPA_KLAW_AIR_PUMMEL,
    356 => KOOPA_KLAW_AIR_WAIT,
    357 => KOOPA_KLAW_AIR_THROW_F,
    358 => KOOPA_KLAW_AIR_THROW_B,
    359 => WHIRLING_FORTRESS_GROUND,
    360 => WHIRLING_FORTRESS_AIR,
    361 => BOMB_GROUND_BEGIN,
    362 => BOMB_AIR,
    363 => BOMB_LAND,
});

pseudo_enum!(CaptainFalcon: u16 {
    347 => FALCON_PUNCH_GROUND,
    348 => FALCON_PUNCH_AIR,
    349 => RAPTOR_BOOST_GROUND,
    350 => RAPTOR_BOOST_GROUND_HIT,
    351 => RAPTOR_BOOST_AIR,
    352 => RAPTOR_BOOST_AIR_HIT,
    353 => FALCON_DIVE_GROUND,
    354 => FALCON_DIVE_AIR,
    355 => FALCON_DIVE_CATCH,
    356 => FALCON_DIVE_ENDING,
    357 => FALCON_KICK_GROUND,
    358 => FALCON_KICK_GROUND_ENDING_ON_GROUND,
    359 => FALCON_KICK_AIR,
    360 => FALCON_KICK_AIR_ENDING_ON_GROUND,
    361 => FALCON_KICK_AIR_ENDING_IN_AIR,
    362 => FALCON_KICK_GROUND_ENDING_IN_AIR,
    363 => FALCON_KICK_HIT_WALL,
});

pseudo_enum!(DonkeyKong: u16 {
    351 => KONG_KARRY_WAIT,
    352 => KONG_KARRY_WALK_SLOW,
    353 => KONG_KARRY_WALK_MIDDLE,
    354 => KONG_KARRY_WALK_FAST,
    355 => KONG_KARRY_TURN,
    356 => KONG_KARRY_JUMP_SQUAT,
    357 => KONG_KARRY_FALL,
    358 => KONG_KARRY_JUMP,
    359 => KONG_KARRY_LANDING,
    361 => KONG_KARRY_GROUND_THROW_FORWARD,
    362 => KONG_KARRY_GROUND_THROW_BACKWARD,
    363 => KONG_KARRY_GROUND_THROW_UP,
    364 => KONG_KARRY_GROUND_THROW_DOWN,
    365 => KONG_KARRY_AIR_THROW_FORWARD,
    366 => KONG_KARRY_AIR_THROW_BACKWARD,
    367 => KONG_KARRY_AIR_THROW_UP,
    368 => KONG_KARRY_AIR_THROW_DOWN,
    369 => GIANT_PUNCH_GROUND_CHARGE_STARTUP,
    370 => GIANT_PUNCH_GROUND_CHARGE_LOOP,
    371 => GIANT_PUNCH_GROUND_CHARGE_STOP,
    372 => GIANT_PUNCH_GROUND_EARLY_PUNCH,
    373 => GIANT_PUNCH_GROUND_FULL_CHARGE_PUNCH,
    374 => GIANT_PUNCH_AIR_CHARGE_STARTUP,
    375 => GIANT_PUNCH_AIR_CHARGE_LOOP,
    376 => GIANT_PUNCH_AIR_CHARGE_STOP,
    377 => GIANT_PUNCH_AIR_EARLY_PUNCH,
    378 => GIANT_PUNCH_AIR_FULL_CHARGE_PUNCH,
    379 => HEADBUTT_GROUND,
    380 => HEADBUTT_AIR,
    381 => SPINNING_KONG_GROUND,
    382 => SPINNING_KONG_AIR,
    383 => HAND_SLAP_STARTUP,
    384 => HAND_SLAP_LOOP,
    385 => HAND_SLAP_END,
});

pseudo_enum!(DrMario: u16 {
    341 => TAUNT_R,
    343 => MEGAVITAMIN_GROUND,
    344 => MEGAVITAMIN_AIR,
    345 => SUPER_SHEET_GROUND,
    346 => SUPER_SHEET_AIR,
    347 => SUPER_JUMP_PUNCH_GROUND,
    348 => SUPER_JUMP_PUNCH_AIR,
    349 => TORNADO_GROUND,
    350 => TORNADO_AIR,
});

pseudo_enum!(Falco: u16 {
    341 => BLASTER_GROUND_STARTUP,
    342 => BLASTER_GROUND_LOOP,
    343 => BLASTER_GROUND_END,
    344 => BLASTER_AIR_STARTUP,
    345 => BLASTER_AIR_LOOP,
    346 => BLASTER_AIR_END,
    347 => PHANTASM_GROUND_STARTUP,
    348 => PHANTASM_GROUND,
    349 => PHANTASM_GROUND_END,
    350 => PHANTASM_STARTUP_AIR,
    351 => PHANTASM_AIR,
    352 => PHANTASM_AIR_END,
    353 => FIRE_BIRD_GROUND_STARTUP,
    354 => FIRE_BIRD_AIR_STARTUP,
    355 => FIRE_BIRD_GROUND,
    356 => FIRE_BIRD_AIR,
    357 => FIRE_BIRD_GROUND_END,
    358 => FIRE_BIRD_AIR_END,
    359 => FIRE_BIRD_BOUNCE_END,
    360 => REFLECTOR_GROUND_STARTUP,
    361 => REFLECTOR_GROUND_LOOP,
    362 => REFLECTOR_GROUND_REFLECT,
    363 => REFLECTOR_GROUND_END,
    364 => REFLECTOR_GROUND_CHANGE_DIRECTION,
    365 => REFLECTOR_AIR_STARTUP,
    366 => REFLECTOR_AIR_LOOP,
    367 => REFLECTOR_AIR_REFLECT,
    368 => REFLECTOR_AIR_END,
    369 => REFLECTOR_AIR_CHANGE_DIRECTION,
    370 => SMASH_TAUNT_RIGHT_STARTUP,
    371 => SMASH_TAUNT_LEFT_STARTUP,
    372 => SMASH_TAUNT_RIGHT_RISE,
    373 => SMASH_TAUNT_LEFT_RISE,
    374 => SMASH_TAUNT_RIGHT_FINISH,
    375 => SMASH_TAUNT_LEFT_FINISH,
});

pseudo_enum!(Fox: u16 {
    341 => BLASTER_GROUND_STARTUP,
    342 => BLASTER_GROUND_LOOP,
    343 => BLASTER_GROUND_END,
    344 => BLASTER_AIR_STARTUP,
    345 => BLASTER_AIR_LOOP,
    346 => BLASTER_AIR_END,
    347 => ILLUSION_GROUND_STARTUP,
    348 => ILLUSION_GROUND,
    349 => ILLUSION_GROUND_END,
    350 => ILLUSION_STARTUP_AIR,
    351 => ILLUSION_AIR,
    352 => ILLUSION_AIR_END,
    353 => FIRE_FOX_GROUND_STARTUP,
    354 => FIRE_FOX_AIR_STARTUP,
    355 => FIRE_FOX_GROUND,
    356 => FIRE_FOX_AIR,
    357 => FIRE_FOX_GROUND_END,
    358 => FIRE_FOX_AIR_END,
    359 => FIRE_FOX_BOUNCE_END,
    360 => REFLECTOR_GROUND_STARTUP,
    361 => REFLECTOR_GROUND_LOOP,
    362 => REFLECTOR_GROUND_REFLECT,
    363 => REFLECTOR_GROUND_END,
    364 => REFLECTOR_GROUND_CHANGE_DIRECTION,
    365 => REFLECTOR_AIR_STARTUP,
    366 => REFLECTOR_AIR_LOOP,
    367 => REFLECTOR_AIR_REFLECT,
    368 => REFLECTOR_AIR_END,
    369 => REFLECTOR_AIR_CHANGE_DIRECTION,
    370 => SMASH_TAUNT_RIGHT_STARTUP,
    371 => SMASH_TAUNT_LEFT_STARTUP,
    372 => SMASH_TAUNT_RIGHT_RISE,
    373 => SMASH_TAUNT_LEFT_RISE,
    374 => SMASH_TAUNT_RIGHT_FINISH,
    375 => SMASH_TAUNT_LEFT_FINISH,
});

pseudo_enum!(GameAndWatch: u16 {
    341 => JAB,
    342 => JAB_2,
    343 => RAPID_JABS,
    344 => RAPID_JABS_END,
    345 => DOWN_TILT,
    346 => SIDE_SMASH,
    347 => NAIR,
    348 => BAIR,
    349 => UAIR,
    350 => NAIR_LANDING,
    351 => BAIR_LANDING,
    352 => UAIR_LANDING,
    353 => CHEF_GROUND,
    354 => CHEF_AIR,
    355 => JUDGMENT_1_GROUND,
    356 => JUDGMENT_2_GROUND,
    357 => JUDGMENT_3_GROUND,
    358 => JUDGMENT_4_GROUND,
    359 => JUDGMENT_5_GROUND,
    360 => JUDGMENT_6_GROUND,
    361 => JUDGMENT_7_GROUND,
    362 => JUDGMENT_8_GROUND,
    363 => JUDGMENT_9_GROUND,
    364 => JUDGMENT_1_AIR,
    365 => JUDGMENT_2_AIR,
    366 => JUDGMENT_3_AIR,
    367 => JUDGMENT_4_AIR,
    368 => JUDGMENT_5_AIR,
    369 => JUDGMENT_6_AIR,
    370 => JUDGMENT_7_AIR,
    371 => JUDGMENT_8_AIR,
    372 => JUDGMENT_9_AIR,
    373 => FIRE_GROUND,
    374 => FIRE_AIR,
    375 => OIL_PANIC_GROUND,
    376 => OIL_PANIC_GROUND_ABSORB,
    377 => OIL_PANIC_GROUND_SPILL,
    378 => OIL_PANIC_AIR,
    379 => OIL_PANIC_AIR_ABSORB,
    380 => OIL_PANIC_AIR_SPILL,
});

pseudo_enum!(Ganondorf: u16 {
    347 => WARLOCK_PUNCH_GROUND,
    348 => WARLOCK_PUNCH_AIR,
    349 => GERUDO_DRAGON_GROUND,
    350 => GERUDO_DRAGON_GROUND_HIT,
    351 => GERUDO_DRAGON_AIR,
    352 => GERUDO_DRAGON_AIR_HIT,
    353 => DARK_DIVE_GROUND,
    354 => DARK_DIVE_AIR,
    355 => DARK_DIVE_CATCH,
    356 => DARK_DIVE_ENDING,
    357 => WIZARDS_FOOT_GROUND,
    358 => WIZARDS_FOOT_GROUND_ENDING_ON_GROUND,
    359 => WIZARDS_FOOT_AIR,
    360 => WIZARDS_FOOT_AIR_ENDING_ON_GROUND,
    361 => WIZARDS_FOOT_AIR_ENDING_IN_AIR,
    362 => WIZARDS_FOOT_GROUND_ENDING_IN_AIR,
    363 => WIZARDS_FOOT_HIT_WALL,
});

pseudo_enum!(Jigglypuff: u16 {
    341 => JUMP_2,
    342 => JUMP_3,
    343 => JUMP_4,
    344 => JUMP_5,
    345 => JUMP_6,
    346 => ROLLOUT_GROUND_START_CHARGE_RIGHT,
    347 => ROLLOUT_GROUND_START_CHARGE_LEFT,
    348 => ROLLOUT_GROUND_CHARGE_LOOP,
    349 => ROLLOUT_GROUND_FULLY_CHARGED,
    350 => ROLLOUT_GROUND_CHARGE_RELEASE,
    351 => ROLLOUT_GROUND_START_TURN,
    352 => ROLLOUT_GROUND_END_RIGHT,
    353 => ROLLOUT_GROUND_END_LEFT,
    354 => ROLLOUT_AIR_START_CHARGE_RIGHT,
    355 => ROLLOUT_AIR_START_CHARGE_LEFT,
    356 => ROLLOUT_AIR_CHARGE_LOOP,
    357 => ROLLOUT_AIR_FULLY_CHARGED,
    358 => ROLLOUT_AIR_CHARGE_RELEASE,
    360 => ROLLOUT_AIR_END_RIGHT,
    361 => ROLLOUT_AIR_END_LEFT,
    362 => ROLLOUT_HIT,
    363 => POUND_GROUND,
    364 => POUND_AIR,
    365 => SING_GROUND_LEFT,
    366 => SING_AIR_LEFT,
    367 => SING_GROUND_RIGHT,
    368 => SING_AIR_RIGHT,
    369 => REST_GROUND_LEFT,
    370 => REST_AIR_LEFT,
    371 => REST_GROUND_RIGHT,
    372 => REST_AIR_RIGHT,
});

pseudo_enum!(Kirby: u16 {
    341 => JUMP_2,
    342 => JUMP_3,
    343 => JUMP_4,
    344 => JUMP_5,
    345 => JUMP_6,
    346 => JUMP_2_WITH_HAT,
    347 => JUMP_3_WITH_HAT,
    348 => JUMP_4_WITH_HAT,
    349 => JUMP_5_WITH_HAT,
    350 => JUMP_6_WITH_HAT,
    351 => DASH_ATTACK_GROUND,
    352 => DASH_ATTACK_AIR,
    353 => SWALLOW_GROUND_STARTUP,
    354 => SWALLOW_GROUND_LOOP,
    355 => SWALLOW_GROUND_END,
    356 => SWALLOW_GROUND_CAPTURE,
    358 => SWALLOW_GROUND_CAPTURED,
    359 => SWALLOW_GROUND_CAPTURE_WAIT,
    360 => SWALLOW_CAPTURE_WALK_SLOW,
    361 => SWALLOW_CAPTURE_WALK_MIDDLE,
    362 => SWALLOW_CAPTURE_WALK_FAST,
    363 => SWALLOW_GROUND_CAPTURE_TURN,
    364 => SWALLOW_CAPTURE_JUMP_SQUAT,
    365 => SWALLOW_CAPTURE_JUMP,
    366 => SWALLOW_CAPTURE_LANDING,
    367 => SWALLOW_GROUND_DIGEST,
    369 => SWALLOW_GROUND_SPIT,
    371 => SWALLOW_AIR_STARTUP,
    372 => SWALLOW_AIR_LOOP,
    373 => SWALLOW_AIR_END,
    374 => SWALLOW_AIR_CAPTURE,
    376 => SWALLOW_AIR_CAPTURED,
    377 => SWALLOW_AIR_CAPTURE_WAIT,
    378 => SWALLOW_AIR_DIGEST,
    380 => SWALLOW_AIR_SPIT,
    382 => SWALLOW_AIR_CAPTURE_TURN,
    383 => HAMMER_GROUND,
    384 => HAMMER_AIR,
    385 => FINAL_CUTTER_GROUND_STARTUP,
    388 => FINAL_CUTTER_GROUND_END,
    389 => FINAL_CUTTER_AIR_STARTUP,
    390 => FINAL_CUTTER_AIR_APEX,
    391 => FINAL_CUTTER_SWORD_DESCENT,
    392 => FINAL_CUTTER_AIR_END,
    393 => STONE_GROUND_STARTUP,
    394 => STONE_GROUND,
    395 => STONE_GROUND_END,
    396 => STONE_AIR_STARTUP,
    397 => STONE_AIR,
    398 => STONE_AIR_END,
    399 => MARIO_FIREBALL_GROUND,
    400 => MARIO_FIREBALL_AIR,
    401 => LINK_BOW_GROUND_CHARGE,
    402 => LINK_BOW_GROUND_FULLY_CHARGED,
    403 => LINK_BOW_GROUND_FIRE,
    404 => LINK_BOW_AIR_CHARGE,
    405 => LINK_BOW_AIR_FULLY_CHARGED,
    406 => LINK_BOW_AIR_FIRE,
    407 => SAMUS_CHARGE_SHOT_GROUND_START,
    408 => SAMUS_CHARGE_SHOT_GROUND_LOOP,
    409 => SAMUS_CHARGE_SHOT_GROUND_END,
    410 => SAMUS_CHARGE_SHOT_GROUND_FIRE,
    411 => SAMUS_CHARGE_SHOT_AIR_START,
    412 => SAMUS_CHARGE_SHOT_AIR_FIRE,
    413 => YOSHI_EGG_LAY_GROUND,
    414 => YOSHI_EGG_LAY_GROUND_CAPTURE_START,
    416 => YOSHI_EGG_LAY_GROUND_CAPTURE,
    418 => YOSHI_EGG_LAY_AIR,
    419 => YOSHI_EGG_LAY_AIR_CAPTURE_START,
    421 => YOSHI_EGG_LAY_AIR_CAPTURE,
    423 => FOX_BLASTER_GROUND_STARTUP,
    424 => FOX_BLASTER_GROUND_LOOP,
    425 => FOX_BLASTER_GROUND_END,
    426 => FOX_BLASTER_AIR_STARTUP,
    427 => FOX_BLASTER_AIR_LOOP,
    428 => FOX_BLASTER_AIR_END,
    429 => PIKACHU_THUNDER_JOLT_GROUND,
    430 => PIKACHU_THUNDER_JOLT_AIR,
    431 => LUIGI_FIREBALL_GROUND,
    432 => LUIGI_FIREBALL_AIR,
    433 => FALCON_FALCON_PUNCH_GROUND,
    434 => FALCON_FALCON_PUNCH_AIR,
    435 => NESS_PK_FLASH_GROUND_STARTUP,
    436 => NESS_PK_FLASH_GROUND_CHARGE,
    437 => NESS_PK_FLASH_GROUND_EXPLODE,
    438 => NESS_PK_FLASH_GROUND_END,
    439 => NESS_PK_FLASH_AIR_STARTUP,
    440 => NESS_PK_FLASH_AIR_CHARGE,
    441 => NESS_PK_FLASH_AIR_EXPLODE,
    442 => NESS_PK_FLASH_AIR_END,
    443 => BOWSER_FIRE_BREATH_GROUND_START,
    444 => BOWSER_FIRE_BREATH_GROUND_LOOP,
    445 => BOWSER_FIRE_BREATH_GROUND_END,
    446 => BOWSER_FIRE_BREATH_AIR_START,
    447 => BOWSER_FIRE_BREATH_AIR_LOOP,
    448 => BOWSER_FIRE_BREATH_AIR_END,
    449 => PEACH_TOAD_GROUND,
    450 => PEACH_TOAD_GROUND_ATTACK,
    451 => PEACH_TOAD_AIR,
    452 => PEACH_TOAD_AIR_ATTACK,
    453 => ICE_CLIMBERS_ICE_SHOT_GROUND,
    454 => ICE_CLIMBERS_ICE_SHOT_AIR,
    455 => DK_GIANT_PUNCH_GROUND_CHARGE_STARTUP,
    456 => DK_GIANT_PUNCH_GROUND_CHARGE_LOOP,
    457 => DK_GIANT_PUNCH_GROUND_CHARGE_STOP,
    458 => DK_GIANT_PUNCH_GROUND_EARLY_PUNCH,
    459 => DK_GIANT_PUNCH_GROUND_FULL_CHARGE_PUNCH,
    460 => DK_GIANT_PUNCH_AIR_CHARGE_STARTUP,
    461 => DK_GIANT_PUNCH_AIR_CHARGE_LOOP,
    462 => DK_GIANT_PUNCH_AIR_CHARGE_STOP,
    463 => DK_GIANT_PUNCH_AIR_EARLY_PUNCH,
    464 => DK_GIANT_PUNCH_AIR_FULL_CHARGE_PUNCH,
    465 => ZELDA_NAYRUS_LOVE_GROUND,
    466 => ZELDA_NAYRUS_LOVE_AIR,
    467 => SHEIK_NEEDLE_STORM_GROUND_START_CHARGE,
    468 => SHEIK_NEEDLE_STORM_GROUND_CHARGE_LOOP,
    469 => SHEIK_NEEDLE_STORM_GROUND_END_CHARGE,
    470 => SHEIK_NEEDLE_STORM_GROUND_FIRE,
    471 => SHEIK_NEEDLE_STORM_AIR_START_CHARGE,
    472 => SHEIK_NEEDLE_STORM_AIR_CHARGE_LOOP,
    473 => SHEIK_NEEDLE_STORM_AIR_END_CHARGE,
    474 => SHEIK_NEEDLE_STORM_AIR_FIRE,
    475 => JIGGLYPUFF_ROLLOUT_GROUND_START_CHARGE_RIGHT,
    476 => JIGGLYPUFF_ROLLOUT_GROUND_START_CHARGE_LEFT,
    477 => JIGGLYPUFF_ROLLOUT_GROUND_CHARGE_LOOP,
    478 => JIGGLYPUFF_ROLLOUT_GROUND_FULLY_CHARGED,
    479 => JIGGLYPUFF_ROLLOUT_GROUND_CHARGE_RELEASE,
    480 => JIGGLYPUFF_ROLLOUT_GROUND_START_TURN,
    481 => JIGGLYPUFF_ROLLOUT_GROUND_END_RIGHT,
    482 => JIGGLYPUFF_ROLLOUT_GROUND_END_LEFT,
    483 => JIGGLYPUFF_ROLLOUT_AIR_START_CHARGE_RIGHT,
    484 => JIGGLYPUFF_ROLLOUT_AIR_START_CHARGE_LEFT,
    485 => JIGGLYPUFF_ROLLOUT_AIR_CHARGE_LOOP,
    486 => JIGGLYPUFF_ROLLOUT_AIR_FULLY_CHARGED,
    487 => JIGGLYPUFF_ROLLOUT_AIR_CHARGE_RELEASE,
    489 => JIGGLYPUFF_ROLLOUT_AIR_END_RIGHT,
    490 => JIGGLYPUFF_ROLLOUT_AIR_END_LEFT,
    491 => JIGGLYPUFF_ROLLOUT_HIT,
    492 => MARTH_SHIELD_BREAKER_GROUND_START_CHARGE,
    493 => MARTH_SHIELD_BREAKER_GROUND_CHARGE_LOOP,
    494 => MARTH_SHIELD_BREAKER_GROUND_EARLY_RELEASE,
    495 => MARTH_SHIELD_BREAKER_GROUND_FULLY_CHARGED,
    496 => MARTH_SHIELD_BREAKER_AIR_START_CHARGE,
    497 => MARTH_SHIELD_BREAKER_AIR_CHARGE_LOOP,
    498 => MARTH_SHIELD_BREAKER_AIR_EARLY_RELEASE,
    499 => MARTH_SHIELD_BREAKER_AIR_FULLY_CHARGED,
    500 => MEWTWO_SHADOW_BALL_GROUND_START_CHARGE,
    501 => MEWTWO_SHADOW_BALL_GROUND_CHARGE_LOOP,
    502 => MEWTWO_SHADOW_BALL_GROUND_FULLY_CHARGED,
    503 => MEWTWO_SHADOW_BALL_GROUND_END_CHARGE,
    504 => MEWTWO_SHADOW_BALL_GROUND_FIRE,
    505 => MEWTWO_SHADOW_BALL_AIR_START_CHARGE,
    506 => MEWTWO_SHADOW_BALL_AIR_CHARGE_LOOP,
    507 => MEWTWO_SHADOW_BALL_AIR_FULLY_CHARGED,
    508 => MEWTWO_SHADOW_BALL_AIR_END_CHARGE,
    509 => MEWTWO_SHADOW_BALL_AIR_FIRE,
    510 => GAMEAND_WATCH_OIL_PANIC_GROUND,
    511 => GAMEAND_WATCH_OIL_PANIC_AIR,
    512 => DOC_MEGAVITAMIN_GROUND,
    513 => DOC_MEGAVITAMIN_AIR,
    514 => YOUNG_LINK_FIRE_BOW_GROUND_CHARGE,
    515 => YOUNG_LINK_FIRE_BOW_GROUND_FULLY_CHARGED,
    516 => YOUNG_LINK_FIRE_BOW_GROUND_FIRE,
    517 => YOUNG_LINK_FIRE_BOW_AIR_CHARGE,
    518 => YOUNG_LINK_FIRE_BOW_AIR_FULLY_CHARGED,
    519 => YOUNG_LINK_FIRE_BOW_AIR_FIRE,
    520 => FALCO_BLASTER_GROUND_STARTUP,
    521 => FALCO_BLASTER_GROUND_LOOP,
    522 => FALCO_BLASTER_GROUND_END,
    523 => FALCO_BLASTER_AIR_STARTUP,
    524 => FALCO_BLASTER_AIR_LOOP,
    525 => FALCO_BLASTER_AIR_END,
    526 => PICHU_THUNDER_JOLT_GROUND,
    527 => PICHU_THUNDER_JOLT_AIR,
    528 => GANON_WARLOCK_PUNCH_GROUND,
    529 => GANON_WARLOCK_PUNCH_AIR,
    530 => ROY_FLARE_BLADE_GROUND_START_CHARGE,
    531 => ROY_FLARE_BLADE_GROUND_CHARGE_LOOP,
    532 => ROY_FLARE_BLADE_GROUND_EARLY_RELEASE,
    533 => ROY_FLARE_BLADE_GROUND_FULLY_CHARGED,
    534 => ROY_FLARE_BLADE_AIR_START_CHARGE,
    535 => ROY_FLARE_BLADE_AIR_CHARGE_LOOP,
    536 => ROY_FLARE_BLADE_AIR_EARLY_RELEASE,
    537 => ROY_FLARE_BLADE_AIR_FULLY_CHARGED,
});

pseudo_enum!(Link: u16 {
    341 => SIDE_SMASH_2,
    344 => BOW_GROUND_CHARGE,
    345 => BOW_GROUND_FULLY_CHARGED,
    346 => BOW_GROUND_FIRE,
    347 => BOW_AIR_CHARGE,
    348 => BOW_AIR_FULLY_CHARGED,
    349 => BOW_AIR_FIRE,
    350 => BOOMERANG_GROUND_THROW,
    351 => BOOMERANG_GROUND_CATCH,
    352 => BOOMERANG_GROUND_THROW_EMPTY,
    353 => BOOMERANG_AIR_THROW,
    354 => BOOMERANG_AIR_CATCH,
    355 => BOOMERANG_AIR_THROW_EMPTY,
    356 => SPIN_ATTACK_GROUND,
    357 => SPIN_ATTACK_AIR,
    358 => BOMB_GROUND,
    359 => BOMB_AIR,
    360 => ZAIR,
    361 => ZAIR_CATCH,
});

pseudo_enum!(Luigi: u16 {
    341 => FIREBALL_GROUND,
    342 => FIREBALL_AIR,
    343 => GREEN_MISSILE_GROUND_STARTUP,
    344 => GREEN_MISSILE_GROUND_CHARGE,
    346 => GREEN_MISSILE_GROUND_LANDING,
    347 => GREEN_MISSILE_GROUND_TAKEOFF,
    348 => GREEN_MISSILE_GROUND_TAKEOFF_MISFIRE,
    349 => GREEN_MISSILE_AIR_STARTUP,
    350 => GREEN_MISSILE_AIR_CHARGE,
    351 => GREEN_MISSILE_AIR,
    352 => GREEN_MISSILE_AIR_END,
    353 => GREEN_MISSILE_AIR_TAKEOFF,
    354 => GREEN_MISSILE_AIR_TAKEOFF_MISFIRE,
    355 => SUPER_JUMP_PUNCH_GROUND,
    356 => SUPER_JUMP_PUNCH_AIR,
    357 => CYCLONE_GROUND,
    358 => CYCLONE_AIR,
});

pseudo_enum!(Mario: u16 {
    343 => FIREBALL_GROUND,
    344 => FIREBALL_AIR,
    345 => CAPE_GROUND,
    346 => CAPE_AIR,
    347 => SUPER_JUMP_PUNCH_GROUND,
    348 => SUPER_JUMP_PUNCH_AIR,
    349 => TORNADO_GROUND,
    350 => TORNADO_AIR,
});

pseudo_enum!(Marth: u16 {
    341 => SHIELD_BREAKER_GROUND_START_CHARGE,
    342 => SHIELD_BREAKER_GROUND_CHARGE_LOOP,
    343 => SHIELD_BREAKER_GROUND_EARLY_RELEASE,
    344 => SHIELD_BREAKER_GROUND_FULLY_CHARGED,
    345 => SHIELD_BREAKER_AIR_START_CHARGE,
    346 => SHIELD_BREAKER_AIR_CHARGE_LOOP,
    347 => SHIELD_BREAKER_AIR_EARLY_RELEASE,
    348 => SHIELD_BREAKER_AIR_FULLY_CHARGED,
    349 => DANCING_BLADE_1_GROUND,
    350 => DANCING_BLADE_2_UP_GROUND,
    351 => DANCING_BLADE_2_SIDE_GROUND,
    352 => DANCING_BLADE_3_UP_GROUND,
    353 => DANCING_BLADE_3_SIDE_GROUND,
    354 => DANCING_BLADE_3_DOWN_GROUND,
    355 => DANCING_BLADE_4_UP_GROUND,
    356 => DANCING_BLADE_4_SIDE_GROUND,
    357 => DANCING_BLADE_4_DOWN_GROUND,
    358 => DANCING_BLADE_1_AIR,
    359 => DANCING_BLADE_2_UP_AIR,
    360 => DANCING_BLADE_2_SIDE_AIR,
    361 => DANCING_BLADE_3_UP_AIR,
    362 => DANCING_BLADE_3_SIDE_AIR,
    363 => DANCING_BLADE_3_DOWN_AIR,
    364 => DANCING_BLADE_4_UP_AIR,
    365 => DANCING_BLADE_4_SIDE_AIR,
    366 => DANCING_BLADE_4_DOWN_AIR,
    367 => DOLPHIN_SLASH_GROUND,
    368 => DOLPHIN_SLASH_AIR,
    369 => COUNTER_GROUND,
    370 => COUNTER_GROUND_HIT,
    371 => COUNTER_AIR,
    372 => COUNTER_AIR_HIT,
});

pseudo_enum!(Mewtwo: u16 {
    341 => SHADOW_BALL_GROUND_START_CHARGE,
    342 => SHADOW_BALL_GROUND_CHARGE_LOOP,
    343 => SHADOW_BALL_GROUND_FULLY_CHARGED,
    344 => SHADOW_BALL_GROUND_END_CHARGE,
    345 => SHADOW_BALL_GROUND_FIRE,
    346 => SHADOW_BALL_AIR_START_CHARGE,
    347 => SHADOW_BALL_AIR_CHARGE_LOOP,
    348 => SHADOW_BALL_AIR_FULLY_CHARGED,
    349 => SHADOW_BALL_AIR_END_CHARGE,
    350 => SHADOW_BALL_AIR_FIRE,
    351 => CONFUSION_GROUND,
    352 => CONFUSION_AIR,
    353 => TELEPORT_GROUND_STARTUP,
    354 => TELEPORT_GROUND_DISAPPEAR,
    355 => TELEPORT_GROUND_REAPPEAR,
    356 => TELEPORT_AIR_STARTUP,
    357 => TELEPORT_AIR_DISAPPEAR,
    358 => TELEPORT_AIR_REAPPEAR,
    359 => DISABLE_GROUND,
    360 => DISABLE_AIR,
});

pseudo_enum!(Nana: u16 {
    341 => ICE_SHOT_GROUND,
    342 => ICE_SHOT_AIR,
    357 => BLIZZARD_GROUND,
    358 => BLIZZARD_AIR,
    359 => SQUALL_HAMMER_GROUND_TOGETHER,
    360 => SQUALL_HAMMER_AIR_TOGETHER,
    361 => BELAY_CATAPULT_STARTUP,
    362 => BELAY_GROUND_CATAPULT_END,
    365 => BELAY_CATAPULTING,
});

pseudo_enum!(Ness: u16 {
    341 => SIDE_SMASH,
    342 => UP_SMASH,
    343 => UP_SMASH_CHARGE,
    344 => UP_SMASH_CHARGED,
    345 => DOWN_SMASH,
    346 => DOWN_SMASH_CHARGE,
    347 => DOWN_SMASH_CHARGED,
    348 => PK_FLASH_GROUND_STARTUP,
    349 => PK_FLASH_GROUND_CHARGE,
    350 => PK_FLASH_GROUND_EXPLODE,
    351 => PK_FLASH_GROUND_END,
    352 => PK_FLASH_AIR_STARTUP,
    353 => PK_FLASH_AIR_CHARGE,
    354 => PK_FLASH_AIR_EXPLODE,
    355 => PK_FLASH_AIR_END,
    356 => PK_FIRE_GROUND,
    357 => PK_FIRE_AIR,
    358 => PK_THUNDER_GROUND_STARTUP,
    359 => PK_THUNDER_GROUND,
    360 => PK_THUNDER_GROUND_END,
    361 => PK_THUNDER_GROUND_HIT,
    362 => PK_THUNDER_AIR_STARTUP,
    363 => PK_THUNDER_AIR,
    364 => PK_THUNDER_AIR_END,
    365 => PK_THUNDER_AIR_HIT,
    366 => PK_THUNDER_AIR_HIT_WALL,
    367 => PSI_MAGNET_GROUND_STARTUP,
    368 => PSI_MAGNET_GROUND_LOOP,
    369 => PSI_MAGNET_GROUND_ABSORB,
    370 => PSI_MAGNET_GROUND_END,
    372 => PSI_MAGNET_AIR_STARTUP,
    373 => PSI_MAGNET_AIR_LOOP,
    374 => PSI_MAGNET_AIR_ABSORB,
    375 => PSI_MAGNET_AIR_END,
});

pseudo_enum!(Peach: u16 {
    341 => FLOAT,
    342 => FLOAT_END_FORWARD,
    343 => FLOAT_END_BACKWARD,
    344 => FLOAT_NAIR,
    345 => FLOAT_FAIR,
    346 => FLOAT_BAIR,
    347 => FLOAT_UAIR,
    348 => FLOAT_DAIR,
    349 => SIDE_SMASH_GOLF_CLUB,
    350 => SIDE_SMASH_FRYING_PAN,
    351 => SIDE_SMASH_TENNIS_RACKET,
    352 => VEGETABLE_GROUND,
    353 => VEGETABLE_AIR,
    354 => BOMBER_GROUND_STARTUP,
    355 => BOMBER_GROUND_END,
    357 => BOMBER_AIR_STARTUP,
    358 => BOMBER_AIR_END,
    359 => BOMBER_AIR_HIT,
    360 => BOMBER_AIR,
    361 => PARASOL_GROUND_START,
    363 => PARASOL_AIR_START,
    365 => TOAD_GROUND,
    366 => TOAD_GROUND_ATTACK,
    367 => TOAD_AIR,
    368 => TOAD_AIR_ATTACK,
    369 => PARASOL_OPENING,
    370 => PARASOL_OPEN,
});

pseudo_enum!(Pichu: u16 {
    341 => THUNDER_JOLT_GROUND,
    342 => THUNDER_JOLT_AIR,
    343 => SKULL_BASH_GROUND_STARTUP,
    344 => SKULL_BASH_GROUND_CHARGE,
    346 => SKULL_BASH_GROUND_LANDING,
    347 => SKULL_BASH_GROUND_TAKEOFF,
    348 => SKULL_BASH_AIR_STARTUP,
    349 => SKULL_BASH_AIR_CHARGE,
    350 => SKULL_BASH_AIR,
    351 => SKULL_BASH_AIR_END,
    352 => SKULL_BASH_AIR_TAKEOFF,
    353 => AGILITY_GROUND_STARTUP,
    354 => AGILITY_GROUND,
    355 => AGILITY_GROUND_END,
    356 => AGILITY_AIR_STARTUP,
    357 => AGILITY_AIR,
    358 => AGILITY_AIR_END,
    359 => THUNDER_GROUND_STARTUP,
    360 => THUNDER_GROUND,
    361 => THUNDER_GROUND_HIT,
    362 => THUNDER_GROUND_END,
    363 => THUNDER_AIR_STARTUP,
    364 => THUNDER_AIR,
    365 => THUNDER_AIR_HIT,
    366 => THUNDER_AIR_END,
});

pseudo_enum!(Pikachu: u16 {
    341 => THUNDER_JOLT_GROUND,
    342 => THUNDER_JOLT_AIR,
    343 => SKULL_BASH_GROUND_STARTUP,
    344 => SKULL_BASH_GROUND_CHARGE,
    346 => SKULL_BASH_GROUND_LANDING,
    347 => SKULL_BASH_GROUND_TAKEOFF,
    348 => SKULL_BASH_AIR_STARTUP,
    349 => SKULL_BASH_AIR_CHARGE,
    350 => SKULL_BASH_AIR,
    351 => SKULL_BASH_AIR_END,
    352 => SKULL_BASH_AIR_TAKEOFF,
    353 => QUICK_ATTACK_GROUND_STARTUP,
    354 => QUICK_ATTACK_GROUND,
    355 => QUICK_ATTACK_GROUND_END,
    356 => QUICK_ATTACK_AIR_STARTUP,
    357 => QUICK_ATTACK_AIR,
    358 => QUICK_ATTACK_AIR_END,
    359 => THUNDER_GROUND_STARTUP,
    360 => THUNDER_GROUND,
    361 => THUNDER_GROUND_HIT,
    362 => THUNDER_GROUND_END,
    363 => THUNDER_AIR_STARTUP,
    364 => THUNDER_AIR,
    365 => THUNDER_AIR_HIT,
    366 => THUNDER_AIR_END,
});

pseudo_enum!(Popo: u16 {
    341 => ICE_SHOT_GROUND,
    342 => ICE_SHOT_AIR,
    343 => SQUALL_HAMMER_GROUND_SOLO,
    344 => SQUALL_HAMMER_GROUND_TOGETHER,
    345 => SQUALL_HAMMER_AIR_SOLO,
    346 => SQUALL_HAMMER_AIR_TOGETHER,
    347 => BELAY_GROUND_STARTUP,
    348 => BELAY_GROUND_CATAPULTING_NANA,
    350 => BELAY_GROUND_FAILED_CATAPULTING,
    351 => BELAY_GROUND_FAILED_CATAPULTING_END,
    352 => BELAY_AIR_STARTUP,
    353 => BELAY_AIR_CATAPULTING_NANA,
    354 => BELAY_CATAPULTING,
    355 => BELAY_AIR_FAILED_CATAPULTING,
    356 => BELAY_AIR_FAILED_CATAPULTING_END,
    357 => BLIZZARD_GROUND,
    358 => BLIZZARD_AIR,
});

pseudo_enum!(Roy: u16 {
    341 => FLARE_BLADE_GROUND_START_CHARGE,
    342 => FLARE_BLADE_GROUND_CHARGE_LOOP,
    343 => FLARE_BLADE_GROUND_EARLY_RELEASE,
    344 => FLARE_BLADE_GROUND_FULLY_CHARGED,
    345 => FLARE_BLADE_AIR_START_CHARGE,
    346 => FLARE_BLADE_AIR_CHARGE_LOOP,
    347 => FLARE_BLADE_AIR_EARLY_RELEASE,
    348 => FLARE_BLADE_AIR_FULLY_CHARGED,
    349 => DOUBLE_EDGE_DANCE_1_GROUND,
    350 => DOUBLE_EDGE_DANCE_2_UP_GROUND,
    351 => DOUBLE_EDGE_DANCE_2_SIDE_GROUND,
    352 => DOUBLE_EDGE_DANCE_3_UP_GROUND,
    353 => DOUBLE_EDGE_DANCE_3_SIDE_GROUND,
    354 => DOUBLE_EDGE_DANCE_3_DOWN_GROUND,
    355 => DOUBLE_EDGE_DANCE_4_UP_GROUND,
    356 => DOUBLE_EDGE_DANCE_4_SIDE_GROUND,
    357 => DOUBLE_EDGE_DANCE_4_DOWN_GROUND,
    358 => DOUBLE_EDGE_DANCE_1_AIR,
    359 => DOUBLE_EDGE_DANCE_2_UP_AIR,
    360 => DOUBLE_EDGE_DANCE_2_SIDE_AIR,
    361 => DOUBLE_EDGE_DANCE_3_UP_AIR,
    362 => DOUBLE_EDGE_DANCE_3_SIDE_AIR,
    363 => DOUBLE_EDGE_DANCE_3_DOWN_AIR,
    364 => DOUBLE_EDGE_DANCE_4_UP_AIR,
    365 => DOUBLE_EDGE_DANCE_4_SIDE_AIR,
    366 => DOUBLE_EDGE_DANCE_4_DOWN_AIR,
    367 => BLAZER_GROUND,
    368 => BLAZER_AIR,
    369 => COUNTER_GROUND,
    370 => COUNTER_GROUND_HIT,
    371 => COUNTER_AIR,
    372 => COUNTER_AIR_HIT,
});

pseudo_enum!(Samus: u16 {
    341 => BOMB_JUMP_GROUND,
    342 => BOMB_JUMP_AIR,
    343 => CHARGE_SHOT_GROUND_START,
    344 => CHARGE_SHOT_GROUND_LOOP,
    345 => CHARGE_SHOT_GROUND_END,
    346 => CHARGE_SHOT_GROUND_FIRE,
    347 => CHARGE_SHOT_AIR_START,
    348 => CHARGE_SHOT_AIR_FIRE,
    349 => MISSILE_GROUND,
    350 => MISSILE_SMASH_GROUND,
    351 => MISSILE_AIR,
    352 => MISSILE_SMASH_AIR,
    353 => SCREW_ATTACK_GROUND,
    354 => SCREW_ATTACK_AIR,
    355 => BOMB_END_GROUND,
    356 => BOMB_AIR,
    357 => ZAIR,
    358 => ZAIR_CATCH,
});

pseudo_enum!(Sheik: u16 {
    341 => NEEDLE_STORM_GROUND_START_CHARGE,
    342 => NEEDLE_STORM_GROUND_CHARGE_LOOP,
    343 => NEEDLE_STORM_GROUND_END_CHARGE,
    344 => NEEDLE_STORM_GROUND_FIRE,
    345 => NEEDLE_STORM_AIR_START_CHARGE,
    346 => NEEDLE_STORM_AIR_CHARGE_LOOP,
    347 => NEEDLE_STORM_AIR_END_CHARGE,
    348 => NEEDLE_STORM_AIR_FIRE,
    349 => CHAIN_GROUND_STARTUP,
    350 => CHAIN_GROUND_LOOP,
    351 => CHAIN_GROUND_END,
    352 => CHAIN_AIR_STARTUP,
    353 => CHAIN_AIR_LOOP,
    354 => CHAIN_AIR_END,
    355 => VANISH_GROUND_STARTUP,
    356 => VANISH_GROUND_DISAPPEAR,
    357 => VANISH_GROUND_REAPPEAR,
    358 => VANISH_AIR_STARTUP,
    359 => VANISH_AIR_DISAPPEAR,
    360 => VANISH_AIR_REAPPEAR,
    361 => TRANSFORM_GROUND,
    362 => TRANSFORM_GROUND_ENDING,
    363 => TRANSFORM_AIR,
    364 => TRANSFORM_AIR_ENDING,
});

pseudo_enum!(Yoshi: u16 {
    342 => SHIELD_HOLD,
    343 => SHIELD_RELEASE,
    344 => SHIELD_DAMAGE,
    345 => SHIELD_STARTUP,
    346 => EGG_LAY_GROUND,
    347 => EGG_LAY_GROUND_CAPTURE_START,
    349 => EGG_LAY_GROUND_CAPTURE,
    351 => EGG_LAY_AIR,
    352 => EGG_LAY_AIR_CAPTURE_START,
    354 => EGG_LAY_AIR_CAPTURE,
    356 => EGG_ROLL_GROUND_STARTUP,
    357 => EGG_ROLL_GROUND,
    358 => EGG_ROLL_GROUND_CHANGE_DIRECTION,
    359 => EGG_ROLL_GROUND_END,
    360 => EGG_ROLL_AIR_START,
    361 => EGG_ROLL_AIR,
    362 => EGG_ROLL_BOUNCE,
    363 => EGG_ROLL_AIR_END,
    364 => EGG_THROW_GROUND,
    365 => EGG_THROW_AIR,
    366 => BOMB_GROUND,
    367 => BOMB_LAND,
    368 => BOMB_AIR,
});

pseudo_enum!(YoungLink: u16 {
    341 => SIDE_SMASH_2,
    342 => TAUNT_R,
    343 => TAUNT_L,
    344 => FIRE_BOW_GROUND_CHARGE,
    345 => FIRE_BOW_GROUND_FULLY_CHARGED,
    346 => FIRE_BOW_GROUND_FIRE,
    347 => FIRE_BOW_AIR_CHARGE,
    348 => FIRE_BOW_AIR_FULLY_CHARGED,
    349 => FIRE_BOW_AIR_FIRE,
    350 => BOOMERANG_GROUND_THROW,
    351 => BOOMERANG_GROUND_CATCH,
    352 => BOOMERANG_GROUND_THROW_EMPTY,
    353 => BOOMERANG_AIR_THROW,
    354 => BOOMERANG_AIR_CATCH,
    355 => BOOMERANG_AIR_THROW_EMPTY,
    356 => SPIN_ATTACK_GROUND,
    357 => SPIN_ATTACK_AIR,
    358 => BOMB_GROUND,
    359 => BOMB_AIR,
    360 => ZAIR,
    361 => ZAIR_CATCH,
});

pseudo_enum!(Zelda: u16 {
    341 => NAYRUS_LOVE_GROUND,
    342 => NAYRUS_LOVE_AIR,
    343 => DINS_FIRE_GROUND_STARTUP,
    344 => DINS_FIRE_GROUND_TRAVEL,
    345 => DINS_FIRE_GROUND_EXPLODE,
    346 => DINS_FIRE_AIR_STARTUP,
    347 => DINS_FIRE_AIR_TRAVEL,
    348 => DINS_FIRE_AIR_EXPLODE,
    349 => FARORES_WIND_GROUND,
    350 => FARORES_WIND_GROUND_DISAPPEAR,
    351 => FARORES_WIND_GROUND_REAPPEAR,
    352 => FARORES_WIND_AIR,
    353 => FARORES_WIND_AIR_DISAPPEAR,
    354 => FARORES_WIND_AIR_REAPPEAR,
    355 => TRANSFORM_GROUND,
    356 => TRANSFORM_GROUND_ENDING,
    357 => TRANSFORM_AIR,
    358 => TRANSFORM_AIR_ENDING,
});
