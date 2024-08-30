///
/// Replace vec old value to new value if old value exists
/// WARNING: This function will change vec
/// ```
/// use lubricant::algorithm::replace;
/// let mut v = vec![1,2,3];
/// replace(&mut v, 1, 2);
/// assert_eq!(v, [2,2,3])
/// ```
pub fn replace<T: PartialEq + Eq + Copy>(
    vec: &mut Vec<T>,
    old_value:T,
    new_value:T
){
    for i in 0..vec.len() {
        if vec[i] == old_value {
            vec[i] = new_value;
        }
    }
}

///
/// Return new vec, repalce old value to new value.
/// ```
/// use lubricant::algorithm::to_replace;
/// let v = vec![1,2,3];
/// let r = to_replace(&v, 1, 2);
/// assert_eq!(v, [1,2,3]);
/// assert_eq!(r, [2,2,3])
/// ```
pub fn to_replace<T: PartialEq + Eq + Copy>(
    vec: &Vec<T>,
    old_value:T,
    new_value:T
) -> Vec<T>{
    let mut ret = vec.clone();
    replace(&mut ret, old_value, new_value);
    ret
}

#[cfg(test)]
mod test {
    #[test]
    fn replace(){
        let mut v = vec![1,2,3];
        super::replace(&mut v, 1, 2);
        assert_eq!(v, [2,2,3])
    }

    #[test]
    fn to_replace(){
        let v = vec![1,2,3];
        let r = super::to_replace(&v, 1, 2);
        assert_eq!(v, [1,2,3]);
        assert_eq!(r, [2,2,3])
    }
}