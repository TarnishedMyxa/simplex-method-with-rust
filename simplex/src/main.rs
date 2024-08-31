struct Simplex {
    // The number of variables in the linear program
    n: usize,
    // The number of constraints in the linear program
    m: usize,
    // The matrix of coefficients of the constraints
    a: Vec<Vec<f64>>,
    // The vector of coefficients of the objective function
    c: Vec<f64>,
    // The vector of coefficients of the right-hand side of the constraints
    b: Vec<f64>,
    // The vector of basic variables
    basic: Vec<usize>,
    // The vector of non-basic variables
    non_basic: Vec<usize>,
    // The vector of the values of the basic variables
    x: Vec<f64>,
    // The value of the objective function
    z: f64,
}


impl Simplex {
    fn new(n: usize, m: usize, a: Vec<Vec<f64>>, c: Vec<f64>, b: Vec<f64>) -> Simplex {
        let basic = (0..m).collect();
        let non_basic = (m..n).collect();
        let x = vec![0.0; m];
        let z = 0.0;
        Simplex { n, m, a, c, b, basic, non_basic, x, z }
    }

    fn pivot(&mut self, l: usize, e: usize) {
        let mut a = vec![vec![0.0; self.n]; self.m];
        let mut b = vec![0.0; self.m];
        let mut c = vec![0.0; self.n];
        let mut x = vec![0.0; self.m];
        let mut z = 0.0;
        let mut basic = vec![0; self.m];
        let mut non_basic = vec![0; self.n];

        let mut i = 0;
        for k in 0..self.m {
            if k == l {
                b[k] = self.b[l] / self.a[l][e];
                for j in 0..self.n {
                    if j == e {
                        a[k][j] = 1.0 / self.a[l][e];
                    } else {
                        a[k][j] = self.a[l][j] / self.a[l][e];
                    }
                }
            } else {
                b[k] = self.b[k] - self.a[k][e] * self.b[l] / self.a[l][e];
                for j in 0..self.n {
                    if j == e {
                        a[k][j] = -self.a[k][e] / self.a[l][e];
                    } else {
                        a[k][j] = self.a[k][j] - self.a[k][e] * self.a[l][j] / self.a[l][e];
                    }
                }
            }
        }

        for j in 0..self.n {
            if j == e {
                c[j] = -self.c[e] / self.a[l][e];
            } else {
                c[j] = self.c[j] - self.c[e] * self.a[l][j] / self.a[l][e];
            }
        }
    }
}


    fn main() {
    println!("Hello, world!");
}


