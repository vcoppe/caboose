use std::{collections::HashMap, error::Error, fs::File, io::Read, sync::Arc, time::Duration};

use ordered_float::OrderedFloat;
use quick_xml::{de::from_str, DeError};
use serde::Deserialize;

use crate::{
    CbsConfig, ConflictBasedSearch, Graph, GraphEdgeId, GraphNodeId, MyTime, ReverseResumableAStar,
    SimpleEdgeData, SimpleHeuristic, SimpleNodeData, SimpleState, SimpleWorld, Task,
};

/// Builds a CBS algorithm and its configuration from the given files.
pub fn get_cbs_from_files(
    map_file: &str,
    task_file: &str,
    config_file: &str,
    n_agents: usize,
) -> (
    Arc<Graph<SimpleNodeData, SimpleEdgeData>>,
    ConflictBasedSearch<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyTime, SimpleHeuristic>,
    CbsConfig<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyTime, SimpleHeuristic>,
    f32,
) {
    let (graph, tasks, config) = parse_inputs(map_file, task_file, config_file, n_agents).unwrap();
    let transition_system = Arc::new(SimpleWorld::new(graph.clone(), config.agent_size));

    let pivots = Arc::new(tasks.iter().map(|t| t.goal_state.clone()).collect());
    let heuristic_to_pivots = Arc::new(
        tasks
            .iter()
            .map(|t| {
                Arc::new(ReverseResumableAStar::new(
                    transition_system.clone(),
                    t.clone(),
                    SimpleHeuristic::new(transition_system.clone(), Arc::new(t.reverse())),
                ))
            })
            .collect(),
    );

    (
        graph,
        ConflictBasedSearch::new(transition_system),
        CbsConfig::new(
            tasks,
            pivots,
            heuristic_to_pivots,
            OrderedFloat(config.precision),
            Some(Duration::from_secs_f32(config.time_limit)),
        ),
        config.agent_size,
    )
}

/// Parse the benchmark maps and scenarios from https://movingai.com/benchmarks/mapf/index.html
pub fn read_from_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parse the benchmark maps and scenarios from https://movingai.com/benchmarks/mapf/index.html
pub fn parse_inputs(
    map_file: &str,
    task_file: &str,
    config_file: &str,
    n_agents: usize,
) -> Result<
    (
        Arc<Graph<SimpleNodeData, SimpleEdgeData>>,
        Vec<Arc<Task<SimpleState, MyTime>>>,
        Config,
    ),
    Box<dyn Error>,
> {
    let contents = read_from_file(map_file)?;
    let data: Result<Map, DeError> = from_str(&contents);
    let map = data?;

    let contents = read_from_file(task_file)?;
    let data: Result<Scenario, DeError> = from_str(&contents);
    let mut scenario = data?;
    scenario.agents.truncate(n_agents);

    let config = parse_config(config_file)?;

    let mut graph = Graph::new();
    let mut tasks = Vec::new();

    if let Some(map) = map.grid {
        let mut grid = vec![vec![GraphNodeId(0); map.width]; map.height];
        for x in 0..map.width {
            for y in 0..map.height {
                if map.grid.rows[y][x] == 1 {
                    // 1 is an obstacle
                    continue;
                }
                grid[y][x] = graph.add_node((x as f32, y as f32));
            }
        }

        // TODO: use connectedness parameter
        for x in 0..map.width {
            for y in 0..map.height {
                if map.grid.rows[y][x] == 1 {
                    // 1 is an obstacle
                    continue;
                }
                let node_id = grid[y][x];
                if x > 0 && map.grid.rows[y][x - 1] == 0 {
                    graph.add_edge(node_id, grid[y][x - 1], 1.0);
                }
                if y > 0 && map.grid.rows[y - 1][x] == 0 {
                    graph.add_edge(node_id, grid[y - 1][x], 1.0);
                }
                if x < map.width - 1 && map.grid.rows[y][x + 1] == 0 {
                    graph.add_edge(node_id, grid[y][x + 1], 1.0);
                }
                if y < map.height - 1 && map.grid.rows[y + 1][x] == 0 {
                    graph.add_edge(node_id, grid[y + 1][x], 1.0);
                }
            }
        }

        for agent in scenario.agents {
            let initial_state = SimpleState(grid[agent.start_i.unwrap()][agent.start_j.unwrap()]);
            let goal_state = SimpleState(grid[agent.goal_i.unwrap()][agent.goal_j.unwrap()]);
            tasks.push(Arc::new(Task::new(
                initial_state,
                goal_state,
                OrderedFloat(0.0),
            )));
        }
    } else if let Some(map) = map.graph {
        let mut nodes = HashMap::new();
        for node in map.nodes {
            let position = node
                .position
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<f32>>();
            nodes.insert(node.id, graph.add_node((position[0], position[1])));
        }
        for edge in map.edges {
            graph.add_edge(nodes[&edge.source], nodes[&edge.target], edge.weight);
        }

        for agent in scenario.agents {
            let start = nodes[&("n".to_string() + &agent.start_id.unwrap().to_string())];
            let goal = nodes[&("n".to_string() + &agent.goal_id.unwrap().to_string())];
            let initial_state = SimpleState(start);
            let goal_state = SimpleState(goal);
            tasks.push(Arc::new(Task::new(
                initial_state,
                goal_state,
                OrderedFloat(0.0),
            )));
        }
    } else {
        return Err("No map found".into());
    }

    Ok((Arc::new(graph), tasks, config))
}

/// A structure that corresponds to the XML format of the mapf.info benchmark maps.
/// Either a grid map, for example:
/// ```xml
/// <?xml version="1.0" ?>
/// <root>
/// <map>
///     <width>5</width>
///     <height>3</height>
///     <grid width="5" height="3">
///         <row>0 1 1 1 0</row>
///         <row>0 0 0 1 0</row>
///         <row>0 1 0 0 0</row>
///     </grid>
/// </map>
/// </root>
/// ```
/// Or a graph map, for example:
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <graphml xmlns="http://graphml.graphdrawing.org/xmlns" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">
/// <key id="key0" for="node" attr.name="coords" attr.type="string" />
/// <key id="key1" for="edge" attr.name="weight" attr.type="double" />
/// <graph id="G" edgedefault="directed" parse.nodeids="free" parse.edgeids="canonical" parse.order="nodesfirst">
///   <node id="n0">
///     <data key="key0">0,0</data>
///   </node>
///   <node id="n1">
///     <data key="key0">0,1</data>
///   </node>
///   <edge id="e0" source="n0" target="n1">
///     <data key="key1">1</data>
///   </edge>
///   <edge id="e1" source="n1" target="n0">
///     <data key="key1">1</data>
///   </edge>
/// </graph>
/// </graphml>
/// ```
#[derive(Debug, Deserialize)]
pub struct Map {
    #[serde(rename = "map")]
    grid: Option<GridMap>,
    graph: Option<GraphMap>,
}

#[derive(Debug, Deserialize)]
pub struct GridMap {
    width: usize,
    height: usize,
    grid: Grid,
}

#[derive(Debug, Deserialize)]
pub struct Grid {
    #[serde(rename = "row")]
    rows: Vec<Vec<usize>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphMap {
    #[serde(rename = "node")]
    nodes: Vec<Node>,
    #[serde(rename = "edge")]
    edges: Vec<Edge>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "data")]
    position: String,
}

#[derive(Debug, Deserialize)]
pub struct Edge {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@target")]
    target: String,
    #[serde(rename = "data")]
    weight: f32,
}
/// A structure that corresponds to the XML format of the mapf.info benchmark scenarios.
/// Either a scenario for a grid map, for example:
/// ```xml
/// <?xml version="1.0" ?>
/// <root>
/// <agent id="0" start_i="8" start_j="13" goal_i="7" goal_j="8"/>
/// <agent id="1" start_i="2" start_j="15" goal_i="9" goal_j="2"/>
/// </root>
/// ```
/// Or a scenario for a graph map, for example:
/// ```xml
/// <?xml version="1.0" ?>
/// <root>
/// <agent start_id="136" goal_id="50"/>
/// <agent start_id="143" goal_id="169"/>
/// </root>
/// ```
#[derive(Debug, Deserialize)]
pub struct Scenario {
    #[serde(rename = "agent")]
    pub agents: Vec<Agent>,
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    #[serde(rename = "@start_i")]
    pub start_i: Option<usize>,
    #[serde(rename = "@start_j")]
    pub start_j: Option<usize>,
    #[serde(rename = "@goal_i")]
    pub goal_i: Option<usize>,
    #[serde(rename = "@goal_j")]
    pub goal_j: Option<usize>,
    #[serde(rename = "@start_id")]
    pub start_id: Option<usize>,
    #[serde(rename = "@goal_id")]
    pub goal_id: Option<usize>,
}

/// Parse a configuration file.
pub fn parse_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let contents = read_from_file(filename)?;
    let data: Result<ConfigRoot, DeError> = from_str(&contents);
    let config = data?.config;
    Ok(config)
}

/// Parameters for the algorithm, example:
/// ```xml
/// <?xml version="1.0" ?>
/// <root>
/// <algorithm>
///     <use_cardinal>true</use_cardinal>
///     <use_disjoint_splitting>true</use_disjoint_splitting>
///     <hlh_type>2</hlh_type>
///     <connectedness>5</connectedness>
///     <focal_weight>1.0</focal_weight>
///     <agent_size>0.4</agent_size>
///     <timelimit>5</timelimit>
///     <precision>0.0000001</precision>
/// </algorithm>
/// </root>
/// ```
/// We only use the `agent_size`, `connectedness` and `precision` parameters.
#[derive(Debug, Deserialize)]
pub struct ConfigRoot {
    #[serde(rename = "algorithm")]
    pub config: Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub agent_size: f32,
    pub connectedness: usize,
    pub precision: f32,
    #[serde(rename = "timelimit")]
    pub time_limit: f32,
}
