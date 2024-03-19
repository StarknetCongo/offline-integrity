pub fn vector_scalar_multiplication(vector : Vec<i128>, scalar : i128) -> Vec<i128>{
    vector.into_iter().map(|elt|{
        elt * scalar
    }).collect::<Vec<i128>>()
}
