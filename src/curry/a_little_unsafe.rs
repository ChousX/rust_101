//given the width (x) and the hight (y) compute how many destint 
//squares from agesent points
pub fn how_many_squares(x: usize, y: usize) -> usize{
    todo!()
}



//now lets actuly get refrences to the values in your 2d grid
//using the aray below generate a coppy of every point for every box
//into a vector
pub const ARRAY_2D_0: [[f32; 10]; 10] = [
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.0, 0.5, 0.0, 1.0, 0.7, 0.0, 0.5, 0.0, 1.0, 0.7],
[0.8, 0.8, 0.8, 1.8, 0.8, 0.8, 0.8, 0.8, 1.8, 0.8]
];

pub fn square_2d_array(input: &[[f32; 10]; 10]) -> Vec<[&f32; 4]>{
    todo!()
}





#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn how_many_squares_test(){
        let two_by_two = how_many_squares(2, 2);
        assert_eq!(two_by_two, 1);

        let three_by_two = how_many_squares(3, 2);
        assert_eq!(three_by_two, 2);
        
        let zero = how_many_squares(1, 1);
        assert_eq!(zero, 0);
        
        let zero = how_many_squares(0, 0);
        assert_eq!(zero, 0);
        
        let zero = how_many_squares(1000, 1);
        assert_eq!(zero, 0);
        
        let four_by_four = how_many_squares(4, 4);
        assert_eq!(four_by_four, 9);
        
        let ten_by_ten = how_many_squares(10, 10);
        assert_eq!(ten_by_ten, 81);
    }

    #[test]
    fn square_2d_array_test(){
        let correct = crate::not_for_your_eyes::square_2d_array(&ARRAY_2D_0);
        let yours = square_2d_array(&ARRAY_2D_0);
        assert_eq!(yours, correct);
    }
}