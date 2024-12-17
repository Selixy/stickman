#[derive(Debug, Clone, Copy)]
pub struct Joint {
    pub angle_deg: f32,
    pub angle_min_deg: f32,
    pub angle_max_deg: f32,
    pub position: (f32, f32),
    pub is_colliding: bool, // Indique si le joint touche une surface (par exemple le sol)
}

impl Joint {
    /// Crée un nouveau joint avec des angles spécifiés.
    pub fn new(angle_deg: f32, angle_min_deg: f32, angle_max_deg: f32) -> Self {
        Self {
            angle_deg,
            angle_min_deg,
            angle_max_deg,
            position: (0.0, 0.0),
            is_colliding: false,
        }
    }

    /// Clamp l'angle entre angle_min_deg et angle_max_deg
    pub fn clamp_angle(&mut self) {
        if self.angle_deg < self.angle_min_deg {
            self.angle_deg = self.angle_min_deg;
        } else if self.angle_deg > self.angle_max_deg {
            self.angle_deg = self.angle_max_deg;
        }
    }
}

/// Initialise les joints avec leurs angles min, max, et valeurs initiales.
pub fn init_joints() -> Vec<Joint> {
    vec![
        Joint::new(0.0, -180.0, 180.0), // 0 - root
        Joint::new(0.0, -180.0, 180.0), // 1 - articulation corps
        Joint::new(0.0, -180.0, 180.0), // 2 - articulation cou
        Joint::new(0.0, -180.0, 180.0), // 3 - articulation tête

        Joint::new(0.0, -180.0, 180.0), // 4 - articulation épaule gauche
        Joint::new(0.0, -180.0, 180.0), // 5 - articulation coude gauche
        Joint::new(0.0, -180.0, 180.0), // 6 - articulation main gauche

        Joint::new(0.0, -180.0, 180.0), // 7 - articulation épaule droite
        Joint::new(0.0, -180.0, 180.0), // 8 - articulation coude droite
        Joint::new(0.0, -180.0, 180.0), // 9 - articulation main droite

        Joint::new(0.0, -180.0, 180.0), // 10 - articulation hanche gauche
        Joint::new(0.0, -180.0, 180.0), // 11 - articulation genou gauche
        Joint::new(0.0, -180.0, 180.0), // 12 - articulation pied gauche

        Joint::new(0.0, -180.0, 180.0), // 13 - articulation hanche droite
        Joint::new(0.0, -180.0, 180.0), // 14 - articulation genou droite
        Joint::new(0.0, -180.0, 180.0), // 15 - articulation pied droite
    ]
}
