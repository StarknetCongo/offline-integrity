use baby_stark_math_lib::linear_multiiplication::vector_scalar_multiplication;
use std::cmp::max;

use crate::field::{self, FieldElement};

#[derive(Debug)]
pub struct Polynomial {
    coeficients: Vec<FieldElement>,
}

impl Polynomial {
    pub fn from(coeficients: Vec<FieldElement>) -> Self {
        Self { coeficients }
    }

    pub fn degree(self) -> i128 {
        if self.coeficients.len() == 0 {
            return -1_i128;
        }
        let zero = match self.coeficients.get(0) {
            Some(zero) => zero,
            None => {
                panic!("Not found");
            }
        };

        // let zero_vec = [zero; 23].to_vec();
        // to implement in RUST
        // if self.coeficients == zero_vec {
        //     return -1_i128;
        // }

        (self.coeficients.len() - 1) as i128
    }

    pub fn __neg__(self) -> Self {
        Polynomial::from(
            self.coeficients
                .into_iter()
                .map(|c| c.__neg__())
                .collect::<Vec<FieldElement>>(),
        )
    }

    pub fn __add__(self, other : Polynomial) -> Polynomial{
        if self.degree() == -1 {
            return other;
        } else if other.degree() == -1 {
            return self;
        } 
        let field = self.coeficients.get(0).unwrap().field;

        let zero_vec = [field.zero(); 10].to_vec();
        let largest_vec = max(self.coeficients.len(), other.coeficients.len());
        let coeffs = vector_scalar_multiplication(zero_vec, largest_vec as i128);

    }

    pub fn __sub__(self, other : Polynomial) -> Polynomial{
        self.__add__(other.__neg__())
    }

    pub fn __mul__(self, other : Polynomial) -> Polynomial{
        let field = self.coeficients.get(0).unwrap().field;
        let zero = field.zero();
        let mut coeffs = vec![zero; self.coeficients.len() + other.coeficients.len() - 1];
        for i in 0..self.coeficients.len() {
            for j in 0..other.coeficients.len() {
                coeffs[i+j] = coeffs[i+j].__add__(self.coeficients[i].__mul__(other.coeficients[j]));
            }
        }
        Polynomial::from(coeffs)
  }
  
    pub fn __eq__(self, other : Polynomial) -> bool{
        if self.degree() != other.degree() {
            return false;
        }
        if self.degree() == -1 {
            return true;
        }
        self.coeficients.iter().zip(other.coeficients.iter()).all(|(a, b)| a == b)
    }

    pub fn __neq__(self, other : Polynomial) -> bool{
        !self.__eq__(other)
    }

    pub fn is_zero(self) -> bool{
        if self.degree() == -1 {
            return true;
        }
        return false;
    }

    pub fn leading_coefficient(self){
        self.coeficients[self.degree()]
    }

    pub fn evaluate(&self, point: &FieldElement)->FieldElement{
        let mut xi = FieldElement::one();
        let mut value = FieldElement::zero();
        for c in &self.coeficients{
            value = value.__add__(c.__mul__(xi));
            xi = xi.__mul_(point);
        }
        return value;
    }
}
