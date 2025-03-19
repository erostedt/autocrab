use autocrab::forward::evaluate::evaluate;
use autocrab::forward::functions::*;
use autocrab::forward::variable::Variable;

use std::io::{self, Write};

fn objective_function(variables: [Variable; 2]) -> [Variable; 2]
{
    let x = variables[0];
    let y = variables[1];
    let x_out = pow(x, 3.0) - 3.0 * x * square(y) - 1.0;
    let y_out = 3.0 * square(x) * y - pow(y, 3.0);
    [x_out, y_out]
}

fn negate(vector: [f64; 2]) -> [f64; 2]
{
    [-vector[0], -vector[1]]
}

fn iadd(dst: &mut [f64; 2], src: [f64; 2])
{
    dst[0] += src[0];
    dst[1] += src[1];
}

fn sub(a: &[f64; 2], b: &[f64; 2]) -> [f64; 2]
{
    [a[0] - b[0], a[1] - b[1]]
}

fn norm_squared(v: [f64; 2]) -> f64
{
    let x = v[0];
    let y = v[1];
    x * x + y * y
}

fn solve(matrix: [[f64; 2]; 2], vector: [f64; 2]) -> Option<[f64; 2]>
{
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[1][0];
    let d = matrix[1][1];

    let det = a * d - b * c;
    if det.abs() < 1e-8 {
        return None;
    }

    let e = vector[0];
    let f = vector[1];
    let x1 = (e * d - b * f) / det;
    let x2 = (a * f - e * c) / det;
    Some([x1, x2])
}

fn lerp(a: f64, b: f64, t: f64) -> f64
{
    a + (b - a) * t
}

fn find_root(max_iter: usize, start: [f64; 2], roots: &[[f64; 2]; 3], tol: f64) -> Option<usize>
{
    let mut x = start;
    for _ in 0..max_iter {
        let (current_values, current_jacobian) = evaluate(objective_function, x);
        let neg = negate(current_values);

        let res = solve(current_jacobian, neg)?;

        iadd(&mut x, res);

        for (k, root) in roots.iter().enumerate() {
            if norm_squared(sub(&x, root)) < tol {
                return Some(k);
            }
        }
    }
    None
}

fn main()
{
    let roots = [
        [1.0, 0.0],
        [-0.5, 0.5 * f64::sqrt(3.0)],
        [-0.5, -0.5 * f64::sqrt(3.0)],
    ];

    let colors: [[u8; 3]; 3] = [[255, 0, 0], [0, 255, 0], [0, 0, 255]];
    let black: [u8; 3] = [0, 0, 0];

    let max_iter = 50;
    let tol = 1e-6;

    let left = -2.0;
    let right = 2.0;
    let top = -2.0;
    let bottom = 2.0;

    let rows = 720;
    let cols = 1280;

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "P6\n{} {}\n255", cols, rows).unwrap();

    for r in 0..rows {
        let y = lerp(top, bottom, r as f64 / rows as f64);
        for c in 0..cols {
            let x = lerp(left, right, c as f64 / cols as f64);
            match find_root(max_iter, [x, y], &roots, tol) {
                Some(k) => {
                    handle.write_all(&colors[k]).unwrap();
                }
                None => {
                    handle.write_all(&black).unwrap();
                }
            }
        }
    }

    handle.flush().unwrap();
}
