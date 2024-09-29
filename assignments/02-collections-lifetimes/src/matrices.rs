use std::vec;

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    EmptyVector,
    DimensionMismatch,
    InvalidShape,
}

// use standard dot product sum multiply definition
fn dot_product_prescriptive(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Result<f64, MatrixError> {
    if vec1.is_empty() || vec2.is_empty() {
        return Err(MatrixError::EmptyVector);
    }
    if vec1.len() != vec2.len() {
        return Err(MatrixError::DimensionMismatch);
    }
    let mut res: f64 = 0.0;
    for i in 0..vec1.len() {
        res += vec1[i] * vec2[i];
    }
    Ok(res)
}

// use standard dot product sum multiply definition
fn dot_product_functional(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Result<f64, MatrixError> {
    match (vec1.len(), vec2.len()) {
        (0,_) | (_,0) => Err(MatrixError::EmptyVector),
        (a,b) if a != b => Err(MatrixError::DimensionMismatch),
        _ => Ok(vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum())
    }
}

// use standard matrix multiplication definition
fn multiply_matrices(
    vec1: &Vec<Vec<f64>>,
    vec2: &Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, MatrixError> {
    // check dimension match cases
    if !vec1.into_iter().all(|x| x.len() == vec1[0].len()) {
        return Err(MatrixError::InvalidShape);
    }
    if !vec2.into_iter().all(|x| x.len() == vec2[0].len()) {
        return Err(MatrixError::InvalidShape);
    }
    if vec1.is_empty() || vec2.is_empty() {
        return Err(MatrixError::EmptyVector);
    }
    if vec1[0].len() != vec2.len() {
        return Err(MatrixError::DimensionMismatch);
    }
    // apply standard matrix multiplication 
    let mut res = vec![vec![0.0; vec2[0].len()]; vec1.len()];
    for c2 in 0..vec2[0].len() {
        for r1 in 0..vec1.len() {
            for c1 in 0..vec1[0].len() {
                res[r1][c2] += vec1[r1][c1] * vec2[c1][c2];
            }
        }
    } 
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product_prescriptive() {
        let empty_vec = Vec::new();
        assert_eq!(
            dot_product_prescriptive(&empty_vec, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0, 2.0, 3.0], &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&empty_vec, &vec![4.0, 3.0]),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0, 2.0], &vec![3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![0.0, 1.0], &vec![2.0, 3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(dot_product_prescriptive(&vec![0.0], &vec![0.0]), Ok(0.0));
        assert_eq!(
            dot_product_prescriptive(&vec![1.0, 0.0], &vec![0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            dot_product_prescriptive(
                &vec![0.0, 1.0, 2.0, 3.0, 4.0],
                &vec![0.0, 1.0, 2.0, 3.0, 4.0]
            ),
            Ok(30.0)
        );
        assert_eq!(
            dot_product_prescriptive(&vec![2.0, 3.0, -1.0], &vec![-1.0, 0.0, 4.0]),
            Ok(-6.0)
        );
    }
    #[test]
    fn test_dot_product_functional() {
        let empty_vec = Vec::new();
        assert_eq!(
            dot_product_functional(&empty_vec.clone(), &empty_vec.clone()),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0, 2.0, 3.0], &empty_vec.clone()),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&empty_vec.clone(), &vec![4.0, 3.0]),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0, 2.0], &vec![3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            dot_product_functional(&vec![0.0, 1.0], &vec![2.0, 3.0, 4.0]),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(dot_product_functional(&vec![0.0], &vec![0.0]), Ok(0.0));
        assert_eq!(
            dot_product_functional(&vec![1.0, 0.0], &vec![0.0, 1.0]),
            Ok(0.0)
        );
        assert_eq!(
            dot_product_functional(
                &vec![0.0, 1.0, 2.0, 3.0, 4.0],
                &vec![0.0, 1.0, 2.0, 3.0, 4.0]
            ),
            Ok(30.0)
        );
        assert_eq!(
            dot_product_functional(&vec![2.0, 3.0, -1.0], &vec![-1.0, 0.0, 4.0]),
            Ok(-6.0)
        );
    }

    #[test]
    fn test_multiply_matrices() {
        let empty_vec: Vec<Vec<f64>> = Vec::new();
        let two_by_two_identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let three_by_three_identity = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_eq!(
            multiply_matrices(&empty_vec, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&empty_vec, &two_by_two_identity),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &empty_vec),
            Err(MatrixError::EmptyVector)
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &two_by_two_identity),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0]]
            ),
            Err(MatrixError::DimensionMismatch)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0, 4.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]
            ),
            Err(MatrixError::InvalidShape)
        );
        assert_eq!(
            multiply_matrices(
                &vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0],],
                &vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]
            ),
            Ok(vec![vec![22.0, 28.0], vec![49.0, 64.0]])
        );
        assert_eq!(
            multiply_matrices(
                &vec![
                    vec![1.0, 2.0, 3.0],
                    vec![0.0, 0.0, 0.0],
                    vec![10.0, 20.0, 30.0],
                    vec![0.0, 0.0, 0.0],
                    vec![-1.0, -1.0, -1.0]
                ],
                &vec![vec![1.0, 2.0], vec![1.0, 4.0], vec![1.0, 6.0]]
            ),
            Ok(vec![
                vec![6.0, 28.0],
                vec![0.0, 0.0],
                vec![60.0, 280.0],
                vec![0.0, 0.0],
                vec![-3.0, -12.0]
            ])
        );
        assert_eq!(
            multiply_matrices(&three_by_three_identity, &three_by_three_identity),
            Ok(three_by_three_identity)
        );
    }
}
