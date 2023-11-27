// Cette fonction calcule la distance la plus courte entre un caractère spécifié et un autre caractère spécifié.
pub fn short_distance(
    grid: &Vec<Vec<char>>,
    distance: f32,
    play: &Vec<char>,
    enemy: &Vec<char>,
) -> f32 {
    let mut min_dist = distance; // Cette variable stocke la distance minimale trouvée.

    // Parcourir chaque ligne de la grille.
    for (yg, row) in grid.iter().enumerate() {
        // Parcourir chaque colonne de la ligne actuelle.
        for (xg, &cell) in row.iter().enumerate() {
            // Vérifier si le caractère actuel correspond au deuxième caractère dans `play`.
            if cell == play[1] {
                // Une autre boucle pour chaque ligne de la grille pour comparer avec `enemy`.
                for (ye, enemy_row) in grid.iter().enumerate() {
                    // Boucle sur chaque colonne de la ligne courante pour cette deuxième boucle.
                    for (xe, &enemy_cell) in enemy_row.iter().enumerate() {
                        // Vérifier si le caractère à cette position est dans `enemy`.
                        if enemy.contains(&enemy_cell) {
                            // Calcul de la distance euclidienne entre (yg, xg) et (ye, xe) en utilisant le théorème de Pythagore.
                            let dist = (((ye as f32) - (yg as f32)).powf(2.)
                                + ((xe as f32) - (xg as f32)).powf(2.))
                            .sqrt();
                            // Si la distance calculée est inférieure à `min_dist`, la mettre à jour.
                            min_dist = min_dist.min(dist);
                        }
                    }
                }
            }
        }
    }
    // Retourner la distance minimale trouvée.
    min_dist
}

/****************************************************\
    Fonction pour les meilleures coordonnées pour
    placer une pièce sur la grille.
\****************************************************/
pub fn place_piece(
    grid: &Vec<Vec<char>>,  // La grille du jeu
    piece: &Vec<Vec<char>>, // La pièce à placer
    play: &Vec<char>,       // Caractères valides pour le placement
    enemy: &Vec<char>,      // Caractères adverses ou non valides
) -> (usize, usize) {
    // Initialisation des variables et calcul de la distance initiale
    let grid_rows = grid[0].len();
    let piece_rows = piece[0].len();
    let mut distance = ((grid_rows as f32).powf(2.) + (grid.len() as f32).powf(2.)).sqrt(); //powf(2.) = puissance 2(multiplication par lui-même)
    let mut sol = (0, 0);

    // Calcul des limites basées sur les caractères valides dans play
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (grid.len(), 0, grid_rows, 0);
    for yg in 0..grid.len() {
        for xg in 0..grid_rows {
            // Si la cellule actuelle contient un caractère valide pour le jeu, ajuste les limites.
            if play.contains(&grid[yg][xg]) {
                xmin = xmin.min(xg);
                xmax = xmax.max(xg);
                ymin = ymin.min(yg);
                ymax = ymax.max(yg);
            }
        }
    }

    // Ajustement de la plage de recherche pour le placement de la pièce
    let (mut xi, mut xf, mut yi, mut yf) = (
        0,
        grid_rows - piece_rows + 1,
        0,
        grid.len() - piece.len() + 1,
    );

    // Ajustement des limites pour le placement de la pièce
    let temp = xmin as i32 - piece_rows as i32 - 1;
    if temp > 0 {
        xi = xmin - piece_rows + 1
    }
    if (xmax + piece_rows - 1) < grid_rows {
        xf = xmax + 1
    }
    let temp = ymin as i32 - piece.len() as i32 + 1;
    if temp > 0 {
        yi = ymin - piece.len() + 1
    }
    if (ymax + piece.len() - 1) < grid.len() {
        yf = ymax + 1
    }

    // Recherche chaque cellule potentielle de la grille pour le placement de la pièce.
    for yg in yi..yf {
        for xg in xi..xf {
            if put_piece(grid, piece, play, xg, yg) {
                let min_dist = short_distance(
                    &grid_with_piece(grid, piece, play, (xg, yg)),
                    distance,
                    play,
                    enemy,
                );
                if min_dist < distance {
                    distance = min_dist;
                    sol = (xg, yg);
                }
            }
        }
    }
    // Retourne les meilleures coordonnées trouvées
    sol
}
///////////////////////// FIN ////////////////////////////////

/****************************************************\
 Fonction pour placer une pièce sur la grille existante.
\****************************************************/
pub fn grid_with_piece(
    grid: &[Vec<char>],     // La grille du jeu actuelle
    piece: &[Vec<char>],    // La pièce à placer
    play: &[char],          // Caractères spécifiques pour certaines actions
    coords: (usize, usize), // Coordonnées pour placer la pièce
) -> Vec<Vec<char>> {
    // Création d'une copie de la grille pour la modification
    let mut new_grid = grid.to_vec();
    let prows = piece[0].len(); // number of piece rows

    // Remplacement des caractères adverses par les caractères valides
    for yg in 0..new_grid.len() {
        for xg in 0..new_grid[0].len() {
            if new_grid[yg][xg] == play[1] {
                new_grid[yg][xg] = play[0]
            }
        }
    }

    for yp in 0..piece.len() {
        for xp in 0..prows {
            if piece[yp][xp] != '.' {
                new_grid[yp + coords.1][xp + coords.0] = play[1]
            }
        }
    }
    new_grid
}
///////////////////////// FIN //////////////////////////////

/****************************************************\
    Fonction pour placer une pièce sur une grille.
\****************************************************/
pub fn put_piece(
    grid: &[Vec<char>],  // La grille du jeu
    piece: &[Vec<char>], // La pièce à placer
    play: &[char],       // Caractères valides pour le placement
    xg: usize,           // Coordonnée X sur la grille pour le placement
    yg: usize,           // Coordonnée Y sur la grille pour le placement
) -> bool {
    let mut cross = 0; // Compteur pour les intersections valides

    // Parcourir chaque ligne (row) et colonne (val) de la pièce
    for (yp, row) in piece.iter().enumerate() {
        for (xp, &val) in row.iter().enumerate() {
            // Traitement seulement pour les caractères non vides
            if val != '.' {
                // Obtention du caractère correspondant sur la grille
                let cell = grid.get(yg + yp).and_then(|r| r.get(xg + xp));

                match cell {
                    // Incrémentation de cross si le caractère est valide
                    Some(&c) if play.contains(&c) => cross += 1,
                    // Retour immédiat de false si un caractère non valide est trouvé
                    Some(&c) if c != '.' => return false,
                    _ => {}
                }

                // Retour de false si plus d'une intersection valide est trouvée
                if cross > 1 {
                    return false;
                }
            }
        }
    }
    // Retourne true si exactement une intersection valide est trouvée
    cross == 1
}
///////////////////////// FIN ////////////////////////////
