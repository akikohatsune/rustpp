/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use crate::constants::*;

pub const OD10_MS: [f32; 2] = [20.0, 20.0]; // std, taiko
pub const OD0_MS: [f32; 2] = [80.0, 50.0];
pub const AR0_MS: f32 = 1800.0;
pub const AR5_MS: f32 = 1200.0;
pub const AR10_MS: f32 = 450.0;

pub const OD_MS_STEP: [f32; 2] = [6.0, 3.0];
pub const AR_MS_STEP1: f32 = 120.0; // ar0-5
pub const AR_MS_STEP2: f32 = 150.0; // ar5-10

#[derive(Clone, Debug, Default)]
pub struct ModStats {
    pub ar: f32,
    pub od: f32,
    pub cs: f32,
    pub hp: f32,
    pub odms: f32,
    pub speed_mul: f32,
}

pub fn mods_apply(
    mode: i32,
    mods: u32,
    base_ar: f32,
    base_od: f32,
    base_cs: f32,
    base_hp: f32,
) -> Result<ModStats, i32> {
    if mode != MODE_STD && mode != MODE_TAIKO {
        eprintln!("this gamemode is not yet supported for mods calc");
        return Err(ERR_NOTIMPLEMENTED);
    }

    let m_idx = mode as usize;
    let mut stats = ModStats {
        ar: base_ar,
        od: base_od,
        cs: base_cs,
        hp: base_hp,
        odms: 0.0,
        speed_mul: 1.0,
    };

    if (mods & MODS_MAP_CHANGING) == 0 {
        stats.odms = OD0_MS[m_idx] - (OD_MS_STEP[m_idx] * stats.od).ceil();
        return Ok(stats);
    }

    if (mods & (MODS_DT | MODS_NC)) != 0 {
        stats.speed_mul *= 1.5;
    }
    if (mods & MODS_HT) != 0 {
        stats.speed_mul *= 0.75;
    }

    let mut od_ar_hp_multiplier = 1.0f32;
    if (mods & MODS_HR) != 0 {
        od_ar_hp_multiplier *= 1.4;
    }
    if (mods & MODS_EZ) != 0 {
        od_ar_hp_multiplier *= 0.5;
    }

    stats.od *= od_ar_hp_multiplier;
    stats.odms = OD0_MS[m_idx] - (OD_MS_STEP[m_idx] * stats.od).ceil();
    stats.odms = stats.odms.clamp(OD10_MS[m_idx], OD0_MS[m_idx]);
    stats.odms /= stats.speed_mul;
    stats.od = (OD0_MS[m_idx] - stats.odms) / OD_MS_STEP[m_idx];

    stats.ar *= od_ar_hp_multiplier;
    let mut arms = if stats.ar <= 5.0 {
        AR0_MS - AR_MS_STEP1 * stats.ar
    } else {
        AR5_MS - AR_MS_STEP2 * (stats.ar - 5.0)
    };
    arms = arms.clamp(AR10_MS, AR0_MS);
    arms /= stats.speed_mul;
    stats.ar = if arms > AR5_MS {
        (AR0_MS - arms) / AR_MS_STEP1
    } else {
        5.0 + (AR5_MS - arms) / AR_MS_STEP2
    };

    let mut cs_multiplier = 1.0f32;
    if (mods & MODS_HR) != 0 {
        cs_multiplier = 1.3;
    }
    if (mods & MODS_EZ) != 0 {
        cs_multiplier = 0.5;
    }
    stats.cs *= cs_multiplier;
    stats.cs = stats.cs.clamp(0.0, 10.0);

    stats.hp = (stats.hp * od_ar_hp_multiplier).min(10.0);

    Ok(stats)
}
