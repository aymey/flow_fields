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
        grid: Grid::new(5, 200, app.window_rect()),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.grid.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    for &field in model.grid.fields.iter() {
        draw.ellipse()
            .radius(2.*(PI*t).sin())
            .color(BURLYWOOD)
            .xy(field);
    }
    for cell in model.grid.cells.iter() {
        draw.ellipse()
            .radius(1.)
            .color(GREY)
            .xy(cell.pos);

        // debug
        draw.line()
            .weight(1.)
            .color(GREEN)
            .start(cell.pos)
            .end(cell.pos + cell.direction.normalize()*50.);
    }


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

macro_rules! random_in_window {
    ($rect: expr) => {
        Vec2::new(random_range(-$rect.w()/2., $rect.w()/2.), random_range(-$rect.h()/2., $rect.h()/2.))
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

            lifetime: random_range(100, 900),
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
    const ATTRACTION: f32 = 1000.;

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

                // cheap hacky way instead of dot product solve for theta and use that to get resultant direction
                // since these vectors are noramlized they form a circle of radius 1 so we can comput the midpoint between two point of the circumference and normalize that for a good approx
                cell.direction = midpoint(cell.direction, direction);

                // let distance = cell.pos.distance(field);
                // let attraction = distance/Self::ATTRACTION;
                // cell.pos += cell.direction * attraction;
            }
        }
    }
}

fn midpoint(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new((a.x+b.x)/2., (a.y+b.y)/2.)
}
