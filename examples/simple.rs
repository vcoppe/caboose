use std::{sync::Arc, time::Instant};

use cbs::{
    CbsConfig, CbsNode, ConflictBasedSearch, Graph, GraphEdgeId, GraphNodeId, MyTime,
    ReverseResumableAStar, SimpleHeuristic, SimpleState, SimpleWorld, Task,
};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use ordered_float::OrderedFloat;

struct Model {
    graph_size: usize,
    scale: f32,
    graph: Arc<Graph>,
    cbs:
        ConflictBasedSearch<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyTime, SimpleHeuristic>,
    config: CbsConfig<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyTime, SimpleHeuristic>,
    nodes: Vec<Arc<CbsNode<SimpleState, GraphEdgeId, MyTime, MyTime>>>,
    index: usize,
    start_time: f32,
    colors: Vec<rgb::Rgb<nannou::color::encoding::Srgb, u8>>,
    egui: Option<Egui>,
}

fn main() {
    if false {
        nannou::app(model).update(update).run();
    } else {
        let mut model = get_model();

        let start = Instant::now();
        let solution = model.cbs.solve(&model.config);
        let duration = start.elapsed();

        if let Some(solution) = solution {
            println!(
                "Solution cost: {}",
                solution
                    .iter()
                    .map(|sol| *sol.costs.last().unwrap() - *sol.costs.first().unwrap())
                    .sum::<MyTime>()
                    .0
            );
        } else {
            println!("No solution found");
        }
        println!("{:?}", model.cbs.get_stats(&model.config));
        println!("Time elapsed: {:?}", duration);
    }
}

fn get_model() -> Model {
    let graph_size = 10;
    let scale = 80.0;

    let graph = simple_graph(graph_size);
    let transition_system = Arc::new(SimpleWorld::new(graph.clone()));

    let mut tasks = vec![];
    for (from, to) in vec![
        (0, 9),
        (9, 0),
        (1, 8),
        (8, 1),
        (2, 7),
        (7, 2),
        (3, 6),
        //(6, 3),
    ] {
        tasks.push(Arc::new(Task::new(
            Arc::new(SimpleState(GraphNodeId(from))),
            Arc::new(SimpleState(GraphNodeId(to))),
            OrderedFloat(0.0),
        )));
    }

    let colors = vec![BLUE, GREEN, RED, GOLD, HOTPINK, ORANGE, PURPLE, TEAL];

    let pivots = Arc::new(tasks.iter().map(|t| t.goal_state.clone()).collect());
    let heuristic_to_pivots = Arc::new(
        tasks
            .iter()
            .map(|t| {
                Arc::new(ReverseResumableAStar::new(
                    transition_system.clone(),
                    t.clone(),
                    Arc::new(SimpleHeuristic::new(
                        transition_system.clone(),
                        Arc::new(t.reverse()),
                    )),
                ))
            })
            .collect(),
    );

    let config = CbsConfig::new(tasks, pivots, heuristic_to_pivots, OrderedFloat(1e-6));

    let mut cbs = ConflictBasedSearch::new(transition_system);
    cbs.init(&config);
    let nodes = vec![cbs.solve_iter(&config).unwrap()];

    Model {
        graph_size,
        scale,
        graph,
        cbs,
        config,
        nodes,
        index: 0,
        start_time: 0.0,
        colors,
        egui: None,
    }
}

fn model(app: &App) -> Model {
    let mut model = get_model();

    let window_id = app
        .new_window()
        .size(
            ((model.graph_size + 10) * model.scale.round() as usize) as u32,
            (model.graph_size * model.scale.round() as usize) as u32,
        )
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    model.egui = Some(Egui::from_window(&window));
    model.start_time = app.time;

    model
}

fn get_node_at(
    model: &mut Model,
    index: usize,
) -> Option<Arc<CbsNode<SimpleState, GraphEdgeId, MyTime, MyTime>>> {
    while index >= model.nodes.len() {
        if let Some(node) = model.cbs.solve_iter(&model.config) {
            model.nodes.push(node);
        } else {
            return None;
        }
    }
    Some(model.nodes[index].clone())
}

fn update(app: &App, model: &mut Model, update: Update) {
    model
        .egui
        .as_mut()
        .unwrap()
        .set_elapsed_time(update.since_start);

    let ctx = model.egui.as_mut().unwrap().begin_frame().clone();
    egui::Window::new("Settings").show(&ctx, |ui| {
        let prev = ui.button("Previous CBS node").clicked();
        if prev {
            model.index = model.index.saturating_sub(1);
        }

        let next = ui.button("Next CBS node").clicked();
        if next {
            model.index = model.index.saturating_add(1);
        }

        let next_sol = ui.button("Next solution").clicked();
        if next_sol {
            let start = model.index;
            let mut found = false;
            while let Some(node) = get_node_at(model, model.index + 1) {
                model.index += 1;
                if node.conflicts.is_empty() {
                    found = true;
                    break;
                }
            }
            if !found {
                model.index = start;
            }
        }

        if prev || next || next_sol {
            if get_node_at(model, model.index).is_none() {
                model.index = model.nodes.len() - 1;
            }
            model.start_time = app.time;
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.as_mut().unwrap().handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let to_coordinate = |node: GraphNodeId| {
        let node = model.graph.get_node(node);
        (vec2(node.position.0 as f32, node.position.1 as f32)
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
    let initial_time = model
        .config
        .tasks
        .iter()
        .map(|t| t.initial_cost)
        .min()
        .unwrap();
    let mut elapsed_time = OrderedFloat(elapsed);
    let mut current_time = initial_time + elapsed;

    let solutions = model.nodes[model.index].get_solutions(model.config.n_agents);

    let max_elapsed_time = *solutions
        .iter()
        .map(|solution| solution.costs.last().unwrap())
        .max()
        .unwrap()
        - initial_time;

    while elapsed_time > max_elapsed_time {
        elapsed_time = elapsed_time - max_elapsed_time;
        current_time = current_time - max_elapsed_time;
    }

    for (agent, solution) in solutions.iter().enumerate() {
        let mut drawn = false;
        for i in 0..(solution.costs.len() - 1) {
            if current_time >= solution.costs[i] && current_time <= solution.costs[i + 1] {
                let from = to_coordinate(solution.states[i].internal_state.0);
                let to = to_coordinate(solution.states[i + 1].internal_state.0);

                let delta = to - from;
                let progress_time = current_time - solution.costs[i];
                let move_time = solution.costs[i + 1] - solution.costs[i];
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
            let center = to_coordinate(solution.states.last().unwrap().internal_state.0);
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

    let mut text = String::new();
    let mut constraint_texts = vec![];

    text += &format!("Current cost: {}\n", model.nodes[model.index].total_cost.0);

    // Draw constraints
    for agent in 0..model.config.n_agents {
        text += &format!("Constraints for agent {}:\n", agent);

        let (constraints, landmarks) = model.nodes[model.index].get_constraints(agent);
        for ((from, to), constraint_set) in constraints.action_constraints.iter() {
            for constraint in constraint_set {
                if current_time >= constraint.interval.start
                    && current_time <= constraint.interval.end
                {
                    let from = to_coordinate(from.0);
                    let to = to_coordinate(to.0);

                    let delta = to - from;
                    let delta = delta.rotate(0.5 * PI).clamp_length_max(model.scale / 10.0);

                    let from = from + delta;
                    let to = to + delta;

                    draw.line()
                        .color(rgba8(
                            model.colors[agent].red,
                            model.colors[agent].green,
                            model.colors[agent].blue,
                            120,
                        ))
                        .start(from)
                        .end(to)
                        .weight(model.scale / 5.0);
                }

                constraint_texts.push(format!(
                    "- Action constraint between nodes {} and {}, between {} and {}\n",
                    from.0 .0, to.0 .0, constraint.interval.start.0, constraint.interval.end.0
                ));
            }
        }

        constraint_texts.sort_unstable();
        text += &constraint_texts.join("");
        constraint_texts.clear();

        for (state, constraint_set) in constraints.state_constraints.iter() {
            for constraint in constraint_set {
                if current_time >= constraint.interval.start
                    && current_time <= constraint.interval.end
                {
                    draw.rect()
                        .w_h(model.scale, model.scale)
                        .color(rgba8(
                            model.colors[agent].red,
                            model.colors[agent].green,
                            model.colors[agent].blue,
                            120,
                        ))
                        .xy(to_coordinate(state.0));
                }

                constraint_texts.push(format!(
                    "- State constraint at node {}, between {} and {}\n",
                    state.0 .0, constraint.interval.start.0, constraint.interval.end.0
                ));
            }
        }

        constraint_texts.sort_unstable();
        text += &constraint_texts.join("");
        constraint_texts.clear();

        for constraint in landmarks.iter() {
            if current_time >= constraint.interval.start && current_time <= constraint.interval.end
            {
                draw.ellipse()
                    .radius(model.scale / 2.0)
                    .no_fill()
                    .stroke(rgba8(
                        model.colors[agent].red,
                        model.colors[agent].green,
                        model.colors[agent].blue,
                        120,
                    ))
                    .stroke_weight(model.scale / 10.0)
                    .xy(to_coordinate(constraint.state.0));
            }

            constraint_texts.push(format!(
                "- Landmark at node {}, between {} and {}\n",
                constraint.state.0 .0, constraint.interval.start.0, constraint.interval.end.0
            ));
        }
    }

    constraint_texts.sort_unstable();
    text += &constraint_texts.join("");
    constraint_texts.clear();

    text += &format!("Remaining conflicts\n");
    for conflict in model.nodes[model.index].conflicts.iter() {
        text += &format!(
            "- Agent {} moving between nodes {} and {}, between {} and {}, and agent {} moving between {} and {}, between {} and {}\n",
            conflict.moves.0.agent,
            conflict.moves.0.from.0 .0,
            conflict.moves.0.to.0 .0,
            conflict.moves.0.interval.start.0,
            conflict.moves.0.interval.end.0,
            conflict.moves.1.agent,
            conflict.moves.1.from.0 .0,
            conflict.moves.1.to.0 .0,
            conflict.moves.1.interval.start.0,
            conflict.moves.1.interval.end.0,
        );
    }

    draw.text(text.as_str())
        .left_justify()
        .color(BLACK)
        .font_size(12)
        .xy(vec2(model.graph_size as f32 * model.scale * 0.75, 0.0))
        .width(model.scale * 5.0);

    draw.to_frame(app, &frame).unwrap();
    model.egui.as_ref().unwrap().draw_to_frame(&frame).unwrap();
}

fn simple_graph(size: usize) -> Arc<Graph> {
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
