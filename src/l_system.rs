use macroquad::prelude::*;

/// Fonction pour dessiner un arbre L-System avec des branches et des feuilles
pub fn draw_l_system_tree(start_x: f32, start_y: f32, angle: f32, length: f32, iterations: usize) {
    draw_branch(start_x, start_y, angle, length, iterations);
}

/// Fonction récursive pour dessiner les branches et les feuilles
fn draw_branch(x: f32, y: f32, angle: f32, length: f32, iterations: usize) {
    if iterations == 0 {
        // Dessiner une feuille sous forme de cercle
        draw_circle(x, y, 5.0, GREEN);
        return;
    }

    let radian_angle = angle.to_radians();
    let end_x = x + length * radian_angle.cos();
    let end_y = y + length * radian_angle.sin();

    // Dessiner la branche
    draw_line(x, y, end_x, end_y, 2.0, BROWN);

    // Réduire la longueur pour les branches suivantes
    let new_length = length * 0.7;

    // Appel récursif pour les deux branches
    draw_branch(end_x, end_y, angle - 25.0, new_length, iterations - 1); // Branche gauche
    draw_branch(end_x, end_y, angle + 25.0, new_length, iterations - 1); // Branche droite
}
