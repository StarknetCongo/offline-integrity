use crate::field::FieldElement;
use std::collections::HashMap;
pub struct MPolynomial {
    dictionary: HashMap<String, FieldElement>,
}

impl MPolynomial {
    pub fn from(dictionary: HashMap<String, FieldElement>) -> MPolynomial {
        MPolynomial { dictionary }
    }

    pub fn zero() -> MPolynomial {
        let my_hash_map: HashMap<String, FieldElement> = HashMap::new();

        MPolynomial {
            dictionary: my_hash_map,
        }
    }
    use std::collections::HashMap;

struct MPolynomial {
    dictionary: HashMap<Vec<i32>, f64>,
}

impl MPolynomial {
    fn is_zero(&self) -> bool {
        if self.dictionary.is_empty() {
            return true;
        } else {
            for &v in self.dictionary.values() {
                if !v.is_zero() {
                    return false;
                }
            }
            return true;
        }
    }
}

fn variables(num_variables: usize, field: f64) -> Vec<MPolynomial> {
    let mut variables = Vec::new();
    for i in 0..num_variables {
        let mut exponent = vec![0; i];
        exponent.push(1);
        exponent.extend(vec![0; num_variables - i - 1]);
        let mut dictionary = HashMap::new();
        dictionary.insert(exponent.clone(), field);
        variables.push(MPolynomial { dictionary });
    }
    variables
}

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_creation_polynomial() {
        let mut my_dic: HashMap<String, FieldElement> = HashMap::new();
        my_dic.insert("x".to_string(), FieldElement::new());
        my_dic.insert("y".to_string(), FieldElement::new());
        my_dic.insert("y".to_string(), FieldElement::new());
        let x = MPolynomial { dictionary: my_dic };
    }

    #[test]
    fn test_init_polynomial() {
        let mut my_dic: HashMap<String, FieldElement> = HashMap::new();
        my_dic.insert("x".to_string(), FieldElement::new());
        let poly = MPolynomial::from(my_dic);
    }
}
