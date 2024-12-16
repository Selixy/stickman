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

/// On crée un tableau de 12 articulations correspondant exactement
/// à l'ancien StickmanParts :
/// 0: pelvis
/// 1: corps_end (haut du tronc)
/// 2: cou_end
/// 3: tete_center
/// 4: bras_l_end
/// 5: avant_bras_l_end
/// 6: bras_r_end
/// 7: avant_bras_r_end
/// 8: cuisse_l_end
/// 9: jambe_l_end
/// 10: cuisse_r_end
/// 11: jambe_r_end
///
/// Chacun stocke un angle, min, max (à adapter).
pub fn init_joints() -> Vec<Joint> {
    let mut joints = Vec::new();
    joints.reserve(12);

    // pelvis
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 0
    // corps_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 1
    // cou_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 2
    // tete_center
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 3

    // bras_l_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 4
    // avant_bras_l_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 5

    // bras_r_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 6
    // avant_bras_r_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 7

    // cuisse_l_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 8
    // jambe_l_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 9

    // cuisse_r_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 10
    // jambe_r_end
    joints.push(Joint::new(0.0, -180.0, 180.0)); // 11

    joints
}
