# Future Integration: ternary-lattice

## Current State
Provides lattice structures for ternary values {-1, 0, +1}: partial orders, semilattice operations (meet/join), lattice maps (`BTreeMap`-backed), and lattice morphisms. `no_std` compatible, suitable for bare-metal targets.

## Integration Opportunities

### With ternary-cell (Room Hierarchies)
Every `CellGrid` in ternary-cell forms a spatial lattice. ternary-lattice's `TernaryLattice` meet/join operations can define how room states merge when cells synchronize. A `LatticeMap<Ternary, CellState>` maps directly to the room's tile store — keys are cell coordinates, values are ternary-encoded states. The information ordering (0 = unknown/bottom, ±1 = concrete) matches ternary-cell's predict→perceive cycle: predictions start at Zero (unknown) and resolve to Neg/Pos during perception.

### With construct-core (Capability Ordering)
construct-core's `SkillSpec` capabilities form a lattice — some skills subsume others. `LatticeMorphism` can express capability refinement: a Layer 2 async skill morphs into a Layer 1 sync skill (information loss). The partial order becomes a skill dependency graph where `meet` = shared capability subset, `join` = combined capability superset.

### With ternary-diff (Structured Merge)
ternary-diff's three-way merge needs a semilattice for conflict resolution. `TernaryLattice::join(a, b)` provides a deterministic merge when both branches modified the same cell — the join selects the "most informative" value, preferring concrete (±1) over unknown (0).

## Potential in Mature Systems
In room-as-codespace, each room has a lattice position. PLATO's room registry becomes a lattice where rooms are partially ordered by capability — a "sensor processing" room subsumes a "basic monitoring" room. Agent navigation through rooms follows lattice paths: you can only move "up" (more capable) or "down" (more restricted). The ESP32 runs a single-element lattice (bottom); the DGX runs the top element.

## Cross-Pollination Ideas
- **ternary-topology**: Lattice structure defines the topology of the strategy space. Peaks/valleys in the fitness landscape correspond to join-irreducible elements.
- **ternary-fuzzy**: Fuzzy membership degrees (Low/Medium/High) form a lattice. Fuzzy inference rules become lattice morphisms preserving ordering.
- **Music theory (flux-algebra)**: Tonal gravity is a lattice — tension moves toward resolution. The PLR group operations are lattice automorphisms.

## Dependencies for Next Steps
- Define `RoomLattice` trait mapping room IDs to lattice positions in construct-core
- Add `LatticeMap` serialization for ternary-protocol wire format
- Implement `LatticeMorphism` for skill capability refinement across construct-core layers
- Benchmark lattice operations on ESP32 to verify `no_std` performance targets
