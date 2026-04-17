use super::*;

pub fn compile_pulses(antenna: &Antenna) -> Vec<Pulse> {
    let mut pulses = Vec::new();

    let nodes = &antenna.nodes;

    for (i, node) in nodes.iter().enumerate() {
        match node.segments.len() {
            1 => continue,
            2 => {
                let seg_in = node.segments[0];
                let seg_out = node.segments[1];

                let seg_in_vec = antenna.segments[seg_in].unit_vector;
                let seg_out_vec = antenna.segments[seg_out].unit_vector;

                let total_length = antenna.segments[seg_in].length + antenna.segments[seg_out].length;

                pulses.push(Pulse {
                    center_node: i,
                    seg_in,
                    seg_out,
                    total_length,
                    unit_in: seg_in_vec,
                    unit_out: seg_out_vec,
                });
            },
            _ => {

            }
        }


    }



    pulses
}