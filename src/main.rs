mod window_conf;
mod stickman;

use macroquad::prelude::*;
use crate::window_conf::window_conf;

use crate::stickman::{
    init_joints,
    calculate_stickman_positions,
    draw_stickman,
    DIMENSIONS,
};

#[macroquad::main(window_conf)]
async fn main() {
    let mut joints = init_joints();

    loop {
        clear_background(WHITE);

        // Dessin d'une ligne de sol
        draw_line(0.0, 550.0, 800.0, 550.0, 10.0, DARKGREEN);


    // 1 articulation root
    joints[1].angle_deg = 0.0;
    // 2 articulation cou
    joints[2].angle_deg = 0.0;

    // 4 articulation épaule gauche
    joints[4].angle_deg = 0.0;
    // 5 articulation coude gauche
    joints[5].angle_deg = 0.0;

    // 6 articulation épaule droite
    joints[6].angle_deg = 0.0;
    // 7 articulation coude droite
    joints[7].angle_deg = 0.0;

    // 8 articulation hanche gauche
    joints[8].angle_deg = 0.0;
    // 9 articulation genou gauche
    joints[9].angle_deg = 0.0;

    // 10 articulation hanche droite
    joints[10].angle_deg = 0.0;
    // 11 articulation genou droite
    joints[11].angle_deg = 0.0;


        // Mise à jour des positions
        calculate_stickman_positions(&mut joints, &DIMENSIONS);

        // Dessin
        draw_stickman(&joints);

        next_frame().await;
    }
}
