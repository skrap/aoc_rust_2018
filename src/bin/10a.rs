extern crate regex;

#[derive(Clone)]
struct Light {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Light {
    fn new(pos: (i32, i32), vel: (i32, i32)) -> Light {
        Light {pos, vel}
    }

    fn tick(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }
}

fn bounding_box(lights: &Vec<Light>) -> ((i32,i32), (i32,i32)) {
    let mut x_bounds = (lights[0].pos.0, lights[0].pos.0);
    let mut y_bounds = (lights[0].pos.1, lights[0].pos.1);
    for light in lights {
        x_bounds.0 = x_bounds.0.min(light.pos.0);
        x_bounds.1 = x_bounds.1.max(light.pos.0);
        y_bounds.0 = y_bounds.0.min(light.pos.1);
        y_bounds.1 = y_bounds.1.max(light.pos.1);
    }
    ((x_bounds.0,y_bounds.0), (x_bounds.1,y_bounds.1))
}

fn bounding_area(lights: &Vec<Light>) -> i64 {
    let (topleft, botright) = bounding_box(lights);
    (topleft.0-botright.0) as i64 * (topleft.1-botright.1) as i64
}

fn main() {
    let input = include_str!("10_input");
    let mut lights = Vec::new();

    let re = regex::Regex::new(r"position=< *(.+), *(.+)> velocity=< *(.+), *(.+)>").unwrap();
    for m in re.captures_iter(input) {
        lights.push(Light::new(
            (m[1].parse().unwrap(),m[2].parse().unwrap()), 
            (m[3].parse().unwrap(),m[4].parse().unwrap())
        ));
    }

    let mut area = bounding_area(&lights);
    let mut ticks = 0;
    loop {
        let mut next = lights.clone();
        for l in next.iter_mut() {
            l.tick();
        }
        let next_area = bounding_area(&next);
        if next_area > area {
            break;
        }
        ticks += 1;
        area = next_area;
        lights = next;
    }
    let bbox = bounding_box(&lights);
    println!("bounding area {} box {:?}", bounding_area(&lights), &bbox);

    let mut render : Vec<Vec<char>> = Vec::new();
    for _row in ((bbox.0).1)..=((bbox.1).1) {
        render.push(Vec::new());
        for _col in ((bbox.0).0)..=((bbox.1).0) {
            render.last_mut().unwrap().push(' ');
        }
    }
    let xmin = (bbox.0).0;
    let ymin = (bbox.0).1;

    for l in lights {
        let mut pos = l.pos.clone();
        pos.0 -= xmin;
        pos.1 -= ymin;
        render[pos.1 as usize][pos.0 as usize] = '#';
    }
    for row in render {
        for c in row {
            print!("{}", c);
        }
        print!("\n");
    }
    println!("message appeared in {} ticks", ticks);
}