# ternary-lattice

Lattice-based cryptographic primitives over ℤ₃. Implements vector addition, matrix multiplication, inner products, and Learning-With-Errors (LWE) sample generation — all operating in the ternary ring where every element is {-1, 0, +1}.

## Why It Matters

Post-quantum cryptography (PQC) relies heavily on lattice problems — the shortest vector problem (SVP) and learning-with-errors (LWE) are believed hard even for quantum computers (Regev, 2005). Operating these schemes over ℤ₃ offers unique advantages:

- **Minimal key size**: 2 bits per coefficient = 16× smaller keys than FP32-based schemes
- **No multiplication needed**: All products are in {-1, 0, +1} — computable with a sign flip or zero check
- **Constant-time operations**: ℤ₃ arithmetic has no data-dependent branching, eliminating timing side-channels
- **Hardware-friendly**: Maps directly to ternary logic gates (ternary ALUs)

The Learning-With-Errors problem over ℤ₃: given random vector **a** and scalar b = ⟨**a**, **s**⟩ + e (mod 3), find secret **s**. The error term e makes this intractable even for quantum adversaries.

## How It Works

### ℤ₃ Arithmetic

All operations use modular arithmetic over the field ℤ₃ = {−1, 0, +1} (isomorphic to {0, 1, 2}):

**Addition table** (tadd):

| +  | −1 |  0 | +1 |
|----|----|----|-----|
| −1 | +1 | −1 |  0 |
|  0 | −1 |  0 | +1 |
| +1 |  0 | +1 | −1 |

Note: 1 + 1 = 2 ≡ −1 (mod 3). This is the key identity — addition wraps around.

**Multiplication table** (tmul):

| ×  | −1 |  0 | +1 |
|----|----|----|-----|
| −1 | +1 |  0 | −1 |
|  0 |  0 |  0 |  0 |
| +1 | −1 |  0 | +1 |

Multiplication is equivalent to standard integer multiplication reduced mod 3. Zero is absorbing.

### Vector Operations

**Addition**: element-wise ℤ₃ addition.

```
[a₁, a₂] + [b₁, b₂] = [tadd(a₁,b₁), tadd(a₂,b₂)]
```

**Inner product**:

```
⟨a, b⟩ = Σᵢ tmul(aᵢ, bᵢ)   (reduced in ℤ₃)
```

This is the bilinear form over ℤ₃ⁿ. It satisfies:
- Symmetry: ⟨a, b⟩ = ⟨b, a⟩
- Bilinearity: ⟨a₁+a₂, b⟩ = ⟨a₁, b⟩ + ⟨a₂, b⟩
- Non-degeneracy: if ⟨a, b⟩ = 0 for all b, then a = 0

### Matrix Multiplication

Standard matrix product, but with ℤ₃ arithmetic:

```
C = A × B:   cᵢⱼ = Σₖ tmul(aᵢₖ, bₖⱼ)   (ℤ₃ sum)
```

### LWE Sampling

Given secret vector **s** ∈ ℤ₃ⁿ, generate sample (**a**, b):

```
a ← random(ℤ₃ⁿ)
b = ⟨a, s⟩ + e   (in ℤ₃)
```

where e is a small error term. In this implementation, e = 0 (noiseless LWE) — the hardness comes from the ℤ₃ structure alone. Production systems would add a discrete Gaussian error.

**Security note**: Noiseless LWE over ℤ₃ is NOT cryptographically secure. This crate demonstrates the algebraic structure. For real cryptography, add error from a discrete Gaussian distribution (σ ≥ 3/√(2π)).

### Hamming Weight

The "norm" in ℤ₃ is the Hamming weight — count of non-zero entries:

```
‖v‖₀ = |{i : vᵢ ≠ 0}|
```

This is the relevant measure for lattice hardness: secret vectors with low Hamming weight are easier to find (sparse secret attacks).

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `vec_add(a, b)` | O(n) | O(n) |
| `inner_product(a, b)` | O(n) | O(1) |
| `mat_vec_mul(M, v)` | O(m·n) | O(m) |
| `mat_mul(A, B)` | O(m·n·p) | O(m·p) |
| `random_vec(n, seed)` | O(n) | O(n) |
| `lwe_sample(s, seed)` | O(n) | O(n) |
| `hamming_weight(v)` | O(n) | O(1) |

## Quick Start

```rust
use ternary_lattice::{vec_add, inner_product, mat_mul, lwe_sample, hamming_weight};

// ℤ₃ vector arithmetic
let a = vec![1, -1, 0, 1];
let b = vec![0, 1, 1, -1];
let sum = vec_add(&a, &b);
// sum = [1, 0, 1, 0]  (mod 3)

// Inner product in ℤ₃
let dot = inner_product(&a, &b);
// dot = tmul(1,0) + tmul(-1,1) + tmul(0,1) + tmul(1,-1)
//     = 0 + (-1) + 0 + (-1) = -2 ≡ 1 (mod 3) → represented as 1

// LWE sampling (demonstration only — not secure!)
let secret = vec![1, -1, 0, 1, -1];
let (sample_a, sample_b) = lwe_sample(&secret, 42);
println!("a = {:?}", sample_a);
println!("b = ⟨a, s⟩ = {}", sample_b);
println!("Secret Hamming weight: {}", hamming_weight(&secret));
```

## API

| Function | Description |
|----------|-------------|
| `vec_add(a, b) -> Vec<i8>` | Element-wise ℤ₃ addition |
| `inner_product(a, b) -> i8` | Bilinear form in ℤ₃ |
| `mat_vec_mul(M, v) -> Vec<i8>` | Matrix-vector product |
| `mat_mul(A, B) -> Vec<Vec<i8>>` | Full matrix product |
| `random_vec(n, seed) -> Vec<i8>` | Uniformly random ℤ₃ vector |
| `lwe_sample(s, seed) -> (Vec<i8>, i8)` | LWE sample (a, ⟨a,s⟩+e) |
| `hamming_weight(v) -> usize` | Count of non-zero entries |

## Architecture Notes

This crate implements **η (eta) layer** cryptographic primitives in the γ + η = C framework:

- **η (eta)**: The algebraic engine — ℤ₃ arithmetic, lattice operations, sample generation. This crate provides the η-layer mathematical substrate.
- **γ (gamma)**: External protocol layer — key exchange, commitment schemes, zero-knowledge proofs built on top of these primitives (provided by other ecosystem crates).
- **C**: The complete post-quantum cryptographic system. The ℤ₃ field {-1, 0, +1} is shared with the entire ecosystem, making lattice-based keys composable with ternary neural network weights.

## References

- **Learning With Errors**: Regev, O., "On Lattices, Learning with Errors, Random Linear Codes, and Cryptography," STOC 2005.
- **Post-Quantum Cryptography**: Bernstein, D.J. & Lange, T., "Post-Quantum Cryptography," Nature, 549, 188-194, 2017.
- **Lattice-Based Crypto**: Micciancio, D. & Regev, O., "Lattice-based Cryptography," Post-Quantum Cryptography, Springer, 2009.
- **Shortest Vector Problem**: Ajtai, M., "The Shortest Vector Problem in L₂ is NP-hard for Randomized Reductions," STOC 1998.
- **Ring-LWE**: Lyubashevsky, V., Peikert, C. & Regev, O., "On Ideal Lattices and Learning with Errors over Rings," EUROCRYPT 2010.
- **Hamming Weight Attacks**: Arora, S. & Ge, R., "New Algorithms for Learning in Presence of Errors," ICALP 2011.

## License

Apache-2.0
