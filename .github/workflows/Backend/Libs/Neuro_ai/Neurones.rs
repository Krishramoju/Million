// neurokernel/src/neuronet.rs

use std::collections::{HashMap, HashSet};

/// Represents a connection between digital neurons in the NeuroNet
#[derive(Debug, Clone)]
pub struct Synapse {
    pub target: String,
    pub weight: f32,
}

/// Represents a neuron in the NeuroNet graph
#[derive(Debug, Clone)]
pub struct Neuron {
    pub id: String,
    pub state: f32,
    pub synapses: Vec<Synapse>,
}

/// The NeuroNet engine: A graph of digital neurons used for intent prediction, memory routing, and semantic flow control
pub struct NeuroNet {
    neurons: HashMap<String, Neuron>,
}

impl NeuroNet {
    /// Creates a new, empty NeuroNet
    pub fn new() -> Self {
        NeuroNet {
            neurons: HashMap::new(),
        }
    }

    /// Adds a neuron to the network
    pub fn add_neuron(&mut self, id: &str) {
        if !self.neurons.contains_key(id) {
            self.neurons.insert(
                id.to_string(),
                Neuron {
                    id: id.to_string(),
                    state: 0.0,
                    synapses: Vec::new(),
                },
            );
        }
    }

    /// Connects two neurons with a synapse of a given weight
    pub fn connect(&mut self, from: &str, to: &str, weight: f32) {
        self.add_neuron(from);
        self.add_neuron(to);

        if let Some(neuron) = self.neurons.get_mut(from) {
            neuron.synapses.push(Synapse {
                target: to.to_string(),
                weight,
            });
        }
    }

    /// Fires a neuron with a given activation, propagating activation to connected neurons
    pub fn fire(&mut self, id: &str, activation: f32) {
        let mut visited = HashSet::new();
        self._propagate(id, activation, &mut visited);
    }

    /// Internal recursive propagation method
    fn _propagate(&mut self, id: &str, activation: f32, visited: &mut HashSet<String>) {
        if visited.contains(id) {
            return;
        }
        visited.insert(id.to_string());

        if let Some(neuron) = self.neurons.get_mut(id) {
            neuron.state += activation;
            for synapse in &neuron.synapses {
                let propagated = activation * synapse.weight;
                self._propagate(&synapse.target, propagated, visited);
            }
        }
    }

    /// Gets the state of a neuron
    pub fn get_state(&self, id: &str) -> Option<f32> {
        self.neurons.get(id).map(|n| n.state)
    }

    /// Resets all neuron states
    pub fn reset(&mut self) {
        for neuron in self.neurons.values_mut() {
            neuron.state = 0.0;
        }
    }

    /// Visualizes the network as a string (for debugging)
    pub fn visualize(&self) -> String {
        let mut out = String::new();
        for (id, neuron) in &self.neurons {
            out.push_str(&format!("Neuron: {} [state: {:.3}]\n", id, neuron.state));
            for syn in &neuron.synapses {
                out.push_str(&format!("  -> {} (weight {:.3})\n", syn.target, syn.weight));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuronet_basic_fire() {
        let mut net = NeuroNet::new();
        net.connect("intent_detected", "generate_response", 0.9);
        net.connect("generate_response", "log_interaction", 0.5);

        net.fire("intent_detected", 1.0);

        assert!(net.get_state("intent_detected").unwrap() > 0.9);
        assert!(net.get_state("generate_response").unwrap() > 0.8);
        assert!(net.get_state("log_interaction").unwrap() > 0.4);
    }
}
