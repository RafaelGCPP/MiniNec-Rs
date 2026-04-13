use super::AntennaFile;
use approx::relative_eq;
use nalgebra::{Point3, Vector3};
use physical_constants;
use std::collections::HashMap;

/// An antenna node composed by its coordinates and incidence
#[derive(Clone, Copy, Debug)]
pub struct Node {
    /// Node coordinate
    p: Point3<f64>,
    /// Number of segments connecting
    incidence: usize,
}

/// The wire metadata, pointing at which nodes the wire starts, ends and its middle point (used for feeding).
#[derive(Clone, Copy, Debug)]
pub struct WireMetadata {
    /// Index of the first node of the wire
    pub first_node: usize,
    /// Index of the center node of the wire
    pub middle_node: usize,
    /// Index of the last node of the wire
    pub last_node: usize,
}

/// Antenna representation composed by its nodes, segments and a map of wire metadata for each wire id.
#[derive(Clone, Debug)]
pub struct Antenna {
    /// Antenna node list
    pub nodes: Vec<Node>,
    /// Antenna segments
    pub segments: Vec<Segment>,
    /// Map of wire metadata for each wire id
    pub wire_map: HashMap<String, WireMetadata>,
}

/// Segment representation
#[derive(Clone, Debug)]
pub struct Segment {
    /// start and end node indices
    pub nodes: (usize, usize),
    /// midpoint of the segment, used for field evaluation
    pub midpoint: Point3<f64>,
    /// radius of the wire at the segment
    pub radius: f64,
    /// segment length
    pub length: f64,
    /// direction vector
    pub unit_vector: Vector3<f64>,
}

const C0: f64 = physical_constants::SPEED_OF_LIGHT_IN_VACUUM;

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
fn push_node(p: Point3<f64>, nodes: &mut Vec<Node>) -> usize {
    if let Some(pos) = nodes.iter().position(|n| relative_eq!(n.p, p)) {
        nodes[pos].incidence += 1;
        pos
    } else {
        let new_node = Node { p: p, incidence: 1 };
        nodes.push(new_node);
        nodes.len() - 1
    }
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
/// A tuple with the index of the first and last nodes generated.
fn segment_line(
    p1: Point3<f64>,
    p2: Point3<f64>,
    radius: f64,
    target_size: f64,
    nodes: &mut Vec<Node>,
    segments: &mut Vec<Segment>,
) -> (usize, usize) {
    let vector = p2 - p1;
    let length = vector.norm();
    let n = (length / target_size).ceil() as usize;
    let n = n.max(1);

    let mut first_node_idx = None;
    let mut last_node_idx = 0;

    for i in 0..n {
        // Evaluate the start and end point of the segment
        let frac_a = i as f64 / n as f64;
        let frac_b = (i + 1) as f64 / n as f64;

        let p_a = p1 + vector * frac_a;
        let p_b = p1 + vector * frac_b;

        // Each segment pushes both nodes
        let idx_a = push_node(p_a, nodes);
        let idx_b = push_node(p_b, nodes);

        let s_vector=p_b-p_a;
        let segment = Segment {
            nodes: (idx_a,idx_b),
            midpoint: nalgebra::center(&p_a, &p_b),
            radius,
            length: s_vector.norm(),
            unit_vector: s_vector.normalize(),
        };

        segments.push(segment);

        // Captures the first node
        if i == 0 {
            first_node_idx = Some(idx_a);
        }
        // Update the last node
        last_node_idx = idx_b;
    }

    (first_node_idx.unwrap(), last_node_idx)
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
fn compile_geometry_file(file: &AntennaFile, segment_size_divider: f64) -> Antenna {
    let mut nodes = Vec::new();
    let mut segments = Vec::new();
    let mut wire_map = HashMap::new();

    let height = &file.added_height;

    let lambda = C0 / &file.frequency;
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
            &mut segments,
        );
        let second_half = segment_line(
            midpoint,
            p_end,
            radius,
            segment_size,
            &mut nodes,
            &mut segments,
        );
        let wire_metadata = WireMetadata {
            first_node: first_half.0,
            middle_node: first_half.1, // should be the same as second_half.0
            last_node: second_half.1,
        };
        println!(
            "Inserting wire: id = {:?}, metadata = {:?}",
            wire.id, wire_metadata
        );
        wire_map.insert(wire.id.clone(), wire_metadata);
        println!("Current wire_map state:");
        for (k, v) in wire_map.iter() {
            println!("  id = {:?}, metadata = {:?}", k, v);
        }
    }

    Antenna {
        nodes,
        segments,
        wire_map,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::read_antenna_from_file;

    #[test]
    fn test_compile_geometry_file_dipole() {
        let file = read_antenna_from_file("TestData/antenna.json").unwrap();
        let antenna = compile_geometry_file(&file, 20.0);

        assert_eq!(antenna.wire_map.len(), 1); // There is only one wire in the test file
        assert_eq!(antenna.nodes.len(), 13); // There should be 13 nodes
        assert_eq!(antenna.segments.len(), 12); // There should be 13 segments
        assert_eq!(antenna.nodes[0].incidence, 1); // the first node is open
        assert_eq!(antenna.nodes[12].incidence, 1); // the last node is open

        // nodes 1..11 must have incidence 2
        for i in 1..11 {
            assert_eq!(
                antenna.nodes[i].incidence, 2,
                "Node {} has incidence {}, expected 2",
                i, antenna.nodes[i].incidence
            );
        }
    }

    #[test]
    fn test_compile_geometry_file_folded_dipole() {
        let file = read_antenna_from_file("TestData/folded.json").unwrap();
        let antenna = compile_geometry_file(&file, 20.0);

        assert_eq!(antenna.wire_map.len(), 4); // There is 4 wires in the test file
        assert_eq!(antenna.nodes.len(), 28); // There should be 28 nodes
        assert_eq!(antenna.segments.len(), 28); // There should be 28 segments
        for i in 0..27 {
            assert_eq!(
                antenna.nodes[i].incidence, 2,
                "Node {} has incidence {}, expected 2",
                i, antenna.nodes[i].incidence
            );
        }
    }
    #[test]
    fn test_compile_geometry_file_vertical() {
        let file = read_antenna_from_file("TestData/vertical.json").unwrap();
        let antenna = compile_geometry_file(&file, 20.0);

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
                antenna.nodes[i].incidence, incidence,
                "Node {} has incidence {}, expected 2",
                i, antenna.nodes[i].incidence
            );
        }
    }
}
