use baby_stark_math_lib::gcd::xgcd;

#[derive(Debug, Clone, Copy)]
pub struct FieldElement{
    pub value : i128,
    pub field : Field
}

impl FieldElement{
    pub fn new() -> Self{
        FieldElement{
            value : 0,
            field : Field::new()
        }
    }

    pub fn from(value : i128, field : Field) -> Self{
        FieldElement{
            value : value,
            field : field
        }
    }

    pub fn __add__(self, right : FieldElement) -> FieldElement{
        FieldElement{
            value : self.value + right.value, 
            field : self.field
        }
    }

    pub fn __mul__(self, right : FieldElement) -> FieldElement{
        FieldElement{
            value : self.value * right.value, 
            field : self.field
        }
    }


    pub fn __sub__(self, right : FieldElement) -> FieldElement{
        FieldElement{
            value : self.value - right.value,
            field : self.field
        }
    }

    // TO BE IMPLEMENTED
    pub fn __truediv__(self, right : FieldElement) -> FieldElement{
        FieldElement::new()
    }

    pub fn __neg__(self) -> FieldElement{
        self.clone().field.negate(self)
    }

    pub fn inverse(self) -> FieldElement{
        self.clone().field.inverse(self)
    }

    pub fn __xor__(self, exponent : i128){
        let acc = FieldElement::from(1, self.field.clone());
        let val = FieldElement::from(self.value, self.field);

        // TBD 
        

    }

    pub fn __eq__(self, other : FieldElement) -> bool {
        self.value == other.value
    }

    pub fn __neq__(self, other : FieldElement) -> bool{
        self.value != other.value
    }

    pub fn __str__(self) -> String{
        format!("{}", self.value)
    }

    pub fn __bytes__(self) -> Vec<u8>{
        let str = format!("{:?}", self);
        str.as_bytes().to_vec()
    }

    pub fn is_zero(self) -> bool {
        self.value == 0
    }




}



#[derive(Debug, Clone, Copy)]
pub struct Field{
    p : i128
}

impl Field{
    pub fn from(p : i128) -> Field{
        Self{
            p 
        }
    }

    pub fn new() -> Field{
        let p : i128 = 85408008396924667383611388730472331217;
        Self { 
            p
        }
    }

    pub fn zero( self ) -> FieldElement{
        FieldElement::from(0, self)
    }

    pub fn one(self) -> FieldElement{
        FieldElement::from(1, self)
    }

    pub fn multiply(self, left : FieldElement, right : FieldElement) -> FieldElement{
        FieldElement::from((left.value * right.value) % self.p, self)
    }

    pub fn add(self, left : FieldElement, right : FieldElement) -> FieldElement{
        FieldElement::from((left.value + right.value) % self.p, self)
    }

    pub fn substruct(self, left : FieldElement, right : FieldElement) -> FieldElement{
        FieldElement::from((self.p + left.value - right.value) % self.p, self)
    }

    pub fn negate(self, operand : FieldElement) -> FieldElement{
        FieldElement::from((self.p - operand.value) % self.p, self)
    }

    pub fn inverse(self, operand : FieldElement) -> FieldElement{
        let (gcd, t1, t2) = xgcd(self.p, operand.value);
        FieldElement::from(t1, self)
    }

    pub fn divide(self, left : FieldElement, right : FieldElement) -> FieldElement{
        if right.clone().is_zero(){
            panic!("DIVIDE_BY_ZERO");
        }
        let (gcd, t1, t2) = xgcd(right.value, self.p);

        FieldElement::from(left.value * t1 % self.p, self)
    }

    pub fn generator(self) -> FieldElement{
        FieldElement{
            value : 85408008396924667383611388730472331217,
            field : self
        }
    }

    // done
    pub fn primitive_nth_root(self, n : i128) -> Option<FieldElement>{
        if self.p == 85408008396924667383611388730472331217 {
            if !(n <= 1 << 119 && (n & (n-1)) == 0) {
                println!("Field does not have nth root of unity where n > 2^119 or not power of two.")
            }
            let mut root = FieldElement::from(85408008396924667383611388730472331217, self);
            let mut order : i128 = 1 << 119;
            while order != n {
                root = root ^ 2;
                order = order / 2;
            }
            Some(root)
        }else {
            None
        }
    }

   // FIXED
    pub fn sample(self, byte_array: Vec<u8>) -> FieldElement {
        let mut acc: i128 = 0;
        for &b in byte_array.iter() {
            acc = (acc << 8) ^ b as i128;
        }
        FieldElement::from(acc % self.p, self)
    }

}


pub fn vector_multiplication_field_scalar ( vector : &Vec<FieldElement>, scalar : i128) -> Vec<FieldElement>{
    let mut res : Vec<FieldElement> = vec![];
    for field_element in vector.iter() {
        res.push(FieldElement::from(field_element.value * scalar, vector.get(0).unwrap().field));
    }

    res
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_my_field(){
        let my_field = Field::new();
        
        let x = FieldElement::from(90, my_field.clone());
        let y = FieldElement::from(129, my_field.clone());

        let z = my_field.add(x, y);

        println!("{:?}", z );

    }
}