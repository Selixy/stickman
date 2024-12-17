use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct GeneticAlgorithm {
    pub population: Vec<Vec<f32>>,
    pub scores: Vec<f32>,
    pub population_size: usize,
    pub num_best: usize,
    pub mutation_rate: f32,
    rng: StdRng,
}

impl GeneticAlgorithm {
    /// Initialise l'algorithme génétique avec une population aléatoire
    pub fn new(population_size: usize, num_best: usize, mutation_rate: f32) -> Self {
        let mut rng = StdRng::from_entropy();
        let population = (0..population_size)
            .map(|_| (0..10).map(|_| rng.gen_range(-45.0..45.0)).collect())
            .collect();

        Self {
            population,
            scores: vec![0.0; population_size],
            population_size,
            num_best,
            mutation_rate,
            rng,
        }
    }

    /// Évalue une pose actuelle et stocke le score
    pub fn evaluate_pose(&mut self, index: usize, score: f32) {
        self.scores[index] = score;
    }

    /// Sélectionne les meilleures poses en fonction des scores
    fn select_best(&self) -> Vec<Vec<f32>> {
        let mut combined: Vec<(f32, &Vec<f32>)> = self.scores
            .iter()
            .zip(self.population.iter())
            .map(|(&score, rotations)| (score, rotations)) // Déréférence &f32
            .collect();

        combined.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap()); // Tri décroissant
        combined
            .iter()
            .take(self.num_best)
            .map(|(_, rotations)| (*rotations).clone())
            .collect()
    }

    /// Croisement entre deux parents pour générer un enfant
    fn crossover(&mut self, parent1: &[f32], parent2: &[f32]) -> Vec<f32> {
        parent1
            .iter()
            .zip(parent2.iter())
            .map(|(a, b)| if self.rng.gen::<bool>() { *a } else { *b })
            .collect()
    }

    /// Mutation pour explorer de nouvelles solutions
    fn mutate(&mut self, rotations: &mut Vec<f32>) {
        for angle in rotations.iter_mut() {
            if self.rng.gen::<f32>() < self.mutation_rate {
                *angle += self.rng.gen_range(-5.0..5.0);
                *angle = angle.clamp(-45.0, 45.0);
            }
        }
    }

    /// Génère une nouvelle génération de solutions
pub fn generate_new_generation(&mut self) {
    if self.population.is_empty() {
        println!("La population est vide. Impossible de générer une nouvelle génération !");
        return; // Évite un crash si la population est vide
    }

    let best = self.select_best();
    let mut new_population = Vec::new();

    for i in 0..self.population_size {
        let mut child = self.crossover(&best[i % self.num_best], &best[(i + 1) % self.num_best]);
        self.mutate(&mut child);
        new_population.push(child);
    }

    self.population = new_population;
}
}
