/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use crate::acc_calc::*;
use crate::constants::*;

#[derive(Clone, Debug, Default)]
pub struct PpResult {
    pub pp: f32,
    pub aim_pp: f32,
    pub speed_pp: f32,
    pub acc_pp: f32,
    pub accuracy_percent: f32,
}

#[inline]
pub fn base_pp(stars: f32) -> f32 {
    (5.0 * 1.0f32.max(stars / 0.0675) - 4.0).powi(3) / 100000.0
}

pub struct StdPpParams {
    pub aim_stars: f32,
    pub speed_stars: f32,
    pub ar: f32,
    pub od: f32,
    pub mods: u32,
    pub combo: i32,
    pub max_combo: i32,
    pub n300: i32,
    pub n100: i32,
    pub n50: i32,
    pub nmiss: i32,
    pub ncircles: i32,
    pub nsliders: i32,
    pub nspinners: i32,
    pub nobjects: i32,
    pub score_version: i32,
}

pub fn pp_std(params: &StdPpParams) -> Result<PpResult, i32> {
    if params.nobjects <= 0 {
        return Ok(PpResult::default());
    }

    let mut ncircles = params.ncircles;
    let max_combo = if params.max_combo <= 0 { 1 } else { params.max_combo };

    let accuracy = acc_calc(params.n300, params.n100, params.n50, params.nmiss);

    let real_acc = match params.score_version {
        1 => acc_calc(
            (params.n300 - params.nsliders - params.nspinners).max(0),
            params.n100,
            params.n50,
            params.nmiss,
        ),
        2 => {
            ncircles = params.nobjects;
            accuracy
        }
        _ => {
            eprintln!("unsupported scorev{}", params.score_version);
            return Err(ERR_NOTIMPLEMENTED);
        }
    };

    let nobjects_over_2k = params.nobjects as f32 / 2000.0;
    let length_bonus = 0.95
        + 0.4 * 1.0f32.min(nobjects_over_2k)
        + if params.nobjects > 2000 {
            nobjects_over_2k.log10() * 0.5
        } else {
            0.0
        };

    let miss_ratio = params.nmiss as f64 / params.nobjects as f64;
    let miss_penalty_aim = 0.97 * (1.0 - miss_ratio.powf(0.775)).powf(params.nmiss as f64) as f32;
    let miss_penalty_speed = 0.97
        * (1.0 - miss_ratio.powf(0.775)).powf((params.nmiss as f64).powf(0.875)) as f32;

    let combo_break = (params.combo as f32).powf(0.8) / (max_combo as f32).powf(0.8);

    let mut ar_bonus = 0.0f32;
    if params.ar > 10.33 {
        ar_bonus += 0.4 * (params.ar - 10.33);
    } else if params.ar < 8.0 {
        ar_bonus += 0.01 * (8.0 - params.ar);
    }

    // Aim PP
    let mut aim_pp = base_pp(params.aim_stars);
    aim_pp *= length_bonus;
    if params.nmiss > 0 {
        aim_pp *= miss_penalty_aim;
    }
    aim_pp *= combo_break;
    aim_pp *= 1.0 + ar_bonus.min(ar_bonus * (params.nobjects as f32 / 1000.0));

    let mut hd_bonus = 1.0f32;
    if (params.mods & MODS_HD) != 0 {
        hd_bonus += 0.04 * (12.0 - params.ar);
    }
    aim_pp *= hd_bonus;

    if (params.mods & MODS_FL) != 0 {
        let mut fl_bonus = 1.0 + 0.35 * 1.0f32.min(params.nobjects as f32 / 200.0);
        if params.nobjects > 200 {
            fl_bonus += 0.3 * 1.0f32.min((params.nobjects - 200) as f32 / 300.0);
        }
        if params.nobjects > 500 {
            fl_bonus += (params.nobjects - 500) as f32 / 1200.0;
        }
        aim_pp *= fl_bonus;
    }

    let acc_bonus = 0.5 + accuracy / 2.0;
    let od_squared = params.od.powi(2);
    let od_bonus = 0.98 + od_squared / 2500.0;
    aim_pp *= acc_bonus;
    aim_pp *= od_bonus;

    // Speed PP
    let mut speed_pp = base_pp(params.speed_stars);
    speed_pp *= length_bonus;
    if params.nmiss > 0 {
        speed_pp *= miss_penalty_speed;
    }
    speed_pp *= combo_break;
    if params.ar > 10.33 {
        speed_pp *= 1.0 + ar_bonus.min(ar_bonus * (params.nobjects as f32 / 1000.0));
    }
    speed_pp *= hd_bonus;
    speed_pp *= (0.95 + od_squared / 750.0)
        * accuracy.powf((14.5 - params.od.max(8.0)) / 2.0);

    let n50_threshold = params.nobjects as f32 / 500.0;
    let penalty_exp = if (params.n50 as f32) < n50_threshold {
        0.0
    } else {
        params.n50 as f32 - n50_threshold
    };
    speed_pp *= 0.98f32.powf(penalty_exp);

    // Acc PP
    let mut acc_pp = 1.52163f32.powf(params.od) * real_acc.powf(24.0) * 2.83;
    acc_pp *= 1.15f32.min((ncircles as f32 / 1000.0).powf(0.3));
    if (params.mods & MODS_HD) != 0 {
        acc_pp *= 1.08;
    }
    if (params.mods & MODS_FL) != 0 {
        acc_pp *= 1.02;
    }

    // Total PP
    let mut final_multiplier = 1.12f32;
    if (params.mods & MODS_NF) != 0 {
        final_multiplier *= 0.9f32.max(1.0 - 0.2 * params.nmiss as f32);
    }
    if (params.mods & MODS_SO) != 0 {
        let spin_ratio = params.nspinners as f64 / params.nobjects as f64;
        final_multiplier *= (1.0 - spin_ratio.powf(0.85)) as f32;
    }

    let pp = (aim_pp.powf(1.1) + speed_pp.powf(1.1) + acc_pp.powf(1.1)).powf(1.0 / 1.1)
        * final_multiplier;

    Ok(PpResult {
        pp,
        aim_pp,
        speed_pp,
        acc_pp,
        accuracy_percent: accuracy * 100.0,
    })
}

pub struct TaikoPpParams {
    pub stars: f32,
    pub odms: f32,
    pub mods: u32,
    pub combo: i32,
    pub max_combo: i32,
    pub n300: i32,
    pub n100: i32,
    pub nmiss: i32,
}

pub fn pp_taiko(params: &TaikoPpParams) -> Result<PpResult, i32> {
    let accuracy = taiko_acc_calc(params.n300, params.n100, params.nmiss);

    let mut acc_pp = (150.0 / params.odms).powf(1.1);
    acc_pp *= accuracy.powf(15.0) * 22.0;
    acc_pp *= 1.15f32.min((params.max_combo as f32 / 1500.0).powf(0.3));

    let mut speed_pp = (5.0 * 1.0f32.max(params.stars / 0.0075) - 4.0).powi(2) / 100000.0;
    let length_bonus = 1.0 + 0.1 * 1.0f32.min(params.max_combo as f32 / 1500.0);
    speed_pp *= length_bonus;
    speed_pp *= 0.985f32.powi(params.nmiss);

    if params.max_combo > 0 {
        let cur_combo = (params.combo - params.nmiss).max(0) as f32;
        speed_pp *= 1.0f32.min(cur_combo.powf(0.5) / (params.max_combo as f32).powf(0.5));
    }

    if (params.mods & MODS_HD) != 0 {
        speed_pp *= 1.025;
    }
    if (params.mods & MODS_FL) != 0 {
        speed_pp *= 1.05 * length_bonus;
    }
    speed_pp *= accuracy;

    let mut final_multiplier = 1.1f32;
    if (params.mods & MODS_NF) != 0 {
        final_multiplier *= 0.90;
    }
    if (params.mods & MODS_HD) != 0 {
        final_multiplier *= 1.10;
    }

    let pp = (speed_pp.powf(1.1) + acc_pp.powf(1.1)).powf(1.0 / 1.1) * final_multiplier;

    Ok(PpResult {
        pp,
        aim_pp: 0.0,
        speed_pp,
        acc_pp,
        accuracy_percent: accuracy * 100.0,
    })
}
