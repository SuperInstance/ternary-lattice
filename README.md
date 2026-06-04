# ternary-lattice

Lattice structures for ternary values — partial orders, semilattice operations, lattice maps, and morphisms over the domain {-1, 0, +1}.

## Why This Exists

Ternary logic shows up in fuzzy systems, three-valued logics, signal processing, and decision models — but the algebraic backbone (lattices, orders, morphisms) rarely gets a dedicated treatment. This crate provides that foundation: well-defined partial orders, meet/join operations, pointwise maps, and order-preserving morphisms, all on the compact ternary domain. It's `no_std` and `forbid(unsafe_code)` so it works in embedded and safety-critical contexts.

## Core Concepts

- **Ternary values**: `Neg` (-1), `Zero` (0), `Pos` (+1) — the three-element domain.
- **Flat ordering** (information ordering): `Zero` is bottom (unknown), `Neg` and `Pos` are incomparable concrete values.
- **Linear ordering** (numeric): `-1 ≤ 0 ≤ +1` — a total order.
- **Meet / Join**: Greatest lower bound and least upper bound under either ordering.
- **MeetSemilattice / JoinSemilattice**: Collect elements and compute the meet or join of the entire set.
- **LatticeMap**: A BTreeMap keyed by ternary lattice points, with pointwise merge operations.
- **LatticeMorphism**: Order-preserving (monotone) functions between ternary lattices, with composition and monotonicity checking.
- **Structural metrics**: `chain_height` and `lattice_width` for each ordering.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-lattice = "0.1"
```

```rust
use ternary_lattice::{Ternary, TernaryLattice, LatticeOrder, LatticeMorphism, MeetSemilattice};

fn main() {
    // Create a flat-ordered lattice
    let lattice = TernaryLattice::new(LatticeOrder::Flat);

    // Meet and join operations
    let meet = lattice.meet(Ternary::Neg, Ternary::Pos); // Zero (incomparable → bottom)
    let join = lattice.join(Ternary::Zero, Ternary::Pos); // Pos (bottom ∨ x = x)

    // Semilattice: accumulate values and compute the global meet
    let mut sl = MeetSemilattice::new(LatticeOrder::Flat);
    sl.insert(Ternary::Pos);
    sl.insert(Ternary::Pos);
    sl.insert(Ternary::Neg);
    assert_eq!(sl.meet_all(), Some(Ternary::Zero)); // conflict → bottom

    // Morphisms: compose and check monotonicity
    let neg = LatticeMorphism::negation(LatticeOrder::Linear);
    let double_neg = neg.compose(&neg); // identity
    assert!(!neg.is_monotone()); // negation reverses linear order
}
```

## API Overview

| Type | Description |
|---|---|
| `Ternary` | The fundamental value: `Neg`, `Zero`, `Pos` |
| `LatticeOrder` | `Flat` (information) or `Linear` (numeric) ordering |
| `TernaryLattice` | Lattice with `le`, `meet`, `join`, `bottom`, `top` |
| `MeetSemilattice` | Accumulate values, compute global meet |
| `JoinSemilattice` | Accumulate values, compute global join |
| `LatticeMap<V>` | Map from ternary points to values, with `merge_meet` |
| `LatticeMorphism` | Monotone map between lattices, with `compose` and `is_monotone` |
| `chain_height` / `lattice_width` | Structural metrics |

## How It Works

The crate defines two partial orders on the three-element set. Under **flat ordering**, `Zero` acts as bottom (unknown/undefined) while `Neg` and `Pos` are incomparable — this mirrors three-valued logic (Kleene). Under **linear ordering**, the natural numeric total order is used.

Meet and join are computed pointwise under the chosen ordering. The `MeetSemilattice` and `JoinSemilattice` types fold over collections of ternary values. `LatticeMap` stores arbitrary values keyed by ternary positions and supports merge with conflict detection. `LatticeMorphism` encodes a function from one lattice to another (identity, constant, negation, or custom) and verifies monotonicity by checking all pairs under the source ordering.

## Use Cases

- **Three-valued logic engines**: Use the flat ordering as the algebraic backbone for Kleene-style logics where `0` means "unknown."
- **Fuzzy/uncertainty aggregation**: Merge multiple ternary signals (approve/abstain/reject) using meet/join with conflict detection.
- **Formal verification**: Lattice morphisms and monotonicity checking provide a lightweight framework for proving properties of ternary transformations.
- **Embedded signal classification**: `no_std` compatible — use lattice operations for ternary sensor fusion on microcontrollers.

## Known Limitations

- **Flat ordering treats conflicting inputs as bottom**: Under `LatticeOrder::Flat`, the meet of `Neg` and `Pos` is `Zero` (bottom). This means if two sources disagree (one says Neg, one says Pos), the result is "unknown" — information is discarded rather than preserved as a conflict flag. Use `LatticeMap::merge_meet()` and check for key collisions if you need conflict detection.

- **`LatticeMorphism::is_monotone()` is O(n²)**: Monotonicity checking tests all pairs of input values under the source ordering. For the ternary domain this is fine (3² = 9 pairs), but the implementation doesn't generalize — it hardcodes the three-element lattice, so composing morphisms with `compose()` and re-checking monotonicity always runs the same 9 checks regardless of the composed function's structure.

- **`LatticeMap` uses `BTreeMap`, not a flat array**: Since there are only 3 possible keys (Neg, Zero, Pos), using `BTreeMap<Ternary, V>` allocates heap memory for tree nodes. A flat `[Option<V>; 3]` array would be more efficient and avoid allocation, which matters in `no_std` environments.

- **No Galois connection support**: The crate provides morphisms (functions between lattices) but not Galois connections (adjoint pairs of monotone functions), which are the standard tool for abstract interpretation and program analysis.

- **`MeetSemilattice` and `JoinSemilattice` don't track insertion order**: The `meet_all()` / `join_all()` results depend only on the set of inserted values, not their order. If you need to know *when* a conflict occurred, you must track it externally.

## Ecosystem

Part of the **SuperInstance** ternary computing suite:

- `ternary-lattice` — this crate
- `ternary-codes` — error-correcting codes for ternary data
- `ternary-gradient` — gradient-free optimization on ternary landscapes
- `ternary-language` — ternary NLP and grammar processing
- `ternary-trees` — ternary decision trees and forests
- `ternary-transform` — wavelet, Fourier, and kernel transforms
- `ternary-planning` — planning and scheduling with ternary priorities
- `ternary-rl` — reinforcement learning with ternary actions
- `ternary-som` — self-organizing maps for ternary data
- `ternary-failure` — failure analysis with ternary classification

## License

MIT
