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
