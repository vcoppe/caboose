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

    let (_, mut cbs, mut config, _) = get_cbs_from_files(&map, &task, &config);
    config.use_n_agents(n_agents);

    let solution = cbs.solve(&config).unwrap();

    solution.iter().map(|sol| sol.cost).sum()
}

#[test]
fn example() {
    assert!(
        (solve(
            "empty-16-16-random",
            "empty-16-16-random-1.xml",
            "config-2.xml",
            2
        )
        .0 - 26.0)
            .abs()
            < 1e-3
    );
}
