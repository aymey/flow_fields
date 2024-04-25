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
    grid: Vec<Vec2>,
    cells: Vec<Cell>,
    noise: Perlin
}

fn model(app: &App) -> Model {
    app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();
    let window = app.window_rect();

    let mut noise = Perlin::new().set_seed(random());
    let mut grid: Vec<Vec2> = Vec::with_capacity(window.w() as usize *window.h() as usize);
    for y in 0..window.h() as usize {
        for x in 0..window.w() as usize {
            let theta = ((noise.get([x as  f64 + 0.003, y as f64 + 0.01])+1.)*1./2.)*2.*PI as f64;
            grid.push(Vec2::new(theta.cos() as _, theta.sin() as _));
        }
    }

    let mut cells = Vec::with_capacity(100);
    for _ in 0..cells.capacity() {
        cells.push(Cell::new(random_in_window!(window)))
    }

    Model {
        grid,
        cells,
        noise
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for (i, cell) in model.cells.iter_mut().enumerate() {
        // cell.dir = model.grid[(cell.pos.x + cell.pos.y * app.window_rect().w()) as usize];

        let t = app.elapsed_frames() as f64/ 100.;
        let x = model.noise.get([ cell.pos.x as f64 / 128., cell.pos.y as f64 / 137., t + i as f64/ 1000. ]);
        let y = model.noise.get([ -cell.pos.y as f64 / 128., cell.pos.x as f64 / 137., t + i as f64/ 1000. ]);

        cell.dir = vec2(x as f32, y as f32);


        cell.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for cell in model.cells.iter() {
        draw.ellipse()
            .radius(5.)
            .color(BURLYWOOD)
            .xy(cell.pos);

        draw.line()
            .color(GREEN)
            .weight(1.)
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
            dir: Vec2::default() + 100.
        }
    }

    fn update(&mut self) {
        self.pos += self.dir.normalize()
    }
}
