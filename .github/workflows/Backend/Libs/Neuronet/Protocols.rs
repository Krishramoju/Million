// neurokernel/src/neuro_met/protocols.rs

use std::collections::HashMap;

/// Defines supported protocol types for neural interactions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NeuroProtocol {
    IntentSync,
    MemoryStream,
    EmotionalFeedback,
    ReflexTrigger,
    NeuroHandshake,
}

/// Represents a metadata-rich communication packet in NeuroOS
#[derive(Debug, Clone)]
pub struct NeuroPacket {
    pub protocol: NeuroProtocol,
    pub source_id: String,
    pub target_id: String,
    pub payload: String,
    pub metadata: HashMap<String, String>,
}

/// Central router for NeuroProtocol messages within NeuroOS
pub struct ProtocolDispatcher {
    pub routes: HashMap<NeuroProtocol, Vec<Box<dyn Fn(&NeuroPacket)>>>,
}

impl ProtocolDispatcher {
    /// Creates a new dispatcher
    pub fn new() -> Self {
        ProtocolDispatcher {
            routes: HashMap::new(),
        }
    }

    /// Register a handler for a given protocol
    pub fn register_handler<F>(&mut self, protocol: NeuroProtocol, handler: F)
    where
        F: Fn(&NeuroPacket) + 'static,
    {
        self.routes
            .entry(protocol)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }

    /// Dispatch a packet to all appropriate handlers
    pub fn dispatch(&self, packet: &NeuroPacket) {
        if let Some(handlers) = self.routes.get(&packet.protocol) {
            for handler in handlers {
                handler(packet);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_dispatching() {
        let mut dispatcher = ProtocolDispatcher::new();
        let mut triggered = false;

        dispatcher.register_handler(NeuroProtocol::IntentSync, |packet| {
            if packet.payload.contains("wake") {
                println!("Wake intent received");
            }
        });

        dispatcher.register_handler(NeuroProtocol::ReflexTrigger, |_| {
            triggered = true;
        });

        let test_packet = NeuroPacket {
            protocol: NeuroProtocol::IntentSync,
            source_id: "user_001".into(),
            target_id: "cortex".into(),
            payload: "wake cortex".into(),
            metadata: HashMap::new(),
        };

        dispatcher.dispatch(&test_packet);
    }
}
