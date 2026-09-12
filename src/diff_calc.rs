/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use std::f32::consts::PI;
use crate::constants::*;
use crate::parser::Beatmap;
use crate::types::*;

pub const SINGLE_SPACING: f32 = 125.0;
pub const STAR_SCALING_FACTOR: f32 = 0.0675;
pub const EXTREME_SCALING_FACTOR: f32 = 0.5;
pub const STRAIN_STEP: f32 = 400.0;
pub const DECAY_WEIGHT: f32 = 0.9;
pub const MAX_SPEED_BONUS: f32 = 45.0;
pub const MIN_SPEED_BONUS: f32 = 75.0;
pub const ANGLE_BONUS_SCALE: f32 = 90.0;
pub const AIM_TIMING_THRESHOLD: f32 = 107.0;
pub const SPEED_ANGLE_BONUS_BEGIN: f32 = 5.0 * PI / 6.0;
pub const AIM_ANGLE_BONUS_BEGIN: f32 = PI / 3.0;

pub const DECAY_BASE: [f32; 2] = [0.3, 0.15];
pub const WEIGHT_SCALING: [f32; 2] = [1400.0, 26.25];

pub const TAIKO_STAR_SCALING_FACTOR: f32 = 0.04125;
pub const TAIKO_TYPE_CHANGE_BONUS: f32 = 0.75;
pub const TAIKO_RHYTHM_CHANGE_BONUS: f32 = 1.0;
pub const TAIKO_RHYTHM_CHANGE_BASE_THRESHOLD: f32 = 0.2;
pub const TAIKO_RHYTHM_CHANGE_BASE: f32 = 2.0;

#[derive(Clone, Debug, Default)]
pub struct DiffResult {
    pub stars: f32,
    pub aim_stars: f32,
    pub aim_difficulty: f32,
    pub aim_length_bonus: f32,
    pub speed_stars: f32,
    pub speed_difficulty: f32,
    pub speed_length_bonus: f32,
}

pub fn d_spacing_weight(
    mut distance: f32,
    mut delta_time: f32,
    prev_distance: f32,
    prev_delta_time: f32,
    angle: f32,
    diff_type: usize,
    is_single: &mut bool,
) -> f32 {
    let strain_time = delta_time.max(50.0);
    match diff_type {
        DIFF_SPEED => {
            *is_single = distance > SINGLE_SPACING;
            distance = distance.min(SINGLE_SPACING);
            delta_time = delta_time.max(MAX_SPEED_BONUS);
            let mut speed_bonus = 1.0f32;
            if delta_time < MIN_SPEED_BONUS {
                speed_bonus += ((MIN_SPEED_BONUS - delta_time) / 40.0).powi(2);
            }
            let mut angle_bonus = 1.0f32;
            if !angle.is_nan() && angle < SPEED_ANGLE_BONUS_BEGIN {
                let s = (1.5 * (SPEED_ANGLE_BONUS_BEGIN - angle)).sin();
                angle_bonus += s.powi(2) / 3.57;
                if angle < PI / 2.0 {
                    angle_bonus = 1.28;
                    if distance < ANGLE_BONUS_SCALE && angle < PI / 4.0 {
                        angle_bonus += (1.0 - angle_bonus) * ((ANGLE_BONUS_SCALE - distance) / 10.0).min(1.0);
                    } else if distance < ANGLE_BONUS_SCALE {
                        angle_bonus += (1.0 - angle_bonus)
                            * ((ANGLE_BONUS_SCALE - distance) / 10.0).min(1.0)
                            * ((PI / 2.0 - angle) * 4.0 / PI).sin();
                    }
                }
            }
            (
                (1.0 + (speed_bonus - 1.0) * 0.75)
                    * angle_bonus
                    * (0.95 + speed_bonus * (distance / SINGLE_SPACING).powf(3.5))
            ) / strain_time
        }
        DIFF_AIM => {
            let mut result = 0.0f32;
            let prev_strain_time = prev_delta_time.max(50.0);
            if !angle.is_nan() && angle > AIM_ANGLE_BONUS_BEGIN {
                let angle_bonus = (
                    (prev_distance - ANGLE_BONUS_SCALE).max(0.0)
                        * (angle - AIM_ANGLE_BONUS_BEGIN).sin().powi(2)
                        * (distance - ANGLE_BONUS_SCALE).max(0.0)
                ).sqrt();
                result = 1.5 * angle_bonus.max(0.0).powf(0.99) / AIM_TIMING_THRESHOLD.max(prev_strain_time);
            }
            let weighted_distance = distance.powf(0.99);
            (result + weighted_distance / AIM_TIMING_THRESHOLD.max(strain_time))
                .max(weighted_distance / strain_time)
        }
        _ => 0.0,
    }
}

pub fn d_calc_strain(
    diff_type: usize,
    obj_idx: usize,
    objects: &mut [HitObject],
    speed_mul: f32,
) {
    if obj_idx == 0 {
        return;
    }

    let (prev_slice, cur_slice) = objects.split_at_mut(obj_idx);
    let prev = &mut prev_slice[obj_idx - 1];
    let cur = &mut cur_slice[0];

    let time_elapsed = (cur.time - prev.time) / speed_mul;
    let decay = DECAY_BASE[diff_type].powf(time_elapsed / 1000.0);
    let scaling = WEIGHT_SCALING[diff_type];

    cur.delta_time = time_elapsed;

    let mut res = 0.0f32;
    if (cur.obj_type & (OBJ_SLIDER | OBJ_CIRCLE)) != 0 {
        let diff = v2f_sub(cur.normpos, prev.normpos);
        cur.d_distance = v2f_len(diff);
        res = d_spacing_weight(
            cur.d_distance,
            time_elapsed,
            prev.d_distance,
            prev.delta_time,
            cur.angle,
            diff_type,
            &mut cur.is_single,
        );
        res *= scaling;
    }

    cur.strains[diff_type] = prev.strains[diff_type] * decay + res;
}

pub fn d_update_max_strains(
    highest_strains: &mut Vec<f32>,
    max_strain: &mut f32,
    interval_end: &mut f32,
    decay_factor: f32,
    cur_time: f32,
    prev_time: f32,
    cur_strain: f32,
    prev_strain: f32,
    first_obj: bool,
    speed_mul: f32,
) {
    while cur_time > *interval_end {
        highest_strains.push(*max_strain);
        if first_obj {
            *max_strain = 0.0;
        } else {
            let decay = decay_factor.powf((*interval_end - prev_time) / 1000.0);
            *max_strain = prev_strain * decay;
        }
        *interval_end += STRAIN_STEP * speed_mul;
    }
    *max_strain = (*max_strain).max(cur_strain);
}

pub fn d_weigh_strains(highest_strains: &mut [f32]) -> (f32, f32) {
    highest_strains.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let mut total = 0.0f32;
    let mut difficulty = 0.0f32;
    let mut weight = 1.0f32;

    for &s in highest_strains.iter() {
        total += s.powf(1.2);
        difficulty += s * weight;
        weight *= DECAY_WEIGHT;
    }
    (difficulty, total)
}

pub fn d_calc_individual(
    map: &mut Beatmap,
    diff_type: usize,
) -> (f32, f32) {
    if map.objects.is_empty() {
        return (0.0, 0.0);
    }

    let mut highest_strains = Vec::new();
    let mut max_strain = 0.0f32;
    let mut interval_end = (map.objects[0].time / (STRAIN_STEP * map.speed_mul)).ceil()
        * STRAIN_STEP * map.speed_mul;

    let n = map.objects.len();
    for i in 0..n {
        let mut prev_time = 0.0f32;
        let mut prev_strain = 0.0f32;
        if i > 0 {
            d_calc_strain(diff_type, i, &mut map.objects, map.speed_mul);
            prev_time = map.objects[i - 1].time;
            prev_strain = map.objects[i - 1].strains[diff_type];
        }

        d_update_max_strains(
            &mut highest_strains,
            &mut max_strain,
            &mut interval_end,
            DECAY_BASE[diff_type],
            map.objects[i].time,
            prev_time,
            map.objects[i].strains[diff_type],
            prev_strain,
            i == 0,
            map.speed_mul,
        );
    }

    highest_strains.push(max_strain);
    d_weigh_strains(&mut highest_strains)
}

pub fn d_length_bonus(stars: f32, difficulty: f32) -> f32 {
    0.32 + 0.5 * ((difficulty + stars).log10() - stars.log10())
}

pub fn d_std(map: &mut Beatmap, mods: u32) -> DiffResult {
    let mut res = DiffResult::default();
    if map.objects.is_empty() {
        return res;
    }

    let (speed_stars, speed_diff) = d_calc_individual(map, DIFF_SPEED);
    res.speed_stars = speed_stars;
    res.speed_difficulty = speed_diff;

    let (aim_stars, aim_diff) = d_calc_individual(map, DIFF_AIM);
    res.aim_stars = aim_stars;
    res.aim_difficulty = aim_diff;

    res.aim_length_bonus = d_length_bonus(res.aim_stars, res.aim_difficulty);
    res.speed_length_bonus = d_length_bonus(res.speed_stars, res.speed_difficulty);

    res.aim_stars = res.aim_stars.sqrt() * STAR_SCALING_FACTOR;
    res.speed_stars = res.speed_stars.sqrt() * STAR_SCALING_FACTOR;

    if (mods & MODS_TOUCH_DEVICE) != 0 {
        res.aim_stars = res.aim_stars.powf(0.8);
    }

    res.stars = res.aim_stars + res.speed_stars
        + (res.speed_stars - res.aim_stars).abs() * EXTREME_SCALING_FACTOR;

    res
}

pub fn taiko_change_bonus(cur: &mut TaikoObject, prev: &TaikoObject) -> f32 {
    if prev.rim != cur.rim {
        cur.last_switch_even = if prev.same_since % 2 == 0 { 1 } else { 0 };
        if prev.last_switch_even != cur.last_switch_even && prev.last_switch_even != -1 {
            return TAIKO_TYPE_CHANGE_BONUS;
        }
    } else {
        cur.last_switch_even = prev.last_switch_even;
        cur.same_since = prev.same_since + 1;
    }
    0.0
}

pub fn taiko_rhythm_bonus(cur: &TaikoObject, prev: &TaikoObject) -> f32 {
    if cur.time_elapsed == 0.0 || prev.time_elapsed == 0.0 {
        return 0.0;
    }
    let ratio = (prev.time_elapsed / cur.time_elapsed).max(cur.time_elapsed / prev.time_elapsed);
    if ratio >= 8.0 {
        return 0.0;
    }
    let diff = (ratio.ln() / TAIKO_RHYTHM_CHANGE_BASE.ln()) % 1.0;
    if diff > TAIKO_RHYTHM_CHANGE_BASE_THRESHOLD && diff < 1.0 - TAIKO_RHYTHM_CHANGE_BASE_THRESHOLD {
        return TAIKO_RHYTHM_CHANGE_BONUS;
    }
    0.0
}

pub fn taiko_strain(cur: &mut TaikoObject, prev: &TaikoObject) {
    let decay = DECAY_BASE[0].powf(cur.time_elapsed / 1000.0);
    let mut addition = 1.0f32;
    let mut factor = 1.0f32;

    if prev.hit && cur.hit && (cur.time - prev.time) < 1000.0 {
        addition += taiko_change_bonus(cur, prev);
        addition += taiko_rhythm_bonus(cur, prev);
    }

    if cur.time_elapsed < 50.0 {
        factor = 0.4 + 0.6 * cur.time_elapsed / 50.0;
    }

    cur.strain = prev.strain * decay + addition * factor;
}

pub fn d_taiko(map: &Beatmap) -> DiffResult {
    let mut res = DiffResult::default();
    if map.objects.is_empty() {
        return res;
    }

    let mut highest_strains = Vec::new();
    let mut max_strain = 0.0f32;
    let mut interval_end = STRAIN_STEP * map.speed_mul;

    let mut cur = TaikoObject::default();
    let mut prev = TaikoObject::default();

    for (i, o) in map.objects.iter().enumerate() {
        cur.hit = (o.obj_type & OBJ_CIRCLE) != 0;
        cur.time = o.time;
        cur.time_elapsed = if i > 0 {
            (cur.time - prev.time) / map.speed_mul
        } else {
            0.0
        };

        cur.strain = 1.0;
        cur.same_since = 1;
        cur.last_switch_even = -1;
        let sound0 = o.sound_types.first().copied().unwrap_or(0);
        cur.rim = (sound0 & (SOUND_CLAP | SOUND_WHISTLE)) != 0;

        if map.original_mode != MODE_TAIKO && (o.obj_type & OBJ_SLIDER) != 0 {
            if !o.slider_is_drum_roll || i == 0 {
                // drum roll, ignore if drum roll or i == 0
                if i > 0 {
                    taiko_strain(&mut cur, &prev);
                }
                d_update_max_strains(
                    &mut highest_strains,
                    &mut max_strain,
                    &mut interval_end,
                    DECAY_BASE[0],
                    cur.time,
                    prev.time,
                    cur.strain,
                    prev.strain,
                    i == 0,
                    map.speed_mul,
                );
                std::mem::swap(&mut prev, &mut cur);
                continue;
            }

            let mut isound = 0usize;
            let mut j = o.time;
            let j_end = o.time + o.duration + o.tick_spacing / 8.0;
            while j < j_end {
                let st = if !o.sound_types.is_empty() {
                    o.sound_types[isound % o.sound_types.len()]
                } else {
                    0
                };
                cur.rim = (st & (SOUND_CLAP | SOUND_WHISTLE)) != 0;
                cur.hit = true;
                cur.time = j;
                cur.time_elapsed = (cur.time - prev.time) / map.speed_mul;
                cur.strain = 1.0;
                cur.same_since = 1;
                cur.last_switch_even = -1;

                if i > 0 || j > o.time {
                    taiko_strain(&mut cur, &prev);
                }

                d_update_max_strains(
                    &mut highest_strains,
                    &mut max_strain,
                    &mut interval_end,
                    DECAY_BASE[0],
                    cur.time,
                    prev.time,
                    cur.strain,
                    prev.strain,
                    i == 0 && j == o.time,
                    map.speed_mul,
                );

                isound += 1;
                std::mem::swap(&mut prev, &mut cur);
                j += o.tick_spacing;
            }
            continue;
        }

        if i > 0 {
            taiko_strain(&mut cur, &prev);
        }

        d_update_max_strains(
            &mut highest_strains,
            &mut max_strain,
            &mut interval_end,
            DECAY_BASE[0],
            cur.time,
            prev.time,
            cur.strain,
            prev.strain,
            i == 0,
            map.speed_mul,
        );

        std::mem::swap(&mut prev, &mut cur);
    }

    let (stars, _) = d_weigh_strains(&mut highest_strains);
    res.speed_stars = stars * TAIKO_STAR_SCALING_FACTOR;
    res.stars = res.speed_stars;

    res
}

pub fn d_calc(map: &mut Beatmap, mods: u32) -> Result<DiffResult, i32> {
    match map.mode {
        MODE_STD => Ok(d_std(map, mods)),
        MODE_TAIKO => Ok(d_taiko(map)),
        _ => Err(ERR_NOTIMPLEMENTED),
    }
}
