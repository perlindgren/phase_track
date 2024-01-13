# Phase Track

A crate for tracking relative position.

## Problem to solve

Relative position tracking is a common problem:

- Mouse wheel (quadrature detection).

  Here we are concerned with coarse granular angle, so a discrete relative position is sufficient. Using two phase shifted signals/sensors we can detect stepwise changes of direction.

- Vinyl record (two phase shifted sinusoidal signals encoded as left/right stereo waveform). Hugely successful, "Digital Vinyl Systems" are widely used by both live DJs and in music production.
  
  - Robust and accurate tracking
  - Flexibility (you can replicate any sound/song even if not available on vinyl).
  
## Solution

- Well known tracking problem solution by phase unwrapping the two phase shifted signals.
  
- Extremely robust, even under extreme condition.
  
## Modelling and simulation

- Octave (Matlab like tool). Rapid prototyping with visualization.
  
## Implementation

- 1-1 manual translation to Rust code, using the [libm](https://crates.io/crates/libm) crate for high accuracy atan2 computations and [rand](https://crates.io/crates/rand) for noise (with uniform distribution).

- Visualization using the [plotters](https://crates.io/crates/plotters) crate.

- Quality assessment and validation against golden model using [csv](https://crates.io/crates/csv).

  Excellent correlation between Rust and octave implementations. [libm](https://crates.io/crates/libm) atan2 claims to produce correct results even for angles near pi/2 or -pi/2 (that is, when x is near 0) and avoids spurious underflows.

## License

This project is dual-licensed under MIT and Apache 2.0.
