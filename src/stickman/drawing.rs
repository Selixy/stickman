use macroquad::prelude::*;
use crate::stickman::{StickmanParts, DIMENSIONS};

pub fn draw_stickman(stickman: StickmanParts) {
    // Corps & cou
    draw_line(
        stickman.pelvis.0, 
        stickman.pelvis.1,
        stickman.corps_end.0,
        stickman.corps_end.1,
        3.0,
        BLACK,
    );
    draw_line(
        stickman.corps_end.0,
        stickman.corps_end.1,
        stickman.cou_end.0,
        stickman.cou_end.1,
        3.0,
        BLACK,
    );

    // Tête
    draw_circle_lines(
        stickman.tete_center.0,
        stickman.tete_center.1,
        DIMENSIONS.rayon_tete,
        3.0,
        BLACK,
    );

    // Bras gauche
    draw_line(
        stickman.corps_end.0,
        stickman.corps_end.1,
        stickman.bras_l_end.0,
        stickman.bras_l_end.1,
        3.0,
        BLACK,
    );
    draw_line(
        stickman.bras_l_end.0,
        stickman.bras_l_end.1,
        stickman.avant_bras_l_end.0,
        stickman.avant_bras_l_end.1,
        3.0,
        BLACK,
    );

    // Bras droit
    draw_line(
        stickman.corps_end.0,
        stickman.corps_end.1,
        stickman.bras_r_end.0,
        stickman.bras_r_end.1,
        3.0,
        BLACK,
    );
    draw_line(
        stickman.bras_r_end.0,
        stickman.bras_r_end.1,
        stickman.avant_bras_r_end.0,
        stickman.avant_bras_r_end.1,
        3.0,
        BLACK,
    );

    // Jambes gauche
    draw_line(
        stickman.pelvis.0,
        stickman.pelvis.1,
        stickman.cuisse_l_end.0,
        stickman.cuisse_l_end.1,
        3.0,
        BLACK,
    );
    draw_line(
        stickman.cuisse_l_end.0,
        stickman.cuisse_l_end.1,
        stickman.jambe_l_end.0,
        stickman.jambe_l_end.1,
        3.0,
        BLACK,
    );

    // Jambes droite
    draw_line(
        stickman.pelvis.0,
        stickman.pelvis.1,
        stickman.cuisse_r_end.0,
        stickman.cuisse_r_end.1,
        3.0,
        BLACK,
    );
    draw_line(
        stickman.cuisse_r_end.0,
        stickman.cuisse_r_end.1,
        stickman.jambe_r_end.0,
        stickman.jambe_r_end.1,
        3.0,
        BLACK,
    );

    // Petits cercles rouges pour marquer les articulations
    let r_articulation = 4.0;
    draw_circle(stickman.pelvis.0, stickman.pelvis.1, r_articulation, RED);
    draw_circle(stickman.corps_end.0, stickman.corps_end.1, r_articulation, RED);
    draw_circle(stickman.bras_l_end.0, stickman.bras_l_end.1, r_articulation, RED);
    draw_circle(stickman.bras_r_end.0, stickman.bras_r_end.1, r_articulation, RED);
    draw_circle(stickman.cuisse_l_end.0, stickman.cuisse_l_end.1, r_articulation, RED);
    draw_circle(stickman.cuisse_r_end.0, stickman.cuisse_r_end.1, r_articulation, RED);
}
