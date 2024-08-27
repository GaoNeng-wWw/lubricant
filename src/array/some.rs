/// The `some()` method will test whether at least one element in the array passes the test implemented by the provided function
/// In other words, if there is an element that can make the function 'f' return true, then the 'some' function will end
/// ```
/// use lubricant::array::some;
/// fn even(ele: &i64) -> bool{
///   ele % 2 == 0
/// }
/// some(
///     vec![1,2,3],
///     even
/// );
/// ```
/// 
pub fn some<T, U: Fn(&T) -> bool>(
    vec:Vec<T>,
    f:U
) -> bool{
    let mut state = false;
    for item in vec.iter() {
        if state{
            return state;
        }
        state = f(item);
    }
    state
}

#[cfg(test)]
mod test{
    use super::some;

    #[test]
    fn even(){
        let even = |item: &i32| {item%2==0};
        let has_even = some(vec![1,2,3], even);
        assert!(has_even == true)
    }
}