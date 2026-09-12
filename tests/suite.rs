mod suite_data;
use suite_data::SUITE;
use rustpp::ezpp::Ezpp;
use std::path::Path;

const ERROR_MARGIN: f32 = 0.02;

#[test]
fn test_oppai_suite() {
    let mut last_id = 0u32;
    let mut count = [0usize; 2];
    let mut avg_err = [0.0f64; 2];
    let mut max_err = [0.0f64; 2];
    let mut max_err_map = [0u32; 2];
    let mut ez = Ezpp::new();

    let mut missing_maps = 0;

    for (i, s) in SUITE.iter().enumerate() {
        let path_str = format!("test/test_suite/{}.osu", s.id);
        let path = Path::new(&path_str);
        if !path.exists() {
            missing_maps += 1;
            continue;
        }

        if s.id != last_id {
            last_id = s.id;
            ez.set_base_cs(-1.0);
            ez.set_base_ar(-1.0);
            ez.set_base_od(-1.0);
            ez.set_base_hp(-1.0);
        }

        ez.set_mods(s.mods);
        ez.set_accuracy(s.n100, s.n50);
        ez.set_nmiss(s.nmiss);
        ez.set_combo(s.max_combo);
        ez.set_mode_override(s.mode);

        let err = ez.parse_file(&path_str);
        assert!(err.is_ok(), "Failed to parse map {}: {:?}", path_str, err);

        let pptotal = ez.pp();
        let expected_pp = s.pp as f32;
        count[s.mode as usize] += 1;

        let mut margin = expected_pp * ERROR_MARGIN;
        if expected_pp < 100.0 {
            margin *= 3.0;
        } else if expected_pp < 200.0 {
            margin *= 2.0;
        } else if expected_pp < 300.0 {
            margin *= 1.5;
        }

        let error = (pptotal - expected_pp).abs();
        let error_percent = (error / expected_pp) as f64;
        avg_err[s.mode as usize] += error_percent;
        if error_percent > max_err[s.mode as usize] {
            max_err[s.mode as usize] = error_percent;
            max_err_map[s.mode as usize] = s.id;
        }

        assert!(
            error < margin,
            "Failed test #{i} on map {}.osu (mode {}): got {pptotal} pp, expected {expected_pp} pp (diff: {error}, margin: {margin})",
            s.id,
            s.mode
        );
    }

    println!("\n=== TEST SUITE RESULTS ===");
    println!("Missing maps: {missing_maps}");
    for mode in 0..2 {
        let name = if mode == 0 { "osu!standard" } else { "osu!taiko" };
        let c = count[mode];
        if c > 0 {
            let avg = avg_err[mode] / c as f64;
            println!("{name}: {c} scores, avg err: {:.4}%, max err: {:.4}% on map {}",
                avg * 100.0, max_err[mode] * 100.0, max_err_map[mode]);
        }
    }
}
