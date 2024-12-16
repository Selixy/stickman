use macroquad::prelude::*;
use crate::stickman::Joint;

pub fn draw_stickman(joints: &[Joint]) {
    // Indices
    let idx_root = 0;
    let idx_corps = 1;
    let idx_cou = 2;
    let idx_tete = 3;

    let idx_epaule_gauche = 4;
    let idx_coude_gauche = 5;
    let idx_main_gauche = 6;

    let idx_epaule_droite = 7;
    let idx_coude_droite = 8;
    let idx_main_droite = 9;

    let idx_hanche_gauche = 10;
    let idx_genou_gauche = 11;
    let idx_pied_gauche = 12;

    let idx_hanche_droite = 13;
    let idx_genou_droite = 14;
    let idx_pied_droite = 15;

    // Corps
    draw_line(
        joints[idx_root].position.0,
        joints[idx_root].position.1,
        joints[idx_corps].position.0,
        joints[idx_corps].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_corps].position.0,
        joints[idx_corps].position.1,
        joints[idx_cou].position.0,
        joints[idx_cou].position.1,
        3.0,
        BLACK,
    );

    // Tête : cercle
    let r_tete = 25.0; // Taille de la tête
    draw_circle_lines(
        joints[idx_tete].position.0,
        joints[idx_tete].position.1,
        r_tete,
        3.0,
        BLACK,
    );

    // Bras gauche
    draw_line(
        joints[idx_corps].position.0,
        joints[idx_corps].position.1,
        joints[idx_epaule_gauche].position.0,
        joints[idx_epaule_gauche].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_epaule_gauche].position.0,
        joints[idx_epaule_gauche].position.1,
        joints[idx_coude_gauche].position.0,
        joints[idx_coude_gauche].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_coude_gauche].position.0,
        joints[idx_coude_gauche].position.1,
        joints[idx_main_gauche].position.0,
        joints[idx_main_gauche].position.1,
        3.0,
        BLACK,
    );

    // Bras droit
    draw_line(
        joints[idx_corps].position.0,
        joints[idx_corps].position.1,
        joints[idx_epaule_droite].position.0,
        joints[idx_epaule_droite].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_epaule_droite].position.0,
        joints[idx_epaule_droite].position.1,
        joints[idx_coude_droite].position.0,
        joints[idx_coude_droite].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_coude_droite].position.0,
        joints[idx_coude_droite].position.1,
        joints[idx_main_droite].position.0,
        joints[idx_main_droite].position.1,
        3.0,
        BLACK,
    );

    // Jambe gauche
    draw_line(
        joints[idx_root].position.0,
        joints[idx_root].position.1,
        joints[idx_hanche_gauche].position.0,
        joints[idx_hanche_gauche].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_hanche_gauche].position.0,
        joints[idx_hanche_gauche].position.1,
        joints[idx_genou_gauche].position.0,
        joints[idx_genou_gauche].position.1,
        3.0,
        BLACK,
    );
    // Pas de ligne pour le pied gauche

    // Jambe droite
    draw_line(
        joints[idx_root].position.0,
        joints[idx_root].position.1,
        joints[idx_hanche_droite].position.0,
        joints[idx_hanche_droite].position.1,
        3.0,
        BLACK,
    );
    draw_line(
        joints[idx_hanche_droite].position.0,
        joints[idx_hanche_droite].position.1,
        joints[idx_genou_droite].position.0,
        joints[idx_genou_droite].position.1,
        3.0,
        BLACK,
    );
    // Pas de ligne pour le pied droit

    // Cercles sur chaque articulation
    let r_articulation = 4.0;

    for (i, joint) in joints.iter().enumerate() {
        // Cou (index 2) ou tête (index 3) => on ne dessine pas le cercle
        if i == idx_cou || i == idx_tete {
            continue;
        }
        // Mains & pieds => vert (index 6, 9, 12, 15)
        else if i == idx_main_gauche
            || i == idx_main_droite
            || i == idx_pied_gauche
            || i == idx_pied_droite
        {
            draw_circle(joint.position.0, joint.position.1, r_articulation, GREEN);
        }
        // Tout le reste => rouge
        else {
            draw_circle(joint.position.0, joint.position.1, r_articulation, RED);
        }
    }
}
