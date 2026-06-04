//! Lattice structures for ternary values.
//!
//! Provides partial order, semilattice operations, lattice maps,
//! and morphisms for the ternary domain {-1, 0, +1}.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// A ternary value with lattice structure.
///
/// The partial order is: -1 ≤ 0 ≤ +1 (with -1 and +1 incomparable
/// in some interpretations). We use the "information ordering" where
/// 0 = unknown/bottom, and -1, +1 are concrete values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Ternary {
    /// Bottom / unknown in information ordering.
    Zero = 0,
    /// Definitely negative.
    Neg = -1,
    /// Definitely positive.
    Pos = 1,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn to_i8(self) -> i8 {
        match self {
            Ternary::Neg => -1,
            Ternary::Zero => 0,
            Ternary::Pos => 1,
        }
    }
}

// ── Ternary Lattice ──────────────────────────────────────────────────

/// The ternary lattice with two orderings:
/// - **Flat order**: -1 ⊥ 0 ⊥ +1 (0 is bottom, -1 and +1 incomparable)
/// - **Linear order**: -1 ≤ 0 ≤ +1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatticeOrder {
    /// Information ordering: 0 is bottom, -1 and +1 incomparable.
    Flat,
    /// Numeric ordering: -1 ≤ 0 ≤ +1.
    Linear,
}

/// Ternary lattice with configurable ordering.
#[derive(Clone, Debug)]
pub struct TernaryLattice {
    order: LatticeOrder,
}

impl TernaryLattice {
    pub fn new(order: LatticeOrder) -> Self {
        Self { order }
    }

    pub fn order(&self) -> LatticeOrder {
        self.order
    }

    /// Compare two values under this lattice's ordering.
    /// Returns true if a ≤ b.
    pub fn le(&self, a: Ternary, b: Ternary) -> bool {
        match self.order {
            LatticeOrder::Linear => a.to_i8() <= b.to_i8(),
            LatticeOrder::Flat => {
                if a == b { return true; }
                if a == Ternary::Zero { return true; } // bottom ≤ anything
                false // -1 and +1 incomparable
            }
        }
    }

    /// Meet (greatest lower bound) of two values.
    pub fn meet(&self, a: Ternary, b: Ternary) -> Ternary {
        match self.order {
            LatticeOrder::Linear => {
                if a.to_i8() <= b.to_i8() { a } else { b }
            }
            LatticeOrder::Flat => {
                if a == b { a }
                else if a == Ternary::Zero { a } // bottom
                else if b == Ternary::Zero { b } // bottom
                else { Ternary::Zero } // -1 ∧ +1 = bottom
            }
        }
    }

    /// Join (least upper bound) of two values.
    pub fn join(&self, a: Ternary, b: Ternary) -> Ternary {
        match self.order {
            LatticeOrder::Linear => {
                if a.to_i8() >= b.to_i8() { a } else { b }
            }
            LatticeOrder::Flat => {
                if a == b { a }
                else if a == Ternary::Zero { b } // bottom ∨ x = x
                else if b == Ternary::Zero { a }
                else { Ternary::Zero } // -1 ∨ +1 = bottom (incomparable)
            }
        }
    }

    /// Bottom element.
    pub fn bottom(&self) -> Ternary {
        match self.order {
            LatticeOrder::Flat => Ternary::Zero,
            LatticeOrder::Linear => Ternary::Neg,
        }
    }

    /// Top element (None for flat ordering, Some(Pos) for linear).
    pub fn top(&self) -> Option<Ternary> {
        match self.order {
            LatticeOrder::Flat => None,
            LatticeOrder::Linear => Some(Ternary::Pos),
        }
    }
}

// ── MeetSemilattice & JoinSemilattice ────────────────────────────────

/// A meet-semilattice over a slice of ternary values.
#[derive(Clone, Debug)]
pub struct MeetSemilattice {
    lattice: TernaryLattice,
    elements: Vec<Ternary>,
}

impl MeetSemilattice {
    pub fn new(order: LatticeOrder) -> Self {
        Self {
            lattice: TernaryLattice::new(order),
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: Ternary) {
        self.elements.push(val);
    }

    /// Compute the meet of all elements.
    pub fn meet_all(&self) -> Option<Ternary> {
        if self.elements.is_empty() {
            return None;
        }
        Some(self.elements.iter().copied().fold(Ternary::Pos, |a, b| self.lattice.meet(a, b)))
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// A join-semilattice over a slice of ternary values.
#[derive(Clone, Debug)]
pub struct JoinSemilattice {
    lattice: TernaryLattice,
    elements: Vec<Ternary>,
}

impl JoinSemilattice {
    pub fn new(order: LatticeOrder) -> Self {
        Self {
            lattice: TernaryLattice::new(order),
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: Ternary) {
        self.elements.push(val);
    }

    /// Compute the join of all elements.
    pub fn join_all(&self) -> Option<Ternary> {
        if self.elements.is_empty() {
            return None;
        }
        Some(self.elements.iter().copied().fold(Ternary::Neg, |a, b| self.lattice.join(a, b)))
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

// ── LatticeMap ───────────────────────────────────────────────────────

/// A map from lattice points to values, supporting pointwise lattice operations.
#[derive(Clone, Debug)]
pub struct LatticeMap<V> {
    lattice: TernaryLattice,
    map: BTreeMap<i8, V>,
}

impl<V: Clone> LatticeMap<V> {
    pub fn new(order: LatticeOrder) -> Self {
        Self {
            lattice: TernaryLattice::new(order),
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Ternary, val: V) {
        self.map.insert(key.to_i8(), val);
    }

    pub fn get(&self, key: Ternary) -> Option<&V> {
        self.map.get(&key.to_i8())
    }

    pub fn remove(&mut self, key: Ternary) -> Option<V> {
        self.map.remove(&key.to_i8())
    }

    pub fn keys(&self) -> Vec<Ternary> {
        self.map.keys().filter_map(|&k| Ternary::from_i8(k)).collect()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Pointwise merge two lattice maps using the lattice's meet operation
    /// on keys that exist in both, keeping values from `self` where keys match.
    pub fn merge_meet(&mut self, other: &LatticeMap<V>) -> Vec<(Ternary, V)>
    where
        V: PartialEq,
    {
        let mut conflicts = Vec::new();
        for (&k, v) in &other.map {
            if let Some(self_v) = self.map.get(&k) {
                if self_v != v {
                    if let Some(key) = Ternary::from_i8(k) {
                        conflicts.push((key, v.clone()));
                    }
                }
            } else {
                self.map.insert(k, v.clone());
            }
        }
        conflicts
    }
}

// ── Morphisms ────────────────────────────────────────────────────────

/// A morphism between two ternary lattices.
/// Represents a monotone (order-preserving) function.
#[derive(Clone, Debug)]
pub struct LatticeMorphism {
    source_order: LatticeOrder,
    target_order: LatticeOrder,
    /// Maps each source value to a target value.
    mapping: [Ternary; 3], // indexed by (to_i8() + 1): Neg=0, Zero=1, Pos=2
}

impl LatticeMorphism {
    /// Create a new morphism with explicit mapping.
    /// mapping: [Neg → ?, Zero → ?, Pos → ?]
    pub fn new(
        source_order: LatticeOrder,
        target_order: LatticeOrder,
        neg_map: Ternary,
        zero_map: Ternary,
        pos_map: Ternary,
    ) -> Self {
        Self {
            source_order,
            target_order,
            mapping: [neg_map, zero_map, pos_map],
        }
    }

    /// Identity morphism (same lattice).
    pub fn identity(order: LatticeOrder) -> Self {
        Self {
            source_order: order,
            target_order: order,
            mapping: [Ternary::Neg, Ternary::Zero, Ternary::Pos],
        }
    }

    /// Constant morphism (maps everything to one value).
    pub fn constant(order: LatticeOrder, val: Ternary) -> Self {
        Self {
            source_order: order,
            target_order: order,
            mapping: [val, val, val],
        }
    }

    /// Negation morphism: swaps Neg and Pos, keeps Zero.
    pub fn negation(order: LatticeOrder) -> Self {
        Self {
            source_order: order,
            target_order: order,
            mapping: [Ternary::Pos, Ternary::Zero, Ternary::Neg],
        }
    }

    /// Apply the morphism to a value.
    pub fn apply(&self, val: Ternary) -> Ternary {
        self.mapping[(val.to_i8() + 1) as usize]
    }

    /// Check if this morphism is monotone (order-preserving).
    pub fn is_monotone(&self) -> bool {
        let source = TernaryLattice::new(self.source_order);
        let target = TernaryLattice::new(self.target_order);

        let values = [Ternary::Neg, Ternary::Zero, Ternary::Pos];
        for i in 0..values.len() {
            for j in 0..values.len() {
                if source.le(values[i], values[j]) {
                    let fi = self.apply(values[i]);
                    let fj = self.apply(values[j]);
                    if !target.le(fi, fj) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Compose two morphisms: (self ∘ other).
    pub fn compose(&self, other: &LatticeMorphism) -> LatticeMorphism {
        let new_mapping = [
            self.apply(other.apply(Ternary::Neg)),
            self.apply(other.apply(Ternary::Zero)),
            self.apply(other.apply(Ternary::Pos)),
        ];
        LatticeMorphism {
            source_order: other.source_order,
            target_order: self.target_order,
            mapping: new_mapping,
        }
    }

    /// Convert from flat to linear ordering.
    pub fn flat_to_linear() -> Self {
        Self {
            source_order: LatticeOrder::Flat,
            target_order: LatticeOrder::Linear,
            mapping: [Ternary::Neg, Ternary::Zero, Ternary::Pos],
        }
    }

    /// Convert from linear to flat ordering.
    pub fn linear_to_flat() -> Self {
        Self {
            source_order: LatticeOrder::Linear,
            target_order: LatticeOrder::Flat,
            mapping: [Ternary::Neg, Ternary::Zero, Ternary::Pos],
        }
    }
}

/// Compute the height of a chain in the lattice.
pub fn chain_height(order: LatticeOrder) -> usize {
    match order {
        LatticeOrder::Flat => 2, // bottom → top element (if any)
        LatticeOrder::Linear => 3, // -1 ≤ 0 ≤ +1
    }
}

/// Compute the width (max antichain size) of the lattice.
pub fn lattice_width(order: LatticeOrder) -> usize {
    match order {
        LatticeOrder::Flat => 2, // {-1, +1} are incomparable
        LatticeOrder::Linear => 1, // total order, no incomparable elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn t(v: i8) -> Ternary {
        Ternary::from_i8(v).unwrap()
    }

    // ── TernaryLattice ──
    #[test]
    fn flat_order_comparisons() {
        let lat = TernaryLattice::new(LatticeOrder::Flat);
        assert!(lat.le(t(0), t(1)));
        assert!(lat.le(t(0), t(-1)));
        assert!(lat.le(t(1), t(1)));
        assert!(!lat.le(t(-1), t(1))); // incomparable
        assert!(!lat.le(t(1), t(-1))); // incomparable
    }

    #[test]
    fn linear_order_comparisons() {
        let lat = TernaryLattice::new(LatticeOrder::Linear);
        assert!(lat.le(t(-1), t(0)));
        assert!(lat.le(t(0), t(1)));
        assert!(lat.le(t(-1), t(1)));
    }

    #[test]
    fn flat_meet() {
        let lat = TernaryLattice::new(LatticeOrder::Flat);
        assert_eq!(lat.meet(t(-1), t(1)), t(0)); // incomparable → bottom
        assert_eq!(lat.meet(t(0), t(1)), t(0));  // bottom ∧ x = bottom
        assert_eq!(lat.meet(t(1), t(1)), t(1));
    }

    #[test]
    fn flat_join() {
        let lat = TernaryLattice::new(LatticeOrder::Flat);
        assert_eq!(lat.join(t(-1), t(1)), t(0)); // incomparable
        assert_eq!(lat.join(t(0), t(1)), t(1));  // bottom ∨ x = x
        assert_eq!(lat.join(t(-1), t(-1)), t(-1));
    }

    #[test]
    fn linear_meet_join() {
        let lat = TernaryLattice::new(LatticeOrder::Linear);
        assert_eq!(lat.meet(t(-1), t(1)), t(-1));
        assert_eq!(lat.join(t(-1), t(1)), t(1));
    }

    #[test]
    fn bottom_top() {
        let flat = TernaryLattice::new(LatticeOrder::Flat);
        assert_eq!(flat.bottom(), t(0));
        assert!(flat.top().is_none());

        let linear = TernaryLattice::new(LatticeOrder::Linear);
        assert_eq!(linear.bottom(), t(-1));
        assert_eq!(linear.top(), Some(t(1)));
    }

    // ── MeetSemilattice ──
    #[test]
    fn meet_semilattice_all_same() {
        let mut ms = MeetSemilattice::new(LatticeOrder::Flat);
        ms.insert(t(1));
        ms.insert(t(1));
        ms.insert(t(1));
        assert_eq!(ms.meet_all(), Some(t(1)));
    }

    #[test]
    fn meet_semilattice_conflict() {
        let mut ms = MeetSemilattice::new(LatticeOrder::Flat);
        ms.insert(t(-1));
        ms.insert(t(1));
        assert_eq!(ms.meet_all(), Some(t(0))); // -1 ∧ +1 = bottom
    }

    #[test]
    fn meet_semilattice_empty() {
        let ms = MeetSemilattice::new(LatticeOrder::Flat);
        assert!(ms.meet_all().is_none());
    }

    // ── JoinSemilattice ──
    #[test]
    fn join_semilattice_conflict() {
        let mut js = JoinSemilattice::new(LatticeOrder::Flat);
        js.insert(t(-1));
        js.insert(t(1));
        assert_eq!(js.join_all(), Some(t(0))); // incomparable
    }

    #[test]
    fn join_semilattice_linear() {
        let mut js = JoinSemilattice::new(LatticeOrder::Linear);
        js.insert(t(-1));
        js.insert(t(0));
        js.insert(t(1));
        assert_eq!(js.join_all(), Some(t(1)));
    }

    // ── LatticeMap ──
    #[test]
    fn lattice_map_insert_get() {
        let mut m: LatticeMap<i64> = LatticeMap::new(LatticeOrder::Flat);
        m.insert(t(1), 42);
        m.insert(t(-1), -10);
        assert_eq!(m.get(t(1)), Some(&42));
        assert_eq!(m.get(t(-1)), Some(&-10));
        assert_eq!(m.get(t(0)), None);
    }

    #[test]
    fn lattice_map_remove() {
        let mut m: LatticeMap<i64> = LatticeMap::new(LatticeOrder::Flat);
        m.insert(t(1), 100);
        assert_eq!(m.remove(t(1)), Some(100));
        assert!(m.is_empty());
    }

    #[test]
    fn lattice_map_keys() {
        let mut m: LatticeMap<i64> = LatticeMap::new(LatticeOrder::Flat);
        m.insert(t(-1), 1);
        m.insert(t(1), 2);
        let mut keys = m.keys();
        keys.sort();
        assert_eq!(keys, vec![t(-1), t(1)]);
    }

    // ── Morphisms ──
    #[test]
    fn identity_morphism() {
        let m = LatticeMorphism::identity(LatticeOrder::Linear);
        assert_eq!(m.apply(t(-1)), t(-1));
        assert_eq!(m.apply(t(0)), t(0));
        assert_eq!(m.apply(t(1)), t(1));
        assert!(m.is_monotone());
    }

    #[test]
    fn constant_morphism() {
        let m = LatticeMorphism::constant(LatticeOrder::Linear, t(0));
        assert_eq!(m.apply(t(-1)), t(0));
        assert_eq!(m.apply(t(1)), t(0));
        assert!(m.is_monotone());
    }

    #[test]
    fn negation_morphism() {
        let m = LatticeMorphism::negation(LatticeOrder::Linear);
        assert_eq!(m.apply(t(-1)), t(1));
        assert_eq!(m.apply(t(1)), t(-1));
        assert_eq!(m.apply(t(0)), t(0));
        // Negation reverses order, NOT monotone on linear ordering
        assert!(!m.is_monotone());
    }

    #[test]
    fn compose_morphisms() {
        let neg = LatticeMorphism::negation(LatticeOrder::Linear);
        let double_neg = neg.compose(&neg);
        assert_eq!(double_neg.apply(t(-1)), t(-1)); // -- = +
        assert_eq!(double_neg.apply(t(1)), t(1));
    }

    #[test]
    fn negation_flat_is_monotone() {
        let m = LatticeMorphism::negation(LatticeOrder::Flat);
        // In flat ordering, negation IS monotone: 0→0 preserves bottom,
        // and self-comparisons are trivially preserved.
        assert!(m.is_monotone());
    }

    #[test]
    fn non_monotone_morphism() {
        // A mapping that reverses order in linear ordering
        let m = LatticeMorphism::new(
            LatticeOrder::Linear,
            LatticeOrder::Linear,
            t(1), t(0), t(-1), // reverses order
        );
        assert!(!m.is_monotone());
    }

    #[test]
    fn chain_height_and_width() {
        assert_eq!(chain_height(LatticeOrder::Flat), 2);
        assert_eq!(chain_height(LatticeOrder::Linear), 3);
        assert_eq!(lattice_width(LatticeOrder::Flat), 2);
        assert_eq!(lattice_width(LatticeOrder::Linear), 1);
    }
}
