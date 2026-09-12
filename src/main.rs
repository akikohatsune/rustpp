/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use std::env;
use std::process;
use rustpp::constants::*;
use rustpp::ezpp::Ezpp;
use rustpp::output::*;

const BANNER: &str = "     /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/ /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/ /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/ /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/ /\u{23bb}/      /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\\    /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/\n    / /\u{23bb}\u{23bb}\u{23bb}/ / / /\u{23bb}\u{23bb}\u{23bb}/ / / /\u{23bb}\u{23bb}\u{23bb}/ /  \u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/ / / / ___  / /\u{23bb}\u{23bb}\\ \\  / /\u{23bb}\u{23bb}\u{23bb}/ /\n   / /   / / / /   / / / /   / / /\u{23bb}\u{23bb}\u{23bb}\u{23bb}\u{23bb}/ / / / /__/ / /   / / / /   / /\n  / /___/ / / /___/ / / /___/ / / /      / /   / / / /___/ /\n /_______/ / ______/ / ______/ /_______/ /_/      /_/   /_/ /_____  /\n          / /       / /                                           / / \n         / /       / /                                    /\u{23bb}/___/ / \n        /_/       /_/                                    /_______/";

fn usage(prog: &str) {
    eprintln!("{BANNER}\n");
    eprintln!("usage: {prog} /path/to/file.osu parameters\n");
    eprintln!("set filename to '-' to read from standard input");
    eprintln!("all parameters are case insensitive\n");
    eprintln!("-o[output_module]\n  output module. pass ? to list modules ({prog} - -o?)\n  default: text\n  example: -ojson\n");
    eprintln!("[accuracy]%\n  accuracy percentage\n  default: 100%\n  example: 95%\n");
    eprintln!("[n]x100\n  amount of 100s\n  default: 0\n  example: 2x100\n");
    eprintln!("[n]x50\n  amount of 50s\n  default: 0\n  example: 2x50\n");
    eprintln!("[n]xm\n[n]xmiss\n[n]m\n  amount of misses\n  default: 0\n  example: 1m\n");
    eprintln!("[combo]x\n  highest combo achieved\n  default: full combo (calculated from map data)\n  example: 500x\n");
    eprintln!("scorev[n]\n  scoring system\n  default: 1\n  example: scorev2\n");
    eprintln!("ar[n]\n  base approach rate override\n  default: map's base approach rate\n  example: AR5\n");
    eprintln!("od[n]\n  base overall difficulty override\n  default: map's base overall difficulty\n  example: OD10\n");
    eprintln!("cs[n]\n  base circle size override\n  default: map's base circle size\n  example: CS6.5\n");
    eprintln!("-m[n]\n  gamemode id override for converted maps\n  default: uses the map's gamemode\n  example: -m1\n");
    eprintln!("-taiko\n  forces gamemode to taiko for converted maps\n  default: disabled\n");
    eprintln!("-touch\n  calculates pp for touchscreen / touch devices. can also be specified as mod TD\n");
    eprintln!("[n]speed\n  override speed stars. useful for maps with incorrect star rating\n  default: uses computed speed stars\n  example: 3.5speed\n");
    eprintln!("[n]aim\n  override aim stars. useful for maps with incorrect star rating\n  default: uses computed aim stars\n  example: 2.4aim\n");
    eprintln!("-end[n]\n  cuts map to a certain number of objects\n");
}

fn parse_mods(s: &str) -> u32 {
    let mut mods = MODS_NOMOD;
    let upper = s.to_ascii_uppercase();
    let mut rest = upper.as_str();

    while !rest.is_empty() {
        let mut matched = false;
        let mod_list: &[(&str, u32)] = &[
            ("NOMOD", MODS_NOMOD),
            ("NF", MODS_NF),
            ("EZ", MODS_EZ),
            ("TD", MODS_TD),
            ("HD", MODS_HD),
            ("HR", MODS_HR),
            ("SD", MODS_SD),
            ("DT", MODS_DT),
            ("RX", MODS_RX),
            ("HT", MODS_HT),
            ("NC", MODS_NC),
            ("FL", MODS_FL),
            ("AT", MODS_AT),
            ("SO", MODS_SO),
            ("AP", MODS_AP),
            ("PF", MODS_PF),
        ];

        for (name, flag) in mod_list {
            if rest.starts_with(name) {
                mods |= flag;
                rest = &rest[name.len()..];
                matched = true;
                break;
            }
        }

        if !matched {
            rest = &rest[1..];
        }
    }
    mods
}

fn print_modules_help() {
    println!("null\nno output\n-");
    println!("text\nplain text\n-");
    println!("json\na single utf-8 json object.\nthe code and errstr fields should be checked for errors. a negative value for code indicates an error\n-");
    println!("csv\nfieldname;value\none value per line. ';' characters in strings will be escaped to \"\\;\". utf-8.\nthe code and errstr fields should be checked for errors. a negative value for code indicates an error\n-");
    println!("binary\nbinary stream of values, encoded in little endian.\nnegative code values indicate an error, which matches the error codes defined in oppai.c\n-");
    println!("gnuplot\ngnuplot .gp script\n-");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.first().map(|s| s.as_str()).unwrap_or("oppai");

    if args.len() < 2 {
        usage(prog);
        process::exit(1);
    }

    if args[1].starts_with('-') && args[1].len() > 1 {
        let sub = args[1][1..].to_ascii_lowercase();
        if sub == "version" || sub == "v" {
            println!("{OPPAI_VERSION_STRING}");
            return;
        }
    }

    let mut ez = Ezpp::new();
    let mut output_name = "text".to_string();
    let mut mods_str: Option<String> = None;
    let mut mods = MODS_NOMOD;
    let mut speed_stars = 0.0f32;
    let mut aim_stars = 0.0f32;
    let mut accuracy_percent = 0.0f32;
    let mut has_accuracy_percent = false;
    let mut n100 = 0i32;
    let mut n50 = 0i32;
    let mut syntax_error = false;

    for arg in &args[2..] {
        let trimmed = arg.trim();
        if trimmed.is_empty() {
            continue;
        }

        let lower = trimmed.to_ascii_lowercase();

        if let Some(out) = lower.strip_prefix("-o") {
            if out == "?" {
                print_modules_help();
                return;
            }
            output_name = out.to_string();
            continue;
        }

        if let Some(acc_str) = lower.strip_suffix('%') {
            if let Ok(acc) = acc_str.parse::<f32>() {
                accuracy_percent = acc;
                has_accuracy_percent = true;
                continue;
            }
        }

        if let Some(n_str) = lower.strip_suffix("x100") {
            if let Ok(n) = n_str.parse::<i32>() {
                n100 = n;
                continue;
            }
        }

        if let Some(n_str) = lower.strip_suffix("x50") {
            if let Ok(n) = n_str.parse::<i32>() {
                n50 = n;
                continue;
            }
        }

        if let Some(s_str) = lower.strip_suffix("speed") {
            if let Ok(s) = s_str.parse::<f32>() {
                speed_stars = s;
                continue;
            }
        }

        if let Some(a_str) = lower.strip_suffix("aim") {
            if let Ok(a) = a_str.parse::<f32>() {
                aim_stars = a;
                continue;
            }
        }

        if let Some(m_str) = lower.strip_suffix("xmiss")
            .or_else(|| lower.strip_suffix("xm"))
            .or_else(|| lower.strip_suffix('m'))
        {
            if let Ok(m) = m_str.parse::<i32>() {
                ez.set_nmiss(m);
                continue;
            }
        }

        if let Some(c_str) = lower.strip_suffix('x') {
            if let Ok(c) = c_str.parse::<i32>() {
                ez.set_combo(c);
                continue;
            }
        }

        if let Some(sv_str) = lower.strip_prefix("scorev") {
            if let Ok(sv) = sv_str.parse::<i32>() {
                ez.set_score_version(sv);
                continue;
            }
        }

        if let Some(ar_str) = lower.strip_prefix("ar") {
            if let Ok(ar) = ar_str.parse::<f32>() {
                ez.set_base_ar(ar);
                continue;
            }
        }

        if let Some(od_str) = lower.strip_prefix("od") {
            if let Ok(od) = od_str.parse::<f32>() {
                ez.set_base_od(od);
                continue;
            }
        }

        if let Some(cs_str) = lower.strip_prefix("cs") {
            if let Ok(cs) = cs_str.parse::<f32>() {
                ez.set_base_cs(cs);
                continue;
            }
        }

        if let Some(m_str) = lower.strip_prefix("-m") {
            if let Ok(m) = m_str.parse::<i32>() {
                ez.set_mode_override(m);
                continue;
            }
        }

        if let Some(end_str) = lower.strip_prefix("-end") {
            if let Ok(end) = end_str.parse::<i32>() {
                ez.set_end(end);
                continue;
            }
        }

        if lower == "-taiko" {
            ez.set_mode_override(MODE_TAIKO);
            continue;
        }

        if lower == "-touch" {
            mods |= MODS_TOUCH_DEVICE;
            continue;
        }

        if trimmed.starts_with('+') {
            let m = &trimmed[1..];
            mods_str = Some(m.to_ascii_uppercase());
            mods |= parse_mods(m);
            continue;
        }

        eprintln!(">{trimmed}");
        syntax_error = true;
        break;
    }

    let result = if syntax_error {
        ERR_SYNTAX
    } else {
        if has_accuracy_percent {
            ez.set_accuracy_percent(accuracy_percent);
        } else {
            ez.set_accuracy(n100, n50);
        }
        ez.set_mods(mods);
        ez.set_speed_stars(speed_stars);
        ez.set_aim_stars(aim_stars);

        match ez.parse_file(&args[1]) {
            Ok(code) => code,
            Err(e) => e,
        }
    };

    let m_str = mods_str.as_deref();
    match output_name.as_str() {
        "text" => output_text(result, &ez, m_str),
        "json" => output_json(result, &ez, m_str),
        "csv" => output_csv(result, &ez, m_str),
        "binary" => output_binary(result, &ez, m_str),
        "gnuplot" => output_gnuplot(result, &ez, m_str),
        "null" => output_null(result, &ez, m_str),
        _ => {
            eprintln!("output module '{output_name}' does not exist. check '{prog} - -o?'");
            process::exit(1);
        }
    }

    if result < 0 {
        process::exit(1);
    }
}
