use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::stickman::{calculate_stickman_positions, init_joints, DIMENSIONS, Joint};

const GRAVITY: f32 = 9.8;
const GROUND_Y: f32 = 545.0;

pub struct Physics {
    joints: Arc<Mutex<Vec<Joint>>>,
    y_velocity: Arc<Mutex<f32>>,
    tick_rate: Arc<Mutex<f32>>,
    hts: Arc<Mutex<f32>>, // HTS (Hertz ou taux de tick)
}

impl Physics {
    /// Crée une instance Physics avec position initiale
    pub fn new_with_position(initial_position: (f32, f32), initial_tick_rate: f32) -> Self {
        let mut joints = init_joints();
        joints[0].position = initial_position;
        Physics {
            joints: Arc::new(Mutex::new(joints)),
            y_velocity: Arc::new(Mutex::new(0.0)),
            tick_rate: Arc::new(Mutex::new(initial_tick_rate)),
            hts: Arc::new(Mutex::new(0.0)),
        }
    }

    /// Démarre un thread pour la physique et mesure le HTS
    pub fn start(&self) {
        let joints = Arc::clone(&self.joints);
        let y_velocity = Arc::clone(&self.y_velocity);
        let tick_rate = Arc::clone(&self.tick_rate);
        let hts = Arc::clone(&self.hts);

        thread::spawn(move || {
            let mut previous_time = Instant::now();
            let mut tick_count = 0;
            let hts_interval = 1.0; // Intervalle de mesure en secondes

            loop {
                let current_time = Instant::now();
                let delta_time = current_time.duration_since(previous_time).as_secs_f32();

                let current_tick_rate = *tick_rate.lock().unwrap();

                // Logique physique
                {
                    let mut joints = joints.lock().unwrap();
                    let mut y_velocity = y_velocity.lock().unwrap();

                    for joint in joints.iter_mut() {
                        joint.is_colliding = joint.position.1 >= GROUND_Y - 11.5;
                    }

                    if joints.iter().any(|joint| joint.is_colliding) {
                        *y_velocity = 0.0;
                    } else {
                        *y_velocity += GRAVITY * delta_time;
                    }

                    joints[0].position.1 += *y_velocity;
                    calculate_stickman_positions(&mut joints, &DIMENSIONS);
                }

                // Compteur de ticks pour mesurer le HTS
                tick_count += 1;
                if delta_time >= hts_interval {
                    let mut hts_lock = hts.lock().unwrap();
                    *hts_lock = tick_count as f32 / hts_interval; // HTS = ticks par seconde
                    tick_count = 0; // Réinitialiser le compteur
                    previous_time = Instant::now(); // Redémarrer l'horloge
                }

                // Gestion du tick rate
                if current_tick_rate > 0.0 {
                    thread::sleep(Duration::from_secs_f32(current_tick_rate));
                }
            }
        });
    }

    /// Récupère les joints de l'instance
    pub fn get_joints(&self) -> Arc<Mutex<Vec<Joint>>> {
        Arc::clone(&self.joints)
    }

    /// Met à jour le tick rate
    pub fn set_tick_rate(&self, rate: f32) {
        let mut tick_rate = self.tick_rate.lock().unwrap();
        *tick_rate = rate;
    }

    /// Récupère le HTS actuel
    pub fn get_hts(&self) -> f32 {
        *self.hts.lock().unwrap()
    }
}
