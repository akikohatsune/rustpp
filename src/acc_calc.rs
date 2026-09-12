/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

#[inline]
pub fn al_round(x: f32) -> f32 {
    (x + 0.5).floor()
}

pub fn acc_calc(n300: i32, n100: i32, n50: i32, misses: i32) -> f32 {
    let total_hits = n300 + n100 + n50 + misses;
    if total_hits > 0 {
        (n50 as f32 * 50.0 + n100 as f32 * 100.0 + n300 as f32 * 300.0) / (total_hits as f32 * 300.0)
    } else {
        0.0
    }
}

pub fn acc_round(
    mut acc_percent: f32,
    nobjects: i32,
    mut misses: i32,
) -> (i32, i32, i32) {
    misses = misses.min(nobjects);
    let max300 = nobjects - misses;
    let maxacc = acc_calc(max300, 0, 0, misses) * 100.0;
    acc_percent = acc_percent.clamp(0.0, maxacc);
    let mut n50 = 0;

    let mut n100 = al_round(
        -3.0 * ((acc_percent * 0.01 - 1.0) * nobjects as f32 + misses as f32) * 0.5
    ) as i32;

    if n100 > nobjects - misses {
        n100 = 0;
        n50 = al_round(
            -6.0 * ((acc_percent * 0.01 - 1.0) * nobjects as f32 + misses as f32) * 0.2
        ) as i32;
        n50 = n50.min(max300);
    } else {
        n100 = n100.min(max300);
    }

    let n300 = nobjects - n100 - n50 - misses;
    (n300, n100, n50)
}

pub fn taiko_acc_calc(n300: i32, n150: i32, nmiss: i32) -> f32 {
    let total_hits = n300 + n150 + nmiss;
    if total_hits > 0 {
        (n150 as f32 * 150.0 + n300 as f32 * 300.0) / (total_hits as f32 * 300.0)
    } else {
        0.0
    }
}

pub fn taiko_acc_round(
    mut acc_percent: f32,
    nobjects: i32,
    mut nmisses: i32,
) -> (i32, i32) {
    nmisses = nmisses.min(nobjects);
    let max300 = nobjects - nmisses;
    let maxacc = acc_calc(max300, 0, 0, nmisses) * 100.0;
    acc_percent = acc_percent.clamp(0.0, maxacc);

    let mut n150 = al_round(
        -2.0 * ((acc_percent * 0.01 - 1.0) * nobjects as f32 + nmisses as f32)
    ) as i32;
    n150 = n150.min(max300);
    let n300 = nobjects - n150 - nmisses;
    (n300, n150)
}
