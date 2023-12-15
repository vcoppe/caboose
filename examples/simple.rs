use std::{sync::Arc, time::Instant};

use cbs::{
    CbsConfig, ConflictBasedSearch, Graph, GraphEdgeId, GraphNodeId, MyTime, ReverseResumableAStar,
    SimpleEdgeData, SimpleHeuristic, SimpleNodeData, SimpleState, SimpleWorld, SippState, Solution,
    Task,
};
use nannou::prelude::*;
use ordered_float::OrderedFloat;

struct Model {
    graph_size: usize,
    scale: f32,
    graph: Arc<Graph<SimpleNodeData, SimpleEdgeData>>,
    solution:
        Option<Vec<Solution<Arc<SippState<SimpleState, MyTime>>, GraphEdgeId, MyTime, MyTime>>>,
    start_time: f32,
    colors: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>,
}

fn main() {
    if true {
        nannou::app(model).update(update).run();
    } else {
        get_model();
    }
}

fn get_model() -> Model {
    let graph_size = 10;
    let scale = 60.0;

    let graph = simple_graph(graph_size);
    let transition_system = Arc::new(SimpleWorld::new(graph.clone()));

    let to_id = |x: usize, y: usize| GraphNodeId(x + y * graph_size);

    let mut tasks = vec![];
    for (from, to) in vec![
        /*((4, 8), (8, 1)),
        ((5, 9), (3, 6)),
        ((9, 5), (0, 7)),
        ((2, 8), (7, 9)),
        ((4, 3), (8, 8)),
        ((2, 5), (0, 2)),
        ((8, 2), (1, 4)),
        ((8, 9), (1, 6)),
        ((2, 7), (9, 4)),
        ((1, 0), (7, 5)),
        ((0, 9), (3, 8)),
        ((9, 1), (7, 3)),
        ((5, 1), (0, 5)),*/
        ((0, 0), (0, 9)),
        ((0, 9), (0, 0)),
        ((0, 1), (0, 8)),
        ((0, 8), (0, 1)),
        ((0, 2), (0, 7)),
        ((0, 7), (0, 2)),
        ((0, 3), (0, 6)),
    ]
    .iter()
    .map(|((x1, y1), (x2, y2))| (to_id(*x1, *y1), to_id(*x2, *y2)))
    {
        tasks.push(Arc::new(Task::new(
            SimpleState(from),
            SimpleState(to),
            OrderedFloat(0.0),
        )));
    }

    // A set of 20 colors that are visually distinct
    let colors = vec![
        BLUE, GREEN, RED, GOLD, HOTPINK, ORANGE, PURPLE, TEAL, YELLOW, CYAN, PINK, LIME, MAROON,
        NAVY, OLIVE, LAVENDER, BROWN, BEIGE, CORAL, GREY, MAGENTA, TURQUOISE,
    ];

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

    let config = CbsConfig::new(tasks, pivots, heuristic_to_pivots, OrderedFloat(1e-6));

    let mut cbs = ConflictBasedSearch::new(transition_system);

    let start = Instant::now();
    let solution = cbs.solve(&config);
    let duration = start.elapsed();

    if let Some(solution) = &solution {
        println!(
            "Solution cost: {}",
            solution.iter().map(|sol| sol.cost).sum::<MyTime>().0
        );
    } else {
        println!("No solution found");
    }

    println!("{:?}", cbs.get_stats());
    println!("Time elapsed: {:?}", duration);

    Model {
        graph_size,
        scale,
        graph,
        solution,
        start_time: 0.0,
        colors,
    }
}

fn model(app: &App) -> Model {
    let mut model = get_model();

    app.new_window()
        .size(
            (model.graph_size * model.scale.round() as usize) as u32,
            (model.graph_size * model.scale.round() as usize) as u32,
        )
        .view(view)
        .build()
        .unwrap();

    model.start_time = app.time;

    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let to_coordinate = |node: GraphNodeId| {
        let node = model.graph.get_node(node);
        (vec2(node.data.0 as f32, node.data.1 as f32)
            - vec2(
                (model.graph_size - 1) as f32 / 2.0,
                (model.graph_size - 1) as f32 / 2.0,
            ))
            * model.scale
    };

    // Draw graph
    for id in 0..model.graph.num_nodes() {
        draw.ellipse()
            .color(BLACK)
            .radius(2.0)
            .xy(to_coordinate(GraphNodeId(id)));
        draw.text(id.to_string().as_str())
            .color(BLACK)
            .font_size(18)
            .xy(to_coordinate(GraphNodeId(id)) + vec2(model.scale, model.scale) / 5.0);
    }

    for id in 0..model.graph.num_edges() {
        let edge = model.graph.get_edge(GraphEdgeId(id));
        draw.line()
            .color(BLACK)
            .start(to_coordinate(edge.from))
            .end(to_coordinate(edge.to));
    }

    // Draw agents
    let elapsed = app.time - model.start_time;
    let mut current_time = OrderedFloat(elapsed);

    let solutions = model.solution.as_ref().unwrap();

    let max_time = solutions
        .iter()
        .map(|solution| solution.cost)
        .max()
        .unwrap();

    while current_time > max_time {
        current_time = current_time - max_time;
    }

    for (agent, solution) in solutions.iter().enumerate() {
        let mut drawn = false;
        for i in 0..(solution.steps.len() - 1) {
            if current_time >= solution.steps[i].1 && current_time <= solution.steps[i + 1].1 {
                let from = to_coordinate(solution.steps[i].0.internal_state.0);
                let to = to_coordinate(solution.steps[i + 1].0.internal_state.0);

                let delta = to - from;
                let progress_time = current_time - solution.steps[i].1;
                let move_time = solution.steps[i + 1].1 - solution.steps[i].1;
                let vel = delta / move_time.0;
                let center = from + vel * progress_time.0;

                draw.ellipse()
                    .color(model.colors[agent])
                    .radius(0.4 * model.scale)
                    .xy(center);
                draw.text(agent.to_string().as_str())
                    .color(WHITE)
                    .font_size(18)
                    .xy(center);
                drawn = true;
                break;
            }
        }

        if !drawn {
            let center = to_coordinate(solution.steps.last().unwrap().0.internal_state.0);
            draw.ellipse()
                .color(model.colors[agent])
                .radius(0.4 * model.scale)
                .xy(center);
            draw.text(agent.to_string().as_str())
                .color(WHITE)
                .font_size(18)
                .xy(center);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn simple_graph(size: usize) -> Arc<Graph<SimpleNodeData, SimpleEdgeData>> {
    let mut graph = Graph::new();
    for x in 0..size {
        for y in 0..size {
            graph.add_node((x as f32, y as f32));
        }
    }
    for x in 0..size {
        for y in 0..size {
            let node_id = GraphNodeId(x + y * size);
            if x > 0 {
                graph.add_edge(node_id, GraphNodeId(x - 1 + y * size), 1.0);
            }
            if y > 0 {
                graph.add_edge(node_id, GraphNodeId(x + (y - 1) * size), 1.0);
            }
            if x < size - 1 {
                graph.add_edge(node_id, GraphNodeId(x + 1 + y * size), 1.0);
            }
            if y < size - 1 {
                graph.add_edge(node_id, GraphNodeId(x + (y + 1) * size), 1.0);
            }
        }
    }
    Arc::new(graph)
}
