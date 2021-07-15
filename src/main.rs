//use std::io;

mod joueur;
mod mechant;
mod menu;
mod utils;

//use crate::joueur::Joueur;

fn main() {
    println!("Hello, world!");
    let mut menu_decision = menu::menu();

    if menu_decision == 1 {
        println!("Nouvelle Partie");
        let mut mon_joueur = joueur::crea_joueur();
        let mut le_mechant = mechant::new_mechant();
        println!("{:?}", mon_joueur);
        println!("{:?}", le_mechant);

        let mut mechant_abattu = 0;
        let mut first_life_mechant = 10;
        let mut pas_force = 0;
        let mut pas_magie = 0;

        while mon_joueur.vie > 0{
            println!("Fight!");


            //notes: si il fait un mauvais choix, il ne fait pas de dégats à l'ennemi mais en prend
                //exemeple: si joueur écrit 1 alors qu'il n'a plus de force ou de magie, il se fait punir et n'inflige pas de dégats

            //le joueur commence, choix de la force à appliquer
            let mut use_force = joueur::use_force();
            if 0 <= use_force && use_force <= mon_joueur.force{
                //choix de la magie à appliquer 
                let mut use_magie = joueur::use_magie();
                if 0 <= use_magie && use_magie <= mon_joueur.magie{
                    //vie ennemi = vie ennemi - force joueur * 2 + magie * 0.5 possible
                    println!("{} utilise {} force et {} magie contre FRANCIS.", mon_joueur.nom, use_force, use_magie);
                    println!("{} inflige {} dégats à FRANCIS.", mon_joueur.nom, (use_force*10) + (use_magie*5));
                    le_mechant.vie = le_mechant.vie - ((use_force*10) + (use_magie*5));
                    mon_joueur.force = mon_joueur.force - use_force;
                    mon_joueur.magie = mon_joueur.magie - use_magie;
                }
            }


            // si plus de force, ni de magie, il faut qu'il se fasse one shot ou two shot

            // c'est au tour du mechant
            if le_mechant.vie > 0{

                //si mechant a de la vie, et joueur a force et magie
                if le_mechant.vie > 0 && mon_joueur.force > 0 && mon_joueur.magie > 0{
                    //vie joueur = vie joueur - force ennemi * 5
                    println!("Vous recevez {} dégats.",(le_mechant.force*2));
                    mon_joueur.vie = mon_joueur.vie - (le_mechant.force*2);
                    println!("Votre état");
                    println!("{:?}", mon_joueur);
                    println!("Etat de FRANCIS");
                    println!("{:?}", le_mechant);

                //si il n'a ni force ni magie, il va subir deux fois des dégats.

                //si mechant a vie et joueur pas de force
                }else if le_mechant.vie > 0 && mon_joueur.force <= 0{
                    println!("Vous n'avez plus de force.");
                    pas_force += 1; //tour sans force

                    println!("Vous recevez {} dégats.",(le_mechant.force*2));
                    mon_joueur.vie = mon_joueur.vie - (le_mechant.force*2);
                    println!("Votre état");
                    println!("{:?}", mon_joueur);
                    println!("Etat de FRANCIS");
                    println!("{:?}", le_mechant);

                    if pas_force == 5{
                        mon_joueur.vie = 0;
                    }

                //si mechant a vie et joueur pas de magie
                }else if le_mechant.vie > 0 && mon_joueur.magie <= 0{
                    println!("Vous n'avez plus de points de magie.");
                    pas_magie += 1; //tour sans magie

                    println!("Vous recevez {} dégats.",(le_mechant.force*2));
                    mon_joueur.vie = mon_joueur.vie - (le_mechant.force*2);
                    println!("Votre état");
                    println!("{:?}", mon_joueur);
                    println!("Etat de FRANCIS");
                    println!("{:?}", le_mechant);

                    if pas_magie == 5{
                        mon_joueur.vie = 0;
                    }
                }


            //si mechant a plus de vie
            }else{
                //si on tue un ennemi, compteur ennemi tué +1 et xp +1
                mechant_abattu += 1;
                println!("Tu as tué Francis {}, Félicitations!", mechant_abattu);
                let mut continuer = utils::continuer_ou_pas();
                if continuer == 1{
                    // on redonne de la vie au mechant
                    le_mechant.vie = first_life_mechant * mechant_abattu;
                    //on augmente la force du méchant
                    le_mechant.force = le_mechant.force + 5;
                    println!("Nouveau FRANCIS");
                    println!("{:?}", le_mechant);
                    println!("Votre état");
                    println!("{:?}", mon_joueur);
                }else{
                    println!("Bye Bye.");
                }
            } 
            
        }

        println!("GAME OVER");
        println!("Tu t'es bien battu. Tu as abattu {} Francis.", mechant_abattu);

    }else if menu_decision == 2 {
        println!("Partie chargée.");

    }else if menu_decision == 3 {
        println!("Aurevoir humain.");

    }else{
        println!("euh... aurevoir(trolleur va).");
        //menu_decision = menu::menu();
    }

        //début partie
            
    
}