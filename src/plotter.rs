use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;

type Points = Vec<(f64, f64)>;

pub fn plot(lines: &[Points], id: usize) {
	let mut v = ContinuousView::new();

	for line in lines.into_iter() {
		v = v.add(Plot::new(line.to_vec()).line_style(LineStyle::new()));
	}
	v = v.x_label("time").y_label("concentration");
	Page::single(&v)
		.dimensions(1000, 200)
		.save(format!("plots/{}.svg", id))
		.unwrap();
}
