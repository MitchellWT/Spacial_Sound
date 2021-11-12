use std::any::Any;
use Entity;
use Color;
use Window;
use Canvas;
use sdl2::rect::Point;
use Rect;

pub struct Wall {
    id:       u32,
    collider: Rect
}

impl Entity for Wall {
    // Needed for downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }
    // Gets Id for entity
    fn id(&self) -> u32 {
        self.id
    }
    // Get collider, not mutable
    fn collider(&self) -> Rect {
        self.collider
    }
    // Draws the audio source to the screen as a rect
    fn render(&self, canvas: &mut Canvas<Window>) -> bool {
        canvas.set_draw_color(Color::RGB(0, 0, 155));
        match canvas.fill_rect(self.collider) {
            Ok(_)  => true,
            Err(_) => false
        }
    }
}

impl Wall {
    // Function for creating a wall struct
    pub fn new(id: u32, x: i32, y: i32, width: u32, height: u32) -> Wall {
        let raw = Wall {
            id:       id,
            collider: Rect::from_center(Point::new(x, y), width, height)
        };

        raw
    }
    // Gets the wall interference between two points    
    pub fn get_interference_amount(&self, point_a: Point, point_b: Point) -> i32 {
        match self.collider.intersect_line(point_a, point_b) {
            Some((inter_point_a, inter_point_b)) => {
                return self.between_two_points(&inter_point_a, &inter_point_b) as i32;
            },
            None => 0
        }
    }
    // Gets the distance between two provided points, in our
    // current context these points are the rect intersection
    // for interference
    fn between_two_points(&self, point_a: &Point, point_b: &Point) -> f64 {
        // Calculate delta x
        let mut delta_x: f64 = (point_a.x - point_b.x).into();
        delta_x = delta_x * delta_x;
        // Calculate delta y
        let mut delta_y: f64 = (point_a.y - point_b.y).into();
        delta_y = delta_y * delta_y;

        return (delta_x + delta_y).sqrt();
    }
}
