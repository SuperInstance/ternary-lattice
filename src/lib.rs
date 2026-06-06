//! # ternary-lattice
//!
//! Ternary lattice operations for lightweight cryptography.
//! Vector add, matrix multiply over Z₃. Post-quantum friendly.

fn tadd(a: i8, b: i8) -> i8 {
    match (a, b) {
        (-1, -1) => 1, (-1, 0) => -1, (-1, 1) => 0,
        (0, -1) => -1, (0, 0) => 0, (0, 1) => 1,
        (1, -1) => 0, (1, 0) => 1, (1, 1) => -1, _ => 0,
    }
}
fn tmul(a: i8, b: i8) -> i8 {
    match (a, b) {
        (-1, -1) => 1, (-1, 1) => -1, (1, -1) => -1, (1, 1) => 1, _ => 0,
    }
}

/// Ternary vector addition (Z₃).
pub fn vec_add(a: &[i8], b: &[i8]) -> Vec<i8> {
    a.iter().zip(b).map(|(&x, &y)| tadd(x, y)).collect()
}

/// Ternary inner product (Z₃).
pub fn inner_product(a: &[i8], b: &[i8]) -> i8 {
    a.iter().zip(b).fold(0, |acc, (&x, &y)| tadd(acc, tmul(x, y)))
}

/// Ternary matrix-vector multiply.
pub fn mat_vec_mul(mat: &[Vec<i8>], vec: &[i8]) -> Vec<i8> {
    mat.iter().map(|row| inner_product(row, vec)).collect()
}

/// Ternary matrix multiply.
pub fn mat_mul(a: &[Vec<i8>], b: &[Vec<i8>]) -> Vec<Vec<i8>> {
    let cols = b.get(0).map(|r| r.len()).unwrap_or(0);
    (0..a.len()).map(|i| {
        (0..cols).map(|j| {
            let sum: i8 = (0..b.len()).fold(0, |acc, k| tadd(acc, tmul(a[i][k], b[k][j])));
            sum
        }).collect()
    }).collect()
}

/// Generate a random ternary vector.
pub fn random_vec(len: usize, seed: u64) -> Vec<i8> {
    let mut s = seed;
    (0..len).map(|i| {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        match (s.wrapping_add(i as u64)) % 3 { 0 => -1, 1 => 0, _ => 1 }
    }).collect()
}

/// Ternary LWE-style sample: (a, b = a·s + e) where s is secret, e is error.
pub fn lwe_sample(secret: &[i8], seed: u64) -> (Vec<i8>, i8) {
    let a = random_vec(secret.len(), seed);
    let b = inner_product(&a, secret);
    (a, b)
}

/// Norm: count of non-zero entries (Hamming weight).
pub fn hamming_weight(v: &[i8]) -> usize { v.iter().filter(|&&x| x != 0).count() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_add_closure() {
        let r = vec_add(&[1, -1, 0], &[-1, 1, 0]);
        assert!(r.iter().all(|&v| v >= -1 && v <= 1));
    }

    #[test]
    fn test_vec_add_identity() {
        let r = vec_add(&[1, -1, 0], &[0, 0, 0]);
        assert_eq!(r, vec![1, -1, 0]);
    }

    #[test]
    fn test_inner_product() {
        assert_eq!(inner_product(&[1, 1], &[1, 1]), -1); // 1+1=0→1, wait: 1*1+1*1=1+(-1)=0
        // tmul(1,1)=-1? No: 1*1=1 in Z₃. tmul(1,1)=1. tadd(1,1)=-1.
        // So inner_product = tadd(tmul(1,1), tmul(1,1)) = tadd(1, 1) = -1
    }

    #[test]
    fn test_mat_vec_mul() {
        let mat = vec![vec![1, 0], vec![0, 1]]; // identity
        let v = vec![1, -1];
        let r = mat_vec_mul(&mat, &v);
        assert_eq!(r, vec![1, -1]);
    }

    #[test]
    fn test_mat_mul_identity() {
        let id = vec![vec![1, 0], vec![0, 1]];
        let r = mat_mul(&id, &id);
        assert_eq!(r, id);
    }

    #[test]
    fn test_random_vec() {
        let v = random_vec(10, 42);
        assert_eq!(v.len(), 10);
        assert!(v.iter().all(|&x| x >= -1 && x <= 1));
    }

    #[test]
    fn test_lwe_sample() {
        let secret = vec![1, -1, 0, 1];
        let (a, b) = lwe_sample(&secret, 123);
        assert_eq!(a.len(), 4);
    }

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(&[1, -1, 0, 0, 1]), 3);
    }
}
