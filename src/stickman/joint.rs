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

/// Initialise un vecteur de joints + une map "nom -> index"
pub fn init_joints() -> Vec<Joint> {
    let mut joints = Vec::new();

    // Ordre imposé des articulations
    // 0: bassin_gauche
    joints.push(Joint::new(0.0, -90.0, 90.0));
    // 1: genou_gauche
    joints.push(Joint::new(0.0, 0.0, 130.0));
    // 2: bassin_droit
    joints.push(Joint::new(0.0, -90.0, 90.0));
    // 3: genou_droit
    joints.push(Joint::new(0.0, 0.0, 130.0));

    // 4: epaule_gauche
    joints.push(Joint::new(0.0, -90.0, 90.0));
    // 5: coude_gauche
    joints.push(Joint::new(0.0, 0.0, 130.0));
    // 6: epaule_droite
    joints.push(Joint::new(0.0, -90.0, 90.0));
    // 7: coude_droite
    joints.push(Joint::new(0.0, 0.0, 130.0));

    joints
}

