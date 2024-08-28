/// Binary saech
/// Is the binary search element `item` in `arr`. If exists will return Some(index)
/// ```
/// use lubricant::algorithm::binary_search;
/// let res = binary_search(
///     vec![1,2,3,4,5,6,7,8,9,10,11],
///     1
/// );
/// assert_eq!(res.is_none(), false);
/// assert_eq!(res.is_some(), true);
/// assert_eq!(res.unwrap(), 0);
/// ```
pub fn binary_search<T:Ord>(arr: Vec<T>, item: T) -> Option<usize>{
    let mut l = 0;
    let mut h = arr.len();
    while l<h {
        let mid = l + (h - l) / 2;
        match item.cmp(&arr[mid]) {
            std::cmp::Ordering::Less => h = mid,
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => l = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod test{
    #[test]
    pub fn include(){
        let res = super::binary_search(
            vec![1,2,3,4,5,6,7,8,9,10,11],
             1
        );
        assert_eq!(res.is_none(), false);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap(), 0);
    }
    #[test]
    pub fn not_include(){
        let res = super::binary_search(
            vec![1,2,3,4,5,6,7,8,9,10,11],
             0
        );
        assert_eq!(res.is_none(), true);
        assert_eq!(res.is_some(), false);
    }
}