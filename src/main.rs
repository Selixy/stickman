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
    const GRAVITY: f32 = 9.8;       // Accélération due à la gravité
    const GROUND_Y: f32 = 545.0;    // Niveau du sol

    let mut joints = init_joints(); // Initialisation des joints
    let mut y_velocity = 0.0;       // Vitesse verticale de la racine

    // Initialisation des articulations
    joints[0].angle_deg = 0.0;       // Racine
    joints[0].position = (150.0, 35.0);

    joints[1].angle_deg = 0.0;       // Articulation corps
    joints[2].angle_deg = 0.0;       // Articulation cou
    joints[3].angle_deg = 0.0;       // Articulation tête

    joints[4].angle_deg = 20.0;      // Épaule gauche
    joints[5].angle_deg = 0.0;       // Coude gauche
    joints[6].angle_deg = 0.0;       // Main gauche

    joints[7].angle_deg = -20.0;     // Épaule droite
    joints[8].angle_deg = 0.0;       // Coude droit
    joints[9].angle_deg = 0.0;       // Main droite

    joints[10].angle_deg = 20.0;     // Hanche gauche
    joints[11].angle_deg = 0.0;      // Genou gauche
    joints[12].angle_deg = 0.0;      // Pied gauche

    joints[13].angle_deg = -20.0;    // Hanche droite
    joints[14].angle_deg = 0.0;      // Genou droit
    joints[15].angle_deg = 0.0;      // Pied droit

    loop {
        clear_background(WHITE);
    
        // Dessin d'une ligne de sol
        draw_line(0.0, GROUND_Y, 800.0, GROUND_Y, 10.0, DARKGREEN);
    
        // Vérification des collisions
        let mut is_colliding = false;
        for joint in joints.iter_mut() {
            if joint.position.1 >= GROUND_Y - 13.0 {
                is_colliding = true;
                break; // Dès qu'une collision est détectée, pas besoin de continuer
            }
        }
    
        // Appliquer la gravité uniquement si pas de collision
        if is_colliding {
            y_velocity = 0.0; // Arrêter la gravité
        } else {
            y_velocity += GRAVITY * get_frame_time(); // Appliquer la gravité
        }
    
        // Appliquer la gravité sur la racine uniquement
        let idx_root = 0;
        joints[idx_root].position.1 += y_velocity;
    
        // Mise à jour des positions
        calculate_stickman_positions(&mut joints, &DIMENSIONS);
    
        // Dessin
        draw_stickman(&joints);
    
        next_frame().await;
    }
}
