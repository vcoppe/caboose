#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphNodeId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphEdgeId(pub usize);

pub struct GraphNode<NodeData> {
    pub data: NodeData,
}

pub struct GraphEdge<EdgeData> {
    pub from: GraphNodeId,
    pub to: GraphNodeId,
    pub data: EdgeData,
}

/// Definition a weighted directed graph.
pub struct Graph<NodeData, EdgeData> {
    edges: Vec<GraphEdge<EdgeData>>,
    nodes: Vec<GraphNode<NodeData>>,
    edges_in: Vec<Vec<GraphEdgeId>>,
    edges_out: Vec<Vec<GraphEdgeId>>,
}

impl<NodeData, EdgeData> Graph<NodeData, EdgeData> {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: Vec::new(),
            edges_in: Vec::new(),
            edges_out: Vec::new(),
        }
    }

    pub fn add_node(&mut self, data: NodeData) -> GraphNodeId {
        let id = GraphNodeId(self.nodes.len());
        self.nodes.push(GraphNode { data });
        self.edges_in.push(Vec::new());
        self.edges_out.push(Vec::new());
        id
    }

    pub fn add_edge(&mut self, from: GraphNodeId, to: GraphNodeId, data: EdgeData) -> GraphEdgeId {
        let id = GraphEdgeId(self.edges.len());
        self.edges.push(GraphEdge { from, to, data });
        self.edges_in[to.0].push(id);
        self.edges_out[from.0].push(id);
        id
    }

    pub fn get_node(&self, id: GraphNodeId) -> &GraphNode<NodeData> {
        &self.nodes[id.0]
    }

    pub fn get_edge(&self, id: GraphEdgeId) -> &GraphEdge<EdgeData> {
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
