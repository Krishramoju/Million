use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    LLMCore,
    MemoryUnit,
    UserProfile,
    EmotionUnit,
    TaskHandler,
    SensorNode,
    OutputModule,
}

#[derive(Debug, Clone)]
pub struct NeuroNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub metadata: HashMap<String, String>,
    pub connections: HashSet<Uuid>,
}

#[derive(Debug)]
pub struct NeuroNet {
    pub nodes: HashMap<Uuid, NeuroNode>,
}

impl NeuroNet {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_type: NodeType, metadata: HashMap<String, String>) -> Uuid {
        let id = Uuid::new_v4();
        let node = NeuroNode {
            id,
            node_type,
            metadata,
            connections: HashSet::new(),
        };
        self.nodes.insert(id, node);
        id
    }

    pub fn connect_nodes(&mut self, from: Uuid, to: Uuid) {
        if let Some(node) = self.nodes.get_mut(&from) {
            node.connections.insert(to);
        }
    }

    pub fn route_signal(&self, start: Uuid, max_depth: usize) -> Vec<Uuid> {
        let mut visited = HashSet::new();
        let mut queue = vec![(start, 0)];
        let mut path = vec![];

        while let Some((current, depth)) = queue.pop() {
            if visited.contains(&current) || depth > max_depth {
                continue;
            }
            visited.insert(current);
            path.push(current);

            if let Some(node) = self.nodes.get(&current) {
                for neighbor in &node.connections {
                    queue.push((*neighbor, depth + 1));
                }
            }
        }

        path
    }

    pub fn describe_network(&self) {
        for node in self.nodes.values() {
            println!(
                "[{:?}] {} — connected to {} nodes",
                node.node_type,
                node.id,
                node.connections.len()
            );
        }
    }

    pub fn get_node_by_type(&self, t: NodeType) -> Vec<&NeuroNode> {
        self.nodes
            .values()
            .filter(|n| n.node_type == t)
            .collect()
    }
}
