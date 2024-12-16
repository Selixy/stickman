#[derive(Debug, Clone, Copy)]
pub struct Joint {
    pub angle_deg: f32,
    pub angle_min_deg: f32,
    pub angle_max_deg: f32,
    pub position: (f32, f32),
}

impl Joint {
    pub fn new(angle_deg: f32, angle_min_deg: f32, angle_max_deg: f32) -> Self {
        Self {
            angle_deg,
            angle_min_deg,
            angle_max_deg,
            position: (0.0, 0.0),
        }
    }

    pub fn clamp_angle(&mut self) {
        if self.angle_deg < self.angle_min_deg {
            self.angle_deg = self.angle_min_deg;
        } else if self.angle_deg > self.angle_max_deg {
            self.angle_deg = self.angle_max_deg;
        }
    }
}

/// Chacun stocke un angle, min, max (à adapter).
pub fn init_joints() -> Vec<Joint> {
    let mut joints = Vec::new();
    joints.reserve(16); // Réserve l'espace pour 16 joints

    // Initialisation des joints avec angle_deg, angle_min_deg, angle_max_deg
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 0 - root
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 1 - corps
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 2 - cou
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 3 - tete

    joints.push(Joint::new(0.0, -180.0, 180.0)); // 4 - epaule_gauche
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 5 - coude_gauche
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 6 - main_gauche

    joints.push(Joint::new(0.0, -180.0, 180.0)); // 7 - epaule_droite
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 8 - coude_droite
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 9 - main_droite

    joints.push(Joint::new(0.0, -180.0, 180.0)); // 10 - hanche_gauche
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 11 - genou_gauche
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 12 - pied_gauche

    joints.push(Joint::new(0.0, -180.0, 180.0)); // 13 - hanche_droite
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 14 - genou_droite
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 15 - pied_droite

    joints
}
