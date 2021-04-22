use itertools::izip;
use ndarray::prelude::*;
extern crate plotly;
use plotly::common::Mode;
use plotly::{ImageFormat, Plot, Scatter};

fn quick_start() {
    let d = 2;
    let mut w = Array::<f64, Ix1>::zeros(d);
    let mut b = Array::<f64, Ix1>::zeros(1);
    println!("w0: {}", w);

    let x = array![[1., 0.], [0.4, 0.], [1.4, 0.], [1., 1.]];
    let y = array![[1.], [1.], [1.], [-1.]];
    let trace1 = Scatter::new(x.clone().column(0).to_vec(), x.clone().column(1).to_vec())
        .mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.show();
    let mut times = 0;
    loop {
        let mut flag: bool = true;
        for (data, target) in izip!(x.rows(), y.rows()) {
            let obj = &target[0] * (&w.dot(&data) + &b[0]);
            println!("obj={}", obj);
            if obj <= 0. {
                times = times + 1;
                flag = false;
                let g = &data * &target;
                w = w + &g;
                b = b + target[0];
                println!("data={}, target={},g={}", data, target, g);
                println!("w={}, b={}", w, b);
            }
        }
        if flag {
            break;
        }
    }
    println!("Total update times: {}", times);
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
