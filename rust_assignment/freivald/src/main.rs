// Import necessary libraries. Check cargo.toml and the documentation of the libraries.

use ndarray::prelude::*;
use rand::Rng;

type Fq = isize;

struct Freivald {
    // Array/Vec of Fq,
    x: Array1<Fq>,
}

impl Freivald {
    // Create constructor for object
    fn new(array_size: usize) -> Self {
        // Generate random number
        let mut rng = rand::thread_rng();
        let randomNumber: usize = rng.gen_range(0..5);

        // Populate vector with values r^i for i=0..matrix_size
        let mut array = Array1::<Fq>::ones(array_size);

        for i in 0..array_size {
            array[i] = (randomNumber ^ i) as Fq;
        }

        // Return freivald value with this vector as its x value
        return Freivald { x: array };
    }

    // Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(
        &self,
        matrix_a: &Array2<Fq>,
        matrix_b: &Array2<Fq>,
        supposed_ab: &Array2<Fq>,
    ) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));
        // check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct

        println!("self.x: {:?}", self.x);
        let res = matrix_a.dot(matrix_b).dot(&self.x) - supposed_ab.dot(&self.x);

        for n in res.iter() {
            if *n != 0 {
                return false;
            }
        }

        return true;
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // Add types for arguments
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)

// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    todo!()
}

// Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(
    matrix_a: &Array2<Fq>,
    matrix_b: &Array2<Fq>,
    supposed_ab: &Array2<Fq>,
) -> bool {
    // Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    let (a_rows, _) = matrix_a.dim();
    let (_, b_cols) = matrix_b.dim();
    let (c_rows, c_cols) = supposed_ab.dim();

    if c_rows != a_rows || b_cols != c_cols {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        static ref MATRIX_A: Array2<Fq> = array![[1,2],[3,4]];
        static ref MATRIX_A_DOT_A: Array2<Fq> = array![[7,10],[15,22]];
        static ref MATRIX_B: Array2<Fq> = array![[5,6],[7,8]];
        static ref MATRIX_B_DOT_B: Array2<Fq> = array![[67,78],[91,106]];
        // TODO: static ref MATRIX_C: Array2<Fq> = (0..200).map(|_| [0; 200]).collect();
        // TODO: static ref MATRIX_C_DOT_C: Array2<Fq> = /* Correct result of C * C */;
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    // TODO: #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>,
        #[case] matrix_b: &Array2<Fq>,
        #[case] supposed_ab: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    // TODO: #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>,
        #[case] b: &Array2<Fq>,
        #[case] c: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
