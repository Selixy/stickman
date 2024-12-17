mod window_conf;
mod stickman;
mod physics;
mod l_system;
mod genetic;

use std::convert::TryInto; // Pour convertir u32 -> usize
use macroquad::prelude::*;
use crate::window_conf::window_conf;
use crate::stickman::draw_stickman;
use crate::physics::Physics;
use crate::l_system::draw_l_system_tree;

const FIXED_TICK_RATE: f32 = 1.0 / 60.0;
const GROUND_Y: f32 = 545.0;

// Structure pour stocker les informations des arbres
struct Tree {
    position_x: f32,
    seed: u32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut stickman_physics: Vec<Physics> = Vec::new();

    // Utiliser des seeds fixes et des positions prédéfinies pour les arbres
    let trees = vec![
        Tree { position_x: 100.0, seed: 5 },
        Tree { position_x: 300.0, seed: 7 },
        Tree { position_x: 500.0, seed: 9 },
        Tree { position_x: 700.0, seed: 11 },
        Tree { position_x: 900.0, seed: 13 },
    ];

    // Initialisation des Stickmen
    for i in 0..5 {
        let position_x = 150.0 + i as f32 * 100.0;
        let mut physics = Physics::new_with_position((position_x, GROUND_Y - 200.0), FIXED_TICK_RATE);
        physics.start();
        stickman_physics.push(physics);
    }

    loop {
        clear_background(WHITE);

        // Dessiner les arbres avec des seeds fixes
        for tree in &trees {
            draw_l_system_tree(
                tree.position_x,
                GROUND_Y - 10.0,
                -90.0,
                100.0,
                (tree.seed % 5 + 3).try_into().unwrap(), // Conversion en usize
            );
        }

        // Dessiner la ligne de sol
        draw_line(0.0, GROUND_Y, screen_width(), GROUND_Y, 10.0, DARKGREEN);

        // Dessiner chaque Stickman
        for physics in &stickman_physics {
            let joints = physics.get_joints();
            let joints = joints.lock().unwrap();
            draw_stickman(&joints);
        }

        next_frame().await;
    }
}
