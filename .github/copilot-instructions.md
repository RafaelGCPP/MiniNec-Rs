# MiniNEC-rs

This is a Rust implementation of the MiniNEC algorithm, a method for simulating wire antennas using the Method of Moments (MoM). The project is designed to be modular and high-performance, with the goal of eventually running on low-power embedded systems like the ESP32-S3 microcontroller.

## Features
- **Input Interface:** Parses geometric descriptions of antennas from JSON files.
- **Geometry Generator:** Converts wire coordinates into discrete segments and maps node connectivity.
- **Matrix Filler:** Calculates the impedance matrix using Gaussian Quadrature and thin-wire approximation.
- **Linear Solver:** Integrates with high-performance linear algebra libraries (LAPACK/MKL) to solve the system of equations for current distribution.

## Rules of the repository

- English is the main language for code comments, documentation, and commit messages.
- Contributions should be made in English to maintain consistency and accessibility for the global community.
- All code should follow Rust's standard formatting and best practices.
- Pull requests should include clear descriptions of changes and reference any related issues.

## Coding style
- Follow Rust's standard coding conventions and best practices.
- Use descriptive variable and function names for clarity.
- Include comments to explain complex logic and algorithms.
- All methods must include documentation comments (///) to describe their purpose, parameters, and return values.
- Code must be in English, including comments and documentation.

