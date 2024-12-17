use crate::stickman::{calculate_stickman_positions, init_joints, DIMENSIONS, Joint};
use crate::genetic::Individual;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const GRAVITY: f32 = 9.8;
const GROUND_Y: f32 = 545.0;

pub struct Physics {
    pub joints: Arc<Mutex<Vec<Joint>>>,
    pub tick_rate: Arc<Mutex<f32>>,
    cycles: Vec<crate::genetic::Cycle>,
    current_cycle: usize,
    frame_counter: usize,
    is_dead: bool,
}

impl Physics {
    /// Initialise un nouvel objet `Physics` avec un individu génétique.
    pub fn new_with_individual(
        initial_position: (f32, f32),
        tick_rate: f32,
        individual: Individual,
    ) -> Self {
        let mut joints = init_joints();
        joints[0].position = initial_position;

        Self {
            joints: Arc::new(Mutex::new(joints)),
            tick_rate: Arc::new(Mutex::new(tick_rate)),
            cycles: individual.cycles.clone(),
            current_cycle: 0,
            frame_counter: 0,
            is_dead: false,
        }
    }

    pub fn is_alive(&self) -> bool {
        !self.is_dead
    }

    pub fn start(&mut self) {
        let joints = Arc::clone(&self.joints);
        let tick_rate = Arc::clone(&self.tick_rate);
        let cycles = self.cycles.clone();

        thread::spawn(move || {
            let important_indices = [1, 2, 4, 5, 7, 8, 10, 11, 13, 14];
            let mut y_velocity = 0.0;
            let mut previous_time = Instant::now();
            let mut current_cycle = 0;
            let mut frame_counter = 0;

            loop {
                let current_time = Instant::now();
                let delta_time = (current_time - previous_time).as_secs_f32();
                previous_time = current_time;

                // Récupère le cycle actuel
                let cycle = &cycles[current_cycle];

                {
                    let mut joints_guard = joints.lock().unwrap();

                    // Applique les rotations aux articulations importantes
                    for (i, &angle) in important_indices.iter().zip(cycle.rotations.iter()) {
                        joints_guard[*i].angle_deg = angle;
                    }

                    // Applique la gravité à l'ensemble des articulations
                    for joint in joints_guard.iter_mut() {
                        joint.position.1 += y_velocity * delta_time; // Applique la vitesse verticale
                        
                        if joint.position.1 >= GROUND_Y {
                            // Collision avec le sol
                            joint.position.1 = GROUND_Y;
                            y_velocity = 0.0; // Réinitialise la vitesse à 0
                        }
                    }

                    // Met à jour la vitesse verticale avec la gravité
                    y_velocity += GRAVITY * delta_time;

                    // Recalcule les positions des joints
                    calculate_stickman_positions(&mut joints_guard, &DIMENSIONS);
                }

                // Gestion des cycles
                frame_counter += 1;
                if frame_counter >= cycle.duration {
                    frame_counter = 0;
                    current_cycle += 1;

                    if current_cycle >= cycles.len() {
                        println!("Individu terminé !");
                        break;
                    }
                }

                thread::sleep(Duration::from_secs_f32(*tick_rate.lock().unwrap()));
            }
        });
    }

    pub fn get_joints(&self) -> Arc<Mutex<Vec<Joint>>> {
        Arc::clone(&self.joints)
    }
}
