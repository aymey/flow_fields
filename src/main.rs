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
        grid: Grid::new(10, 100, app.window_rect()),
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
    cells: Vec<Cell>,
    original: Vec<Cell>
}

#[derive(Default, Clone, Copy)]
struct Cell {
    pos: Vec2,
    direction: Vec2,

    lifetime: u32,
    id: usize,
}

impl Cell {
    fn new(window: Rect, id: usize) -> Self {
        let r = random_in_window!(window);
        Self {
            pos: r,
            direction: r,

            lifetime: random_range(10, 90),
            id
        }
    }

    fn update(&mut self) -> Option<()> {
        self.lifetime -= 1;
        if self.lifetime == 0 {
            return None;
        }

        self.pos += self.direction.normalize();
        Some(())
    }

    fn revive(&mut self, replacement: Self) {
        *self = replacement;
    }
}

impl Grid {
    const ATTRACTION_STRENGTH: f32 = 10.;

    fn new(fields: usize, cells: usize, window: Rect) -> Self {
        let mut grid = Self::default();
        for _ in 0..fields {
            grid.fields.push(random_in_window!(window))
        }
        for id in 0..cells {
            let cell = Cell::new(window, id);
            grid.cells.push(cell);
            grid.original.push(cell);
        }

        grid
    }

    fn update(&mut self) {
        for cell in self.cells.iter_mut() {
            if let None = cell.update() {
                cell.revive(self.original[cell.id])
            }
        }

        self.attract()
    }

    fn attract(&mut self) {
        for cell in self.cells.iter_mut() {
            for &field in self.fields.iter() {
                let direction = (field - cell.pos).normalize();
                let distance = distance(cell.pos, field);
                let attraction = distance/Self::ATTRACTION_STRENGTH;
                cell.pos += direction * attraction;
            }
        }
    }
}

fn distance(a: Vec2, b: Vec2) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
