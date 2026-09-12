/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use crate::constants::*;
use crate::mods::*;
use crate::types::*;

pub const PLAYFIELD_WIDTH: f32 = 512.0;
pub const PLAYFIELD_HEIGHT: f32 = 384.0;
pub const PLAYFIELD_CENTER: [f32; 2] = [PLAYFIELD_WIDTH / 2.0, PLAYFIELD_HEIGHT / 2.0];
pub const CIRCLESIZE_BUFF_THRESHOLD: f32 = 30.0;

#[derive(Clone, Debug, Default)]
pub struct Beatmap {
    pub format_version: i32,
    pub mode: i32,
    pub original_mode: i32,
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub ncircles: i32,
    pub nsliders: i32,
    pub nspinners: i32,
    pub nobjects: i32,
    pub max_combo: i32,

    pub base_ar: f32,
    pub base_od: f32,
    pub base_cs: f32,
    pub base_hp: f32,
    pub sv: f32,
    pub tick_rate: f32,

    pub ar: f32,
    pub od: f32,
    pub cs: f32,
    pub hp: f32,
    pub odms: f32,
    pub speed_mul: f32,

    pub objects: Vec<HitObject>,
    pub timing_points: Vec<TimingPoint>,
}

fn parse_float(s: &str) -> f32 {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return 0.0;
    }
    if trimmed == "\u{221e}" {
        return f32::INFINITY;
    }
    if trimmed == "-\u{221e}" {
        return f32::NEG_INFINITY;
    }
    trimmed.parse::<f32>().unwrap_or(0.0)
}

fn parse_int(s: &str) -> Option<i32> {
    s.trim().parse::<i32>().ok()
}

pub fn parse_beatmap(
    data: &str,
    mode_override: i32,
    mods: u32,
    base_ar_override: f32,
    base_od_override: f32,
    base_cs_override: f32,
    base_hp_override: f32,
    end: i32,
    end_time: f32,
) -> Result<Beatmap, i32> {
    let mut map = Beatmap {
        base_ar: -1.0,
        base_od: -1.0,
        base_cs: -1.0,
        base_hp: -1.0,
        ar: 5.0,
        od: 5.0,
        cs: 5.0,
        hp: 5.0,
        sv: 1.0,
        tick_rate: 1.0,
        speed_mul: 1.0,
        ..Default::default()
    };

    let mut found_ar = false;
    let mut section = String::new();

    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('_') {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed[1..trimmed.len() - 1].trim().to_string();
            continue;
        }

        if section.is_empty() {
            if let Some(pos) = trimmed.find("file format v") {
                let v_str = &trimmed[pos + 13..];
                if let Some(v) = parse_int(v_str) {
                    map.format_version = v;
                }
            }
            continue;
        }

        match section.as_str() {
            "General" => {
                if let Some((name, val)) = trimmed.split_once(':') {
                    let name = name.trim();
                    let val = val.trim();
                    if name == "Mode" {
                        if let Some(m) = parse_int(val) {
                            map.original_mode = m;
                            if mode_override != 0 {
                                map.mode = mode_override;
                            } else {
                                map.mode = map.original_mode;
                            }
                            if map.mode != MODE_STD && map.mode != MODE_TAIKO {
                                return Err(ERR_NOTIMPLEMENTED);
                            }
                        }
                    }
                }
            }
            "Metadata" => {
                if let Some((name, val)) = trimmed.split_once(':') {
                    let name = name.trim();
                    let val = val.trim();
                    match name {
                        "Title" => map.title = val.to_string(),
                        "TitleUnicode" => map.title_unicode = val.to_string(),
                        "Artist" => map.artist = val.to_string(),
                        "ArtistUnicode" => map.artist_unicode = val.to_string(),
                        "Creator" => map.creator = val.to_string(),
                        "Version" => map.version = val.to_string(),
                        _ => {}
                    }
                }
            }
            "Difficulty" => {
                if let Some((name, val)) = trimmed.split_once(':') {
                    let name = name.trim();
                    let val = val.trim();
                    match name {
                        "CircleSize" => map.cs = parse_float(val),
                        "OverallDifficulty" => map.od = parse_float(val),
                        "ApproachRate" => {
                            map.ar = parse_float(val);
                            found_ar = true;
                        }
                        "HPDrainRate" => map.hp = parse_float(val),
                        "SliderMultiplier" => map.sv = parse_float(val),
                        "SliderTickRate" => map.tick_rate = parse_float(val),
                        _ => {}
                    }
                }
            }
            "TimingPoints" => {
                let parts: Vec<&str> = trimmed.split(',').collect();
                if parts.len() >= 2 {
                    let time = parse_float(parts[0]);
                    let ms_per_beat = parse_float(parts[1]);
                    let change = if parts.len() >= 7 {
                        let c = parts[6].trim();
                        if c.is_empty() {
                            true
                        } else {
                            c != "0"
                        }
                    } else {
                        true
                    };

                    map.timing_points.push(TimingPoint {
                        time,
                        ms_per_beat,
                        change,
                        px_per_beat: 0.0,
                        beat_len: 0.0,
                        velocity: 0.0,
                    });
                }
            }
            "HitObjects" => {
                if end > 0 && map.objects.len() >= end as usize {
                    continue;
                }

                let parts: Vec<&str> = trimmed.split(',').collect();
                if parts.len() < 5 {
                    continue;
                }

                let mut obj_time = parse_float(parts[2]);
                if obj_time.is_infinite() {
                    obj_time = 0.0;
                }

                if end_time > 0.0 && obj_time >= end_time {
                    continue;
                }

                let obj_type = parse_int(parts[3]).unwrap_or(OBJ_CIRCLE);
                let mut obj = HitObject {
                    time: obj_time,
                    obj_type,
                    ..Default::default()
                };

                if map.mode == MODE_TAIKO {
                    let sound_type = parse_int(parts[4]).unwrap_or(SOUND_NORMAL);
                    obj.sound_types = vec![sound_type];
                }

                if (obj.obj_type & OBJ_CIRCLE) != 0 {
                    map.ncircles += 1;
                    obj.pos[0] = parse_float(parts[0]);
                    obj.pos[1] = parse_float(parts[1]);
                } else if (obj.obj_type & OBJ_SPINNER) != 0 {
                    map.nspinners += 1;
                } else if (obj.obj_type & OBJ_SLIDER) != 0 {
                    map.nsliders += 1;
                    if parts.len() >= 7 {
                        obj.pos[0] = parse_float(parts[0]);
                        obj.pos[1] = parse_float(parts[1]);
                        obj.repetitions = parse_int(parts[6]).unwrap_or(1);
                        if parts.len() > 7 {
                            obj.distance = parse_float(parts[7]);
                        }

                        if map.mode == MODE_TAIKO && parts.len() > 8 && !parts[8].trim().is_empty() {
                            let nodes = obj.repetitions + 1;
                            let sound_type = obj.sound_types.first().copied().unwrap_or(SOUND_NORMAL);
                            obj.sound_types = vec![sound_type; nodes as usize];

                            let sound_parts: Vec<&str> = parts[8].split('|').collect();
                            for (idx, sp) in sound_parts.into_iter().enumerate() {
                                if idx >= nodes as usize {
                                    break;
                                }
                                if let Some(st) = parse_int(sp) {
                                    obj.sound_types[idx] = st;
                                }
                            }
                        }
                    }
                }

                map.objects.push(obj);
            }
            _ => {}
        }
    }

    // p_end processing
    if !found_ar {
        map.ar = map.od;
    }
    if map.title_unicode.is_empty() {
        map.title_unicode = map.title.clone();
    }
    if map.artist_unicode.is_empty() {
        map.artist_unicode = map.artist.clone();
    }

    map.base_ar = if base_ar_override < 0.0 { map.ar } else { base_ar_override };
    map.ar = map.base_ar;

    map.base_cs = if base_cs_override < 0.0 { map.cs } else { base_cs_override };
    map.cs = map.base_cs;

    map.base_od = if base_od_override < 0.0 { map.od } else { base_od_override };
    map.od = map.base_od;

    map.base_hp = if base_hp_override < 0.0 { map.hp } else { base_hp_override };
    map.hp = map.base_hp;

    let mod_stats = mods_apply(map.mode, mods, map.base_ar, map.base_od, map.base_cs, map.base_hp)?;
    map.ar = mod_stats.ar;
    map.od = mod_stats.od;
    map.cs = mod_stats.cs;
    map.hp = mod_stats.hp;
    map.odms = mod_stats.odms;
    map.speed_mul = mod_stats.speed_mul;

    let mut legacy_multiplier = 1.0f32;
    if map.mode == MODE_TAIKO && map.mode != map.original_mode {
        legacy_multiplier = 1.4;
        map.sv *= legacy_multiplier;
    }

    let mut ms_per_beat = f32::INFINITY;
    for t in &mut map.timing_points {
        let mut sv_multiplier = 1.0f32;
        if t.change {
            ms_per_beat = t.ms_per_beat;
        }
        if !t.change && t.ms_per_beat < 0.0 {
            sv_multiplier = -100.0 / t.ms_per_beat;
        }
        t.beat_len = ms_per_beat / sv_multiplier;
        t.px_per_beat = map.sv * 100.0;
        t.velocity = 100.0 * map.sv / t.beat_len;
        if map.format_version >= 8 {
            t.beat_len *= sv_multiplier;
            t.px_per_beat *= sv_multiplier;
        }
    }

    map.nobjects = map.objects.len() as i32;
    map.max_combo = map.nobjects;

    if map.mode == MODE_TAIKO {
        map.max_combo -= map.nspinners + map.nsliders;
    }

    let radius = (PLAYFIELD_WIDTH / 16.0) * (1.0 - 0.7 * (map.cs - 5.0) / 5.0);
    let mut scaling_factor = 52.0 / radius;
    if radius < CIRCLESIZE_BUFF_THRESHOLD {
        scaling_factor *= 1.0 + (CIRCLESIZE_BUFF_THRESHOLD - radius).min(5.0) / 50.0;
    }

    let mut tindex = 0usize;
    let mut tnext = if !map.timing_points.is_empty() {
        if map.timing_points.len() > 1 {
            map.timing_points[1].time
        } else {
            f32::INFINITY
        }
    } else {
        f32::INFINITY
    };

    let n_objects = map.objects.len();
    for i in 0..n_objects {
        let pos = if (map.objects[i].obj_type & OBJ_SPINNER) != 0 {
            PLAYFIELD_CENTER
        } else {
            map.objects[i].pos
        };

        map.objects[i].normpos[0] = pos[0] * scaling_factor;
        map.objects[i].normpos[1] = pos[1] * scaling_factor;

        if i >= 2 {
            let p1 = map.objects[i - 1].normpos;
            let p2 = map.objects[i - 2].normpos;
            let cur = map.objects[i].normpos;
            let v1 = v2f_sub(p2, p1);
            let v2 = v2f_sub(cur, p1);
            let dot = v2f_dot(v1, v2);
            let det = v1[0] * v2[1] - v1[1] * v2[0];
            map.objects[i].angle = det.atan2(dot).abs();
        } else {
            map.objects[i].angle = f32::NAN;
        }

        if !map.timing_points.is_empty() {
            while map.objects[i].time >= tnext {
                tindex += 1;
                if tindex + 1 < map.timing_points.len() {
                    tnext = map.timing_points[tindex + 1].time;
                } else {
                    tnext = f32::INFINITY;
                }
            }
            map.objects[i].timing_point = tindex;
            let t = &map.timing_points[tindex];

            if (map.objects[i].obj_type & OBJ_SLIDER) != 0 {
                let duration = (map.objects[i].distance * map.objects[i].repetitions as f32 / t.velocity) * legacy_multiplier;
                let tick_spacing = (t.beat_len / map.tick_rate).min(duration / map.objects[i].repetitions as f32);
                let slider_is_drum_roll = tick_spacing > 0.0 && duration < 2.0 * t.beat_len;

                map.objects[i].duration = duration;
                map.objects[i].tick_spacing = tick_spacing;
                map.objects[i].slider_is_drum_roll = slider_is_drum_roll;

                match map.mode {
                    MODE_TAIKO => {
                        if slider_is_drum_roll && map.mode != map.original_mode {
                            map.max_combo += ((duration + tick_spacing / 8.0) / tick_spacing).ceil() as i32;
                        }
                    }
                    MODE_STD => {
                        let num_beats = (map.objects[i].distance * map.objects[i].repetitions as f32) / t.px_per_beat;
                        let mut ticks = ((num_beats - 0.1) / map.objects[i].repetitions as f32 * map.tick_rate).ceil() as i32;
                        ticks -= 1;
                        ticks *= map.objects[i].repetitions;
                        ticks += map.objects[i].repetitions + 1;
                        map.max_combo += (ticks - 1).max(0);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(map)
}
