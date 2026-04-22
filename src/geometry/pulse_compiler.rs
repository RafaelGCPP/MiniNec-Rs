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

        let seg_in_len = antenna.segments[seg_in].length;

        for seg_idx in 1..n_segs {

            // all segments must leave center_node
            let seg_out = node.segments[seg_idx];
            let unit_out = {
                if antenna.segments[seg_out].nodes.0 ==i {
                    antenna.segments[seg_out].unit_vector
                } else {
                    -antenna.segments[seg_out].unit_vector
                }
            };

            let seg_out_len=antenna.segments[seg_out].length;
            let total_length=(seg_in_len +seg_out_len)/2.0;

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


#[cfg(test)]
mod tests {
    use super::*;
    use super::geometry_file::read_antenna_from_file;
    use super::segments_compiler::compile_geometry_file;


    #[test]
    fn test_compile_pulse_file_dipole() {
        let file = read_antenna_from_file("TestData/antenna.json");
        assert!(file.is_ok());
        let file = file.unwrap();
        let antenna = compile_geometry_file(&file, 20.0);
        assert!(antenna.is_ok());
        let antenna = antenna.unwrap();

        assert_eq!(antenna.wire_map.len(), 1); // There is only one wire in the test file
        assert_eq!(antenna.nodes.len(), 13); // There should be 13 nodes
        assert_eq!(antenna.segments.len(), 12); // There should be 13 segments
        assert_eq!(antenna.nodes[0].segments.len(), 1); // the first node is open
        assert_eq!(antenna.nodes[12].segments.len(), 1); // the last node is open

        // nodes 1..11 must have incidence 2
        for i in 1..12 {
            assert_eq!(
                antenna.nodes[i].segments.len(),
                2,
                "Node {} has {} segments connected, expected {}",
                i,
                antenna.nodes[i].segments.len(),
                2
            );
        }

        let pulses=compile_pulses(&antenna);

        assert_eq!(pulses.len(),11); // 12 nodes create 11 pulses for the dipole

        for i in 0..11 {
            assert!( (pulses[i].total_length - (5.0/12.0)).abs() < 1e-6); // pulses have 5/12 m
            assert_eq!(pulses[i].center_node, i+1); // the first node has incidence zero!
            assert_eq!(pulses[i].seg_in, i); // the pulse goes from the segment just before the node
            assert_eq!(pulses[i].seg_out, i+1); // to the segment just after the node!
            assert!((pulses[i].unit_in- pulses[i].unit_out).norm() <1e-6); // all pulses are collinear
        }
    }

/*

    #[test]
    fn test_compile_geometry_file_folded_dipole() {
        let file = read_antenna_from_file("TestData/folded.json");
        assert!(file.is_ok());
        let file = file.unwrap();
        let antenna = compile_geometry_file(&file, 20.0);
        assert!(antenna.is_ok());
        let antenna = antenna.unwrap();

        assert_eq!(antenna.wire_map.len(), 4); // There is 4 wires in the test file
        assert_eq!(antenna.nodes.len(), 28); // There should be 28 nodes
        assert_eq!(antenna.segments.len(), 28); // There should be 28 segments
        for i in 0..27 {
            assert_eq!(
                antenna.nodes[i].segments.len(),
                2,
                "Node {} has {} segments connected, expected {}",
                i,
                antenna.nodes[i].segments.len(),
                2
            );
        }
    }
    #[test]
    fn test_compile_geometry_file_vertical() {
        let file = read_antenna_from_file("TestData/vertical.json");
        assert!(file.is_ok());
        let file = file.unwrap();
        let antenna = compile_geometry_file(&file, 20.0);
        assert!(antenna.is_ok());
        let antenna = antenna.unwrap();

        assert_eq!(antenna.wire_map.len(), 5); // There is 4 wires in the test file
        assert_eq!(antenna.nodes.len(), 23); // There should be 28 nodes
        assert_eq!(antenna.segments.len(), 22); // There should be 28 segments
        for i in 0..23 {
            let mut incidence = 2;
            if i == 0 {
                incidence = 5;
            }
            if (i == 6) || (i == 10) || (i == 14) || (i == 18) || (i == 22) {
                incidence = 1;
            }

            assert_eq!(
                antenna.nodes[i].segments.len(),
                incidence,
                "Node {} has {} segments connected, expected {}",
                i,
                antenna.nodes[i].segments.len(),
                incidence
            );
        }
    } */
}
