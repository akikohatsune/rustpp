/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use std::fs;
use std::io::{self, Read};
use crate::acc_calc::*;
use crate::constants::*;
use crate::diff_calc::*;
use crate::mods::*;
use crate::parser::*;
use crate::pp_calc::*;

#[derive(Clone, Debug)]
pub struct Ezpp {
    pub autocalc: bool,
    pub raw_data: Option<String>,
    pub map_path: Option<String>,

    pub mode: i32,
    pub mode_override: i32,
    pub original_mode: i32,
    pub score_version: i32,
    pub mods: u32,
    pub combo: i32,
    pub accuracy_percent: f32,
    pub n300: i32,
    pub n100: i32,
    pub n50: i32,
    pub nmiss: i32,
    pub end: i32,
    pub end_time: f32,

    pub base_ar: f32,
    pub base_cs: f32,
    pub base_od: f32,
    pub base_hp: f32,

    pub max_combo: i32,
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

    pub ar: f32,
    pub od: f32,
    pub cs: f32,
    pub hp: f32,
    pub odms: f32,
    pub sv: f32,
    pub tick_rate: f32,
    pub speed_mul: f32,

    pub stars: f32,
    pub aim_stars: f32,
    pub aim_difficulty: f32,
    pub aim_length_bonus: f32,
    pub speed_stars: f32,
    pub speed_difficulty: f32,
    pub speed_length_bonus: f32,

    pub pp: f32,
    pub aim_pp: f32,
    pub speed_pp: f32,
    pub acc_pp: f32,

    pub parsed_beatmap: Option<Beatmap>,
}

impl Default for Ezpp {
    fn default() -> Self {
        Self::new()
    }
}

impl Ezpp {
    pub fn new() -> Self {
        Self {
            autocalc: false,
            raw_data: None,
            map_path: None,

            mode: MODE_STD,
            mode_override: 0,
            original_mode: MODE_STD,
            score_version: 1,
            mods: MODS_NOMOD,
            combo: -1,
            accuracy_percent: -1.0,
            n300: 0,
            n100: 0,
            n50: 0,
            nmiss: 0,
            end: 0,
            end_time: 0.0,

            base_ar: -1.0,
            base_cs: -1.0,
            base_od: -1.0,
            base_hp: -1.0,

            max_combo: 0,
            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            version: String::new(),
            ncircles: 0,
            nsliders: 0,
            nspinners: 0,
            nobjects: 0,

            ar: 5.0,
            od: 5.0,
            cs: 5.0,
            hp: 5.0,
            odms: 0.0,
            sv: 1.0,
            tick_rate: 1.0,
            speed_mul: 1.0,

            stars: 0.0,
            aim_stars: 0.0,
            aim_difficulty: 0.0,
            aim_length_bonus: 0.0,
            speed_stars: 0.0,
            speed_difficulty: 0.0,
            speed_length_bonus: 0.0,

            pp: 0.0,
            aim_pp: 0.0,
            speed_pp: 0.0,
            acc_pp: 0.0,

            parsed_beatmap: None,
        }
    }

    pub fn set_autocalc(&mut self, autocalc: bool) {
        self.autocalc = autocalc;
    }

    pub fn autocalc(&self) -> bool {
        self.autocalc
    }

    pub fn set_mods(&mut self, mods: u32) {
        if ((mods ^ self.mods) & (MODS_MAP_CHANGING | MODS_SPEED_CHANGING)) != 0 {
            self.aim_stars = 0.0;
            self.speed_stars = 0.0;
            self.stars = 0.0;
            self.max_combo = 0;
        }
        self.mods = mods;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_combo(&mut self, combo: i32) {
        self.combo = combo;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_nmiss(&mut self, nmiss: i32) {
        self.accuracy_percent = -1.0;
        self.aim_stars = 0.0;
        self.speed_stars = 0.0;
        self.stars = 0.0;
        self.max_combo = 0;
        self.nmiss = nmiss;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_score_version(&mut self, score_version: i32) {
        self.score_version = score_version;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_accuracy_percent(&mut self, accuracy_percent: f32) {
        self.accuracy_percent = accuracy_percent;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_accuracy(&mut self, n100: i32, n50: i32) {
        self.accuracy_percent = -1.0;
        self.n100 = n100;
        self.n50 = n50;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_base_ar(&mut self, ar: f32) {
        self.base_ar = ar;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_base_od(&mut self, od: f32) {
        self.base_od = od;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_base_cs(&mut self, cs: f32) {
        self.aim_stars = 0.0;
        self.speed_stars = 0.0;
        self.stars = 0.0;
        self.max_combo = 0;
        self.base_cs = cs;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_base_hp(&mut self, hp: f32) {
        self.base_hp = hp;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_mode_override(&mut self, mode_override: i32) {
        self.aim_stars = 0.0;
        self.speed_stars = 0.0;
        self.stars = 0.0;
        self.max_combo = 0;
        self.mode_override = mode_override;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_mode(&mut self, mode: i32) {
        self.mode = mode;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_aim_stars(&mut self, aim_stars: f32) {
        self.aim_stars = aim_stars;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_speed_stars(&mut self, speed_stars: f32) {
        self.speed_stars = speed_stars;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_end(&mut self, end: i32) {
        self.accuracy_percent = -1.0;
        self.aim_stars = 0.0;
        self.speed_stars = 0.0;
        self.stars = 0.0;
        self.max_combo = 0;
        self.end = end;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn set_end_time(&mut self, end_time: f32) {
        self.accuracy_percent = -1.0;
        self.aim_stars = 0.0;
        self.speed_stars = 0.0;
        self.stars = 0.0;
        self.max_combo = 0;
        self.end_time = end_time;
        if self.autocalc {
            let _ = self.calc();
        }
    }

    pub fn parse_file(&mut self, path: &str) -> Result<i32, i32> {
        let content = if path == "-" {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).map_err(|_| ERR_IO)?;
            buf
        } else {
            fs::read_to_string(path).map_err(|_| ERR_IO)?
        };

        self.map_path = Some(path.to_string());
        self.raw_data = Some(content);
        self.calc()
    }

    pub fn parse_data(&mut self, data: &str) -> Result<i32, i32> {
        self.map_path = None;
        self.raw_data = Some(data.to_string());
        self.calc()
    }

    fn params_from_map(&mut self) -> Result<i32, i32> {
        let data = match &self.raw_data {
            Some(d) => d.clone(),
            None => return Err(ERR_IO),
        };

        let mut map = parse_beatmap(
            &data,
            self.mode_override,
            self.mods,
            self.base_ar,
            self.base_od,
            self.base_cs,
            self.base_hp,
            self.end,
            self.end_time,
        )?;

        self.mode = map.mode;
        self.original_mode = map.original_mode;
        self.title = map.title.clone();
        self.title_unicode = map.title_unicode.clone();
        self.artist = map.artist.clone();
        self.artist_unicode = map.artist_unicode.clone();
        self.creator = map.creator.clone();
        self.version = map.version.clone();
        self.ncircles = map.ncircles;
        self.nsliders = map.nsliders;
        self.nspinners = map.nspinners;
        self.nobjects = map.nobjects;
        self.max_combo = map.max_combo;

        self.ar = map.ar;
        self.od = map.od;
        self.cs = map.cs;
        self.hp = map.hp;
        self.odms = map.odms;
        self.sv = map.sv;
        self.tick_rate = map.tick_rate;
        self.speed_mul = map.speed_mul;

        if self.aim_stars == 0.0 && self.speed_stars == 0.0 {
            let diff = d_calc(&mut map, self.mods)?;
            self.stars = diff.stars;
            self.aim_stars = diff.aim_stars;
            self.aim_difficulty = diff.aim_difficulty;
            self.aim_length_bonus = diff.aim_length_bonus;
            self.speed_stars = diff.speed_stars;
            self.speed_difficulty = diff.speed_difficulty;
            self.speed_length_bonus = diff.speed_length_bonus;
        }

        self.parsed_beatmap = Some(map);
        Ok(0)
    }

    pub fn calc(&mut self) -> Result<i32, i32> {
        if self.max_combo == 0 && self.raw_data.is_some() {
            if self.autocalc {
                self.base_ar = -1.0;
                self.base_od = -1.0;
                self.base_cs = -1.0;
                self.base_hp = -1.0;
            }
            self.params_from_map()?;
        } else {
            if self.parsed_beatmap.is_some() {
                if self.base_ar >= 0.0 { self.ar = self.base_ar; }
                if self.base_od >= 0.0 { self.od = self.base_od; }
                if self.base_cs >= 0.0 { self.cs = self.base_cs; }
                if self.base_hp >= 0.0 { self.hp = self.base_hp; }

                let mod_stats = mods_apply(self.mode, self.mods, self.ar, self.od, self.cs, self.hp)?;
                self.ar = mod_stats.ar;
                self.od = mod_stats.od;
                self.cs = mod_stats.cs;
                self.hp = mod_stats.hp;
                self.odms = mod_stats.odms;
                self.speed_mul = mod_stats.speed_mul;
            }
        }

        if self.mode == MODE_TAIKO {
            self.stars = self.speed_stars;
        }

        if self.accuracy_percent >= 0.0 {
            match self.mode {
                MODE_STD => {
                    let (n300, n100, n50) = acc_round(self.accuracy_percent, self.nobjects, self.nmiss);
                    self.n300 = n300;
                    self.n100 = n100;
                    self.n50 = n50;
                }
                MODE_TAIKO => {
                    let (n300, n100) = taiko_acc_round(self.accuracy_percent, self.max_combo, self.nmiss);
                    self.n300 = n300;
                    self.n100 = n100;
                    self.n50 = 0;
                }
                _ => {}
            }
        }

        if self.combo < 0 {
            self.combo = self.max_combo - self.nmiss;
        }

        self.n300 = self.nobjects - self.n100 - self.n50 - self.nmiss;

        match self.mode {
            MODE_STD => {
                let params = StdPpParams {
                    aim_stars: self.aim_stars,
                    speed_stars: self.speed_stars,
                    ar: self.ar,
                    od: self.od,
                    mods: self.mods,
                    combo: self.combo,
                    max_combo: self.max_combo,
                    n300: self.n300,
                    n100: self.n100,
                    n50: self.n50,
                    nmiss: self.nmiss,
                    ncircles: self.ncircles,
                    nsliders: self.nsliders,
                    nspinners: self.nspinners,
                    nobjects: self.nobjects,
                    score_version: self.score_version,
                };
                let res = pp_std(&params)?;
                self.pp = res.pp;
                self.aim_pp = res.aim_pp;
                self.speed_pp = res.speed_pp;
                self.acc_pp = res.acc_pp;
                self.accuracy_percent = res.accuracy_percent;
            }
            MODE_TAIKO => {
                let params = TaikoPpParams {
                    stars: self.stars,
                    odms: self.odms,
                    mods: self.mods,
                    combo: self.combo,
                    max_combo: self.max_combo,
                    n300: self.n300,
                    n100: self.n100,
                    nmiss: self.nmiss,
                };
                let res = pp_taiko(&params)?;
                self.pp = res.pp;
                self.aim_pp = res.aim_pp;
                self.speed_pp = res.speed_pp;
                self.acc_pp = res.acc_pp;
                self.accuracy_percent = res.accuracy_percent;
            }
            _ => {
                eprintln!("pp calc for this mode is not yet supported");
                return Err(ERR_NOTIMPLEMENTED);
            }
        }

        Ok(0)
    }

    // Getters matching C API
    pub fn pp(&self) -> f32 { self.pp }
    pub fn stars(&self) -> f32 { self.stars }
    pub fn mode(&self) -> i32 { self.mode }
    pub fn combo(&self) -> i32 { self.combo }
    pub fn max_combo(&self) -> i32 { self.max_combo }
    pub fn mods(&self) -> u32 { self.mods }
    pub fn score_version(&self) -> i32 { self.score_version }
    pub fn aim_stars(&self) -> f32 { self.aim_stars }
    pub fn speed_stars(&self) -> f32 { self.speed_stars }
    pub fn aim_pp(&self) -> f32 { self.aim_pp }
    pub fn speed_pp(&self) -> f32 { self.speed_pp }
    pub fn acc_pp(&self) -> f32 { self.acc_pp }
    pub fn accuracy_percent(&self) -> f32 { self.accuracy_percent }
    pub fn n300(&self) -> i32 { self.n300 }
    pub fn n100(&self) -> i32 { self.n100 }
    pub fn n50(&self) -> i32 { self.n50 }
    pub fn nmiss(&self) -> i32 { self.nmiss }
    pub fn title(&self) -> &str { &self.title }
    pub fn title_unicode(&self) -> &str { &self.title_unicode }
    pub fn artist(&self) -> &str { &self.artist }
    pub fn artist_unicode(&self) -> &str { &self.artist_unicode }
    pub fn creator(&self) -> &str { &self.creator }
    pub fn version(&self) -> &str { &self.version }
    pub fn ncircles(&self) -> i32 { self.ncircles }
    pub fn nsliders(&self) -> i32 { self.nsliders }
    pub fn nspinners(&self) -> i32 { self.nspinners }
    pub fn nobjects(&self) -> i32 { self.nobjects }
    pub fn ar(&self) -> f32 { self.ar }
    pub fn cs(&self) -> f32 { self.cs }
    pub fn od(&self) -> f32 { self.od }
    pub fn hp(&self) -> f32 { self.hp }
    pub fn odms(&self) -> f32 { self.odms }

    pub fn time_at(&self, i: usize) -> f32 {
        if let Some(map) = &self.parsed_beatmap {
            if i < map.objects.len() {
                return map.objects[i].time;
            }
        }
        0.0
    }

    pub fn strain_at(&self, i: usize, diff_type: usize) -> f32 {
        if let Some(map) = &self.parsed_beatmap {
            if i < map.objects.len() && diff_type < 2 {
                return map.objects[i].strains[diff_type];
            }
        }
        0.0
    }

    pub fn ntiming_points(&self) -> usize {
        self.parsed_beatmap.as_ref().map_or(0, |m| m.timing_points.len())
    }

    pub fn timing_time(&self, i: usize) -> f32 {
        if let Some(map) = &self.parsed_beatmap {
            if i < map.timing_points.len() {
                return map.timing_points[i].time;
            }
        }
        0.0
    }

    pub fn timing_ms_per_beat(&self, i: usize) -> f32 {
        if let Some(map) = &self.parsed_beatmap {
            if i < map.timing_points.len() {
                return map.timing_points[i].ms_per_beat;
            }
        }
        0.0
    }

    pub fn timing_change(&self, i: usize) -> bool {
        if let Some(map) = &self.parsed_beatmap {
            if i < map.timing_points.len() {
                return map.timing_points[i].change;
            }
        }
        false
    }
}
