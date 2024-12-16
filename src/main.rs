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


    // 0 root
    joints[0].angle_deg =    0.0;

    // 1 articulation corps
    joints[1].angle_deg =    0.0;

    // 2 articulation cou
    joints[2].angle_deg =    0.0;

    // 3 articulation tête
    joints[3].angle_deg =    0.0;

    // 4 articulation épaule gauche
    joints[4].angle_deg =   20.0;

    // 5 articulation coude gauche
    joints[5].angle_deg =    0.0;

    // 6 articulation main gauche
    joints[6].angle_deg =    0.0;

    // 7 articulation épaule droite
    joints[7].angle_deg =  -20.0;

    // 8 articulation coude droite
    joints[8].angle_deg =    0.0;

    // 9 articulation main droite
    joints[9].angle_deg =    0.0;

    // 10 articulation hanche gauche
    joints[10].angle_deg =   20.0;

    // 11 articulation genou gauche
    joints[11].angle_deg =    0.0;

    // 12 articulation pied gauche
    joints[12].angle_deg =    0.0;

    // 13 articulation hanche droite
    joints[13].angle_deg =  -20.0;

    // 14 articulation genou droite
    joints[14].angle_deg =    0.0;

    // 15 articulation pied droite
    joints[15].angle_deg =    0.0;


        // Mise à jour des positions
        calculate_stickman_positions(&mut joints, &DIMENSIONS);

        // Dessin
        draw_stickman(&joints);

        next_frame().await;
    }
}
