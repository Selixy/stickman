use crate::stickman::{calculate_stickman_positions, init_joints, DIMENSIONS, Joint};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng}; // Pour créer un StdRng avec une seed

const GRAVITY: f32 = 9.8;
const GROUND_Y: f32 = 545.0;

pub struct Physics {
    pub joints: Arc<Mutex<Vec<Joint>>>,
    pub tick_rate: Arc<Mutex<f32>>,
    previous_rotations: Vec<f32>, // Rotations précédentes
}

impl Physics {
    /// Crée une nouvelle instance de `Physics` avec une position initiale
    pub fn new_with_position(initial_position: (f32, f32), tick_rate: f32) -> Self {
        let mut joints = init_joints();
        joints[0].position = initial_position;

        Self {
            joints: Arc::new(Mutex::new(joints)),
            tick_rate: Arc::new(Mutex::new(tick_rate)),
            previous_rotations: vec![0.0; 10], // Initialise les rotations précédentes à 0
        }
    }

    /// Génère une nouvelle liste de rotations aléatoires pour les articulations importantes
    pub fn generate_random_rotations(rng: &mut StdRng) -> Vec<f32> {
        (0..10) // Nombre d'articulations importantes
            .map(|_| rng.gen_range(-45.0..45.0)) // Angle entre -45° et 45°
            .collect()
    }

    /// Interpole entre deux ensembles de rotations
    pub fn interpolate_rotations(from: &[f32], to: &[f32], t: f32) -> Vec<f32> {
        from.iter()
            .zip(to.iter())
            .map(|(&a, &b)| a + t * (b - a)) // Lerp : a + t * (b - a)
            .collect()
    }

    /// Démarre la simulation de physique dans un thread séparé
    pub fn start(&mut self) {
        let joints = Arc::clone(&self.joints);
        let tick_rate = Arc::clone(&self.tick_rate);

        let mut previous_rotations = self.previous_rotations.clone(); // Rotations précédentes
        let mut rng = StdRng::from_entropy(); // Crée un StdRng thread-safe avec une seed aléatoire
        let mut next_rotations = Self::generate_random_rotations(&mut rng);

        thread::spawn(move || {
            let important_indices = [1, 2, 4, 5, 7, 8, 10, 11, 13, 14]; // Indices des articulations importantes
            let mut y_velocity = 0.0;
            let mut previous_time = Instant::now();
            let mut frame_counter = 0;
            let mut next_update = rng.gen_range(20..=40);

            loop {
                let current_time = Instant::now();
                let delta_time = (current_time - previous_time).as_secs_f32();
                previous_time = current_time;

                frame_counter += 1;

                // Interpolation entre rotations précédentes et nouvelles
                let t = frame_counter as f32 / next_update as f32;
                let interpolated_rotations =
                    Physics::interpolate_rotations(&previous_rotations, &next_rotations, t);

                {
                    let mut joints = joints.lock().unwrap();

                    // Appliquer les rotations interpolées
                    for (i, &angle) in important_indices.iter().zip(interpolated_rotations.iter()) {
                        joints[*i].angle_deg = angle;
                    }

                    for joint in joints.iter_mut() {
                        joint.is_colliding = joint.position.1 >= GROUND_Y - 11.5;
                    }

                    if joints.iter().any(|joint| joint.is_colliding) {
                        y_velocity = 0.0;
                    } else {
                        y_velocity += GRAVITY * delta_time;
                    }
                    for joint in joints.iter() {
                        if joint.position.1 > GROUND_Y {
                            let penetration = joint.position.1 - GROUND_Y; // Distance sous le sol
                            y_velocity -= penetration * 1.0; // Force proportionnelle au dépassement
                        }
                    }
                    

                    joints[0].position.1 += y_velocity;

                    calculate_stickman_positions(&mut joints, &DIMENSIONS);
                }

                // Générer de nouvelles rotations aléatoires toutes les 20-40 frames
                if frame_counter >= next_update {
                    previous_rotations = next_rotations.clone();
                    next_rotations = Physics::generate_random_rotations(&mut rng);
                    frame_counter = 0;
                    next_update = rng.gen_range(20..=40);
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
