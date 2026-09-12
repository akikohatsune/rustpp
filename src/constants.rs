/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

pub const OPPAI_VERSION_MAJOR: i32 = 4;
pub const OPPAI_VERSION_MINOR: i32 = 1;
pub const OPPAI_VERSION_PATCH: i32 = 0;
pub const OPPAI_VERSION_STRING: &str = "4.1.0";

// Modes
pub const MODE_STD: i32 = 0;
pub const MODE_TAIKO: i32 = 1;

// Difficulty types
pub const DIFF_SPEED: usize = 0;
pub const DIFF_AIM: usize = 1;

// Mods bitflags
pub const MODS_NOMOD: u32 = 0;
pub const MODS_NF: u32 = 1 << 0;
pub const MODS_EZ: u32 = 1 << 1;
pub const MODS_TD: u32 = 1 << 2;
pub const MODS_HD: u32 = 1 << 3;
pub const MODS_HR: u32 = 1 << 4;
pub const MODS_SD: u32 = 1 << 5;
pub const MODS_DT: u32 = 1 << 6;
pub const MODS_RX: u32 = 1 << 7;
pub const MODS_HT: u32 = 1 << 8;
pub const MODS_NC: u32 = 1 << 9;
pub const MODS_FL: u32 = 1 << 10;
pub const MODS_AT: u32 = 1 << 11;
pub const MODS_SO: u32 = 1 << 12;
pub const MODS_AP: u32 = 1 << 13;
pub const MODS_PF: u32 = 1 << 14;
pub const MODS_KEY4: u32 = 1 << 15;
pub const MODS_KEY5: u32 = 1 << 16;
pub const MODS_KEY6: u32 = 1 << 17;
pub const MODS_KEY7: u32 = 1 << 18;
pub const MODS_KEY8: u32 = 1 << 19;
pub const MODS_FADEIN: u32 = 1 << 20;
pub const MODS_RANDOM: u32 = 1 << 21;
pub const MODS_CINEMA: u32 = 1 << 22;
pub const MODS_TARGET: u32 = 1 << 23;
pub const MODS_KEY9: u32 = 1 << 24;
pub const MODS_KEYCOOP: u32 = 1 << 25;
pub const MODS_KEY1: u32 = 1 << 26;
pub const MODS_KEY3: u32 = 1 << 27;
pub const MODS_KEY2: u32 = 1 << 28;
pub const MODS_SCOREV2: u32 = 1 << 29;
pub const MODS_TOUCH_DEVICE: u32 = MODS_TD;
pub const MODS_NOVIDEO: u32 = MODS_TD;
pub const MODS_SPEED_CHANGING: u32 = MODS_DT | MODS_HT | MODS_NC;
pub const MODS_MAP_CHANGING: u32 = MODS_HR | MODS_EZ | MODS_SPEED_CHANGING;

// Hitobject types
pub const OBJ_CIRCLE: i32 = 1 << 0;
pub const OBJ_SLIDER: i32 = 1 << 1;
pub const OBJ_SPINNER: i32 = 1 << 3;

// Sound types
pub const SOUND_NONE: i32 = 0;
pub const SOUND_NORMAL: i32 = 1 << 0;
pub const SOUND_WHISTLE: i32 = 1 << 1;
pub const SOUND_FINISH: i32 = 1 << 2;
pub const SOUND_CLAP: i32 = 1 << 3;

// Errors
pub const ERR_MORE: i32 = -1;
pub const ERR_SYNTAX: i32 = -2;
pub const ERR_TRUNCATED: i32 = -3;
pub const ERR_NOTIMPLEMENTED: i32 = -4;
pub const ERR_IO: i32 = -5;
pub const ERR_FORMAT: i32 = -6;
pub const ERR_OOM: i32 = -7;

pub fn errstr(err: i32) -> &'static str {
    match err {
        ERR_MORE => "call me again with more data",
        ERR_SYNTAX => "syntax error",
        ERR_TRUNCATED => "data was truncated, possibly because it was too big",
        ERR_NOTIMPLEMENTED => "requested a feature that isn't implemented",
        ERR_IO => "i/o error",
        ERR_FORMAT => "invalid input format",
        ERR_OOM => "out of memory",
        _ => "unknown error",
    }
}
