pub mod joint;
pub mod calculations;
pub mod drawing;

use macroquad::prelude::*;

/// Dimensions globales du Stickman
pub struct StickmanDimensions {
    pub longueur_cuisse: f32,
    pub longueur_jambe: f32,
    pub longueur_cou: f32,
    pub longueur_corps: f32,
    pub longueur_bras: f32,
    pub longueur_avant_bras: f32,
    pub rayon_tete: f32,
    pub origin: (f32, f32),
}

pub const DIMENSIONS: StickmanDimensions = StickmanDimensions {
    longueur_cuisse: 75.0,
    longueur_jambe: 75.0,
    longueur_cou: 25.0,
    longueur_corps: 140.0,
    longueur_bras: 60.0,
    longueur_avant_bras: 50.0,
    rayon_tete: 25.0,
    origin: (200.0, 545.0),
};

/// Résultat du calcul pour dessiner le Stickman
pub struct StickmanParts {
    pub pelvis: (f32, f32),
    pub corps_end: (f32, f32),
    pub cou_end: (f32, f32),
    pub tete_center: (f32, f32),
    pub bras_l_end: (f32, f32),
    pub avant_bras_l_end: (f32, f32),
    pub bras_r_end: (f32, f32),
    pub avant_bras_r_end: (f32, f32),
    pub cuisse_l_end: (f32, f32),
    pub jambe_l_end: (f32, f32),
    pub cuisse_r_end: (f32, f32),
    pub jambe_r_end: (f32, f32),
} // <-- Supprimer le point-virgule ici !

// Ré-export des items publics
pub use joint::{Joint, init_joints};
pub use calculations::calculate_stickman_positions;
pub use drawing::draw_stickman;
