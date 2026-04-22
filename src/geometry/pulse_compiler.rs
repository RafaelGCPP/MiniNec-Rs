use super::*;

pub fn compile_pulses(antenna: &Antenna) -> Vec<Pulse> {
    let mut pulses = Vec::new();

    let nodes = &antenna.nodes;

    for (i, node) in nodes.iter().enumerate() {
        let n_segs=node.segments.len();

        if  n_segs == 1 {
            continue;
        }

        let seg_in = node.segments[0];
        let center_node = i;

        // we need unit_in to point TO center_node
        let unit_in = {
            if antenna.segments[seg_in].nodes.1 == i {
                antenna.segments[seg_in].unit_vector
            } else {
                -antenna.segments[seg_in].unit_vector
            }
        };

        let segin_len= antenna.segments[seg_in].length;

        for seg_idx in 1..n_segs+1 {

            // all segments must leave center_node
            let seg_out = node.segments[seg_idx];
            let unit_out = {
                if antenna.segments[seg_out].nodes.0 ==i {
                    antenna.segments[seg_out].unit_vector
                } else {
                    -antenna.segments[seg_out].unit_vector
                }
            };

            let segout_len=antenna.segments[seg_out].length;
            let total_length=(segin_len+segout_len)/2.0;

            pulses.push( Pulse {
                center_node,
                seg_in,
                seg_out,
                total_length,
                unit_in,
                unit_out
            })
        }
    }
    pulses
}