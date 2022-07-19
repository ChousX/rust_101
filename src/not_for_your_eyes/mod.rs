pub fn square_2d_array(input: &[[f32; 10]; 10]) -> Vec<[&f32; 4]>{
    let mut output = Vec::with_capacity(81);
    for y in 0..9{
        for x in 0..9{
            let top_left = &input[y][x];
            let top_right = &input[y][x+1];
            let bot_left = &input[y+1][x];
            let bot_right = &input[y+1][x+1];

            output.push([top_left, top_right, bot_left, bot_right])
        }
    }

    output
}