# MiniNEC Simulation Engine

## Overview
This project consists of the development of an electromagnetic simulation engine for wire antennas, based on the **MiniNEC** algorithm. The main objective is to create a modern, modular, and high-performance implementation, capable of running initially in desktop environments (PC) and, subsequently, on low-power embedded systems.

The simulation uses the **Method of Moments (MoM)** to solve the Electric Field Integral Equation (EFIE), allowing for the analysis of input impedance, current distribution, and far-field radiation patterns.

## Project Objectives
1.  **Method of Moments Study:** Understand the discretization of radiant structures and the assembly of complex interaction matrices.
2.  **Modular Architecture:** Develop a platform-independent calculation core, facilitating transitions between languages (Rust/C) and hardware.
3.  **Computational Performance:** Utilize high-performance linear algebra libraries (LAPACK/MKL) to optimize the solution of dense linear systems.
4.  **Embedded Portability:** Port the calculation engine to the ESP32-S3 microcontroller, exploring the use of PSRAM and floating-point acceleration.

## Development Roadmap

### Phase 1: Simulation Core (Free Space)
- [ ] **Input Interface:** Implementation of a JSON parser for geometric description (wires, radii, segmentation) and excitation sources.
- [ ] **Geometry Generator:** Transformation of wire coordinates into discrete segments and node connectivity mapping.
- [ ] **Matrix Filler:** Implementation of the impedance matrix $[Z]$ calculation using Gaussian Quadrature and thin-wire approximation for self-impedance.
- [ ] **Linear Solver:** Integration with LAPACK/MKL for solving the system $[Z][I] = [V]$ via LU decomposition.
- [ ] **Post-Processing:** Calculation of input impedance, VSWR, and far-field integration for gain in dBi.

### Phase 2: Advanced Physics and Environment
- [ ] **Perfect Ground:** Implementation of the Image Method in the geometry generator and radiation calculation.
- [ ] **Lumped Loads:** Support for including passive components ($R, L, C$) in specific segments.
- [ ] **Variable Diameter Junctions:** Implementation of corrections for telescopic elements and radius transitions.
- [ ] **Real Ground:** Inclusion of the reflection coefficient model (MiniNEC Ground) for simulations near the earth.

### Phase 3: Optimization and Interface
- [ ] **Port to ESP32-S3:** Adaptation of the code for embedded environment, memory optimization, and use of PSRAM.
- [ ] **Web Visualization:** Exporting data for rendering polar and 3D diagrams via a browser-based interface.

## Technical Requirements (Desktop Development)
* **Language:** Rust 
* **Linear Algebra:** Intel MKL / LAPACK
* **Data Format:** JSON for model and results persistence.

## Reasoning on some choices

* ### Why Rust?
I needed a toy project to learn Rust, and this seemed like a fun and challenging problem to solve. 
Rust's performance and safety features make it an excellent choice for computationally intensive tasks.

* ### Why MiniNEC?
MiniNEC is a well-known algorithm in the field of antenna simulation, and it provides some great approximation
when compared to more complex methods, like NEC2 or NEC4. It is also relatively simple to implement and accurate for
simple antennas, making it a good starting point for learning and experimentation.

* ### Why ESP32-S3?
The ESP32-S3 is a powerful microcontroller with integrated Wi-Fi capabilities, but I'll eventually do it just for the giggles.
The first MiniNEC was implemented on a 64Kb Apple II computer, so it should be possible to run it on a modern microcontroller with much more resources. 
It also has support for PSRAM, which can be useful for handling larger matrices and thus larger models.