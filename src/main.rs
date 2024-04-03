extern crate itertools;
extern crate ndarray;
extern crate rayon;

use itertools::Itertools;
use ndarray::{Array2, ArrayView1};
use rayon::prelude::*;

fn signed_permutation_matrix(perm: ArrayView1<usize>, signs: ArrayView1<i32>) -> Array2<i32> {
    let n = perm.len();
    let mut matrix = Array2::<i32>::zeros((n, n));
    for (i, &p) in perm.iter().enumerate() {
        matrix[[i, p]] = signs[i];
    }
    matrix
}

fn generate_signed_permutation_matrices(n: usize) -> Vec<Array2<i32>> {
    let perms: Vec<_> = (0..n).permutations(n).collect();

    perms
        .into_par_iter()
        .flat_map(|perm| {
            let perm_array = ndarray::arr1(&perm);
            (0..1 << n)
                .into_par_iter()
                .map(move |i| {
                    let signs: Vec<i32> = (0..n)
                        .map(|j| if i & (1 << j) == 0 { -1 } else { 1 })
                        .collect();
                    let signs_array = ndarray::arr1(&signs);
                    signed_permutation_matrix(perm_array.view(), signs_array.view())
                })
                .collect::<Vec<Array2<i32>>>()
        })
        .collect()
}

fn main() {
    let n = 9;
    let matrices = generate_signed_permutation_matrices(n);
}
