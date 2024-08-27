/// Execute function f on each element in the array 'vec' return new value, if function f have return value
/// ```
/// use lubricant::array::map;
/// let vec = vec![1,2,3];
/// let vec2 = map(vec,|item|{
///     item + 2
/// });
/// assert_eq!(vec2[0], 3);
/// assert_eq!(vec2[1], 4);
/// assert_eq!(vec2[2], 5);
/// ```
/// 
pub fn map<T, U: FnMut(&T) -> R, R>(vec: Vec<T>,mut f: U) -> Vec<R>{
    let mut ans = vec![];
    for item in vec.iter() {
        ans.push(
            f(item)
        )
    }
    ans
}

#[cfg(test)]
mod test {
    use super::map;

    #[test]
    fn empty_loop(){
        map(vec![1,2,3], |_|{});
    }
    
    #[test]
    fn number_list(){
        let v = vec![1,2,3];
        let res = map(v, |item|{ item * 2 });
        assert_eq!(
            res[0], 2
        );
        assert_eq!(
            res[1], 4
        );
        assert_eq!(
            res[2], 6
        );
    }

    #[test]
    fn sturct_list(){
        struct T {
            val: i32
        }
        let v = vec![
            T{
                val: 1
            },

            T{
                val: 2
            },
            T{
                val: 3
            },
        ];
        let res = map(v, |item| { 
            T{
                val: item.val * 2
            }
        });
        assert_eq!(res[0].val, 2);
        assert_eq!(res[1].val, 4);
        assert_eq!(res[2].val, 6);
    }

}