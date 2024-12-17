mod window_conf;
mod stickman;
mod physics;

use macroquad::prelude::*;
use crate::window_conf::window_conf;
use crate::stickman::draw_stickman;
use crate::physics::Physics;

const FIXED_TICK_RATE: f32 = 1.0 / 60.0;
const GROUND_Y: f32 = 545.0;

const TEXT_START_Y: f32 = 20.0; // Position de départ verticale pour les HTS
const TEXT_GAP_Y: f32 = 15.0;   // Décalage vertical entre chaque HTS

#[macroquad::main(window_conf)]
async fn main() {
    let mut stickman_physics: Vec<Physics> = Vec::new();

    for i in 0..5 {
        let position_x = 150.0 + i as f32 * 100.0;
        let physics = Physics::new_with_position((position_x, GROUND_Y - 200.0), FIXED_TICK_RATE);
        physics.start();
        stickman_physics.push(physics);
    }

    let mut use_fixed_rate = true;

    loop {
        clear_background(WHITE);

        // Détecter le clic sous le sol pour basculer le mode global
        if is_mouse_button_pressed(MouseButton::Left) {
            let (_mx, my) = mouse_position();
            if my > GROUND_Y + 10.0 {
                use_fixed_rate = !use_fixed_rate;
                let tick_rate = if use_fixed_rate { FIXED_TICK_RATE } else { 0.0 };
                for physics in &stickman_physics {
                    physics.set_tick_rate(tick_rate);
                }
            }
        }

        // Dessiner la ligne de sol
        draw_line(0.0, GROUND_Y, screen_width(), GROUND_Y, 10.0, DARKGREEN);

        // Dessiner chaque Stickman
        for physics in &stickman_physics {
            let joints = physics.get_joints();
            let joints = joints.lock().unwrap();
            draw_stickman(&joints);
        }

        // Afficher les HTS de chaque thread en haut à gauche
        let mut y_offset = TEXT_START_Y;
        for (i, physics) in stickman_physics.iter().enumerate() {
            let hts_text = format!("T{} {}Hrz", i + 1, physics.get_hts().round() as i32);
            draw_text(&hts_text, 20.0, y_offset, 20.0, RED);
            y_offset += TEXT_GAP_Y;
        }

        // Affichage global du mode sous le sol
        let mode_text = if use_fixed_rate { "Réel mod" } else { "Learning mod" };
        let text_dimensions = measure_text(mode_text, None, 30, 1.0);
        let text_x = (screen_width() - text_dimensions.width) / 2.0;
        let text_y = GROUND_Y + 40.0;
        draw_text(mode_text, text_x, text_y, 30.0, BLUE);

        next_frame().await;
    }
}
