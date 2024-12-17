use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Représente un cycle avec des rotations et une durée.
#[derive(Clone)] // Implémente le trait Clone pour permettre les copies
pub struct Cycle {
    pub rotations: Vec<f32>, // 10 angles pour les articulations
    pub duration: usize,     // Durée du cycle en frames
}

/// Représente un individu avec 20 cycles.
#[derive(Clone)] // Implémente le trait Clone pour permettre les copies
pub struct Individual {
    pub cycles: Vec<Cycle>, // Un individu est composé de 20 cycles
}

pub struct GeneticAlgorithm {
    pub population: Vec<Individual>, // Population d'individus
    pub scores: Vec<f32>,            // Scores des individus
    pub population_size: usize,
    pub num_best: usize,
    pub mutation_rate: f32,
    rng: StdRng, // Générateur aléatoire
}

impl GeneticAlgorithm {
    /// Initialise l'algorithme génétique avec une population d'individus.
    pub fn new(size: usize, num_best: usize, mutation_rate: f32) -> Self {
        let mut rng = StdRng::from_entropy();
        let population = (0..size)
            .map(|_| Self::generate_random_individual(&mut rng))
            .collect();

        Self {
            population,
            scores: vec![0.0; size],
            population_size: size,
            num_best,
            mutation_rate,
            rng,
        }
    }

    /// Génère un individu aléatoire avec 20 cycles.
    fn generate_random_individual(rng: &mut StdRng) -> Individual {
        let cycles = (0..20)
            .map(|_| Cycle {
                rotations: (0..10).map(|_| rng.gen_range(-45.0..45.0)).collect(),
                duration: rng.gen_range(10..30),
            })
            .collect();

        Individual { cycles }
    }

    /// Évalue une pose actuelle et stocke le score.
    pub fn evaluate_pose(&mut self, index: usize, score: f32) {
        if index < self.scores.len() {
            self.scores[index] = score;
        }
    }

    /// Sélectionne les meilleurs individus en fonction des scores.
    fn select_best(&self) -> Vec<Individual> {
        let mut combined: Vec<(f32, &Individual)> = self
            .scores
            .iter()
            .zip(self.population.iter())
            .map(|(&score, individual)| (score, individual))
            .collect();

        combined.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        combined
            .iter()
            .take(self.num_best)
            .map(|(_, individual)| (*individual).clone())
            .collect()
    }

    /// Croisement entre deux parents pour générer un enfant.
    fn crossover(&mut self, parent1: &Individual, parent2: &Individual) -> Individual {
        let cycles = parent1
            .cycles
            .iter()
            .zip(parent2.cycles.iter())
            .map(|(c1, c2)| {
                let rotations = c1
                    .rotations
                    .iter()
                    .zip(c2.rotations.iter())
                    .map(|(a, b)| if self.rng.gen::<bool>() { *a } else { *b })
                    .collect();

                let duration = if self.rng.gen::<bool>() {
                    c1.duration
                } else {
                    c2.duration
                };

                Cycle { rotations, duration }
            })
            .collect();

        Individual { cycles }
    }

    /// Mutation pour introduire de la variabilité.
    fn mutate(&mut self, individual: &mut Individual) {
        for cycle in individual.cycles.iter_mut() {
            for angle in cycle.rotations.iter_mut() {
                if self.rng.gen::<f32>() < self.mutation_rate {
                    *angle += self.rng.gen_range(-5.0..5.0);
                    *angle = angle.clamp(-45.0, 45.0);
                }
            }

            if self.rng.gen::<f32>() < self.mutation_rate {
                cycle.duration = self.rng.gen_range(10..30);
            }
        }
    }

    /// Génère une nouvelle génération.
    pub fn generate_new_generation(&mut self) {
        let best = self.select_best();
        if best.is_empty() {
            println!("Aucun meilleur individu trouvé !");
            return;
        }

        let mut new_population = Vec::new();
        for i in 0..self.population_size {
            let parent1 = &best[i % self.num_best];
            let parent2 = &best[(i + 1) % self.num_best];
            let mut child = self.crossover(parent1, parent2);
            self.mutate(&mut child);
            new_population.push(child);
        }

        self.population = new_population;
        println!("Nouvelle génération créée avec succès !");
    }
}
