///
/// Swap two variable
/// ```
/// use lubricant::algorithm::swap;
/// let mut a = 1;
/// let mut b = 2;
/// swap(&mut a,&mut b);
/// assert_eq!(a, 2);
/// assert_eq!(b, 1)
/// ```
pub fn swap<T>(
    a: &mut T,
    b: &mut T
){
    std::mem::swap(a, b);
}

#[cfg(test)]
mod test{
    #[test]
    fn swap_value(){
        let mut a = 1;
        let mut b = 2;
        super::swap(&mut a,&mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1)
    }
}