#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphNodeId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphEdgeId(usize);

pub struct GraphNode {
    pub position: (f64, f64),
    max_turn_distance: f64,
}

pub struct GraphEdge {
    pub from: GraphNodeId,
    pub to: GraphNodeId,
    pub distance: f64,
    max_speed: f64,
}

/// Definition a weighted directed graph.
pub struct Graph {
    edges: Vec<GraphEdge>,
    nodes: Vec<GraphNode>,
    edges_in: Vec<Vec<GraphEdgeId>>,
    edges_out: Vec<Vec<GraphEdgeId>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: Vec::new(),
            edges_in: Vec::new(),
            edges_out: Vec::new(),
        }
    }

    pub fn add_node(&mut self, position: (f64, f64), max_turn_distance: f64) -> GraphNodeId {
        let id = GraphNodeId(self.nodes.len());
        self.nodes.push(GraphNode {
            position,
            max_turn_distance,
        });
        self.edges_in.push(Vec::new());
        self.edges_out.push(Vec::new());
        id
    }

    pub fn add_edge(
        &mut self,
        from: GraphNodeId,
        to: GraphNodeId,
        distance: f64,
        max_speed: f64,
    ) -> GraphEdgeId {
        let id = GraphEdgeId(self.edges.len());
        self.edges.push(GraphEdge {
            from,
            to,
            distance,
            max_speed,
        });
        self.edges_in[to.0].push(id);
        self.edges_out[from.0].push(id);
        id
    }

    pub fn get_node(&self, id: GraphNodeId) -> &GraphNode {
        &self.nodes[id.0]
    }

    pub fn get_edge(&self, id: GraphEdgeId) -> &GraphEdge {
        &self.edges[id.0]
    }

    /// Returns the edges that go into the given node.
    pub fn get_edges_in(&self, id: GraphNodeId) -> &[GraphEdgeId] {
        &self.edges_in[id.0]
    }

    /// Returns the edges that go out of the given node.
    pub fn get_edges_out(&self, id: GraphNodeId) -> &[GraphEdgeId] {
        &self.edges_out[id.0]
    }
}
