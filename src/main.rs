use nannou::{noise::{NoiseFn, Perlin, Seedable}, prelude::*};

macro_rules! random_in_window {
    ($rect: expr) => {
        Vec2::new(random_range(-$rect.w()/2., $rect.w()/2.), random_range(-$rect.h()/2., $rect.h()/2.))
    };
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Perlin,
    cells: Vec<Cell>
}

fn model(app: &App) -> Model {
    app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();



    let mut cells = Vec::with_capacity(100);
    for _ in 0..cells.capacity() {
        cells.push(Cell::new(random_in_window!(app.window_rect())))
    }
    Model {
        grid: Perlin::new().set_seed(1),
        cells
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for cell in model.cells.iter_mut() {
        let theta = (model.grid.get([cell.pos.x as _, cell.pos.y as _])+1.)*1./2.*2.*PI as f64;
        cell.dir = Vec2::new(theta.cos() as _, theta.sin() as _);
        cell.update()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    for cell in model.cells.iter() {
        draw.ellipse()
            .radius(5.)
            .color(BURLYWOOD)
            .xy(cell.pos);

        draw.line()
            .color(GREEN)
            .weight(5.)
            .start(cell.pos)
            .end(cell.pos + cell.dir.normalize()*50.);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

struct Cell {
    pos: Vec2,
    dir: Vec2
}

impl Cell {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            dir: Vec2::default()
        }
    }

    fn update(&mut self) {
        self.pos += self.dir.normalize()
    }
}
