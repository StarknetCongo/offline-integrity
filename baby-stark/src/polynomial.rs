use baby_stark_math_lib::linear_multiiplication::vector_scalar_multiplication;
use core::num;
use std::cmp::max;

use crate::field::{self, vector_multiplication_field_scalar, FieldElement};

pub fn multi_poly_and_scalar(poly : Polynomial, scalar : i128) -> Polynomial{
    Polynomial{
        coeficients : vector_multiplication_field_scalar(&poly.coeficients, scalar)
    }
}

#[derive(Debug, Clone)]
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

    // TO FIX : ADD RETURN ELEMENT
    pub fn __add__(self, other: Polynomial) -> Polynomial {
        if self.clone().degree() == -1 {
            return other;
        } else if other.clone().degree() == -1 {
            return self;
        }
        // let field = self.coeficients.get(0).unwrap().field;

        // let zero_vec = [field.zero(); 10].to_vec();
        // let largest_vec = max(self.coeficients.len(), other.coeficients.len());
        // // let coeffs = vector_scalar_multiplication(zero_vec, largest_vec as i128);

        Polynomial{
            coeficients : vec![]
        }
    }

    pub fn __sub__(self, other: Polynomial) -> Polynomial {
        self.__add__(other.__neg__())
    }

    pub fn __mul__(self, other: Polynomial) -> Polynomial {
        let field = self.coeficients.get(0).unwrap().field;
        let zero = field.zero();
        let mut coeffs = vec![zero; self.coeficients.len() + other.coeficients.len() - 1];
        for i in 0..self.coeficients.len() {
            for j in 0..other.coeficients.len() {
                coeffs[i + j] =
                    coeffs[i + j].__add__(self.coeficients[i].__mul__(other.coeficients[j]));
            }
        }
        Polynomial::from(coeffs)
    }

    pub fn __eq__(self, other: Polynomial) -> bool {
        if self.clone().degree() != other.clone().degree() {
            return false;
        }
        if self.clone().degree() == -1 {
            return true;
        }
        self.clone().coeficients
            .iter()
            .zip(other.coeficients.iter())
            .all(|(a, b)| (*a).__eq__(*b))
    }

    pub fn __neq__(self, other: Polynomial) -> bool {
        !self.__eq__(other)
    }

    pub fn is_zero(self) -> bool {
        if self.degree() == -1 {
            return true;
        }
        return false;
    }

    // TO BE FIXED
    pub fn leading_coefficient(self) -> FieldElement{
        // self.coeficients[self.degree()]
        self.coeficients[0_usize]
    }

    pub fn divide(numerator : Polynomial, denominator : Polynomial) -> Option<(Polynomial, Polynomial)>{
        if denominator.clone().degree() == -1 {
            return None;
        }
        
        if numerator.clone().degree() < denominator.clone().degree() {
            return Some((Polynomial::from(vec![]), numerator));
        }

        let field = denominator.clone().coeficients.get(0).unwrap().field;
        let mut remainder = Polynomial::from(numerator.coeficients.clone());
        
        let mut quotient_coefficients : Vec<FieldElement> = vec![];

        let degree_diff = numerator.clone().degree() - denominator.clone().degree() + 1;
        
        for i in 0 .. degree_diff {
            quotient_coefficients.push(field.zero());
        }

        for i in 0 .. degree_diff {
            if remainder.clone().degree() < denominator.clone().degree() {
                break;
            }

            let coefficent = remainder.clone().leading_coefficient().__truediv__(denominator.clone().leading_coefficient());
            let shift = remainder.clone().degree() - denominator.clone().degree();

            let poly_product = multi_poly_and_scalar(Polynomial::from(vec![field.clone().zero()]), shift);

            let subsctractee = (poly_product.__add__(Polynomial::from(vec![coefficent]))).__mul__(denominator.clone());

            quotient_coefficients[shift as usize] = coefficent;

            remainder = remainder.__sub__(subsctractee)
        }   

        let quotient = Polynomial::from(quotient_coefficients);

        Some((quotient, remainder))
    }

    pub fn __truediv__(self, other : Polynomial) -> Result<Polynomial, String> {
        if other.clone().is_zero() {
            return Err("CANT_DIVIDE_BY_0".to_string());
        }
        let divide_result = Polynomial::divide(self, other);
        match divide_result {
            Some(divide_result) => Ok(divide_result.0),
            None => Err("COULD_NOT_DIVIDE".to_string())
        }
    }

    pub fn __mod__(self, other : Polynomial) -> Result<Polynomial, String> {
        let divide_result = Polynomial::divide(self, other);
        match divide_result {
            Some(divide_result) => Ok(divide_result.1),
            None => Err("COULD_NOT_GET_MODULO".to_string())
        }
    }
   


   pub fn test_colinearity(points: [FieldElement]) -> bool {
        let domain: Vec<_> = points.iter().map(|point| point.clone()).collect();
        let values: Vec<_> = points.iter().map(|point| point.clone()).collect();
        let polynomial = Polynomial::interpolate_domain(domain, values);
        polynomial.degree() <= 1
    }
    
    


}
