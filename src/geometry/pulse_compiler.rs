use super::*;

pub fn compile_pulses(antenna: &Antenna) -> Vec<Pulse> {
    let mut pulses = Vec::new();

    let nodes = &antenna.nodes;

    for (i, node) in nodes.iter().enumerate() {
        if node.segments.len()<2 {
            continue; // Skip nodes that are not connected to at least two segments
        }
    }



    pulses
}