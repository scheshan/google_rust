/***
数组可以包含其他数组：

let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
What is the type of this variable?

Use an array such as the above to write a function transpose which will transpose a matrix (turn rows into columns):

硬编码这两个函数，让它们处理 3 × 3 的矩阵。

将下面的代码复制到 https://play.rust-lang.org/ 并实现上述函数：
 */


fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res = [[0i32; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            res[i][j] = matrix[j][i]
        }
    }

    res
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
