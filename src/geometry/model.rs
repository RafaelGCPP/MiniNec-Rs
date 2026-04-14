use nalgebra::{Point3, Vector3};
use num_complex::Complex;
use serde::Deserialize;
use std::collections::HashMap;

/// Represents a single wire element in the antenna geometry.
#[derive(Deserialize, Debug)]
pub(crate) struct Wire {
    /// Unique identifier for the wire.
    pub(crate) id: String,
    /// Start point of the wire in 3D space.
    pub(crate) start: Point3<f64>,
    /// End point of the wire in 3D space.
    pub(crate) end: Point3<f64>,
    /// Diameter of the wire in meters.
    pub(crate) diameter: f64,
}

/// Supported Ground Types.

#[derive(Deserialize, Debug)]
pub(crate) enum GroundType {
    /// Free Space or no ground
    FreeSpace,
    /// Perfect Ground
    PerfectGround,
    // /// MiniNEC Ground model
    // MiniNec,
}

/// Source
#[derive(Deserialize, Debug)]
pub(crate) struct Source {
    /// Source id
    pub(crate) id: String,
    /// ID of the wire where the source is located
    pub(crate) wire_id: String,
    /// Position on the wire
    pub(crate) position: SourcePosition,
    /// Voltage amplitude
    pub(crate) amplitude: f64,
    /// Source phase
    pub(crate) phase: f64,
}

/// Position of a source relative to a wire
#[derive(Deserialize, Debug)]
pub(crate) enum SourcePosition {
    /// starting node
    Start,
    /// midpoint
    Center,
    /// ending node
    End,
}

/// Represents the antenna geometry and simulation parameters loaded from a JSON file.
#[derive(Deserialize, Debug)]
pub(crate) struct AntennaFile {
    /// List of wires that make up the antenna.
    pub(crate) wires: Vec<Wire>,
    /// Ground type or model (e.g., "free_space", "perfect_ground").
    pub(crate) ground: GroundType,
    /// Simulation frequency in Hz.
    pub(crate) frequency: f64,
    /// Additional height above ground in meters.
    pub(crate) added_height: f64,
    /// List of sources
    pub(crate) sources: Vec<Source>,
}

/// An antenna node composed by its coordinates and incidence
#[derive(Clone, Debug)]
pub(crate) struct Node {
    /// Node coordinate
    pub(crate) p: Point3<f64>,
    /// List of connecting segments
    pub(crate) segments: Vec<usize>,
}

/// The wire metadata, pointing at which nodes the wire starts, ends and its middle point (used for feeding).
#[derive(Clone, Copy, Debug)]
pub(crate) struct WireMetadata {
    /// Index of the first node of the wire
    pub(crate) first_node: usize,
    /// Index of the center node of the wire
    pub(crate) middle_node: usize,
    /// Index of the last node of the wire
    pub(crate) last_node: usize,
}

/// Antenna representation composed by its nodes, segments and a map of wire metadata for each wire id.
#[derive(Clone, Debug)]
pub(crate) struct Antenna {
    /// Antenna node list
    pub(crate) nodes: Vec<Node>,
    /// Antenna segments
    pub(crate) segments: Vec<Segment>,
    /// Voltage sources
    pub(crate) sources: Vec<VoltageSource>,
    /// Map of wire metadata for each wire id
    pub(crate) wire_map: HashMap<String, WireMetadata>,
}

/// Segment representation
#[derive(Clone, Debug)]
pub(crate) struct Segment {
    /// start and end node indices
    pub(crate) nodes: (usize, usize),
    /// midpoint of the segment, used for field evaluation
    pub(crate) midpoint: Point3<f64>,
    /// radius of the wire at the segment
    pub(crate) radius: f64,
    /// segment length
    pub(crate) length: f64,
    /// direction vector
    pub(crate) unit_vector: Vector3<f64>,
}

/// Voltage source representation, composed by the node index where the source
/// is applied and the complex voltage value.
#[derive(Clone, Debug)]
pub(crate) struct VoltageSource {
    // Node index where the gap is present
    pub(crate) node_index: usize,
    // Complex voltage amplitude.
    pub(crate) voltage: Complex<f64>,
}

/// Current pulse structure for Z matrix
#[derive(Clone, Debug)]
pub(crate) struct Pulse {
    /// Node at the center of the pulse
    pub(crate) center_node: usize,
    /// Incoming segment
    pub(crate) seg_in: usize,
    /// Outgoing segment
    pub(crate) seg_out: usize,
    /// pulse length
    pub(crate) total_length: f64,
    /// unit vector for the incoming segment
    pub(crate) unit_in: Vector3<f64>,
    /// unit vector for the outgoing segment
    pub(crate) unit_out: Vector3<f64>,
}
