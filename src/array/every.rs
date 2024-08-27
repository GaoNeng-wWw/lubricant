pub fn every<T, U: Fn(&T) -> bool>(
    vec: Vec<T>,
    f: U
) -> bool {
    let mut state = false;
    for item in vec.iter(){
        state = f(item);
    }
    state
}

#[cfg(test)]
mod test{
    use super::every;
    #[test]
    fn should_to_be_true(){
        let all_is_even = every(vec![1,2,3], |item| { item % 2 == 0});
        assert_eq!(all_is_even, false);
        assert_eq!(
            every(vec![2,2,2], |item| {item % 2 == 0}),
            true
        )
    }
}