use std::io;


pub fn continuer_ou_pas() -> i32 {
    println!("
    \n Que souhaitez vous faire? 
    \n 1) Continuer
    \n 2) Quitter le jeu");

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