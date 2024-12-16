mod window_conf;
mod stickman;

use macroquad::prelude::*;
use crate::window_conf::window_conf;

// On importe la logique du stickman
use crate::stickman::{
    init_joints,
    calculate_stickman_positions,
    draw_stickman,
    DIMENSIONS,
};

#[macroquad::main(window_conf)]
async fn main() {
    let mut joints = init_joints(); // plus de map, juste un Vec<Joint>

    loop {
        clear_background(WHITE);

        draw_line(0.0, 550.0, 800.0, 550.0, 10.0, DARKGREEN);

        // Mettons à jour les angles en fonction de l'index
        // 0: bassin_gauche
        joints[0].angle_deg = -40.0;
        // 1: genou_gauche
        joints[1].angle_deg = 0.0;
        // 2: bassin_droit
        joints[2].angle_deg = 40.0;
        // 3: genou_droit
        joints[3].angle_deg = 0.0;
        // 4: epaule_gauche
        joints[4].angle_deg = -40.0;
        // 5: coude_gauche
        joints[5].angle_deg = 0.0;
        // 6: epaule_droite
        joints[6].angle_deg = 40.0;
        // 7: coude_droite
        joints[7].angle_deg = 0.0;

        let stickman_positions = calculate_stickman_positions(&mut joints, &DIMENSIONS);

        draw_stickman(stickman_positions);

        next_frame().await;
    }
}
