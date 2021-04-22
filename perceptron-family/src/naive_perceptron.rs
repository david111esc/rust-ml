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

    let x = array![[1., 0.2], [0.4, 0.6], [1.4, 0.9], [-1., -1.], [-1., 0.5]];
    let y = array![[1.], [1.], [1.], [-1.], [-1.]];

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
    let trace1 = Scatter::new(x.clone().column(0).to_vec(), x.clone().column(1).to_vec())
        .mode(Mode::Markers);

    let trace2_x = Array::linspace(-2.0, 2.0, 10);

    let trace2_y = -(&trace2_x * w[0] + b) / w[1];
    println!("{:?}\n {:?}", trace2_x, trace2_y);
    let trace2 = Scatter::new(trace2_x.to_vec(), trace2_y.to_vec());
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
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
