// sum one and two and return it
pub fn add_ints(one: i32, two: i32) -> i32{
    todo!()
}





//Implement the flipfront function. 
//Given an array of integers and a number n between 2 and the length of the array (inclusive),
//return the array with the order of the first n elements reversed.
pub fn flipfront(subject: &Vec<i32>, n: usize) -> Vec<i32>{
    subject.clone()
}
//Challenge
//Given an array of integers, sort the array (smallest to largest)
//using the flipfront function on the entire array. For example, the array:
pub const CHALLENGE0: [i32; 4] = [3, 1, 2, 1];
//let vec = CHALLENGE0.to_vec();




//Given a number n, determine the number of times the digit "1" appears if you write out
// all numbers from 1 to n inclusive.
pub fn one_sitings(input: usize) -> usize{
    0
}




//A binary array is an array consisting of only the values 0 and 1.
//Given a binary array of any length, return an array of positive integers that represent
//the lengths of the sets of consecutive 1's in the input array, in order from left to right.
pub fn nonogramrow(input: Vec<bool>) -> Vec<usize>{
    let output =  Vec::new();
    output
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add_two_ints() {
        //let sum: i32 = add_ints(-1, 2);
    }

    #[test]
    fn flipfront_test(){
        let sample0 = vec![0, 1, 2, 3, 4];
        let sample1 = vec![1, 2, 2, 2];
        assert_eq!(flipfront(&sample0, 2), vec![1, 0, 2, 3, 4]);
        assert_eq!(flipfront(&sample0, 3), vec![2, 1, 0, 3, 4]);
        assert_eq!(flipfront(&sample0, 5), vec![4, 3, 2, 1, 0]);
        assert_eq!(flipfront(&sample1, 3), vec![2, 2, 1, 2]);
    }


    #[test]
    fn one_sitings_test(){
        assert_eq!(one_sitings(1), 1);
        assert_eq!(one_sitings(5), 1);
        assert_eq!(one_sitings(10), 2);
        assert_eq!(one_sitings(20), 12);
        assert_eq!(one_sitings(1234), 689);
        assert_eq!(one_sitings(5123), 2557);
        assert_eq!(one_sitings(70000), 38000);
        assert_eq!(one_sitings(123321), 93395);
    }

    #[test]
    fn nonogramrow_test(){
        assert_eq!(nonogramrow(vec![]), vec![]);
        assert_eq!(nonogramrow(vec![false,false,false,false,false]), vec![]);
        assert_eq!(nonogramrow(vec![false,true,true,false,true,false,false,true,true,false]), vec![2, 1, 2]);
        assert_eq!(nonogramrow(vec![true,true,true,true,true,true]), vec![6]);
    }
}
