use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Grid
}

fn model(app: &App) -> Model {
    app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model {
        grid: Grid::new(1, 1000, app.window_rect())
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.grid.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    for &field in model.grid.fields.iter() {
        draw.ellipse()
            .radius(2.)
            .color(BURLYWOOD)
            .xy(field);
    }
    for cell in model.grid.cells.iter() {
        draw.ellipse()
            .radius(5.)
            .color(GREY)
            .xy(cell.pos);
    }


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

macro_rules! random_in_window {
    ($rect: expr) => {
        Vec2::new(random_range(-$rect.w(), $rect.w()), random_range(-$rect.h(), $rect.h()))
    };
}

// TODO: maybe each field can have an attraction strength (even negative like some repulse cells)
#[derive(Default)]
struct Grid {
    fields: Vec<Vec2>,
    cells: Vec<Cell>
}

#[derive(Default)]
struct Cell {
    pos: Vec2,
    direction: Vec2,
}

impl Cell {
    fn new(window: Rect) -> Self {
        let r = random_in_window!(window);
        Self {
            pos: r,
            direction: r
        }
    }

    fn update(&mut self) {
        self.pos += self.direction.normalize();
    }
}

impl Grid {
    fn new(fields: usize, cells: usize, window: Rect) -> Self {
        let mut grid = Self::default();
        for i in 0..fields {
            grid.fields.push(random_in_window!(window))
        }
        for i in 0..cells {
            grid.cells.push(Cell::new(window))
        }

        grid
    }

    fn update(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.update()
        }
    }

    fn attract(&mut self) {

    }
}
