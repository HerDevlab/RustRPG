use std::io;
#[derive(Debug)]

pub struct Joueur {
    pub nom: String, //input par le joueur
    pub vie: isize, //predef
    pub magie: isize, //predef
    pub force: isize, //predef
}

pub fn crea_joueur() -> Joueur {
    //création du joueur
    println!("Quel est ton nom, humain?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let trim_name = name.trim();
    let trimmed_name = trim_name.to_string();
    let mut player = Joueur{nom: trimmed_name, 
                            vie: 100,
                            magie: 2, 
                            force: 5};
    println!("{:?}", player);

    //repartition des 5 points bonus
    let mut point_a_repartir = 5;
    
    //tant qu'il y a des points à répartir
    while point_a_repartir != 0{

        println!("Vous disposez de {} points à répartir:", point_a_repartir);
        println!(" 1) vie \n 2) magie \n 3) force");
        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Failed to read line");

        //format choix et récupération du chiffre inséré
        let choixx = choix.trim();
        //si il écrit des lettres, il y aura une erreur. A revoir.
        let num_choix: i32 = choixx.parse().unwrap();

        //choix et action
        if num_choix == 1 {
            println!("Tu as choisis la vie.");
            player.vie += 1;
            //décrementer
            point_a_repartir -= 1;

        }else if num_choix == 2 {
            println!("Tu as choisis la magie.");
            player.magie += 1;
            //décrementer
            point_a_repartir -= 1;

        }else if num_choix == 3 {
            println!("Tu as choisis la force.");
            player.force += 1;
            //décrementer
            point_a_repartir -= 1;

        }else{
            println!("{} n'est pas une option. Recommence.",num_choix);
        }

        //on affiche à nouveau
        //println!("{:?}", player);
    }

    return player;
}


pub fn use_force() -> isize {
    println!("Quelle quantité de force souhaites tu utiliser contre Francis?");

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Failed to read line");

    //format choix et récupération du chiffre inséré
    let choixx = choix.trim();
    //si il écrit des lettres, il y aura une erreur. A revoir.
    let num_choix: isize = choixx.parse().unwrap();

    return num_choix;
}


pub fn use_magie() -> isize {
    println!("Quelle quantité de magie souhaites tu utiliser contre Francis?");

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Failed to read line");

    //format choix et récupération du chiffre inséré
    let choixx = choix.trim();
    //si il écrit des lettres, il y aura une erreur. A revoir.
    let num_choix: isize = choixx.parse().unwrap();

    return num_choix;
}