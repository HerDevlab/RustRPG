use std::io;

//apparament dans ce langage, pour dire le type qui va être retourné, on utilise -> type
pub fn menu() -> i32 {
    println!(" Menu: 
    \n Que souhaitez vous faire? 
    \n 1) Nouvelle partie 
    \n 2) Charger une partie 
    \n 3) Quitter le jeu");

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Failed to read line");

    //format choix et récupération du chiffre inséré
    let choixx = choix.trim();
    //si il écrit des lettres, il y aura une erreur. A revoir.
    let num_choix: i32 = choixx.parse().unwrap();

    return num_choix;
}