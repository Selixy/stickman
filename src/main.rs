mod window_conf;
mod stickman;
mod physics;
mod l_system;
mod genetic;

use std::sync::{Arc, Mutex};
use std::thread;
use macroquad::prelude::*;
use crate::window_conf::window_conf;
use crate::stickman::draw_stickman;
use crate::physics::Physics;
use crate::l_system::draw_l_system_tree;
use crate::genetic::{GeneticAlgorithm, Individual};

const FIXED_TICK_RATE: f32 = 1.0 / 60.0;
const GROUND_Y: f32 = 545.0;

struct Tree {
    position_x: f32,
    seed: u32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let shared_stickmen = Arc::new(Mutex::new(Vec::new()));

    // Algorithme génétique pour créer des individus
    let mut ga = GeneticAlgorithm::new(5, 2, 0.1);
    let initial_population = ga.population.clone();

    {
        let mut stickmen_guard = shared_stickmen.lock().unwrap();
        for (i, individual) in initial_population.into_iter().enumerate() {
            let position_x = 150.0 + i as f32 * 100.0;
            let mut physics = Physics::new_with_individual(
                (position_x, GROUND_Y - 200.0),
                FIXED_TICK_RATE,
                individual,
            );
            physics.start();
            stickmen_guard.push(physics);
        }
    }

    let stickmen_clone = Arc::clone(&shared_stickmen);

    // Thread de gestion des cycles et de la régénération des stickmen
    thread::spawn(move || {
        let mut ga = GeneticAlgorithm::new(5, 2, 0.1);

        loop {
            {
                let mut stickmen_guard = stickmen_clone.lock().unwrap();

                // Supprime les stickmen morts
                stickmen_guard.retain(|stickman| stickman.is_alive());

                // Régénération si aucun stickman n'existe
                if stickmen_guard.is_empty() {
                    println!("Regénération de 5 Stickmen !");
                    ga.generate_new_generation();
                    let new_population = ga.population.clone();

                    for (i, individual) in new_population.into_iter().enumerate() {
                        let position_x = 150.0 + i as f32 * 100.0;
                        let mut physics = Physics::new_with_individual(
                            (position_x, GROUND_Y - 200.0),
                            FIXED_TICK_RATE,
                            individual,
                        );
                        physics.start();
                        stickmen_guard.push(physics);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    let trees = vec![
        Tree { position_x: 100.0, seed: 5 },
        Tree { position_x: 300.0, seed: 7 },
        Tree { position_x: 500.0, seed: 9 },
        Tree { position_x: 700.0, seed: 11 },
        Tree { position_x: 900.0, seed: 13 },
    ];

    // Boucle principale de rendu
    loop {
        clear_background(WHITE);

        // Dessine les arbres
        for tree in &trees {
            draw_l_system_tree(
                tree.position_x,
                GROUND_Y - 10.0,
                -90.0,
                100.0,
                (tree.seed % 5 + 3).try_into().unwrap(),
            );
        }

        // Dessine la ligne du sol
        draw_line(0.0, GROUND_Y, screen_width(), GROUND_Y, 10.0, DARKGREEN);

        // Dessine les stickmen
        {
            let stickmen_guard = shared_stickmen.lock().unwrap();
            for physics in stickmen_guard.iter() {
                let joints = physics.get_joints();
                let joints = joints.lock().unwrap();
                draw_stickman(&joints);
            }
        }

        next_frame().await;
    }
}
