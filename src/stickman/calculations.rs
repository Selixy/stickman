use std::f32::consts::PI;
use crate::stickman::{Joint, StickmanDimensions};

pub fn calculate_stickman_positions(joints: &mut [Joint], dims: &StickmanDimensions) {
    // Vérifier que le nombre de joints est correct
    assert_eq!(joints.len(), 16, "Le tableau de joints doit contenir 16 éléments.");

    // Clamp les angles (min, max) pour chaque joint
    for j in joints.iter_mut() {
        j.clamp_angle();
    }

    fn deg_to_rad(deg: f32) -> f32 {
        deg * PI / 180.0
    }

    fn calcule_position(x: f32, y: f32, angle_rad: f32, longueur: f32) -> (f32, f32) {
        (
            x + angle_rad.cos() * longueur,
            y + angle_rad.sin() * longueur
        )
    }

    // ---------------------------------------------------------------------
    // 0) Définition des index correspondant à chaque articulation
    // ---------------------------------------------------------------------
    let idx_root             = 0;
    let idx_corps            = 1;
    let idx_cou              = 2;
    let idx_tete             = 3;

    let idx_epaule_gauche    = 4;
    let idx_coude_gauche     = 5;
    let idx_main_gauche      = 6;

    let idx_epaule_droite    = 7;
    let idx_coude_droite     = 8;
    let idx_main_droite      = 9;

    let idx_hanche_gauche    = 10;
    let idx_genou_gauche     = 11;
    let idx_pied_gauche      = 12;

    let idx_hanche_droite    = 13;
    let idx_genou_droite     = 14;
    let idx_pied_droite      = 15;

    // ---------------------------------------------------------------------
    // 1) Extraire les angles de chaque articulation (en degrés)
    // ---------------------------------------------------------------------
    let angle_root_deg        = joints[idx_root].angle_deg; 

    let angle_corps_deg       = joints[idx_corps].angle_deg;
    let angle_cou_deg         = joints[idx_cou].angle_deg;
    let angle_tete_deg        = joints[idx_tete].angle_deg;

    let angle_epaule_g_deg    = joints[idx_epaule_gauche].angle_deg;
    let angle_coude_g_deg     = joints[idx_coude_gauche].angle_deg;

    let angle_epaule_d_deg    = joints[idx_epaule_droite].angle_deg;
    let angle_coude_d_deg     = joints[idx_coude_droite].angle_deg;

    let angle_hanche_g_deg    = joints[idx_hanche_gauche].angle_deg;
    let angle_genou_g_deg     = joints[idx_genou_gauche].angle_deg;

    let angle_hanche_d_deg    = joints[idx_hanche_droite].angle_deg;
    let angle_genou_d_deg     = joints[idx_genou_droite].angle_deg;

    // ---------------------------------------------------------------------
    // 2) Convertir en radians, en tenant compte des offsets
    //    (ex: -PI/2 pour le tronc vertical)
    // ---------------------------------------------------------------------
    let rad_root        = -PI / 2.0        + deg_to_rad(angle_root_deg);

    let rad_corps       = rad_root         + deg_to_rad(angle_corps_deg);
    let rad_cou         = rad_corps        + deg_to_rad(angle_cou_deg);
    let rad_tete        = rad_cou          + deg_to_rad(angle_tete_deg);

    let rad_epaule_g    = rad_corps - PI   + deg_to_rad(angle_epaule_g_deg);
    let rad_coude_g     = rad_epaule_g     + deg_to_rad(angle_coude_g_deg);

    let rad_epaule_d    = rad_corps - PI   + deg_to_rad(angle_epaule_d_deg);
    let rad_coude_d     = rad_epaule_d     + deg_to_rad(angle_coude_d_deg);

    let rad_hanche_g    = rad_root - PI    + deg_to_rad(angle_hanche_g_deg);
    let rad_genou_g     = rad_hanche_g     + deg_to_rad(angle_genou_g_deg);

    let rad_hanche_d    = rad_root - PI    + deg_to_rad(angle_hanche_d_deg);
    let rad_genou_d     = rad_hanche_d     + deg_to_rad(angle_genou_d_deg);

    // ---------------------------------------------------------------------
    // 3) Calcul de position, dans l'ordre de la parenté
    // ---------------------------------------------------------------------

    // (A) Root (point de base)
    joints[idx_root].position = (dims.origin.0, dims.origin.1 - 150.0);

    // (B) Corps → Cou → Tête
    joints[idx_corps].position = calcule_position(
        joints[idx_root].position.0,
        joints[idx_root].position.1,
        rad_corps,
        dims.longueur_corps
    );

    joints[idx_cou].position = calcule_position(
        joints[idx_corps].position.0,
        joints[idx_corps].position.1,
        rad_cou,
        dims.longueur_cou
    );

    joints[idx_tete].position = calcule_position(
        joints[idx_cou].position.0,
        joints[idx_cou].position.1,
        rad_tete,
        dims.rayon_tete
    );

    // (C) Corps → Épaule gauche → Coude gauche → Main gauche
    joints[idx_epaule_gauche].position = joints[idx_corps].position;

    joints[idx_coude_gauche].position = calcule_position(
        joints[idx_epaule_gauche].position.0,
        joints[idx_epaule_gauche].position.1,
        rad_epaule_g,
        dims.longueur_bras
    );

    joints[idx_main_gauche].position = calcule_position(
        joints[idx_coude_gauche].position.0,
        joints[idx_coude_gauche].position.1,
        rad_coude_g,
        dims.longueur_avant_bras
    );

    // (D) Corps → Épaule droite → Coude droit → Main droite
    joints[idx_epaule_droite].position = joints[idx_corps].position;

    joints[idx_coude_droite].position = calcule_position(
        joints[idx_epaule_droite].position.0,
        joints[idx_epaule_droite].position.1,
        rad_epaule_d,
        dims.longueur_bras
    );

    joints[idx_main_droite].position = calcule_position(
        joints[idx_coude_droite].position.0,
        joints[idx_coude_droite].position.1,
        rad_coude_d,
        dims.longueur_avant_bras
    );

    // (E) Root → Hanche gauche → Genou gauche → Pied gauche
    joints[idx_hanche_gauche].position = calcule_position(
        joints[idx_root].position.0,
        joints[idx_root].position.1,
        rad_hanche_g,
        dims.longueur_cuisse
    );

    joints[idx_genou_gauche].position = calcule_position(
        joints[idx_hanche_gauche].position.0,
        joints[idx_hanche_gauche].position.1,
        rad_genou_g,
        dims.longueur_jambe
    );

    joints[idx_pied_gauche].position = calcule_position(
        joints[idx_genou_gauche].position.0,
        joints[idx_genou_gauche].position.1,
        0.0, // Pas de rotation supplémentaire pour le pied gauche
        0.0 // Longueur nulle car vous ne voulez pas de ligne pour les pieds
    );

    // (F) Root → Hanche droite → Genou droit → Pied droit
    joints[idx_hanche_droite].position = calcule_position(
        joints[idx_root].position.0,
        joints[idx_root].position.1,
        rad_hanche_d,
        dims.longueur_cuisse
    );

    joints[idx_genou_droite].position = calcule_position(
        joints[idx_hanche_droite].position.0,
        joints[idx_hanche_droite].position.1,
        rad_genou_d,
        dims.longueur_jambe
    );

    joints[idx_pied_droite].position = calcule_position(
        joints[idx_genou_droite].position.0,
        joints[idx_genou_droite].position.1,
        0.0, // Pas de rotation supplémentaire pour le pied droit
        0.0
    );

}
