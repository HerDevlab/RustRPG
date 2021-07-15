#[derive(Debug)]

pub struct Ennemi {
    pub nom: String,
    pub vie: isize,
    pub force: isize, 
}

pub fn new_mechant() -> Ennemi {
    let name_mechant = "Francis";
    let name_ennemi = name_mechant.to_string();
    let mut mechant = Ennemi{nom: name_ennemi, 
        vie: 10, 
        force: 0};
    return mechant;
}

/*
pub fn attaque() {
    
}


pub fn meurt() {
    println!("aaaah c'est la fin!");
}*/