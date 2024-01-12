use std::sync::Arc;

use cbs::{
    get_cbs_from_files, Graph, GraphEdgeId, GraphNodeId, MyTime, SimpleEdgeData, SimpleNodeData,
    SimpleState, SippState, Solution,
};
use nannou::prelude::*;
use ordered_float::OrderedFloat;

struct Model {
    agent_size: f64,
    graph: Arc<Graph<SimpleNodeData, SimpleEdgeData>>,
    solution:
        Option<Vec<Solution<Arc<SippState<SimpleState, MyTime>>, GraphEdgeId, MyTime, MyTime>>>,
    start_time: f32,
    colors: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>,
    limits: ((f32, f32), (f32, f32)),
}

fn main() {
    if true {
        nannou::app(model).update(update).run();
    } else {
        get_model();
    }
}

fn get_model() -> Model {
    let (graph, mut cbs, config, agent_size) = get_cbs_from_files(
        "resources/instances/roadmaps/sparse/map.xml",
        "resources/instances/roadmaps/sparse/22_task.xml",
        "resources/config/config-2.xml",
        9,
    );
    let limits = (0..graph.num_nodes())
        .map(|id| {
            let node = graph.get_node(GraphNodeId(id));
            (node.data.0, node.data.1)
        })
        .fold(
            ((f32::MAX, f32::MAX), (f32::MIN, f32::MIN)),
            |((min_x, min_y), (max_x, max_y)), (x, y)| {
                (
                    (min_x.min(x as f32), min_y.min(y as f32)),
                    (max_x.max(x as f32), max_y.max(y as f32)),
                )
            },
        );

    // A set of 20 colors that are visually distinct
    let colors = vec![
        BLUE, GREEN, RED, GOLD, HOTPINK, ORANGE, PURPLE, TEAL, YELLOW, CYAN, PINK, LIME, MAROON,
        NAVY, OLIVE, LAVENDER, BROWN, BEIGE, CORAL, GREY, MAGENTA, TURQUOISE,
    ];

    let solution = cbs.solve(&config);

    if let Some(solution) = &solution {
        println!(
            "Solution cost: {}",
            solution.iter().map(|sol| sol.cost).sum::<MyTime>().0
        );
    } else {
        println!("No solution found");
    }

    println!("{:?}", cbs.get_stats());

    Model {
        graph,
        agent_size,
        solution,
        start_time: 0.0,
        colors,
        limits,
    }
}

fn model(app: &App) -> Model {
    let mut model = get_model();

    app.new_window().view(view).build().unwrap();
    model.start_time = app.time;

    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let window = app.window_rect().pad(50.0);
    let scale = (window.w() / (model.limits.1 .0 - model.limits.0 .0))
        .min(window.h() / (model.limits.1 .1 - model.limits.0 .1));

    let to_coordinate = |node: GraphNodeId| {
        let node = model.graph.get_node(node);
        // map node coordinates to window coordinates
        vec2(
            (node.data.0 as f32 - (model.limits.0 .0 + model.limits.1 .0) / 2.0) * scale,
            (node.data.1 as f32 - (model.limits.0 .1 + model.limits.1 .1) / 2.0) * scale,
        )
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
            .xy(to_coordinate(GraphNodeId(id)) + vec2(scale, scale) / 5.0);
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
    let mut current_time = OrderedFloat(elapsed as f64);

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
                let progress_time = (current_time - solution.steps[i].1).0 as f32;
                let move_time = solution.steps[i + 1].1 - solution.steps[i].1;
                let vel = delta / move_time.0 as f32;
                let center = from + vel * progress_time;

                draw.ellipse()
                    .color(model.colors[agent])
                    .radius(model.agent_size as f32 * scale)
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
                .radius(model.agent_size as f32 * scale)
                .xy(center);
            draw.text(agent.to_string().as_str())
                .color(WHITE)
                .font_size(18)
                .xy(center);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
