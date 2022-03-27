fn right_rotate<T>(graph: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let H = graph.len();
    let W = graph[0].len();
    let mut ret = vec![vec![graph[0][0]; H]; W];
    for h in 0..H {
        for w in 0..W {
            ret[w][H - h - 1] = graph[h][w];
        }
    }
    ret
}
fn left_rotate<T>(graph: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let H = graph.len();
    let W = graph[0].len();
    let mut ret = vec![vec![graph[0][0]; H]; W];
    for h in 0..H {
        for w in 0..W {
            ret[W - w - 1][h] = graph[h][w];
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotated_char() {
        // .....
        // ..#..
        // .###.
        // .....
        let original: Vec<Vec<char>> = vec![
            ".....".chars().collect::<Vec<char>>(),
            "..#..".chars().collect::<Vec<char>>(),
            ".###.".chars().collect::<Vec<char>>(),
            ".....".chars().collect::<Vec<char>>(),
        ];

        // ....
        // ..#.
        // .##.
        // ..#.
        // ....

        let expected: Vec<Vec<char>> = vec![
            "....".chars().collect::<Vec<char>>(),
            "..#.".chars().collect::<Vec<char>>(),
            ".##.".chars().collect::<Vec<char>>(),
            "..#.".chars().collect::<Vec<char>>(),
            "....".chars().collect::<Vec<char>>(),
        ];

        let transformed = left_rotate(&original);
        assert_eq!(transformed, expected);
    }

    #[test]
    fn test_right_rotated_char() {
        // ....
        // ..#.
        // .##.
        // ..#.
        // ....
        let original: Vec<Vec<char>> = vec![
            "....".chars().collect::<Vec<char>>(),
            "..#.".chars().collect::<Vec<char>>(),
            ".##.".chars().collect::<Vec<char>>(),
            "..#.".chars().collect::<Vec<char>>(),
            "....".chars().collect::<Vec<char>>(),
        ];

        // .....
        // ..#..
        // .###.
        // .....
        let expected: Vec<Vec<char>> = vec![
            ".....".chars().collect::<Vec<char>>(),
            "..#..".chars().collect::<Vec<char>>(),
            ".###.".chars().collect::<Vec<char>>(),
            ".....".chars().collect::<Vec<char>>(),
        ];

        let transformed = right_rotate(&original);
        assert_eq!(transformed, expected);
    }
    #[test]
    fn test_left_rotated_usize() {
        // 17,13,9,5,1
        // 18,14,10,6,2
        // 19,15,11,7,3
        // 20,16,12,8,4
        let original: Vec<Vec<usize>> = vec![
            vec![17, 13, 9, 5, 1],
            vec![18, 14, 10, 6, 2],
            vec![19, 15, 11, 7, 3],
            vec![20, 16, 12, 8, 4],
        ];

        // 1,2,3,4
        // 5,6,7,8
        // 9,10,11,12
        // 13,14,15,16
        // 17,18,19,20
        let expected: Vec<Vec<usize>> = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            vec![17, 18, 19, 20],
        ];

        let transformed = left_rotate(&original);
        assert_eq!(transformed, expected);
    }

    #[test]
    fn test_right_rotated_usize() {
        // 1,2,3,4
        // 5,6,7,8
        // 9,10,11,12
        // 13,14,15,16
        // 17,18,19,20
        let original: Vec<Vec<usize>> = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            vec![17, 18, 19, 20],
        ];

        // 17,13,9,5,1
        // 18,14,10,6,2
        // 19,15,11,7,3
        // 20,16,12,8,4
        let expected: Vec<Vec<usize>> = vec![
            vec![17, 13, 9, 5, 1],
            vec![18, 14, 10, 6, 2],
            vec![19, 15, 11, 7, 3],
            vec![20, 16, 12, 8, 4],
        ];

        let transformed = right_rotate(&original);
        assert_eq!(transformed, expected);
    }
}
