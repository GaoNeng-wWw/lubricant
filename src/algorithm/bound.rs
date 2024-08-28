use super::binary_search;
/// Binary search vec[start..min(end, vec.len())] and return item index. If exists will return Some(idx) else will return None
/// ```
/// use lubricant::algorithm::bound;
/// let res = bound(vec![1,2,3], 0, 3, 1);
/// assert_eq!(res.is_some(), true)
/// ```
pub fn bound<T: Ord + Clone>(
    vec: Vec<T>,
    start: usize,
    end:usize,
    item: T
) -> Option<usize>{
    let new_vec = &vec[start..std::cmp::min(end, vec.len())];
    binary_search(new_vec.to_vec(), item)
}

#[cfg(test)]
mod test {
    use super::bound;
    #[test]
    pub fn include(){
        let res = bound(vec![1,2,3], 0, 3, 1);
        assert_eq!(res.is_some(), true)
    }
    #[test]
    pub fn end_overflow(){
        let res = bound(vec![1,2,3], 0, 300, 1);
        assert_eq!(res.is_some(), true)
    }

    #[test]
    pub fn item_exists(){
        let res = bound(vec![1,2,3], 0, 3, 1);
        assert_eq!(res.is_some(), true)
    }

    
    #[test]
    pub fn item_not_exist(){
        let res = bound(vec![1,2,3], 0, 3, 0);
        assert_eq!(res.is_some(), false)
    }
}