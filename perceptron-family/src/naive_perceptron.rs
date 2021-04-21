use itertools::izip;
use ndarray::prelude::*;
fn quick_start() {
    let d = 2;
    let mut w = Array::<f64, _>::zeros((1, d));
    let mut b: f64 = 0_f64;
    println!("w0: {}", w);
    let x = array![[1., 0.], [1., 1.]];
    let y = array![[1.], [-1.]];
    for (data, target) in izip!(x.rows(), y.rows()) {
        let g = &data * &target;
        println!("data={}, target={},g={}", data, target, g);
    }
}

trait NaivePerceptron {
    fn init();
    fn train();
    fn export_weight();
}

#[cfg(test)]
mod tests {
    use super::quick_start;
    #[test]
    fn it_works() {
        quick_start();
        assert_eq!(2 + 2, 4);
    }
}
