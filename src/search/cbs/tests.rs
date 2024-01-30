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

    let (_, mut cbs, config, _) = get_cbs_from_files(&map, &task, &config, n_agents, 1);

    let solution = cbs.solve(&config).unwrap();

    solution.iter().map(|sol| sol.cost).sum()
}

#[test]
fn den520d_random_den520d_random_1_9() {
    assert!(
        (solve("den520d_random", "den520d-random-1.xml", "config-2.xml", 9).0 - 1754.0).abs()
            / 1754.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_10_5() {
    assert!(
        (solve("den520d_random", "den520d-random-10.xml", "config-2.xml", 5).0 - 922.0).abs()
            / 922.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_11_3() {
    assert!(
        (solve("den520d_random", "den520d-random-11.xml", "config-2.xml", 3).0 - 750.0).abs()
            / 750.0
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
            / 1948.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_13_15() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-13.xml",
            "config-2.xml",
            15
        )
        .0 - 2369.0)
            .abs()
            / 2369.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_14_22() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-14.xml",
            "config-2.xml",
            22
        )
        .0 - 3167.13)
            .abs()
            / 3167.13
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_15_14() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-15.xml",
            "config-2.xml",
            14
        )
        .0 - 2589.0)
            .abs()
            / 2589.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_16_10() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-16.xml",
            "config-2.xml",
            10
        )
        .0 - 1802.0)
            .abs()
            / 1802.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_17_5() {
    assert!(
        (solve("den520d_random", "den520d-random-17.xml", "config-2.xml", 5).0 - 1023.0).abs()
            / 1023.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_18_8() {
    assert!(
        (solve("den520d_random", "den520d-random-18.xml", "config-2.xml", 8).0 - 1491.0).abs()
            / 1491.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_19_11() {
    assert!(
        (solve(
            "den520d_random",
            "den520d-random-19.xml",
            "config-2.xml",
            11
        )
        .0 - 1733.0)
            .abs()
            / 1733.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_2_14() {
    assert!(
        (solve("den520d_random", "den520d-random-2.xml", "config-2.xml", 14).0 - 2329.0).abs()
            / 2329.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_20_9() {
    assert!(
        (solve("den520d_random", "den520d-random-20.xml", "config-2.xml", 9).0 - 1619.0).abs()
            / 1619.0
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
            / 2655.0
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
            / 1625.0
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
            / 2159.13
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_24_5() {
    assert!(
        (solve("den520d_random", "den520d-random-24.xml", "config-2.xml", 5).0 - 1003.0).abs()
            / 1003.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_25_6() {
    assert!(
        (solve("den520d_random", "den520d-random-25.xml", "config-2.xml", 6).0 - 1403.0).abs()
            / 1403.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_3_5() {
    assert!(
        (solve("den520d_random", "den520d-random-3.xml", "config-2.xml", 5).0 - 890.0).abs()
            / 890.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_4_4() {
    assert!(
        (solve("den520d_random", "den520d-random-4.xml", "config-2.xml", 4).0 - 851.0).abs()
            / 851.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_5_14() {
    assert!(
        (solve("den520d_random", "den520d-random-5.xml", "config-2.xml", 14).0 - 2276.0).abs()
            / 2276.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_6_3() {
    assert!(
        (solve("den520d_random", "den520d-random-6.xml", "config-2.xml", 3).0 - 546.0).abs()
            / 546.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_7_5() {
    assert!(
        (solve("den520d_random", "den520d-random-7.xml", "config-2.xml", 5).0 - 827.0).abs()
            / 827.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_8_9() {
    assert!(
        (solve("den520d_random", "den520d-random-8.xml", "config-2.xml", 9).0 - 1056.0).abs()
            / 1056.0
            < 1e-2
    );
}

#[test]
fn den520d_random_den520d_random_9_14() {
    assert!(
        (solve("den520d_random", "den520d-random-9.xml", "config-2.xml", 14).0 - 2133.0).abs()
            / 2133.0
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_10_task_16() {
    assert!(
        (solve("roadmaps/dense", "10_task.xml", "config-2.xml", 16).0 - 2562.67).abs() / 2562.67
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_11_task_9() {
    assert!(
        (solve("roadmaps/dense", "11_task.xml", "config-2.xml", 9).0 - 1367.21).abs() / 1367.21
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_12_task_19() {
    assert!(
        (solve("roadmaps/dense", "12_task.xml", "config-2.xml", 19).0 - 2675.34).abs() / 2675.34
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_13_task_14() {
    assert!(
        (solve("roadmaps/dense", "13_task.xml", "config-2.xml", 14).0 - 2146.74).abs() / 2146.74
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_14_task_10() {
    assert!(
        (solve("roadmaps/dense", "14_task.xml", "config-2.xml", 10).0 - 1326.93).abs() / 1326.93
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_15_task_10() {
    assert!(
        (solve("roadmaps/dense", "15_task.xml", "config-2.xml", 10).0 - 1537.36).abs() / 1537.36
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_16_task_16() {
    assert!(
        (solve("roadmaps/dense", "16_task.xml", "config-2.xml", 16).0 - 2069.25).abs() / 2069.25
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_17_task_10() {
    assert!(
        (solve("roadmaps/dense", "17_task.xml", "config-2.xml", 10).0 - 1122.02).abs() / 1122.02
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_18_task_15() {
    assert!(
        (solve("roadmaps/dense", "18_task.xml", "config-2.xml", 15).0 - 2144.63).abs() / 2144.63
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_19_task_11() {
    assert!(
        (solve("roadmaps/dense", "19_task.xml", "config-2.xml", 11).0 - 1380.95).abs() / 1380.95
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_1_task_11() {
    assert!(
        (solve("roadmaps/dense", "1_task.xml", "config-2.xml", 11).0 - 1497.06).abs() / 1497.06
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_20_task_11() {
    assert!(
        (solve("roadmaps/dense", "20_task.xml", "config-2.xml", 11).0 - 1962.32).abs() / 1962.32
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_21_task_3() {
    assert!(
        (solve("roadmaps/dense", "21_task.xml", "config-2.xml", 3).0 - 492.74).abs() / 492.74
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_22_task_16() {
    assert!(
        (solve("roadmaps/dense", "22_task.xml", "config-2.xml", 16).0 - 1779.8).abs() / 1779.8
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_23_task_8() {
    assert!(
        (solve("roadmaps/dense", "23_task.xml", "config-2.xml", 8).0 - 1116.13).abs() / 1116.13
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_24_task_5() {
    assert!(
        (solve("roadmaps/dense", "24_task.xml", "config-2.xml", 5).0 - 700.113).abs() / 700.113
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_25_task_10() {
    assert!(
        (solve("roadmaps/dense", "25_task.xml", "config-2.xml", 10).0 - 1819.37).abs() / 1819.37
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_2_task_13() {
    assert!(
        (solve("roadmaps/dense", "2_task.xml", "config-2.xml", 13).0 - 2277.92).abs() / 2277.92
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_3_task_9() {
    assert!(
        (solve("roadmaps/dense", "3_task.xml", "config-2.xml", 9).0 - 1256.68).abs() / 1256.68
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_4_task_14() {
    assert!(
        (solve("roadmaps/dense", "4_task.xml", "config-2.xml", 14).0 - 1785.15).abs() / 1785.15
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_5_task_14() {
    assert!(
        (solve("roadmaps/dense", "5_task.xml", "config-2.xml", 14).0 - 1589.31).abs() / 1589.31
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_6_task_10() {
    assert!(
        (solve("roadmaps/dense", "6_task.xml", "config-2.xml", 10).0 - 1266.52).abs() / 1266.52
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_7_task_4() {
    assert!(
        (solve("roadmaps/dense", "7_task.xml", "config-2.xml", 4).0 - 611.161).abs() / 611.161
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_8_task_8() {
    assert!(
        (solve("roadmaps/dense", "8_task.xml", "config-2.xml", 8).0 - 1112.13).abs() / 1112.13
            < 1e-2
    );
}

#[test]
fn roadmaps_dense_9_task_9() {
    assert!(
        (solve("roadmaps/dense", "9_task.xml", "config-2.xml", 9).0 - 1716.9).abs() / 1716.9 < 1e-2
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
            / 214.394
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
            / 209.131
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
            / 165.131
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
            / 59.2627
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_13_7() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-13.xml",
            "config-2.xml",
            7
        )
        .0 - 102.0)
            .abs()
            / 102.0
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
            / 164.263
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
            / 120.131
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_16_11() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-16.xml",
            "config-2.xml",
            11
        )
        .0 - 88.1314)
            .abs()
            / 88.1314
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_17_8() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-17.xml",
            "config-2.xml",
            8
        )
        .0 - 75.1314)
            .abs()
            / 75.1314
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
            / 228.394
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
            / 192.0
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
            / 173.0
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
            / 187.0
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
            / 91.0
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
            / 192.131
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
            / 131.131
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
            / 157.131
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_25_11() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-25.xml",
            "config-2.xml",
            11
        )
        .0 - 130.131)
            .abs()
            / 130.131
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_3_10() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-3.xml",
            "config-2.xml",
            10
        )
        .0 - 141.0)
            .abs()
            / 141.0
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
            / 171.0
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
            / 146.131
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
            / 214.131
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
            / 274.0
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
            / 115.131
            < 1e-2
    );
}

#[test]
fn empty_16_16_random_empty_16_16_random_9_15() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-9.xml",
            "config-2.xml",
            15
        )
        .0 - 158.131)
            .abs()
            / 158.131
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_10_task_11() {
    assert!(
        (solve("roadmaps/sparse", "10_task.xml", "config-2.xml", 11).0 - 1913.41).abs() / 1913.41
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_11_task_14() {
    assert!(
        (solve("roadmaps/sparse", "11_task.xml", "config-2.xml", 14).0 - 3028.45).abs() / 3028.45
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_12_task_11() {
    assert!(
        (solve("roadmaps/sparse", "12_task.xml", "config-2.xml", 11).0 - 2225.12).abs() / 2225.12
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_13_task_17() {
    assert!(
        (solve("roadmaps/sparse", "13_task.xml", "config-2.xml", 17).0 - 3110.4).abs() / 3110.4
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_14_task_10() {
    assert!(
        (solve("roadmaps/sparse", "14_task.xml", "config-2.xml", 10).0 - 2485.15).abs() / 2485.15
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_15_task_19() {
    assert!(
        (solve("roadmaps/sparse", "15_task.xml", "config-2.xml", 19).0 - 3643.1).abs() / 3643.1
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_16_task_15() {
    assert!(
        (solve("roadmaps/sparse", "16_task.xml", "config-2.xml", 15).0 - 3250.03).abs() / 3250.03
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_17_task_17() {
    assert!(
        (solve("roadmaps/sparse", "17_task.xml", "config-2.xml", 17).0 - 4175.64).abs() / 4175.64
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_18_task_7() {
    assert!(
        (solve("roadmaps/sparse", "18_task.xml", "config-2.xml", 7).0 - 1416.24).abs() / 1416.24
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_19_task_14() {
    assert!(
        (solve("roadmaps/sparse", "19_task.xml", "config-2.xml", 14).0 - 2971.03).abs() / 2971.03
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_1_task_12() {
    assert!(
        (solve("roadmaps/sparse", "1_task.xml", "config-2.xml", 12).0 - 2071.6).abs() / 2071.6
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_20_task_4() {
    assert!(
        (solve("roadmaps/sparse", "20_task.xml", "config-2.xml", 4).0 - 1251.01).abs() / 1251.01
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_21_task_12() {
    assert!(
        (solve("roadmaps/sparse", "21_task.xml", "config-2.xml", 12).0 - 2598.4).abs() / 2598.4
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_22_task_9() {
    assert!(
        (solve("roadmaps/sparse", "22_task.xml", "config-2.xml", 9).0 - 2595.34).abs() / 2595.34
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_23_task_13() {
    assert!(
        (solve("roadmaps/sparse", "23_task.xml", "config-2.xml", 13).0 - 3459.67).abs() / 3459.67
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_24_task_16() {
    assert!(
        (solve("roadmaps/sparse", "24_task.xml", "config-2.xml", 16).0 - 3411.86).abs() / 3411.86
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_25_task_8() {
    assert!(
        (solve("roadmaps/sparse", "25_task.xml", "config-2.xml", 8).0 - 1798.88).abs() / 1798.88
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_2_task_11() {
    assert!(
        (solve("roadmaps/sparse", "2_task.xml", "config-2.xml", 11).0 - 2003.75).abs() / 2003.75
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_3_task_17() {
    assert!(
        (solve("roadmaps/sparse", "3_task.xml", "config-2.xml", 17).0 - 2403.08).abs() / 2403.08
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_4_task_14() {
    assert!(
        (solve("roadmaps/sparse", "4_task.xml", "config-2.xml", 14).0 - 2109.17).abs() / 2109.17
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_5_task_5() {
    assert!(
        (solve("roadmaps/sparse", "5_task.xml", "config-2.xml", 5).0 - 1010.23).abs() / 1010.23
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_6_task_15() {
    assert!(
        (solve("roadmaps/sparse", "6_task.xml", "config-2.xml", 15).0 - 2573.07).abs() / 2573.07
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_7_task_11() {
    assert!(
        (solve("roadmaps/sparse", "7_task.xml", "config-2.xml", 11).0 - 2734.26).abs() / 2734.26
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_8_task_14() {
    assert!(
        (solve("roadmaps/sparse", "8_task.xml", "config-2.xml", 14).0 - 3082.52).abs() / 3082.52
            < 1e-2
    );
}

#[test]
fn roadmaps_sparse_9_task_19() {
    assert!(
        (solve("roadmaps/sparse", "9_task.xml", "config-2.xml", 19).0 - 4789.15).abs() / 4789.15
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_10_task_5() {
    assert!(
        (solve("roadmaps/super-dense", "10_task.xml", "config-2.xml", 5).0 - 486.929).abs()
            / 486.929
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_11_task_8() {
    assert!(
        (solve("roadmaps/super-dense", "11_task.xml", "config-2.xml", 8).0 - 904.465).abs()
            / 904.465
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_12_task_3() {
    assert!(
        (solve("roadmaps/super-dense", "12_task.xml", "config-2.xml", 3).0 - 507.828).abs()
            / 507.828
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_13_task_9() {
    assert!(
        (solve("roadmaps/super-dense", "13_task.xml", "config-2.xml", 9).0 - 1325.11).abs()
            / 1325.11
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_14_task_6() {
    assert!(
        (solve("roadmaps/super-dense", "14_task.xml", "config-2.xml", 6).0 - 1047.62).abs()
            / 1047.62
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_15_task_9() {
    assert!(
        (solve("roadmaps/super-dense", "15_task.xml", "config-2.xml", 9).0 - 1107.76).abs()
            / 1107.76
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_16_task_4() {
    assert!(
        (solve("roadmaps/super-dense", "16_task.xml", "config-2.xml", 4).0 - 486.16).abs() / 486.16
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_17_task_6() {
    assert!(
        (solve("roadmaps/super-dense", "17_task.xml", "config-2.xml", 6).0 - 1042.31).abs()
            / 1042.31
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_18_task_2() {
    assert!(
        (solve("roadmaps/super-dense", "18_task.xml", "config-2.xml", 2).0 - 127.458).abs()
            / 127.458
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_19_task_6() {
    assert!(
        (solve("roadmaps/super-dense", "19_task.xml", "config-2.xml", 6).0 - 676.237).abs()
            / 676.237
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_1_task_2() {
    assert!(
        (solve("roadmaps/super-dense", "1_task.xml", "config-2.xml", 2).0 - 270.928).abs()
            / 270.928
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_21_task_3() {
    assert!(
        (solve("roadmaps/super-dense", "21_task.xml", "config-2.xml", 3).0 - 368.855).abs()
            / 368.855
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_22_task_11() {
    assert!(
        (solve("roadmaps/super-dense", "22_task.xml", "config-2.xml", 11).0 - 1169.24).abs()
            / 1169.24
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_23_task_3() {
    assert!(
        (solve("roadmaps/super-dense", "23_task.xml", "config-2.xml", 3).0 - 660.246).abs()
            / 660.246
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_24_task_3() {
    assert!(
        (solve("roadmaps/super-dense", "24_task.xml", "config-2.xml", 3).0 - 416.385).abs()
            / 416.385
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_25_task_9() {
    assert!(
        (solve("roadmaps/super-dense", "25_task.xml", "config-2.xml", 9).0 - 1427.98).abs()
            / 1427.98
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_2_task_2() {
    assert!(
        (solve("roadmaps/super-dense", "2_task.xml", "config-2.xml", 2).0 - 120.464).abs()
            / 120.464
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_3_task_7() {
    assert!(
        (solve("roadmaps/super-dense", "3_task.xml", "config-2.xml", 7).0 - 831.063).abs()
            / 831.063
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_4_task_7() {
    assert!(
        (solve("roadmaps/super-dense", "4_task.xml", "config-2.xml", 7).0 - 939.471).abs()
            / 939.471
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_5_task_10() {
    assert!(
        (solve("roadmaps/super-dense", "5_task.xml", "config-2.xml", 10).0 - 1247.99).abs()
            / 1247.99
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_6_task_7() {
    assert!(
        (solve("roadmaps/super-dense", "6_task.xml", "config-2.xml", 7).0 - 825.722).abs()
            / 825.722
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_7_task_3() {
    assert!(
        (solve("roadmaps/super-dense", "7_task.xml", "config-2.xml", 3).0 - 269.109).abs()
            / 269.109
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_8_task_18() {
    assert!(
        (solve("roadmaps/super-dense", "8_task.xml", "config-2.xml", 18).0 - 1861.1).abs() / 1861.1
            < 1e-2
    );
}

#[test]
fn roadmaps_super_dense_9_task_3() {
    assert!(
        (solve("roadmaps/super-dense", "9_task.xml", "config-2.xml", 3).0 - 532.647).abs()
            / 532.647
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
            / 1911.0
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
            / 2793.0
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
            / 2353.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_12_15() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-12.xml",
            "config-2.xml",
            15
        )
        .0 - 1438.0)
            .abs()
            / 1438.0
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
            / 974.0
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
            / 1154.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_15_19() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-15.xml",
            "config-2.xml",
            19
        )
        .0 - 1659.0)
            .abs()
            / 1659.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_16_33() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-16.xml",
            "config-2.xml",
            33
        )
        .0 - 3314.0)
            .abs()
            / 3314.0
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
            / 890.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_18_17() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-18.xml",
            "config-2.xml",
            17
        )
        .0 - 1564.0)
            .abs()
            / 1564.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_19_29() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-19.xml",
            "config-2.xml",
            29
        )
        .0 - 2747.0)
            .abs()
            / 2747.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_2_21() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-2.xml",
            "config-2.xml",
            21
        )
        .0 - 2232.0)
            .abs()
            / 2232.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_20_28() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-20.xml",
            "config-2.xml",
            28
        )
        .0 - 2930.0)
            .abs()
            / 2930.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_21_16() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-21.xml",
            "config-2.xml",
            16
        )
        .0 - 1602.0)
            .abs()
            / 1602.0
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
            / 2769.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_23_6() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-23.xml",
            "config-2.xml",
            6
        )
        .0 - 535.0)
            .abs()
            / 535.0
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
            / 3077.0
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
            / 2532.0
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
            / 1126.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_4_19() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-4.xml",
            "config-2.xml",
            19
        )
        .0 - 1827.0)
            .abs()
            / 1827.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_5_22() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-5.xml",
            "config-2.xml",
            22
        )
        .0 - 2562.0)
            .abs()
            / 2562.0
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
            / 2097.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_7_23() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-7.xml",
            "config-2.xml",
            23
        )
        .0 - 2036.0)
            .abs()
            / 2036.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_8_18() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-8.xml",
            "config-2.xml",
            18
        )
        .0 - 1578.0)
            .abs()
            / 1578.0
            < 1e-2
    );
}

#[test]
fn warehouse_10_20_10_2_2_random_warehouse_10_20_10_2_2_random_9_17() {
    assert!(
        (solve(
            "warehouse-10-20-10-2-2_random",
            "warehouse-10-20-10-2-2-random-9.xml",
            "config-2.xml",
            17
        )
        .0 - 1715.0)
            .abs()
            / 1715.0
            < 1e-2
    );
}
