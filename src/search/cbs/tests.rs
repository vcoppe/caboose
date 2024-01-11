use std::path::PathBuf;

use crate::{get_cbs_from_files, MyTime};

pub fn solve(map_file: &str, task_file: &str, config_file: &str, n_agents: usize) -> MyTime {
    let map = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("resources/instances")
        .join(map_file)
        .join("map.xml");
    let map = map.to_str().unwrap();

    let task = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("resources/instances")
        .join(map_file)
        .join(task_file);
    let task = task.to_str().unwrap();

    let config = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("resources/config")
        .join(config_file);
    let config = config.to_str().unwrap();

    let (_, mut cbs, config, _) = get_cbs_from_files(&map, &task, &config, n_agents);

    let solution = cbs.solve(&config).unwrap();

    solution.iter().map(|sol| sol.cost).sum()
}

#[test]
fn den520d_random_den520d_random_1_10() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 10).0 - 1968.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_2() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 2).0 - 313.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_3() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 3).0 - 351.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_4() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 4).0 - 527.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_5() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 5).0 - 735.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_6() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 6).0 - 1130.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_7() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 7).0 - 1381.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_8() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 8).0 - 1458.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_1_9() {
    assert!((solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 9).0 - 1754.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_10_2() {
    assert!((solve("den520d_random", "den520d-random-10.xml", "config-2.xml", 2).0 - 519.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_10_3() {
    assert!((solve("den520d_random", "den520d-random-10.xml", "config-2.xml", 3).0 - 595.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_10_4() {
    assert!((solve("den520d_random", "den520d-random-10.xml", "config-2.xml", 4).0 - 647.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_10_5() {
    assert!((solve("den520d_random", "den520d-random-10.xml", "config-2.xml", 5).0 - 922.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_11_2() {
    assert!((solve("den520d_random", "den520d-random-11.xml", "config-2.xml", 2).0 - 484.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_11_3() {
    assert!((solve("den520d_random", "den520d-random-11.xml", "config-2.xml", 3).0 - 750.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_10() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 10).0 - 1948.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_2() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 2).0 - 525.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_3() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 3).0 - 958.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_4() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 4).0 - 1003.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_5() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 5).0 - 1080.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_6() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 6).0 - 1140.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_7() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 7).0 - 1446.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_8() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 8).0 - 1584.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_12_9() {
    assert!((solve("den520d_random", "den520d-random-12.xml", "config-2.xml", 9).0 - 1719.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_10() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 10).0 - 1415.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_11() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 11).0 - 1630.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_12() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 12).0 - 1901.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_13() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 13).0 - 2083.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_14() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 14).0 - 2109.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_15() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 15).0 - 2369.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_16() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 16).0 - 2417.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_17() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 17).0 - 2660.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_2() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 2).0 - 329.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_3() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 3).0 - 504.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_4() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 4).0 - 747.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_5() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 5).0 - 763.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_6() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 6).0 - 929.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_7() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 7).0 - 1041.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_8() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 8).0 - 1052.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_13_9() {
    assert!((solve("den520d_random", "den520d-random-13.xml", "config-2.xml", 9).0 - 1214.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_10() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 10).0 - 1228.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_11() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 11).0 - 1316.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_12() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 12).0 - 1433.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_13() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 13).0 - 1591.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_14() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 14).0 - 1835.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_15() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 15).0 - 2128.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_16() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 16).0 - 2273.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_17() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 17).0 - 2536.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_18() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 18).0 - 2669.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_19() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 19).0 - 2761.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_2() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 2).0 - 321.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_20() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 20).0 - 2907.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_21() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 21).0 - 3014.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_22() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 22).0 - 3167.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_23() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 23).0 - 3352.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_24() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 24).0 - 3379.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_25() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 25).0 - 3577.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_3() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 3).0 - 354.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_4() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 4).0 - 487.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_5() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 5).0 - 580.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_6() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 6).0 - 734.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_7() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 7).0 - 809.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_8() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 8).0 - 896.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_14_9() {
    assert!((solve("den520d_random", "den520d-random-14.xml", "config-2.xml", 9).0 - 1011.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_10() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 10).0 - 2079.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_11() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 11).0 - 2099.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_12() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 12).0 - 2255.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_13() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 13).0 - 2335.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_14() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 14).0 - 2589.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_15() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 15).0 - 2798.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_16() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 16).0 - 3163.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_2() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 2).0 - 361.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_3() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 3).0 - 434.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_4() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 4).0 - 707.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_5() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 5).0 - 802.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_6() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 6).0 - 949.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_7() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 7).0 - 1254.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_8() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 8).0 - 1434.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_15_9() {
    assert!((solve("den520d_random", "den520d-random-15.xml", "config-2.xml", 9).0 - 1770.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_10() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 10).0 - 1802.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_11() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 11).0 - 2010.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_12() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 12).0 - 2205.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_13() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 13).0 - 2387.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_14() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 14).0 - 2555.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_15() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 15).0 - 2613.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_16() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 16).0 - 2750.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_17() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 17).0 - 2975.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_18() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 18).0 - 3311.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_19() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 19).0 - 3355.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_2() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 2).0 - 407.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_3() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 3).0 - 699.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_4() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 4).0 - 914.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_5() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 5).0 - 1012.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_6() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 6).0 - 1149.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_7() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 7).0 - 1327.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_8() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 8).0 - 1468.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_16_9() {
    assert!((solve("den520d_random", "den520d-random-16.xml", "config-2.xml", 9).0 - 1727.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_2() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 2).0 - 500.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_3() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 3).0 - 669.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_4() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 4).0 - 986.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_5() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 5).0 - 1023.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_6() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 6).0 - 1305.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_7() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 7).0 - 1519.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_17_8() {
    assert!((solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 8).0 - 1853.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_10() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 10).0 - 1950.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_11() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 11).0 - 2080.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_2() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 2).0 - 370.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_3() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 3).0 - 500.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_4() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 4).0 - 822.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_5() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 5).0 - 1122.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_6() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 6).0 - 1265.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_7() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 7).0 - 1374.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_8() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 8).0 - 1491.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_18_9() {
    assert!((solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 9).0 - 1743.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_10() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 10).0 - 1635.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_11() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 11).0 - 1733.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_12() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 12).0 - 1887.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_2() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 2).0 - 370.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_3() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 3).0 - 489.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_4() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 4).0 - 546.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_5() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 5).0 - 641.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_6() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 6).0 - 836.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_7() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 7).0 - 1049.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_8() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 8).0 - 1276.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_19_9() {
    assert!((solve("den520d_random", "den520d-random-19.xml", "config-2.xml", 9).0 - 1570.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_10() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 10).0 - 1911.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_11() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 11).0 - 2082.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_12() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 12).0 - 2226.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_13() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 13).0 - 2250.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_14() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 14).0 - 2329.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_15() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 15).0 - 2433.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_16() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 16).0 - 2627.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_17() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 17).0 - 2775.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_18() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 18).0 - 3048.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_2() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 2).0 - 241.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_3() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 3).0 - 274.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_4() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 4).0 - 379.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_5() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 5).0 - 700.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_6() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 6).0 - 989.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_7() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 7).0 - 1317.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_8() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 8).0 - 1497.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_2_9() {
    assert!((solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 9).0 - 1688.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_2() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 2).0 - 365.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_3() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 3).0 - 580.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_4() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 4).0 - 816.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_5() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 5).0 - 1049.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_6() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 6).0 - 1213.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_7() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 7).0 - 1458.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_8() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 8).0 - 1572.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_20_9() {
    assert!((solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 9).0 - 1619.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_10() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 10).0 - 2010.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_11() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 11).0 - 2290.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_12() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 12).0 - 2414.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_13() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 13).0 - 2553.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_14() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 14).0 - 2655.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_2() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 2).0 - 444.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_3() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 3).0 - 776.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_4() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 4).0 - 956.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_5() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 5).0 - 1118.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_6() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 6).0 - 1399.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_7() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 7).0 - 1643.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_8() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 8).0 - 1714.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_21_9() {
    assert!((solve("den520d_random", "den520d-random-21.xml", "config-2.xml", 9).0 - 1897.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_10() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 10).0 - 1415.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_11() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 11).0 - 1511.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_12() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 12).0 - 1625.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_2() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 2).0 - 290.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_3() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 3).0 - 295.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_4() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 4).0 - 525.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_5() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 5).0 - 785.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_6() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 6).0 - 874.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_7() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 7).0 - 923.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_8() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 8).0 - 1164.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_22_9() {
    assert!((solve("den520d_random", "den520d-random-22.xml", "config-2.xml", 9).0 - 1294.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_10() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 10).0 - 1372.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_11() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 11).0 - 1424.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_12() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 12).0 - 1650.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_13() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 13).0 - 1800.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_14() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 14).0 - 1984.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_15() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 15).0 - 2159.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_2() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 2).0 - 159.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_3() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 3).0 - 297.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_4() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 4).0 - 396.131).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_5() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 5).0 - 582.131).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_6() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 6).0 - 731.131).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_7() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 7).0 - 886.131).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_8() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 8).0 - 1122.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_23_9() {
    assert!((solve("den520d_random", "den520d-random-23.xml", "config-2.xml", 9).0 - 1358.13).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_24_2() {
    assert!((solve("den520d_random", "den520d-random-24.xml", "config-2.xml", 2).0 - 303.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_24_3() {
    assert!((solve("den520d_random", "den520d-random-24.xml", "config-2.xml", 3).0 - 586.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_24_4() {
    assert!((solve("den520d_random", "den520d-random-24.xml", "config-2.xml", 4).0 - 790.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_24_5() {
    assert!((solve("den520d_random", "den520d-random-24.xml", "config-2.xml", 5).0 - 1003.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_25_2() {
    assert!((solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 2).0 - 461.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_25_3() {
    assert!((solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 3).0 - 693.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_25_4() {
    assert!((solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 4).0 - 886.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_25_5() {
    assert!((solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 5).0 - 1167.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_25_6() {
    assert!((solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 6).0 - 1403.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_3_2() {
    assert!((solve("den520d_random", "den520d-random-3.xml", "config-2.xml", 2).0 - 422.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_3_3() {
    assert!((solve("den520d_random", "den520d-random-3.xml", "config-2.xml", 3).0 - 535.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_3_4() {
    assert!((solve("den520d_random", "den520d-random-3.xml", "config-2.xml", 4).0 - 812.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_3_5() {
    assert!((solve("den520d_random", "den520d-random-3.xml", "config-2.xml", 5).0 - 890.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_10() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 10).0 - 1768.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_2() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 2).0 - 447.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_3() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 3).0 - 609.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_4() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 4).0 - 851.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_5() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 5).0 - 1083.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_6() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 6).0 - 1117.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_7() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 7).0 - 1234.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_8() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 8).0 - 1332.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_4_9() {
    assert!((solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 9).0 - 1569.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_10() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 10).0 - 1631.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_11() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 11).0 - 1834.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_12() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 12).0 - 2159.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_13() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 13).0 - 2165.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_14() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 14).0 - 2276.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_15() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 15).0 - 2428.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_16() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 16).0 - 2436.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_17() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 17).0 - 2525.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_18() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 18).0 - 2781.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_2() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 2).0 - 258.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_3() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 3).0 - 532.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_4() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 4).0 - 774.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_5() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 5).0 - 1031.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_6() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 6).0 - 1158.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_7() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 7).0 - 1336.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_8() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 8).0 - 1367.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_5_9() {
    assert!((solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 9).0 - 1521.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_6_2() {
    assert!((solve("den520d_random", "den520d-random-6.xml", "config-2.xml", 2).0 - 353.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_6_3() {
    assert!((solve("den520d_random", "den520d-random-6.xml", "config-2.xml", 3).0 - 546.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_10() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 10).0 - 1843.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_11() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 11).0 - 2235.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_2() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 2).0 - 307.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_3() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 3).0 - 451.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_4() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 4).0 - 773.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_5() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 5).0 - 827.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_6() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 6).0 - 1158.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_7() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 7).0 - 1256.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_8() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 8).0 - 1443.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_7_9() {
    assert!((solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 9).0 - 1651.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_10() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 10).0 - 1265.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_11() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 11).0 - 1303.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_12() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 12).0 - 1555.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_13() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 13).0 - 1617.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_14() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 14).0 - 1683.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_15() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 15).0 - 1912.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_16() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 16).0 - 2025.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_2() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 2).0 - 198.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_3() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 3).0 - 253.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_4() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 4).0 - 408.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_5() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 5).0 - 496.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_6() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 6).0 - 632.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_7() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 7).0 - 684.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_8() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 8).0 - 1010.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_8_9() {
    assert!((solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 9).0 - 1056.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_10() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 10).0 - 1424.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_11() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 11).0 - 1583.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_12() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 12).0 - 1677.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_13() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 13).0 - 1821.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_14() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 14).0 - 2133.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_15() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 15).0 - 2372.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_16() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 16).0 - 2571.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_17() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 17).0 - 2718.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_2() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 2).0 - 242.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_3() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 3).0 - 302.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_4() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 4).0 - 378.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_5() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 5).0 - 643.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_6() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 6).0 - 973.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_7() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 7).0 - 1062.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_8() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 8).0 - 1159.0).abs() < 1e-2);
}

#[test]
fn den520d_random_den520d_random_9_9() {
    assert!((solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 9).0 - 1321.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 10).0 - 102.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 11).0 - 108.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 12).0 - 116.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 13).0 - 120.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 14).0 - 126.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 15).0 - 135.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 16).0 - 152.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 17).0 - 159.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 18).0 - 169.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_19() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 19).0 - 185.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 2).0 - 26.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_20() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 20).0 - 189.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_21() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 21).0 - 195.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_22() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 22).0 - 198.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_23() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 23).0 - 214.394).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 3).0 - 36.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 4).0 - 47.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 5).0 - 55.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 6).0 - 72.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 7).0 - 78.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 8).0 - 84.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-1.xml", "config-2.xml", 9).0 - 92.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 10).0 - 134.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 11).0 - 143.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 12).0 - 147.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 13).0 - 162.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 14).0 - 173.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 15).0 - 186.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 16).0 - 198.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 17).0 - 201.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 18).0 - 209.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 2).0 - 23.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 3).0 - 31.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 4).0 - 47.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 5).0 - 58.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 6).0 - 79.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 7).0 - 83.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 8).0 - 101.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-10.xml", "config-2.xml", 9).0 - 116.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 10).0 - 65.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 11).0 - 78.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 12).0 - 93.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 13).0 - 111.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 14).0 - 122.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 15).0 - 124.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 16).0 - 134.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 17).0 - 144.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 18).0 - 156.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_19() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 19).0 - 165.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 2).0 - 16.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 3).0 - 22.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 4).0 - 25.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 5).0 - 30.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 6).0 - 37.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 7).0 - 44.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 8).0 - 46.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-11.xml", "config-2.xml", 9).0 - 59.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_12_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-12.xml", "config-2.xml", 2).0 - 27.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_12_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-12.xml", "config-2.xml", 3).0 - 39.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_12_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-12.xml", "config-2.xml", 4).0 - 46.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_12_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-12.xml", "config-2.xml", 5).0 - 52.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_12_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-12.xml", "config-2.xml", 6).0 - 59.2627).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 10).0 - 129.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 11).0 - 136.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 2).0 - 36.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 3).0 - 49.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 4).0 - 66.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 5).0 - 73.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 6).0 - 92.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 7).0 - 102.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 8).0 - 104.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-13.xml", "config-2.xml", 9).0 - 112.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 10).0 - 112.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 11).0 - 121.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 12).0 - 130.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 13).0 - 142.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 14).0 - 150.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 15).0 - 159.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 16).0 - 164.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 2).0 - 15.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 3).0 - 32.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 4).0 - 44.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 5).0 - 52.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 6).0 - 59.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 7).0 - 70.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 8).0 - 87.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-14.xml", "config-2.xml", 9).0 - 99.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 10).0 - 106.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 11).0 - 107.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 12).0 - 114.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 13).0 - 120.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 2).0 - 26.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 3).0 - 29.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 4).0 - 49.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 5).0 - 57.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 6).0 - 67.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 7).0 - 75.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 8).0 - 87.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-15.xml", "config-2.xml", 9).0 - 98.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 10).0 - 84.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 11).0 - 88.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 12).0 - 94.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 13).0 - 99.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 14).0 - 102.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 15).0 - 104.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 16).0 - 124.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 2).0 - 18.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 3).0 - 33.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 4).0 - 39.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 5).0 - 51.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 6).0 - 56.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 7).0 - 66.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 8).0 - 73.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-16.xml", "config-2.xml", 9).0 - 83.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 10).0 - 102.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 11).0 - 113.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 2).0 - 16.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 3).0 - 26.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 4).0 - 37.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 5).0 - 49.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 6).0 - 52.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 7).0 - 69.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 8).0 - 75.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-17.xml", "config-2.xml", 9).0 - 88.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 10).0 - 133.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 11).0 - 137.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 12).0 - 139.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 13).0 - 157.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 14).0 - 168.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 15).0 - 177.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 16).0 - 184.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 17).0 - 197.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 18).0 - 207.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_19() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 19).0 - 213.263).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 2).0 - 32.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_20() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 20).0 - 228.394).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 3).0 - 41.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 4).0 - 57.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 5).0 - 72.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 6).0 - 81.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 7).0 - 87.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 8).0 - 104.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-18.xml", "config-2.xml", 9).0 - 113.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 10).0 - 122.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 11).0 - 130.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 12).0 - 145.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 13).0 - 157.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 14).0 - 172.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 15).0 - 192.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 2).0 - 9.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 3).0 - 14.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 4).0 - 24.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 5).0 - 42.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 6).0 - 56.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 7).0 - 74.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 8).0 - 84.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-19.xml", "config-2.xml", 9).0 - 98.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 10).0 - 122.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 11).0 - 127.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 12).0 - 141.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 13).0 - 151.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 14).0 - 160.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 15).0 - 173.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 2).0 - 32.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 3).0 - 43.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 4).0 - 57.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 5).0 - 63.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 6).0 - 70.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 7).0 - 81.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 8).0 - 95.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-2.xml", "config-2.xml", 9).0 - 111.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 10).0 - 119.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 11).0 - 128.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 12).0 - 144.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 13).0 - 153.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 14).0 - 160.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 15).0 - 171.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 16).0 - 187.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 2).0 - 30.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 3).0 - 32.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 4).0 - 44.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 5).0 - 60.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 6).0 - 69.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 7).0 - 74.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 8).0 - 87.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-20.xml", "config-2.xml", 9).0 - 102.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 2).0 - 19.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 3).0 - 21.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 4).0 - 35.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 5).0 - 61.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 6).0 - 67.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 7).0 - 76.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 8).0 - 84.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-21.xml", "config-2.xml", 9).0 - 91.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 10).0 - 115.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 11).0 - 126.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 12).0 - 138.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 13).0 - 147.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 14).0 - 155.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 15).0 - 172.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 16).0 - 184.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 17).0 - 192.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 2).0 - 10.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 3).0 - 21.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 4).0 - 26.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 5).0 - 41.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 6).0 - 48.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 7).0 - 67.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 8).0 - 75.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-22.xml", "config-2.xml", 9).0 - 95.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 10).0 - 113.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 11).0 - 131.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 2).0 - 30.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 3).0 - 39.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 4).0 - 56.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 5).0 - 66.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 6).0 - 79.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 7).0 - 82.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 8).0 - 86.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-23.xml", "config-2.xml", 9).0 - 99.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 10).0 - 81.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 11).0 - 103.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 12).0 - 118.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 13).0 - 127.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 14).0 - 139.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 15).0 - 146.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 16).0 - 152.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 17).0 - 157.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 2).0 - 15.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 3).0 - 22.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 4).0 - 26.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 5).0 - 30.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 6).0 - 36.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 7).0 - 42.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 8).0 - 58.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-24.xml", "config-2.xml", 9).0 - 60.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 10).0 - 109.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 11).0 - 130.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 12).0 - 140.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 2).0 - 17.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 3).0 - 29.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 4).0 - 47.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 5).0 - 49.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 6).0 - 58.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 7).0 - 68.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 8).0 - 77.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-25.xml", "config-2.xml", 9).0 - 96.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 10).0 - 141.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 11).0 - 151.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 12).0 - 168.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 2).0 - 27.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 3).0 - 36.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 4).0 - 52.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 5).0 - 63.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 6).0 - 82.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 7).0 - 97.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 8).0 - 111.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-3.xml", "config-2.xml", 9).0 - 128.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 10).0 - 135.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 11).0 - 145.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 12).0 - 156.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 13).0 - 171.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 2).0 - 25.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 3).0 - 36.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 4).0 - 52.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 5).0 - 55.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 6).0 - 79.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 7).0 - 88.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 8).0 - 101.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-4.xml", "config-2.xml", 9).0 - 120.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 10).0 - 105.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 11).0 - 125.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 12).0 - 136.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 13).0 - 146.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 2).0 - 20.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 3).0 - 31.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 4).0 - 50.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 5).0 - 52.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 6).0 - 63.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 7).0 - 72.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 8).0 - 90.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-5.xml", "config-2.xml", 9).0 - 94.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 10).0 - 119.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 11).0 - 135.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 12).0 - 157.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 13).0 - 159.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 14).0 - 169.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 15).0 - 183.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 16).0 - 185.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 17).0 - 200.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 18).0 - 211.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_19() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 19).0 - 214.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 2).0 - 17.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 3).0 - 36.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 4).0 - 41.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 5).0 - 58.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 6).0 - 74.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 7).0 - 84.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 8).0 - 92.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-6.xml", "config-2.xml", 9).0 - 107.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 10).0 - 102.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 11).0 - 106.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 12).0 - 112.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 13).0 - 120.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 14).0 - 129.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 15).0 - 135.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 16).0 - 146.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 17).0 - 156.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 18).0 - 171.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_19() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 19).0 - 183.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 2).0 - 21.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_20() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 20).0 - 190.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_21() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 21).0 - 206.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_22() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 22).0 - 214.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_23() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 23).0 - 222.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_24() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 24).0 - 227.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_25() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 25).0 - 236.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_26() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 26).0 - 254.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_27() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 27).0 - 274.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 3).0 - 41.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 4).0 - 45.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 5).0 - 51.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 6).0 - 63.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 7).0 - 73.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 8).0 - 77.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-7.xml", "config-2.xml", 9).0 - 97.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 10).0 - 109.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 11).0 - 115.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 2).0 - 13.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 3).0 - 25.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 4).0 - 31.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 5).0 - 42.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 6).0 - 63.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 7).0 - 66.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 8).0 - 81.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-8.xml", "config-2.xml", 9).0 - 91.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_10() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 10).0 - 96.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_11() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 11).0 - 108.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_12() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 12).0 - 124.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_13() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 13).0 - 132.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_14() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 14).0 - 151.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_15() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 15).0 - 158.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_16() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 16).0 - 173.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_17() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 17).0 - 189.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_18() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 18).0 - 198.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_19() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 19).0 - 212.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_2() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 2).0 - 21.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_20() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 20).0 - 224.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_21() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 21).0 - 236.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_22() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 22).0 - 242.131).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_3() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 3).0 - 25.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_4() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 4).0 - 32.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_5() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 5).0 - 43.0).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_6() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 6).0 - 48.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_7() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 7).0 - 65.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_8() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 8).0 - 80.1314).abs() < 1e-2);
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_9() {
    assert!((solve("empty-16-16-random", "empty-16-16-random-9.xml", "config-2.xml", 9).0 - 81.1314).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 10).0 - 1087.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 11).0 - 1148.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 12).0 - 1341.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 13).0 - 1399.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 14).0 - 1494.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 15).0 - 1601.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 16).0 - 1699.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 17).0 - 1840.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 18).0 - 1911.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 2).0 - 108.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 3).0 - 280.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 4).0 - 334.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 5).0 - 378.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 6).0 - 483.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 7).0 - 659.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 8).0 - 757.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-1.xml", "config-2.xml", 9).0 - 908.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 10).0 - 1074.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 11).0 - 1138.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 12).0 - 1360.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 13).0 - 1430.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 14).0 - 1482.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 15).0 - 1597.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 16).0 - 1628.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 17).0 - 1781.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 18).0 - 1862.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 19).0 - 2029.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 2).0 - 122.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 20).0 - 2197.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 21).0 - 2322.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 22).0 - 2403.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 23).0 - 2452.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 24).0 - 2539.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 25).0 - 2683.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 26).0 - 2793.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 3).0 - 283.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 4).0 - 375.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 5).0 - 410.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 6).0 - 495.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 7).0 - 684.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 8).0 - 750.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-10.xml", "config-2.xml", 9).0 - 924.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 10).0 - 935.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 11).0 - 995.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 12).0 - 1098.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 13).0 - 1144.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 14).0 - 1183.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 15).0 - 1292.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 16).0 - 1319.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 17).0 - 1478.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 18).0 - 1559.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 19).0 - 1728.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 2).0 - 80.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 20).0 - 1879.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 21).0 - 2040.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 22).0 - 2081.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 23).0 - 2157.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 24).0 - 2240.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 25).0 - 2353.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 3).0 - 231.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 4).0 - 298.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 5).0 - 418.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 6).0 - 505.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 7).0 - 682.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 8).0 - 759.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-11.xml", "config-2.xml", 9).0 - 871.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 10).0 - 1002.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 11).0 - 1042.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 12).0 - 1156.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 13).0 - 1190.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 14).0 - 1321.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 15).0 - 1438.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 16).0 - 1465.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 17).0 - 1580.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 18).0 - 1645.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 19).0 - 1813.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 2).0 - 69.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 20).0 - 1979.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 21).0 - 2164.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 22).0 - 2239.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 23).0 - 2295.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 24).0 - 2340.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 25).0 - 2530.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 26).0 - 2654.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 27).0 - 2722.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 28).0 - 2810.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 3).0 - 231.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 4).0 - 281.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 5).0 - 319.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 6).0 - 440.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 7).0 - 629.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 8).0 - 696.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-12.xml", "config-2.xml", 9).0 - 835.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 10).0 - 974.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 2).0 - 83.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 3).0 - 269.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 4).0 - 341.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 5).0 - 399.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 6).0 - 479.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 7).0 - 667.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 8).0 - 732.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-13.xml", "config-2.xml", 9).0 - 853.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 10).0 - 1154.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 2).0 - 65.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 3).0 - 288.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 4).0 - 424.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 5).0 - 482.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 6).0 - 591.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 7).0 - 779.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 8).0 - 879.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-14.xml", "config-2.xml", 9).0 - 1001.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 10).0 - 927.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 11).0 - 950.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 12).0 - 1009.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 13).0 - 1107.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 14).0 - 1160.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 15).0 - 1257.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 16).0 - 1276.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 17).0 - 1479.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 18).0 - 1585.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 19).0 - 1659.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 2).0 - 176.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 20).0 - 1775.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 3).0 - 303.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 4).0 - 331.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 5).0 - 498.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 6).0 - 552.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 7).0 - 711.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 8).0 - 760.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-15.xml", "config-2.xml", 9).0 - 838.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 10).0 - 930.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 11).0 - 977.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 12).0 - 1108.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 13).0 - 1151.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 14).0 - 1308.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 15).0 - 1431.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 16).0 - 1493.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 17).0 - 1648.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 18).0 - 1716.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 19).0 - 1884.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 2).0 - 85.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 20).0 - 2059.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 21).0 - 2165.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 22).0 - 2239.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 23).0 - 2311.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 24).0 - 2422.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 25).0 - 2568.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 26).0 - 2673.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 27).0 - 2725.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 28).0 - 2808.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 29).0 - 2918.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 3).0 - 252.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_30() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 30).0 - 3070.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_31() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 31).0 - 3177.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_32() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 32).0 - 3227.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_33() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 33).0 - 3314.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_34() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 34).0 - 3368.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_35() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 35).0 - 3424.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 4).0 - 318.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 5).0 - 355.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 6).0 - 447.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 7).0 - 634.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 8).0 - 706.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-16.xml", "config-2.xml", 9).0 - 851.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 2).0 - 113.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 3).0 - 264.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 4).0 - 317.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 5).0 - 379.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 6).0 - 499.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 7).0 - 664.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 8).0 - 724.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-17.xml", "config-2.xml", 9).0 - 890.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 10).0 - 913.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 11).0 - 991.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 12).0 - 1166.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 13).0 - 1203.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 14).0 - 1239.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 15).0 - 1335.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 16).0 - 1453.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 17).0 - 1564.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 18).0 - 1722.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 19).0 - 1818.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 2).0 - 72.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 20).0 - 1939.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 21).0 - 2075.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 22).0 - 2142.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 23).0 - 2171.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 24).0 - 2227.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 25).0 - 2362.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 26).0 - 2448.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 27).0 - 2514.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 28).0 - 2572.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 29).0 - 2667.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 3).0 - 238.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 4).0 - 269.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 5).0 - 300.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 6).0 - 347.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 7).0 - 529.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 8).0 - 692.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-18.xml", "config-2.xml", 9).0 - 795.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 10).0 - 1038.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 11).0 - 1080.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 12).0 - 1202.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 13).0 - 1265.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 14).0 - 1314.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 15).0 - 1428.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 16).0 - 1450.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 17).0 - 1596.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 18).0 - 1664.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 19).0 - 1783.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 2).0 - 97.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 20).0 - 1934.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 21).0 - 2043.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 22).0 - 2101.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 23).0 - 2146.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 24).0 - 2230.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 25).0 - 2371.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 26).0 - 2516.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 27).0 - 2588.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 28).0 - 2660.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 29).0 - 2747.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 3).0 - 266.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_30() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 30).0 - 2917.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_31() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 31).0 - 3047.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_32() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 32).0 - 3087.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_33() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 33).0 - 3175.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_34() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 34).0 - 3212.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_35() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 35).0 - 3302.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_36() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 36).0 - 3469.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_37() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 37).0 - 3565.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_38() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 38).0 - 3616.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_39() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 39).0 - 3764.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 4).0 - 325.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_40() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 40).0 - 3937.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_41() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 41).0 - 4036.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_42() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 42).0 - 4212.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_43() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 43).0 - 4318.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_44() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 44).0 - 4344.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_45() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 45).0 - 4356.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_46() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 46).0 - 4377.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_47() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 47).0 - 4413.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_48() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 48).0 - 4482.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_49() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 49).0 - 4604.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 5).0 - 366.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_50() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 50).0 - 4645.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 6).0 - 476.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 7).0 - 658.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 8).0 - 737.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-19.xml", "config-2.xml", 9).0 - 881.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 10).0 - 1010.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 11).0 - 1142.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 12).0 - 1238.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 13).0 - 1284.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 14).0 - 1425.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 15).0 - 1501.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 16).0 - 1606.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 17).0 - 1679.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 18).0 - 1874.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 19).0 - 2042.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 2).0 - 102.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 20).0 - 2170.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 21).0 - 2232.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 22).0 - 2297.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 23).0 - 2345.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 24).0 - 2391.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 25).0 - 2483.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 26).0 - 2569.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 27).0 - 2605.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 28).0 - 2655.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 29).0 - 2746.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 3).0 - 207.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_30() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 30).0 - 2881.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_31() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 31).0 - 2946.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_32() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 32).0 - 3028.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_33() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 33).0 - 3094.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_34() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 34).0 - 3170.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_35() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 35).0 - 3283.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_36() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 36).0 - 3365.13).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 4).0 - 342.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 5).0 - 525.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 6).0 - 577.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 7).0 - 702.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 8).0 - 796.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-2.xml", "config-2.xml", 9).0 - 873.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 10).0 - 1032.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 11).0 - 1176.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 12).0 - 1268.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 13).0 - 1353.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 14).0 - 1399.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 15).0 - 1494.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 16).0 - 1616.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 17).0 - 1740.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 18).0 - 1827.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 19).0 - 1973.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 2).0 - 66.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 20).0 - 2104.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 21).0 - 2197.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 22).0 - 2271.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 23).0 - 2336.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 24).0 - 2430.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 25).0 - 2583.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 26).0 - 2747.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 27).0 - 2854.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 28).0 - 2930.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 3).0 - 213.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 4).0 - 307.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 5).0 - 364.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 6).0 - 493.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 7).0 - 685.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 8).0 - 752.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-20.xml", "config-2.xml", 9).0 - 898.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 10).0 - 1020.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 11).0 - 1151.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 12).0 - 1322.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 13).0 - 1376.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 14).0 - 1451.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 15).0 - 1577.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 16).0 - 1602.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 17).0 - 1758.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 18).0 - 1842.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 19).0 - 1970.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 2).0 - 78.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 20).0 - 2133.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 3).0 - 230.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 4).0 - 324.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 5).0 - 371.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 6).0 - 505.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 7).0 - 680.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 8).0 - 763.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-21.xml", "config-2.xml", 9).0 - 880.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 10).0 - 1038.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 11).0 - 1078.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 12).0 - 1225.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 13).0 - 1293.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 14).0 - 1336.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 15).0 - 1434.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 16).0 - 1467.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 17).0 - 1594.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 18).0 - 1667.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 19).0 - 1802.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 2).0 - 64.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 20).0 - 1974.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 21).0 - 2072.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 22).0 - 2137.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 23).0 - 2175.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 24).0 - 2279.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 25).0 - 2411.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 26).0 - 2505.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 27).0 - 2615.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 28).0 - 2678.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 29).0 - 2769.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 3).0 - 250.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 4).0 - 344.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 5).0 - 385.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 6).0 - 509.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 7).0 - 709.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 8).0 - 823.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-22.xml", "config-2.xml", 9).0 - 944.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 10).0 - 1095.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 11).0 - 1139.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 2).0 - 88.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 3).0 - 244.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 4).0 - 296.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 5).0 - 402.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 6).0 - 535.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 7).0 - 732.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 8).0 - 794.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-23.xml", "config-2.xml", 9).0 - 932.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 10).0 - 1150.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 11).0 - 1219.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 12).0 - 1333.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 13).0 - 1378.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 14).0 - 1437.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 15).0 - 1551.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 16).0 - 1644.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 17).0 - 1806.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 18).0 - 1898.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 19).0 - 2030.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 2).0 - 77.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 20).0 - 2220.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 21).0 - 2413.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 22).0 - 2484.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 23).0 - 2535.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 24).0 - 2620.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 25).0 - 2814.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 26).0 - 2994.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 27).0 - 3077.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 3).0 - 246.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 4).0 - 391.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 5).0 - 493.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 6).0 - 582.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 7).0 - 755.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 8).0 - 854.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-24.xml", "config-2.xml", 9).0 - 1021.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 10).0 - 1153.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 11).0 - 1208.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 12).0 - 1321.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 13).0 - 1408.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 14).0 - 1464.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 15).0 - 1594.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 16).0 - 1702.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 17).0 - 1833.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 18).0 - 1907.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 19).0 - 2094.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 2).0 - 98.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 20).0 - 2268.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 21).0 - 2451.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 22).0 - 2532.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 3).0 - 274.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 4).0 - 344.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 5).0 - 404.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 6).0 - 542.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 7).0 - 731.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 8).0 - 880.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-25.xml", "config-2.xml", 9).0 - 1038.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 10).0 - 1126.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 2).0 - 108.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 3).0 - 269.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 4).0 - 374.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 5).0 - 471.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 6).0 - 561.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 7).0 - 745.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 8).0 - 894.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-3.xml", "config-2.xml", 9).0 - 1024.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 10).0 - 981.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 11).0 - 1025.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 12).0 - 1137.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 13).0 - 1189.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 14).0 - 1309.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 15).0 - 1414.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 16).0 - 1448.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 17).0 - 1630.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 18).0 - 1700.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 19).0 - 1827.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 2).0 - 87.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 20).0 - 1999.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 21).0 - 2160.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 22).0 - 2239.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 23).0 - 2294.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 24).0 - 2362.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 25).0 - 2515.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 26).0 - 2629.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 27).0 - 2693.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 28).0 - 2750.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 29).0 - 2871.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 3).0 - 257.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 4).0 - 307.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 5).0 - 354.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 6).0 - 446.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 7).0 - 637.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 8).0 - 717.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-4.xml", "config-2.xml", 9).0 - 838.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 10).0 - 1231.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 11).0 - 1308.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 12).0 - 1440.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 13).0 - 1515.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 14).0 - 1659.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 15).0 - 1758.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 16).0 - 1836.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 17).0 - 1976.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 18).0 - 2054.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 19).0 - 2187.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 2).0 - 118.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 20).0 - 2352.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 21).0 - 2499.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 22).0 - 2562.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 23).0 - 2652.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 24).0 - 2729.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 25).0 - 2879.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 3).0 - 279.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 4).0 - 427.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 5).0 - 544.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 6).0 - 670.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 7).0 - 864.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 8).0 - 962.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-5.xml", "config-2.xml", 9).0 - 1112.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 10).0 - 946.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 11).0 - 1006.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 12).0 - 1195.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 13).0 - 1270.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 14).0 - 1351.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 15).0 - 1434.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 16).0 - 1538.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 17).0 - 1736.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 18).0 - 1876.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 19).0 - 1971.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 2).0 - 77.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 20).0 - 2097.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 3).0 - 259.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 4).0 - 294.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 5).0 - 429.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 6).0 - 504.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 7).0 - 652.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 8).0 - 732.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-6.xml", "config-2.xml", 9).0 - 851.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 10).0 - 1190.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 11).0 - 1216.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 12).0 - 1328.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 13).0 - 1359.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 14).0 - 1400.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 15).0 - 1462.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 16).0 - 1551.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 17).0 - 1642.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 18).0 - 1678.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 19).0 - 1768.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 2).0 - 251.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 20).0 - 1863.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 21).0 - 1925.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_22() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 22).0 - 1983.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_23() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 23).0 - 2036.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_24() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 24).0 - 2074.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_25() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 25).0 - 2163.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_26() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 26).0 - 2286.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_27() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 27).0 - 2326.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_28() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 28).0 - 2544.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_29() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 29).0 - 2618.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 3).0 - 409.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_30() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 30).0 - 2736.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_31() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 31).0 - 2904.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_32() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 32).0 - 2992.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_33() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 33).0 - 3044.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 4).0 - 533.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 5).0 - 591.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 6).0 - 684.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 7).0 - 789.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 8).0 - 941.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-7.xml", "config-2.xml", 9).0 - 1069.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 10).0 - 897.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 11).0 - 937.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 12).0 - 1049.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 13).0 - 1138.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 14).0 - 1178.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 15).0 - 1260.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 16).0 - 1342.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 17).0 - 1504.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 18).0 - 1578.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 19).0 - 1706.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 2).0 - 100.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 3).0 - 284.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 4).0 - 321.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 5).0 - 395.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 6).0 - 465.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 7).0 - 636.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 8).0 - 691.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-8.xml", "config-2.xml", 9).0 - 799.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_10() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 10).0 - 1062.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_11() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 11).0 - 1175.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_12() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 12).0 - 1286.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_13() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 13).0 - 1329.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_14() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 14).0 - 1370.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_15() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 15).0 - 1498.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_16() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 16).0 - 1591.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_17() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 17).0 - 1715.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_18() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 18).0 - 1803.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_19() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 19).0 - 1933.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_2() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 2).0 - 167.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_20() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 20).0 - 2084.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_21() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 21).0 - 2188.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_3() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 3).0 - 353.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_4() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 4).0 - 414.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_5() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 5).0 - 460.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_6() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 6).0 - 589.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_7() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 7).0 - 770.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_8() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 8).0 - 857.0).abs() < 1e-2);
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_9() {
    assert!((solve("warehouse-10-20-10-2-2_random", "warehouse-10-20-10-2-2-random-9.xml", "config-2.xml", 9).0 - 977.0).abs() < 1e-2);
}
