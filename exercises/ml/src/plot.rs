use plotters::prelude::*;
use std::f64::consts::PI;

use rand;
use rand::Rng;


fn create_scratter()-> Result<(), Box<dyn std::error::Error>> {
    // create backend
    let root = BitMapBackend::new("sin_with_noise.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create sine with noise data
    let mut rng = rand::rng();
    let x_values: Vec<f64> = (0..1000).map(|x| x as f64 * 4.0 * PI / 1000.0).collect();
    let sin_values: Vec<(f64, f64)> = x_values
        .iter()
        .map(|&x| {
            let noise = rng.random_range(-0.2..0.2);
            (x, x.sin() + noise)
        })
        .collect();

    // create chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Đồ thị hàm Sin với Nhiễu", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..4.0 * PI, -1.5f64..1.5f64)?;

    // grid and labels
    chart
        .configure_mesh()
        .x_desc("X (radians)")
        .y_desc("sin(X) + noise")
        .draw()?;

    // draw func
    chart
        .draw_series(LineSeries::new(sin_values, &RED))?
        .label("sin(x) + noise")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // add notes
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // finish
    root.present()?;

    println!("Đồ thị đã được lưu vào file 'sin_with_noise.png'");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::plot::create_scratter;

    #[test]
    fn test_scratter_plot() {
        create_scratter();
    }
}
