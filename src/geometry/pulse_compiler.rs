use super::*;
use num_complex::Complex;

pub fn compile_pulses(antenna: &Antenna) -> Result<Vec<Pulse>, AntennaFileError> {
    let mut pulses = Vec::new();

    let nodes = &antenna.nodes;

    for (i, node) in nodes.iter().enumerate() {
        let n_segments = node.segments.len();

        if n_segments == 1 {
            continue;
        }

        let seg_in = node.segments[0];

        // we need unit_in to point TO center_node
        let unit_in = {
            if antenna.segments[seg_in].nodes.1 == i {
                antenna.segments[seg_in].unit_vector
            } else {
                -antenna.segments[seg_in].unit_vector
            }
        };

        let seg_in_half_length = antenna.segments[seg_in].length / 2.0;
        let seg_in_radius = antenna.segments[seg_in].radius;
        let seg_in_midpoint = antenna.segments[seg_in].midpoint;

        for seg_idx in 1..n_segments {

            // all segments must leave center_node
            let seg_out = node.segments[seg_idx];
            let unit_out = {
                if antenna.segments[seg_out].nodes.0 == i {
                    antenna.segments[seg_out].unit_vector
                } else {
                    -antenna.segments[seg_out].unit_vector
                }
            };

            let seg_out_half_length = antenna.segments[seg_out].length / 2.0;
            let seg_out_radius = antenna.segments[seg_out].radius;
            let seg_out_midpoint = antenna.segments[seg_out].midpoint;

            let total_length = seg_in_half_length + seg_out_half_length;

            // Get the voltage source for the current node, or zero if none exists
            // This ensures that only nodes with a defined voltage source get a nonzero value
            let voltage_source = antenna.sources.get(&i)
                .cloned()
                .unwrap_or_else(|| Complex::<f64>::new(0.0, 0.0));

            pulses.push(Pulse {
                center_node_idx: i,
                center_node: antenna.nodes[i].p,
                total_length,

                seg_in,
                seg_in_unit: unit_in,
                seg_in_half_length,
                seg_in_radius,
                seg_in_midpoint,

                seg_out,
                seg_out_unit: unit_out,
                seg_out_half_length,
                seg_out_radius,
                seg_out_midpoint,

                voltage_source,
            })
        }
    }
    Ok(pulses)
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

        let pulses = compile_pulses(&antenna).unwrap();

        assert_eq!(pulses.len(), 11); // 12 nodes create 11 pulses for the dipole

        for i in 0..11 {
            assert!((pulses[i].total_length - (5.0 / 12.0)).abs() < 1e-6); // pulses have 5/12 m
            assert_eq!(pulses[i].center_node_idx, i + 1); // the first node has incidence zero!
            assert!((pulses[i].center_node.x - ((i as f64) - 5.0) * (5.0 / 12.0)).abs() < 1e-6); // pulses have 5/12 m
            assert_eq!(pulses[i].seg_in, i); // the pulse goes from the segment just before the node
            assert_eq!(pulses[i].seg_out, i + 1); // to the segment just after the node!
            assert!((pulses[i].seg_in_unit - pulses[i].seg_out_unit).norm() < 1e-6); // all pulses are collinear
            if i != 5 {
                assert!(pulses[i].voltage_source.norm() < 1e-6); // only the middle pulse has a voltage source
            } else {
                assert!((pulses[i].voltage_source.norm() - 1.0) < 1e-6);
            }
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
