#[derive(Debug, Clone)]
struct Polynomial {
    coeffs: Vec<f64>, // Coefficients of the polynomial, where the index is the degree
}

impl Polynomial {
    // Returns the degree of the polynomial
    fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    // Returns the leading coefficient (coefficient of the highest-degree term)
    fn leading_coefficient(&self) -> f64 {
        *self.coeffs.last().unwrap()
    }

    // Polynomial subtraction: P - S
    fn subtract(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs = vec![0.0; self.coeffs.len().max(other.coeffs.len())];
        
        // Subtract the coefficients term by term
        for i in 0..self.coeffs.len() {
            result_coeffs[i] += self.coeffs[i];
        }
        for i in 0..other.coeffs.len() {
            result_coeffs[i] -= other.coeffs[i];
        }

        // Remove leading zeros
        while result_coeffs.len() > 1 && result_coeffs.last() == Some(&0.0) {
            result_coeffs.pop();
        }

        Polynomial {
            coeffs: result_coeffs,
        }
    }

    // Polynomial multiplication by a monomial: S * B
    fn multiply_by_monomial(&self, scalar: f64, degree_diff: usize) -> Polynomial {
        let mut result_coeffs = vec![0.0; self.coeffs.len() + degree_diff];
        for i in 0..self.coeffs.len() {
            result_coeffs[i + degree_diff] = self.coeffs[i] * scalar;
        }
        Polynomial {
            coeffs: result_coeffs,
        }
    }

    // Polynomial long division (returns quotient and remainder)
    fn divide(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        let mut quotient = Polynomial { coeffs: vec![0.0] };
        let mut remainder = self.clone();
        let divisor_degree = divisor.degree();
        let divisor_lc = divisor.leading_coefficient();

        while remainder.degree() >= divisor_degree {
            let degree_diff = remainder.degree() - divisor_degree;
            let leading_term_ratio = remainder.leading_coefficient() / divisor_lc;

            // Compute the term that we need to subtract from the remainder
            let term = Polynomial {
                coeffs: vec![leading_term_ratio],
            }
            .multiply_by_monomial(1.0, degree_diff);

            // Add the term to the quotient
            quotient = quotient.subtract(&term.multiply_by_monomial(-1.0, 0));

            // Subtract the appropriate multiple of the divisor from the remainder
            remainder = remainder.subtract(&divisor.multiply_by_monomial(leading_term_ratio, degree_diff));
        }

        (quotient, remainder)
    }
}

fn main() {
    // Example polynomials
    let a = Polynomial {
        coeffs: vec![1.0, -3.0, 2.0], // Represents x^2 - 3x + 2
    };
    let b = Polynomial {
        coeffs: vec![1.0, -1.0], // Represents x - 1
    };

    let (quotient, remainder) = a.divide(&b);
    
    println!("Quotient: {:?}", quotient);
    println!("Remainder: {:?}", remainder);
}
