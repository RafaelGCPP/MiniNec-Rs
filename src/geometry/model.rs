use nalgebra::{Point3, Vector3};
use num_complex::Complex;
use serde::Deserialize;
use std::collections::HashMap;

/// Represents a single wire element in the antenna geometry.
#[derive(Deserialize, Debug)]
pub struct Wire {
    /// Unique identifier for the wire.
    pub id: String,
    /// Start point of the wire in 3D space.
    pub start: Point3<f64>,
    /// End point of the wire in 3D space.
    pub end: Point3<f64>,
    /// Diameter of the wire in meters.
    pub diameter: f64,
}

/// Supported Ground Types.

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GroundType {
    /// Free Space or no ground
    FreeSpace,
    /// Perfect Ground
    PerfectGround,
    // /// MiniNEC Ground model
    // MiniNec,
}

/// Source
#[derive(Deserialize, Debug)]
pub struct Source {
    /// Source id
    pub id: String,
    /// ID of the wire where the source is located
    pub wire_id: String,
    /// Position on the wire
    pub position: SourcePosition,
    /// Voltage amplitude
    pub amplitude: f64,
    /// Source phase
    pub phase: f64,
}

/// Position of a source relative to a wire
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourcePosition {
    /// starting node
    Start,
    /// midpoint
    Center,
    /// ending node
    End,
}

/// Represents the antenna geometry and simulation parameters loaded from a JSON file.
#[derive(Deserialize, Debug)]
pub struct AntennaFile {
    /// List of wires that make up the antenna.
    pub wires: Vec<Wire>,
    /// Ground type or model (e.g., "free_space", "perfect_ground").
    pub ground: GroundType,
    /// Simulation frequency in Hz.
    pub frequency: f64,
    /// Additional height above ground in meters.
    pub added_height: f64,
    /// List of sources
    pub sources: Vec<Source>,
}

/// An antenna node composed by its coordinates and incidence
#[derive(Clone, Debug)]
pub struct Node {
    /// Node coordinate
    pub p: Point3<f64>,
    /// List of connecting segments
    pub segments: Vec<usize>,
}

/// The wire metadata, pointing at which nodes the wire starts, ends and its middle point (used for feeding).
#[derive(Clone, Debug)]
pub struct WireMetadata {
    /// All nodes of the wire (in order)
    pub nodes: Vec<usize>,
    /// Index of the center node of the wire
    pub middle_node: usize,
}

/// Antenna representation composed by its nodes, segments and a map of wire metadata for each wire id.
#[derive(Clone, Debug)]
pub struct Antenna {
    /// Antenna node list
    pub nodes: Vec<Node>,
    /// Antenna segments
    pub segments: Vec<Segment>,
    /// Voltage sources indexed by node
    pub sources: HashMap<usize, Complex<f64>>,
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

/// Current pulse structure for Z matrix
#[derive(Clone, Debug)]
pub struct Pulse {

    /// Node at the center of the pulse
    pub center_node_idx: usize,
    /// Center node coordinate
    pub center_node: Point3<f64>,
    /// pulse length
    pub total_length: f64,

    //Input segment

    /// Incoming segment
    pub seg_in: usize,
    /// unit vector for the incoming segment
    pub seg_in_unit: Vector3<f64>,
    /// midpoint of the incoming segment
    pub seg_in_midpoint: Point3<f64>,
    /// half-length or the incoming segment
    pub seg_in_half_length: f64,
    /// radius of the wire at the pulse on the incoming segment
    pub seg_in_radius: f64,

    // Output segment

    /// Outgoing segment
    pub seg_out: usize,
    /// unit vector for the outgoing segment
    pub seg_out_unit: Vector3<f64>,
    /// midpoint of the outgoing segment
    pub seg_out_midpoint: Point3<f64>,
    /// half-length or the outgoing segment
    pub seg_out_half_length: f64,
    /// radius of the wire at the pulse on the outgoing segment
    pub seg_out_radius: f64,

    /// voltage source at pulse
    pub voltage_source: Complex<f64>,
}

/// Problem structure containing all the necessary information for the antenna simulation,
/// including the list of pulses, frequency, nodes, segments and wire metadata.
#[derive(Clone, Debug)]
pub struct Problem {
    /// List of pulses
    pub pulses: Vec<Pulse>,
    /// Operating frequency
    pub frequency: f64,
    // Wave number (k)
    pub wave_number:f64,
    /// List of nodes
    pub nodes: Vec<Node>,
    /// Antenna segments
    pub segments: Vec<Segment>,
    /// Wire to node mapping
    pub wire_map: HashMap<String, WireMetadata>,
}
