/*
 * this is free and unencumbered software released into the public domain.
 * refer to the attached UNLICENSE or http://unlicense.org/
 */

use std::io::{self, Write};
use crate::constants::*;
use crate::ezpp::Ezpp;

pub const ASCIIPLT_W: usize = 51;

#[inline]
pub fn twodec(x: f32) -> f32 {
    (x * 100.0 + 0.5).floor() / 100.0
}

pub fn asciiplt<F>(get_val: F, n: usize)
where
    F: Fn(usize) -> f32,
{
    let charset = [" ", "_", ".", "-", "^"];
    let charset_size = charset.len();

    let w = ASCIIPLT_W.min(n);
    if w == 0 {
        println!();
        return;
    }

    let chunksize = ((n as f32) / (w as f32)).ceil() as usize;
    let mut values = vec![0.0f32; ASCIIPLT_W];
    let mut minval = f32::INFINITY;
    let mut maxval = f32::NEG_INFINITY;

    for i in 0..n {
        let chunki = i / chunksize;
        if chunki < ASCIIPLT_W {
            values[chunki] = values[chunki].max(get_val(i));
        }
    }

    for i in 0..n {
        let chunki = i / chunksize;
        if chunki < ASCIIPLT_W {
            maxval = maxval.max(values[chunki]);
            minval = minval.min(values[chunki]);
        }
    }

    let range = (maxval - minval).max(0.00001);

    for i in 0..w {
        let chari = (((values[i] - minval) / range) * charset_size as f32) as isize;
        let clamped_chari = chari.clamp(0, (charset_size - 1) as isize) as usize;
        print!("{}", charset[clamped_chari]);
    }
    println!();
}

pub fn output_text(result: i32, ez: &Ezpp, mods_str: Option<&str>) {
    if result < 0 {
        println!("{}", errstr(result));
        return;
    }

    print!("{} - {} ", ez.artist(), ez.title());
    if ez.artist() != ez.artist_unicode() || ez.title() != ez.title_unicode() {
        print!("({} - {}) ", ez.artist_unicode(), ez.title_unicode());
    }
    println!("[{}] mapped by {}\n", ez.version(), ez.creator());

    let ar = twodec(ez.ar());
    let od = twodec(ez.od());
    let cs = twodec(ez.cs());
    let hp = twodec(ez.hp());
    let stars = twodec(ez.stars());
    let aim_stars = twodec(ez.aim_stars());
    let speed_stars = twodec(ez.speed_stars());
    let accuracy_percent = twodec(ez.accuracy_percent());
    let pp = twodec(ez.pp());
    let aim_pp = twodec(ez.aim_pp());
    let speed_pp = twodec(ez.speed_pp());
    let acc_pp = twodec(ez.acc_pp());

    print!("AR{ar} OD{od} ");
    if ez.mode() == MODE_STD {
        print!("CS{cs} ");
    }
    println!("HP{hp}");
    println!("300 hitwindow: {} ms", ez.odms());
    println!(
        "{} circles, {} sliders, {} spinners",
        ez.ncircles(),
        ez.nsliders(),
        ez.nspinners()
    );

    if ez.mode() == MODE_STD {
        println!("{stars} stars ({aim_stars} aim, {speed_stars} speed)\n");
        print!("speed strain: ");
        asciiplt(|i| ez.strain_at(i, DIFF_SPEED), ez.nobjects() as usize);
        print!("  aim strain: ");
        asciiplt(|i| ez.strain_at(i, DIFF_AIM), ez.nobjects() as usize);
    } else {
        println!("{stars} stars",);
    }
    println!();

    if let Some(m) = mods_str {
        if !m.is_empty() {
            print!("+{m} ");
        }
    }

    println!("{}/{}x ", ez.combo(), ez.max_combo());
    println!("{accuracy_percent}%");
    print!("{pp} pp (");
    if ez.mode() == MODE_STD {
        print!("{aim_pp} aim, ");
    }
    println!("{speed_pp} speed, {acc_pp} acc)\n");
}

fn fix_json_flt(v: f32) -> f32 {
    if v.is_infinite() {
        -1.0
    } else if v.is_nan() {
        0.0
    } else {
        v
    }
}

fn escape_json_str(s: &str) -> String {
    let mut res = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '\\' => res.push_str("\\\\"),
            '"' => res.push_str("\\\""),
            '\n' => res.push_str("\\n"),
            '\r' => res.push_str("\\r"),
            '\t' => res.push_str("\\t"),
            _ => res.push(c),
        }
    }
    res
}

pub fn output_json(result: i32, ez: &Ezpp, mods_str: Option<&str>) {
    print!("{{\"oppai_version\":\"{OPPAI_VERSION_STRING}\",");
    if result < 0 {
        print!(
            "\"code\":{result},\"errstr\":\"{}\"}}",
            escape_json_str(errstr(result))
        );
        println!();
        return;
    }

    let pp = fix_json_flt(ez.pp());
    let aim_pp = fix_json_flt(ez.aim_pp());
    let speed_pp = fix_json_flt(ez.speed_pp());
    let acc_pp = fix_json_flt(ez.acc_pp());
    let stars = fix_json_flt(ez.stars());
    let aim_stars = fix_json_flt(ez.aim_stars());
    let speed_stars = fix_json_flt(ez.speed_stars());

    print!("\"code\":200,\"errstr\":\"no error\",");
    print!("\"artist\":\"{}\"", escape_json_str(ez.artist()));
    if ez.artist() != ez.artist_unicode() {
        print!(",\"artist_unicode\":\"{}\"", escape_json_str(ez.artist_unicode()));
    }
    print!(",\"title\":\"{}\"", escape_json_str(ez.title()));
    if ez.title() != ez.title_unicode() {
        print!(",\"title_unicode\":\"{}\"", escape_json_str(ez.title_unicode()));
    }
    print!(",\"creator\":\"{}\"", escape_json_str(ez.creator()));
    print!(",\"version\":\"{}\",", escape_json_str(ez.version()));

    let m_str = mods_str.unwrap_or("");
    print!(
        "\"mods_str\":\"{}\",\"mods\":{},\"od\":{},\"ar\":{},\"cs\":{},\"hp\":{},\
         \"combo\":{},\"max_combo\":{},\"num_circles\":{},\"num_sliders\":{},\
         \"num_spinners\":{},\"misses\":{},\"score_version\":{},\"stars\":{:.17},\
         \"speed_stars\":{:.17},\"aim_stars\":{:.17},\"aim_pp\":{:.17},\
         \"speed_pp\":{:.17},\"acc_pp\":{:.17},\"pp\":{:.17}}}",
        escape_json_str(m_str),
        ez.mods(),
        ez.od(),
        ez.ar(),
        ez.cs(),
        ez.hp(),
        ez.combo(),
        ez.max_combo(),
        ez.ncircles(),
        ez.nsliders(),
        ez.nspinners(),
        ez.nmiss(),
        ez.score_version(),
        stars,
        speed_stars,
        aim_stars,
        aim_pp,
        speed_pp,
        acc_pp,
        pp
    );
    println!();
}

fn escape_csv_str(s: &str) -> String {
    let mut res = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '\\' => res.push_str("\\\\"),
            ';' => res.push_str("\\;"),
            _ => res.push(c),
        }
    }
    res
}

pub fn output_csv(result: i32, ez: &Ezpp, mods_str: Option<&str>) {
    println!("oppai_version;{OPPAI_VERSION_STRING}");
    if result < 0 {
        println!("code;{result}\nerrstr;{}", escape_csv_str(errstr(result)));
        return;
    }

    println!("code;200\nerrstr;no error");
    println!("artist;{}", escape_csv_str(ez.artist()));
    if ez.artist() != ez.artist_unicode() {
        println!("artist_unicode;{}", escape_csv_str(ez.artist_unicode()));
    }
    println!("title;{}", escape_csv_str(ez.title()));
    if ez.title() != ez.title_unicode() {
        println!("title_unicode;{}", escape_csv_str(ez.title_unicode()));
    }
    println!("version;{}", escape_csv_str(ez.version()));
    println!("creator;{}", escape_csv_str(ez.creator()));

    let m_str = mods_str.unwrap_or("");
    print!(
        "mods_str;{}\nmods;{}\nod;{}\nar;{}\ncs;{}\nhp;{}\n\
         combo;{}\nmax_combo;{}\nnum_circles;{}\n\
         num_sliders;{}\nnum_spinners;{}\nmisses;{}\n\
         score_version;{}\nstars;{:.17}\nspeed_stars;{:.17}\n\
         aim_stars;{:.17}\naim_pp;{:.17}\nspeed_pp;{:.17}\nacc_pp;{:.17}\npp;{:.17}",
        m_str,
        ez.mods(),
        ez.od(),
        ez.ar(),
        ez.cs(),
        ez.hp(),
        ez.combo(),
        ez.max_combo(),
        ez.ncircles(),
        ez.nsliders(),
        ez.nspinners(),
        ez.nmiss(),
        ez.score_version(),
        ez.stars(),
        ez.speed_stars(),
        ez.aim_stars(),
        ez.aim_pp(),
        ez.speed_pp(),
        ez.acc_pp(),
        ez.pp()
    );
    println!();
}

pub fn output_binary(result: i32, ez: &Ezpp, _mods_str: Option<&str>) {
    let mut stdout = io::stdout().lock();
    let _ = stdout.write_all(b"binoppai");
    let _ = stdout.write_all(&[
        OPPAI_VERSION_MAJOR as u8,
        OPPAI_VERSION_MINOR as u8,
        OPPAI_VERSION_PATCH as u8,
    ]);
    let _ = stdout.write_all(&result.to_le_bytes());

    if result < 0 {
        return;
    }

    let write_str = |out: &mut io::StdoutLock, s: &str| {
        let bytes = s.as_bytes();
        let len = (bytes.len().min(0xFFFF)) as u16;
        let _ = out.write_all(&len.to_le_bytes());
        let _ = out.write_all(&bytes[..len as usize]);
        let _ = out.write_all(&[0u8]);
    };

    write_str(&mut stdout, ez.artist());
    write_str(&mut stdout, ez.artist_unicode());
    write_str(&mut stdout, ez.title());
    write_str(&mut stdout, ez.title_unicode());
    write_str(&mut stdout, ez.version());
    write_str(&mut stdout, ez.creator());

    let _ = stdout.write_all(&(ez.mods() as i32).to_le_bytes());
    let _ = stdout.write_all(&ez.od().to_le_bytes());
    let _ = stdout.write_all(&ez.ar().to_le_bytes());
    let _ = stdout.write_all(&ez.cs().to_le_bytes());
    let _ = stdout.write_all(&ez.hp().to_le_bytes());
    let _ = stdout.write_all(&ez.combo().to_le_bytes());
    let _ = stdout.write_all(&ez.max_combo().to_le_bytes());
    let _ = stdout.write_all(&(ez.ncircles() as i16).to_le_bytes());
    let _ = stdout.write_all(&(ez.nsliders() as i16).to_le_bytes());
    let _ = stdout.write_all(&(ez.nspinners() as i16).to_le_bytes());
    let _ = stdout.write_all(&ez.score_version().to_le_bytes());
    let _ = stdout.write_all(&ez.stars().to_le_bytes());
    let _ = stdout.write_all(&ez.speed_stars().to_le_bytes());
    let _ = stdout.write_all(&ez.aim_stars().to_le_bytes());
    let _ = stdout.write_all(&0i16.to_le_bytes()); // legacy nsingles
    let _ = stdout.write_all(&0i16.to_le_bytes()); // legacy nsingles_threshold
    let _ = stdout.write_all(&ez.aim_pp().to_le_bytes());
    let _ = stdout.write_all(&ez.speed_pp().to_le_bytes());
    let _ = stdout.write_all(&ez.acc_pp().to_le_bytes());
    let _ = stdout.write_all(&ez.pp().to_le_bytes());
    let _ = stdout.flush();
}

pub fn output_gnuplot(result: i32, ez: &Ezpp, mods_str: Option<&str>) {
    if result < 0 || ez.mode() != MODE_STD {
        return;
    }

    println!("set encoding utf8;");
    print!("set title \"{} - {} ", ez.artist(), ez.title());
    if ez.artist() != ez.artist_unicode() || ez.title() != ez.title_unicode() {
        print!("({} - {})", ez.artist_unicode(), ez.title_unicode());
    }
    print!(" [{}] mapped by {}", ez.version(), ez.creator());
    if let Some(m) = mods_str {
        if !m.is_empty() {
            print!(" +{m}");
        }
    }
    println!("\";");

    println!("set xlabel 'time (ms)';");
    println!("set ylabel 'strain';");
    println!("set multiplot layout 2,1 rowsfirst;");
    println!("plot '-' with lines lc 1 title 'speed'");
    for i in 0..ez.nobjects() as usize {
        println!("{:.17} {:.17}", ez.time_at(i), ez.strain_at(i, DIFF_SPEED));
    }
    println!("e");
    println!("unset title;");
    println!("plot '-' with lines lc 2 title 'aim'");
    for i in 0..ez.nobjects() as usize {
        println!("{:.17} {:.17}", ez.time_at(i), ez.strain_at(i, DIFF_AIM));
    }
}

pub fn output_null(_result: i32, _ez: &Ezpp, _mods_str: Option<&str>) {}
