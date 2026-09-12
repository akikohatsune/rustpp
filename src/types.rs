/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

#[derive(Clone, Debug, Default)]
pub struct TimingPoint {
    pub time: f32,
    pub ms_per_beat: f32,
    pub change: bool,
    pub px_per_beat: f32,
    pub beat_len: f32,
    pub velocity: f32,
}

#[derive(Clone, Debug, Default)]
pub struct HitObject {
    pub time: f32,
    pub obj_type: i32,
    pub pos: [f32; 2],
    pub normpos: [f32; 2],
    pub angle: f32,
    pub strains: [f32; 2],
    pub is_single: bool,
    pub delta_time: f32,
    pub d_distance: f32,
    pub timing_point: usize,
    pub distance: f32,
    pub repetitions: i32,
    pub duration: f32,
    pub tick_spacing: f32,
    pub slider_is_drum_roll: bool,
    pub sound_types: Vec<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct TaikoObject {
    pub hit: bool,
    pub strain: f32,
    pub time: f32,
    pub time_elapsed: f32,
    pub rim: bool,
    pub same_since: i32,
    pub last_switch_even: i32,
}

#[inline]
pub fn v2f_sub(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

#[inline]
pub fn v2f_len(v: [f32; 2]) -> f32 {
    (v[0] * v[0] + v[1] * v[1]).sqrt()
}

#[inline]
pub fn v2f_dot(a: [f32; 2], b: [f32; 2]) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}
