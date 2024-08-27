/// This function will return all elements that pass through the function `f`
/// ```
/// use lubricant::array;
/// let even = array::filter(vec![1,2,3,4,5,6,7,8,9,10], |item| { item % 2 == 0 });
/// assert_eq!(
///     even,
///     vec![2,4,6,8,10]
/// );
/// ```
pub fn filter<T, U: Fn(&T) -> bool>(
    vec: Vec<T>,
    f: U
) -> Vec<T>
where 
    T: Clone
{
    let mut ret: Vec<T> = vec![];
    for item in vec.iter(){
        if f(item) == true{
            ret.push(item.clone());
        }
    }
    ret
}

#[cfg(test)]
mod test {
    #[test]
    fn filter(){
        let even = super::filter(vec![1,2,3,4,5,6,7,8,9,10], |item| { item % 2 == 0 });
        assert_eq!(
            even,
            vec![2,4,6,8,10]
        );
    }
}