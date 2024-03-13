type Field = i32;

#[derive(Debug)]
pub struct FieldElement{
    pub value : i64,
    pub field : Field
}

impl FieldElement{
    pub fn new() -> Self{
        FieldElement{
            value : 0,
            field : 0
        }
    }

    pub fn __init__(value : i64, field : Field) -> Self{
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

    pub fn __truediv__(self, right : FieldElement) -> FieldElement{
        FieldElement::new()
    }

    pub fn __neg__(self){

    }

    pub fn inverse(self){

    }

    pub fn __xor__(self, exponent : i64){

    }


}