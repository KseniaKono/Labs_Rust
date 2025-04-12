
// Удалите это, когда завершите работу над кодом
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrix_t = [[0; 3]; 3];
    let mut m = 0;
    for a in matrix {
        let mut n = 0;
        for b in a {
            matrix_t[n][m] = b;
            n+=1;
        }
        m+=1;
    }
    return matrix_t;
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
        [101, 102, 103], // <-- комментарий заставляет rustfmt добавить новую строку
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("матрица: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("транспонированная матрица: {:#?}", transposed);
}