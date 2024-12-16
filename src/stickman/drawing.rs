use macroquad::prelude::*;
use crate::stickman::Joint;

pub fn draw_stickman(joints: &[Joint]) {
    // Indices
    let idx_pelvis = 0;
    let idx_corps_end = 1;
    let idx_cou_end = 2;
    let idx_tete_center = 3;

    let idx_bras_l_end = 4;
    let idx_avant_bras_l_end = 5;
    let idx_bras_r_end = 6;
    let idx_avant_bras_r_end = 7;

    let idx_cuisse_l_end = 8;
    let idx_jambe_l_end = 9;
    let idx_cuisse_r_end = 10;
    let idx_jambe_r_end = 11;

    // Corps & cou
    draw_line(
        joints[idx_pelvis].position.0, 
        joints[idx_pelvis].position.1,
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        3.0,
        BLACK
    );
    draw_line(
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        joints[idx_cou_end].position.0,
        joints[idx_cou_end].position.1,
        3.0,
        BLACK
    );

    // Tête : cercle
    let r_tete = 25.0;  // ou si vous voulez, vous pouvez lire depuis DIMENSIONS
    draw_circle_lines(
        joints[idx_tete_center].position.0,
        joints[idx_tete_center].position.1,
        r_tete,
        3.0,
        BLACK
    );

    // Bras gauche
    draw_line(
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        joints[idx_bras_l_end].position.0,
        joints[idx_bras_l_end].position.1,
        3.0,
        BLACK
    );
    draw_line(
        joints[idx_bras_l_end].position.0,
        joints[idx_bras_l_end].position.1,
        joints[idx_avant_bras_l_end].position.0,
        joints[idx_avant_bras_l_end].position.1,
        3.0,
        BLACK
    );

    // Bras droit
    draw_line(
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        joints[idx_bras_r_end].position.0,
        joints[idx_bras_r_end].position.1,
        3.0,
        BLACK
    );
    draw_line(
        joints[idx_bras_r_end].position.0,
        joints[idx_bras_r_end].position.1,
        joints[idx_avant_bras_r_end].position.0,
        joints[idx_avant_bras_r_end].position.1,
        3.0,
        BLACK
    );

    // Jambe gauche
    draw_line(
        joints[idx_pelvis].position.0,
        joints[idx_pelvis].position.1,
        joints[idx_cuisse_l_end].position.0,
        joints[idx_cuisse_l_end].position.1,
        3.0,
        BLACK
    );
    draw_line(
        joints[idx_cuisse_l_end].position.0,
        joints[idx_cuisse_l_end].position.1,
        joints[idx_jambe_l_end].position.0,
        joints[idx_jambe_l_end].position.1,
        3.0,
        BLACK
    );

    // Jambe droite
    draw_line(
        joints[idx_pelvis].position.0,
        joints[idx_pelvis].position.1,
        joints[idx_cuisse_r_end].position.0,
        joints[idx_cuisse_r_end].position.1,
        3.0,
        BLACK
    );
    draw_line(
        joints[idx_cuisse_r_end].position.0,
        joints[idx_cuisse_r_end].position.1,
        joints[idx_jambe_r_end].position.0,
        joints[idx_jambe_r_end].position.1,
        3.0,
        BLACK
    );

    // Cercles rouges sur chaque articulation
    let r_articulation = 4.0;

    for (i, joint) in joints.iter().enumerate() {
        // Cou (index 2) ou tête (index 3) => on ne dessine pas le cercle
        if i == 2 || i == 3 {
            continue; // Skip drawing
        }
        // Mains & pieds => vert (index 5, 7, 9, 11)
        else if i == 5 || i == 7 || i == 9 || i == 11 {
            draw_circle(joint.position.0, joint.position.1, r_articulation, GREEN);
        }
        // Tout le reste => rouge
        else {
            draw_circle(joint.position.0, joint.position.1, r_articulation, RED);
        }
    }
}
