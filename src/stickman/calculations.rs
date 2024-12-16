use std::f32::consts::PI;
use crate::stickman::{Joint, StickmanDimensions};

pub fn calculate_stickman_positions(joints: &mut [Joint], dims: &StickmanDimensions) {
    // Clamp les angles (min, max) pour chaque joint
    for j in joints.iter_mut() {
        j.clamp_angle();
    }

    fn deg_to_rad(deg: f32) -> f32 {
        deg * PI / 180.0
    }

    fn calcule_position(x: f32, y: f32, angle_rad: f32, longueur: f32) -> (f32, f32) {
        (x + angle_rad.cos() * longueur, y + angle_rad.sin() * longueur)
    }

    // Indices (conformes à init_joints)
    let idx_pelvis          = 0;
    let idx_corps_end       = 1;
    let idx_cou_end         = 2;
    let idx_tete_center     = 3;

    let idx_bras_l_end      = 4;
    let idx_avant_bras_l_end= 5;
    let idx_bras_r_end      = 6;
    let idx_avant_bras_r_end= 7;

    let idx_cuisse_l_end    = 8;
    let idx_jambe_l_end     = 9;
    let idx_cuisse_r_end    = 10;
    let idx_jambe_r_end     = 11;

    // -----------------------------
    // 1) Extraire tous les angles en degrés
    // -----------------------------
    let angle_corps_deg       = joints[idx_corps_end].angle_deg;
    let angle_cou_deg         = joints[idx_cou_end].angle_deg;
    let angle_tete_deg        = joints[idx_tete_center].angle_deg;

    let angle_bras_l_deg      = joints[idx_bras_l_end].angle_deg;
    let angle_avant_bras_l_deg= joints[idx_avant_bras_l_end].angle_deg;

    let angle_bras_r_deg      = joints[idx_bras_r_end].angle_deg;
    let angle_avant_bras_r_deg= joints[idx_avant_bras_r_end].angle_deg;

    let angle_cuisse_l_deg    = joints[idx_cuisse_l_end].angle_deg;
    let angle_jambe_l_deg     = joints[idx_jambe_l_end].angle_deg;

    let angle_cuisse_r_deg    = joints[idx_cuisse_r_end].angle_deg;
    let angle_jambe_r_deg     = joints[idx_jambe_r_end].angle_deg;

    // -----------------------------
    // 2) Convertir en radians au même endroit
    // (avec offsets si nécessaire, ex: -PI/2.0 pour le tronc vertical)
    // -----------------------------
    let rad_corps       = -PI / 2.0    + deg_to_rad(angle_corps_deg);
    let rad_cou         = -PI / 2.0    + deg_to_rad(angle_cou_deg);
    let rad_tete        = -PI / 2.0    + deg_to_rad(angle_tete_deg);

    let rad_bras_l      = PI / 2.0     + deg_to_rad(angle_bras_l_deg);
    let rad_avant_bras_l= rad_bras_l   + deg_to_rad(angle_avant_bras_l_deg);

    let rad_bras_r      = PI / 2.0     + deg_to_rad(angle_bras_r_deg);
    let rad_avant_bras_r= rad_bras_r   + deg_to_rad(angle_avant_bras_r_deg);

    let rad_cuisse_l    = PI / 2.0     + deg_to_rad(angle_cuisse_l_deg);
    let rad_jambe_l     = rad_cuisse_l + deg_to_rad(angle_jambe_l_deg);

    let rad_cuisse_r    = PI / 2.0     + deg_to_rad(angle_cuisse_r_deg);
    let rad_jambe_r     = rad_cuisse_r + deg_to_rad(angle_jambe_r_deg);

    // -----------------------------
    // 3) Placer les articulations
    // -----------------------------

    // Pelvis (point de base)
    joints[idx_pelvis].position = (dims.origin.0, dims.origin.1 - 150.0);

    // Corps
    joints[idx_corps_end].position = calcule_position(
        joints[idx_pelvis].position.0,
        joints[idx_pelvis].position.1,
        rad_corps,
        dims.longueur_corps,
    );

    // Cou
    joints[idx_cou_end].position = calcule_position(
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        rad_cou,
        dims.longueur_cou
    );

    // Tête
    joints[idx_tete_center].position = calcule_position(
        joints[idx_cou_end].position.0,
        joints[idx_cou_end].position.1,
        rad_tete,
        dims.rayon_tete
    );

    // Bras gauche
    joints[idx_bras_l_end].position = calcule_position(
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        rad_bras_l,
        dims.longueur_bras
    );

    joints[idx_avant_bras_l_end].position = calcule_position(
        joints[idx_bras_l_end].position.0,
        joints[idx_bras_l_end].position.1,
        rad_avant_bras_l,
        dims.longueur_avant_bras
    );

    // Bras droit
    joints[idx_bras_r_end].position = calcule_position(
        joints[idx_corps_end].position.0,
        joints[idx_corps_end].position.1,
        rad_bras_r,
        dims.longueur_bras
    );

    joints[idx_avant_bras_r_end].position = calcule_position(
        joints[idx_bras_r_end].position.0,
        joints[idx_bras_r_end].position.1,
        rad_avant_bras_r,
        dims.longueur_avant_bras
    );

    // Cuisse gauche
    joints[idx_cuisse_l_end].position = calcule_position(
        joints[idx_pelvis].position.0,
        joints[idx_pelvis].position.1,
        rad_cuisse_l,
        dims.longueur_cuisse
    );

    joints[idx_jambe_l_end].position = calcule_position(
        joints[idx_cuisse_l_end].position.0,
        joints[idx_cuisse_l_end].position.1,
        rad_jambe_l,
        dims.longueur_jambe
    );

    // Cuisse droite
    joints[idx_cuisse_r_end].position = calcule_position(
        joints[idx_pelvis].position.0,
        joints[idx_pelvis].position.1,
        rad_cuisse_r,
        dims.longueur_cuisse
    );

    joints[idx_jambe_r_end].position = calcule_position(
        joints[idx_cuisse_r_end].position.0,
        joints[idx_cuisse_r_end].position.1,
        rad_jambe_r,
        dims.longueur_jambe
    );
}
