use crate::stickman::{StickmanDimensions, StickmanParts, Joint};
use std::f32::consts::PI;

pub fn calculate_stickman_positions(
    joints: &mut [Joint],
    dimensions: &StickmanDimensions,
) -> StickmanParts {
    // On clamp chaque articulation avant calcul
    for j in joints.iter_mut() {
        j.clamp_angle();
    }

    fn deg_to_rad(deg: f32) -> f32 {
        deg * PI / 180.0
    }

    // Indices fixes, ordre imposé :
    // 0: bassin_gauche, 1: genou_gauche, 2: bassin_droit, 3: genou_droit,
    // 4: epaule_gauche, 5: coude_gauche, 6: epaule_droite, 7: coude_droite

    let bassin_gauche_rad  = deg_to_rad(90.0 + joints[0].angle_deg);
    let genou_gauche_rad   = deg_to_rad(joints[1].angle_deg) + bassin_gauche_rad;
    let bassin_droit_rad   = deg_to_rad(90.0 + joints[2].angle_deg);
    let genou_droit_rad    = deg_to_rad(joints[3].angle_deg) + bassin_droit_rad;

    let epaule_gauche_rad  = deg_to_rad(90.0 + joints[4].angle_deg);
    let coude_gauche_rad   = deg_to_rad(joints[5].angle_deg) + epaule_gauche_rad;
    let epaule_droite_rad  = deg_to_rad(90.0 + joints[6].angle_deg);
    let coude_droite_rad   = deg_to_rad(joints[7].angle_deg) + epaule_droite_rad;

    fn calcule_position(x: f32, y: f32, angle_rad: f32, longueur: f32) -> (f32, f32) {
        (x + angle_rad.cos() * longueur, y + angle_rad.sin() * longueur)
    }

    let pelvis = (dimensions.origin.0, dimensions.origin.1 - 150.0);
    let corps_end   = calcule_position(pelvis.0, pelvis.1, -PI / 2.0, dimensions.longueur_corps);
    let cou_end     = calcule_position(corps_end.0, corps_end.1, -PI / 2.0, dimensions.longueur_cou);
    let tete_center = calcule_position(cou_end.0, cou_end.1, -PI / 2.0, dimensions.rayon_tete);

    let bras_l_end = calcule_position(corps_end.0, corps_end.1, epaule_gauche_rad, dimensions.longueur_bras);
    let avant_bras_l_end = calcule_position(bras_l_end.0, bras_l_end.1, coude_gauche_rad, dimensions.longueur_avant_bras);

    let bras_r_end = calcule_position(corps_end.0, corps_end.1, epaule_droite_rad, dimensions.longueur_bras);
    let avant_bras_r_end = calcule_position(bras_r_end.0, bras_r_end.1, coude_droite_rad, dimensions.longueur_avant_bras);

    let cuisse_l_end = calcule_position(pelvis.0, pelvis.1, bassin_gauche_rad, dimensions.longueur_cuisse);
    let jambe_l_end  = calcule_position(cuisse_l_end.0, cuisse_l_end.1, genou_gauche_rad, dimensions.longueur_jambe);

    let cuisse_r_end = calcule_position(pelvis.0, pelvis.1, bassin_droit_rad, dimensions.longueur_cuisse);
    let jambe_r_end  = calcule_position(cuisse_r_end.0, cuisse_r_end.1, genou_droit_rad, dimensions.longueur_jambe);

    // (Optionnel) met à jour la position dans joints[x].position = ...
    joints[0].position = cuisse_l_end;
    joints[1].position = jambe_l_end;
    joints[2].position = cuisse_r_end;
    joints[3].position = jambe_r_end;
    joints[4].position = bras_l_end;
    joints[5].position = avant_bras_l_end;
    joints[6].position = bras_r_end;
    joints[7].position = avant_bras_r_end;

    StickmanParts {
        pelvis,
        corps_end,
        cou_end,
        tete_center,
        bras_l_end,
        avant_bras_l_end,
        bras_r_end,
        avant_bras_r_end,
        cuisse_l_end,
        jambe_l_end,
        cuisse_r_end,
        jambe_r_end,
    }
}
