mod lsystem;

use std::fs::File;
use turtle_graphics::{Canvas, Turtle};
use lsystem::lsystem::LSystem;
use rand::Rng;


pub fn generate_svg(filepath: &str) {

    let mut system = LSystem::from_yml(filepath).unwrap();
    let init_direction = 0.0;
    let iters = 8;

    // let mut system_a = LSystem::new("serpinski", "A", 60);
    // system_a.rules.insert('A', String::from("+B−A−B+"));
    // system_a.rules.insert('B', String::from("−A+B+A−"));
    // let init_direction = 90.0;
    // let iters = 9;

    // system_a.generate_system(iters);
    
    // let mut system = LSystem::new("sepinski", &system_a.endstring, 60);
    // system.rules.insert('A', String::from("F"));
    // system.rules.insert('B', String::from("F"));
    // system.generate_system(1);

    // let mut system = LSystem::new("rand_fractal_plant", "X", 25);
    // system.rules.insert('X', String::from("F-[[X]+X]+F[+FX]-X"));
    // system.rules.insert('F', String::from("FF"));
    // let init_direction = 0.0;
    // let iters = 8;
    system.generate_system(iters);

    let mut t = Canvas::new();
    t.right(init_direction);

    
    let distance = 10.0;
    let angle: f32 = system.turn_angle as f32;
    let mut rng = rand::thread_rng();

    for character in system.endstring.chars() {
        let distance = rng.gen_range(distance * 0.85, distance * 1.15);
        let angle = rng.gen_range(angle * 0.85, angle * 1.15);
        // println!("{}", distance);
        match character {
            'F' => t.forward(distance),
            '−'|'-' => t.rotate(-angle),
            '+' => t.rotate(angle),
            '[' => t.push(),
            ']' => t.pop(),
            _ => (),
        }
    }
    let filename = format!("outputs/{}_{}", system.name, iters);
    t.save_svg(&mut File::create(filename.to_string() + ".svg").unwrap())
    .unwrap();
}