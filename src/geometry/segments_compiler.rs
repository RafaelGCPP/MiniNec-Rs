use super::*;
use approx::relative_eq;
use nalgebra::Point3;
use num_complex::Complex;
use physical_constants::SPEED_OF_LIGHT_IN_VACUUM;
use std::collections::HashMap;

const C0: f64 = SPEED_OF_LIGHT_IN_VACUUM;
const NODE_MERGE_EPS: f64 = 1.0e-9;

type NodeKey = (i64, i64, i64);

fn node_key(p: &Point3<f64>) -> NodeKey {
    (
        (p.x / NODE_MERGE_EPS).round() as i64,
        (p.y / NODE_MERGE_EPS).round() as i64,
        (p.z / NODE_MERGE_EPS).round() as i64,
    )
}

/// Searches for a node in the existing nodes vector that has approximately the same coordinate.
/// If a close enough node is found (using relative_eq!), returns its index and increments its incidence count.
/// Otherwise, inserts a new node and returns its index.
///
/// # Parameters
/// - `p`: The coordinate to be searched
/// - `nodes`: Mutable reference to the vector of existing nodes.
///
/// # Returns
/// The index of the existing or newly inserted node in the vector.
fn push_node(
    p: Point3<f64>,
    nodes: &mut Vec<Node>,
    node_index: &mut HashMap<NodeKey, Vec<usize>>,
) -> usize {
    let (kx, ky, kz) = node_key(&p);

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                let neighbor_key = (kx + dx, ky + dy, kz + dz);
                if let Some(candidates) = node_index.get(&neighbor_key) {
                    for &idx in candidates {
                        if relative_eq!(nodes[idx].p, p, epsilon = NODE_MERGE_EPS) {
                            return idx;
                        }
                    }
                }
            }
        }
    }

    let new_node = Node {
        p,
        segments: Vec::new(),
    };
    nodes.push(new_node);
    let idx = nodes.len() - 1;
    node_index.entry((kx, ky, kz)).or_default().push(idx);
    idx
}

/// Segments a line between two points into smaller segments based on the target segment size.
///
/// # Parameters
/// - `p1`: Starting coordinate of the wire
/// - `p2`: Ending coordinate of the wire
/// - `target_size`: the largest segment size allowed
/// - `nodes`: Mutable reference to the vector of existing nodes.
/// - `segments`: Mutable reference to the vector of segments, where each segment is represented as a tuple of node indices.
///
/// # Returns
/// A list of line nodes
fn segment_line(
    p1: Point3<f64>,
    p2: Point3<f64>,
    radius: f64,
    target_size: f64,
    nodes: &mut Vec<Node>,
    node_index: &mut HashMap<NodeKey, Vec<usize>>,
    segments: &mut Vec<Segment>,
) -> Vec<usize> {
    let vector = p2 - p1;
    let length = vector.norm();
    let n = (length / target_size).ceil() as usize;
    let n = n.max(1);

    let mut result = Vec::new();
    let mut last_node_idx = 0;

    for i in 0..n {
        // Evaluate the start and end point of the segment
        let frac_a = i as f64 / n as f64;
        let frac_b = (i + 1) as f64 / n as f64;

        let p_a = p1 + vector * frac_a;
        let p_b = p1 + vector * frac_b;

        // Each segment pushes both nodes
        let idx_a = push_node(p_a, nodes, node_index);
        let idx_b = push_node(p_b, nodes, node_index);

        result.push(idx_a);

        let s_vector = p_b - p_a;
        let segment = Segment {
            nodes: (idx_a, idx_b),
            midpoint: nalgebra::center(&p_a, &p_b),
            radius,
            length: s_vector.norm(),
            unit_vector: s_vector.normalize(),
        };

        segments.push(segment);

        // Reverse link, used for pulse generation
        let idx_seg = segments.len() - 1;
        nodes[idx_a].segments.push(idx_seg);
        nodes[idx_b].segments.push(idx_seg);

        // Update the last node
        last_node_idx = idx_b;
    }
    result.push(last_node_idx);

    result
}

/// Compiles the antenna description returning an antenna abstraction with nodes and segments.
///
/// # Parameters
/// - `file`: A reference to the `AntennaFile` object read from the JSON file
/// - `segment_size_divider`: A factor that determines the segment size as a fraction of the wavelength (lambda).
///                           For example, if set to 20, the segment size will be lambda/20.
///
/// # Returns
/// An `Antenna` struct containing the nodes, segments, and wire metadata.
pub fn compile_geometry_file(
    file: &AntennaFile,
    segment_size_divider: f64,
) -> Result<Antenna, AntennaFileError> {
    let mut nodes = Vec::new();
    let mut node_index = HashMap::new();
    let mut segments = Vec::new();
    let mut wire_map = HashMap::new();

    let height = &file.added_height;

    let lambda = C0 / file.frequency;
    let segment_size = lambda / segment_size_divider;

    for wire in &file.wires {
        let p_start = Point3::new(wire.start.x, wire.start.y, wire.start.z + height);
        let p_end = Point3::new(wire.end.x, wire.end.y, wire.end.z + height);
        let midpoint = nalgebra::center(&p_start, &p_end);
        let radius = wire.diameter / 2.0;
        let first_half = segment_line(
            p_start,
            midpoint,
            radius,
            segment_size,
            &mut nodes,
            &mut node_index,
            &mut segments,
        );
        let second_half = segment_line(
            midpoint,
            p_end,
            radius,
            segment_size,
            &mut nodes,
            &mut node_index,
            &mut segments,
        );
        let wire_nodes = first_half
            .iter()
            .chain(second_half.iter().skip(1))
            .copied()
            .collect();
        let wire_metadata = WireMetadata {
            nodes: wire_nodes,
            middle_node: *first_half.last().unwrap(),
        };

        wire_map.insert(wire.id.clone(), wire_metadata);
    }

    let sources = collect_sources(file, &wire_map, &nodes)?;

    Ok(Antenna {
        nodes,
        segments,
        sources,
        wire_map,
    })
}

/// Extracts sources from the antenna file struct and assigns them to nodes.
///
/// # Parameters
/// - `file`: A reference to the `AntennaFile` object read from the JSON file
/// - `wire_map`: A mutable reference to the wire metadata map, used to find the node indices for the sources.
///
/// # Returns
/// A `Result` containing a vector of `VoltageSource` values on success, or an
/// `AntennaFileError` if a source cannot be mapped to a valid node.
fn collect_sources(
    file: &AntennaFile,
    wire_map: &HashMap<String, WireMetadata>,
    nodes: &[Node]
) -> Result<Vec<VoltageSource>, AntennaFileError> {
    let mut sources = Vec::new();

    for source in &file.sources {
        let voltage = Complex::from_polar(source.amplitude, source.phase);

        let wire_metadata = wire_map.get(&source.wire_id).ok_or_else(|| {
            AntennaFileError::Compile(format!("Source references unknown wire id: {}", source.wire_id))
        })?;

        let node_index = match source.position {
            SourcePosition::Start => {
                let mut node_iter = wire_metadata.nodes.iter();
                let first_node = *node_iter.next().ok_or_else(|| {
                    AntennaFileError::Compile(format!("Wire {} has no nodes", source.wire_id))
                })?;

                if nodes[first_node].segments.len() <= 1 {
                    node_iter.next().copied().unwrap_or(first_node)
                } else {
                    first_node
                }
            },

            SourcePosition::Center => wire_metadata.middle_node,

            SourcePosition::End => {
                let mut node_iter = wire_metadata.nodes.iter().rev();
                let last_node = *node_iter.next().ok_or_else(|| {
                    AntennaFileError::Compile(format!("Wire {} has no nodes", source.wire_id))
                })?;

                if nodes[last_node].segments.len() <= 1 {
                    node_iter.next().copied().unwrap_or(last_node)
                } else {
                    last_node
                }
            },
        };

        sources.push(VoltageSource { node_index, voltage });
    }
    Ok(sources)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::geometry_file::read_antenna_from_file;

    #[test]
    fn test_compile_geometry_file_dipole() {
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
        for i in 1..11 {
            assert_eq!(
                antenna.nodes[i].segments.len(),
                2,
                "Node {} has {} segments connected, expected {}",
                i,
                antenna.nodes[i].segments.len(),
                2
            );
        }
        assert_eq!(antenna.sources.len(), 1);
        assert_eq!(antenna.sources[0].node_index,6);

        let wire = antenna.wire_map.get("wire1");
        assert!(wire.is_some(), "Expected wire1 in wire_map");
        let wire = wire.unwrap();
        let expected_nodes: Vec<usize> = (0..13).collect();
        assert_eq!(wire.nodes, expected_nodes);

    }

    #[test]
    fn test_compile_geometry_file_bad_source() {
        let file = read_antenna_from_file("TestData/badsource.json");
        assert!(file.is_ok());
        let file = file.unwrap();

        let antenna = compile_geometry_file(&file, 20.0);
        assert!(
            matches!(antenna, Err(AntennaFileError::Compile(_))),
            "Should be Compile(), but found: {:?}",
            antenna
        );
    }

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
        assert_eq!(antenna.sources.len(), 1);
        assert_eq!(antenna.sources[0].node_index,6);


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

        assert_eq!(antenna.sources.len(), 1);
        assert_eq!(antenna.sources[0].node_index,0);
    }

    #[test]
    fn test_compile_geometry_file_end_fed() {
        let file = read_antenna_from_file("TestData/endfed.json");
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
        for i in 1..11 {
            assert_eq!(
                antenna.nodes[i].segments.len(),
                2,
                "Node {} has {} segments connected, expected {}",
                i,
                antenna.nodes[i].segments.len(),
                2
            );
        }
        assert_eq!(antenna.sources.len(), 1);
        assert_eq!(antenna.sources[0].node_index,11);
        assert_eq!(antenna.nodes[antenna.sources[0].node_index].segments.len(), 2);
    }

}
