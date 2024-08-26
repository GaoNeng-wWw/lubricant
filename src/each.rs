/// Execute function f on each element in the array 'vec'
/// ```
/// use lubricant::each::each;
/// let vec = vec![1,2,3];
/// each(vec,|item|{})
/// ```
pub fn each<T, U: FnMut(&T) -> ()>(
    vec:Vec<T>,
    mut f:U
){
    for item in vec.iter() {
        f(item);
    }
}

pub fn each_clone<T, U: FnMut(T) -> ()>(
    vec:Vec<T>,
    mut f:U
)
where
    T: Clone
{
    for item in vec.iter() {
        f(item.clone());
    }
}

#[cfg(test)]
mod test{
    use std::vec;

    use super::each;

    #[test]
    fn static_vec(){
        let vec = vec![1,2,3];
        each(vec, |item| {});
    }

    #[test]
    fn mut_vec(){
        let vec = vec![1,2,3];
        let mut vec2: Vec<i32> = vec![];

        each(vec.clone(), |item| {
            vec2.push(item.clone())
        });

        assert_eq!(vec2.clone(), vec);
    }
}