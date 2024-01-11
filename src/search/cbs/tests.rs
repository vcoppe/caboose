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
    assert!(
        (solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 10).0 - 1968.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_10_5() {
    assert!(
        (solve("den520d_random", "den520d-random-10.xml", "config-2.xml", 5).0 - 922.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_11_3() {
    assert!(
        (solve("den520d_random", "den520d-random-11.xml", "config-2.xml", 3).0 - 750.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_12_10() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-12.xml",
            "config-2.xml",
            10
        )
        .0 - 1948.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_13_17() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-13.xml",
            "config-2.xml",
            17
        )
        .0 - 2660.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_14_25() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-14.xml",
            "config-2.xml",
            25
        )
        .0 - 3577.13)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_15_16() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-15.xml",
            "config-2.xml",
            16
        )
        .0 - 3163.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_16_19() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-16.xml",
            "config-2.xml",
            19
        )
        .0 - 3355.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_17_8() {
    assert!(
        (solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 8).0 - 1853.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_18_11() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-18.xml",
            "config-2.xml",
            11
        )
        .0 - 2080.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_19_12() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-19.xml",
            "config-2.xml",
            12
        )
        .0 - 1887.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_20_9() {
    assert!(
        (solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 9).0 - 1619.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_21_14() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-21.xml",
            "config-2.xml",
            14
        )
        .0 - 2655.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_22_12() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-22.xml",
            "config-2.xml",
            12
        )
        .0 - 1625.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_23_15() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-23.xml",
            "config-2.xml",
            15
        )
        .0 - 2159.13)
            .abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_24_5() {
    assert!(
        (solve("den520d_random", "den520d-random-24.xml", "config-2.xml", 5).0 - 1003.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_25_6() {
    assert!(
        (solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 6).0 - 1403.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_3_5() {
    assert!(
        (solve("den520d_random", "den520d-random-3.xml", "config-2.xml", 5).0 - 890.0).abs() < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_4_10() {
    assert!(
        (solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 10).0 - 1768.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_5_18() {
    assert!(
        (solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 18).0 - 2781.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_6_3() {
    assert!(
        (solve("den520d_random", "den520d-random-6.xml", "config-2.xml", 3).0 - 546.0).abs() < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_7_11() {
    assert!(
        (solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 11).0 - 2235.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_8_16() {
    assert!(
        (solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 16).0 - 2025.0).abs()
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_9_17() {
    assert!(
        (solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 17).0 - 2718.0).abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_1_23() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-1.xml",
            "config-2.xml",
            23
        )
        .0 - 214.394)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_10_18() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-10.xml",
            "config-2.xml",
            18
        )
        .0 - 209.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_11_19() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-11.xml",
            "config-2.xml",
            19
        )
        .0 - 165.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_12_6() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-12.xml",
            "config-2.xml",
            6
        )
        .0 - 59.2627)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_11() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-13.xml",
            "config-2.xml",
            11
        )
        .0 - 136.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_14_16() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-14.xml",
            "config-2.xml",
            16
        )
        .0 - 164.263)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_15_13() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-15.xml",
            "config-2.xml",
            13
        )
        .0 - 120.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_16() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-16.xml",
            "config-2.xml",
            16
        )
        .0 - 124.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_11() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-17.xml",
            "config-2.xml",
            11
        )
        .0 - 113.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_18_20() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-18.xml",
            "config-2.xml",
            20
        )
        .0 - 228.394)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_19_15() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-19.xml",
            "config-2.xml",
            15
        )
        .0 - 192.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_2_15() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-2.xml",
            "config-2.xml",
            15
        )
        .0 - 173.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_20_16() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-20.xml",
            "config-2.xml",
            16
        )
        .0 - 187.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_21_9() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-21.xml",
            "config-2.xml",
            9
        )
        .0 - 91.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_22_17() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-22.xml",
            "config-2.xml",
            17
        )
        .0 - 192.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_23_11() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-23.xml",
            "config-2.xml",
            11
        )
        .0 - 131.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_24_17() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-24.xml",
            "config-2.xml",
            17
        )
        .0 - 157.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_12() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-3.xml",
            "config-2.xml",
            12
        )
        .0 - 168.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_4_13() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-4.xml",
            "config-2.xml",
            13
        )
        .0 - 171.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_5_13() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-5.xml",
            "config-2.xml",
            13
        )
        .0 - 146.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_6_19() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-6.xml",
            "config-2.xml",
            19
        )
        .0 - 214.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_7_27() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-7.xml",
            "config-2.xml",
            27
        )
        .0 - 274.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_8_11() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-8.xml",
            "config-2.xml",
            11
        )
        .0 - 115.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_22() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-9.xml",
            "config-2.xml",
            22
        )
        .0 - 242.131)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_1_18() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-1.xml",
            "config-2.xml",
            18
        )
        .0 - 1911.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_10_26() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-10.xml",
            "config-2.xml",
            26
        )
        .0 - 2793.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_11_25() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-11.xml",
            "config-2.xml",
            25
        )
        .0 - 2353.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_28() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-12.xml",
            "config-2.xml",
            28
        )
        .0 - 2810.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_13_10() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-13.xml",
            "config-2.xml",
            10
        )
        .0 - 974.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_14_10() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-14.xml",
            "config-2.xml",
            10
        )
        .0 - 1154.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_20() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-15.xml",
            "config-2.xml",
            20
        )
        .0 - 1775.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_35() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-16.xml",
            "config-2.xml",
            35
        )
        .0 - 3424.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_17_9() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-17.xml",
            "config-2.xml",
            9
        )
        .0 - 890.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_29() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-18.xml",
            "config-2.xml",
            29
        )
        .0 - 2667.13)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_50() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-19.xml",
            "config-2.xml",
            50
        )
        .0 - 4645.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_36() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-2.xml",
            "config-2.xml",
            36
        )
        .0 - 3365.13)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_20() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-21.xml",
            "config-2.xml",
            20
        )
        .0 - 2133.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_22_29() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-22.xml",
            "config-2.xml",
            29
        )
        .0 - 2769.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_11() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-23.xml",
            "config-2.xml",
            11
        )
        .0 - 1139.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_24_27() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-24.xml",
            "config-2.xml",
            27
        )
        .0 - 3077.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_25_22() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-25.xml",
            "config-2.xml",
            22
        )
        .0 - 2532.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_3_10() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-3.xml",
            "config-2.xml",
            10
        )
        .0 - 1126.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_29() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-4.xml",
            "config-2.xml",
            29
        )
        .0 - 2871.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_25() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-5.xml",
            "config-2.xml",
            25
        )
        .0 - 2879.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_6_20() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-6.xml",
            "config-2.xml",
            20
        )
        .0 - 2097.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_33() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-7.xml",
            "config-2.xml",
            33
        )
        .0 - 3044.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_19() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-8.xml",
            "config-2.xml",
            19
        )
        .0 - 1706.0)
            .abs()
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_21() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-9.xml",
            "config-2.xml",
            21
        )
        .0 - 2188.0)
            .abs()
            < 1e-2
    );
}
