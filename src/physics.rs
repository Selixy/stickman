use crate::stickman::{calculate_stickman_positions, init_joints, DIMENSIONS, Joint};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

const GRAVITY: f32 = 9.8;
const GROUND_Y: f32 = 545.0;

pub struct Physics {
    pub joints: Arc<Mutex<Vec<Joint>>>,
    pub keyframes: Vec<Vec<f32>>, // Angles aléatoires pour les mouvements
    pub tick_rate: Arc<Mutex<f32>>,
    pub current_frame: usize,
}

impl Physics {
    /// Crée une nouvelle instance de `Physics` avec une position initiale
    pub fn new_with_position(initial_position: (f32, f32), tick_rate: f32) -> Self {
        let mut joints = init_joints();
        joints[0].position = initial_position;

        Self {
            joints: Arc::new(Mutex::new(joints)),
            keyframes: vec![],
            tick_rate: Arc::new(Mutex::new(tick_rate)),
            current_frame: 0,
        }
    }

    /// Génère des keyframes aléatoires pour les angles des joints
    pub fn generate_random_keyframes(&mut self) {
        let mut rng = rand::thread_rng();
        self.keyframes = (0..60) // 60 frames de mouvements
            .map(|_| {
                (0..16) // Nombre total de joints
                    .map(|_| rng.gen_range(-90.0..90.0)) // Angles aléatoires entre -90° et 90°
                    .collect()
            })
            .collect();
    }

    /// Démarre la simulation de physique dans un thread séparé
    pub fn start(&mut self) {
        let joints = Arc::clone(&self.joints);
        let keyframes = self.keyframes.clone();
        let tick_rate = Arc::clone(&self.tick_rate);

        thread::spawn(move || {
            let mut y_velocity = 0.0;
            let mut previous_time = Instant::now();
            let mut frame_index = 0;

            loop {
                let current_time = Instant::now();
                let delta_time = (current_time - previous_time).as_secs_f32();
                previous_time = current_time;

                // Appliquer les keyframes
                {
                    let mut joints = joints.lock().unwrap();
                    if let Some(frame) = keyframes.get(frame_index) {
                        for (i, &angle) in frame.iter().enumerate() {
                            joints[i].angle_deg = angle;
                        }
                        frame_index = (frame_index + 1) % keyframes.len();
                    }

                    for joint in joints.iter_mut() {
                        joint.is_colliding = joint.position.1 >= GROUND_Y - 11.5;
                    }

                    if joints.iter().any(|joint| joint.is_colliding) {
                        y_velocity = 0.0;
                    } else {
                        y_velocity += GRAVITY * delta_time;
                    }

                    joints[0].position.1 += y_velocity;

                    calculate_stickman_positions(&mut joints, &DIMENSIONS);
                }

                let current_tick_rate = *tick_rate.lock().unwrap();
                thread::sleep(Duration::from_secs_f32(current_tick_rate));
            }
        });
    }

    /// Récupère les joints pour affichage
    pub fn get_joints(&self) -> Arc<Mutex<Vec<Joint>>> {
        Arc::clone(&self.joints)
    }
}
