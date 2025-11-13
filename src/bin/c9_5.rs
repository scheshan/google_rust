/***
我们将为三维几何图形创建几个实用函数，将点表示为 [f64;3]。函数签名由您自行确定。
 */

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.


fn magnitude(arr: &[f64]) -> f64 {
    let mut res = 0.0f64;
    for v in arr {
        res += v.powi(2);
    }

    res.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(arr: &mut [f64]) {
    let magnitude = magnitude(arr);

    for v in arr {
        *v = *v / magnitude;
    }
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}