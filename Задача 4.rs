// Вычислите модуль вектора просуммировав квадраты его координат
// и вычислив кввдратный корень полученного значения. Используйте метод `sqrt()` для вычисления
// корня, следующим образом: v.sqrt().


fn magnitude(vec: &[f64]) -> f64 {
    let mut sumsq = 0.0;
    for i in vec{
        sumsq = sumsq + i*i;
    }
    return sumsq.sqrt();
}

// Нормализуйте вектор вычислив его модуль и разделив все его координаты на 
// этот модудль.


fn normalize(vec: &mut [f64]) {
    let modul= magnitude(&vec);
    for i in 0..vec.len(){
        vec[i]/=modul;
    }
}

// Используйте эту функцию main для проверки своей работы.

fn main() {
    println!("Модуль единичного вектора: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Модуль {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Модуль {v:?} после нормализации: {}", magnitude(&v));
}