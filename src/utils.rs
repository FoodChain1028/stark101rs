use crate::field::FieldElement;

/// Remove the specific element from the end of the vector
pub fn remove_trailing_elements<T: PartialEq>(v: Vec<T>, element: T) -> Vec<T> {
    v.into_iter().rev().skip_while(|x| *x == element).collect()
}

/// Zip two vectors with different lengths by padding the shorter one with zeros
/// and apply the operation element-wise.
pub fn zip_with<F>(a: Vec<FieldElement>, b: Vec<FieldElement>, op: F) -> Vec<FieldElement>
where
    F: Fn(FieldElement, FieldElement) -> FieldElement,
{
    let max_len = std::cmp::max(a.len(), b.len());
    let mut a = a;
    let mut b = b;
    a.resize(max_len, FieldElement::zero());
    b.resize(max_len, FieldElement::zero());

    a.into_iter().zip(b).map(|(x, y)| op(x, y)).collect()
}
