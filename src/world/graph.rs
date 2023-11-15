#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphNodeId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphEdgeId(pub usize);

pub struct GraphNode {
    pub position: (f32, f32),
    max_turn_distance: f32,
}

pub struct GraphEdge {
    pub from: GraphNodeId,
    pub to: GraphNodeId,
    pub distance: f32,
    max_speed: f32,
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

    pub fn add_node(&mut self, position: (f32, f32), max_turn_distance: f32) -> GraphNodeId {
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
        distance: f32,
        max_speed: f32,
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

    /// Returns the number of nodes in the graph.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}
