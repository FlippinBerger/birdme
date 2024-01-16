use std::{env, io};

struct Bird {
    name: String,
    scientific_name: String,
    //TODO might be able to use a URL type here instead
    wiki_link: String,
}

fn main() {
    println!("Welcome to birdme, I hope you enjoy learning about some of your local birds!");

    let birds = fetch_birds();

    println!("Please select a bird below to learn more:");
    for (i, bird) in birds.iter().enumerate() {
        println!("{}. {} ({})", i + 1, bird.name, bird.scientific_name);
    }

    let mut selection = String::new();

    read_input(&mut selection);

    let index = selection.trim().parse::<usize>();

    match index {
        Ok(i) => {
            println!("index is {}", i);
            println!("you selected the {}", birds[i - 1].name);
        }
        Err(err) => println!("couldnt parse index: {:?}", err),
    }
}

fn read_input(s: &mut String) {
    io::stdin().read_line(s).expect("Failed to read line");
}

fn fetch_birds() -> Vec<Bird> {
    vec![
        Bird {
            name: String::from("American Robin"),
            scientific_name: String::from("Turdus migratorius"),
            wiki_link: String::from("https://en.wikipedia.org/wiki/American_robin"),
        },
        Bird {
            name: String::from("Barn Swallow"),
            scientific_name: String::from("Hirundo rustica"),
            wiki_link: String::from("https://en.wikipedia.org/wiki/Barn_swallow"),
        },
        Bird {
            name: String::from("Barred Owl"),
            scientific_name: String::from("Strix varia"),
            wiki_link: String::from("https://en.wikipedia.org/wiki/Barred_owl"),
        },
        Bird {
            name: String::from("Blue Jay"),
            scientific_name: String::from("Cyanocitta cristata"),
            wiki_link: String::from("https://en.wikipedia.org/wiki/Blue_jay"),
        },
        Bird {
            name: String::from("Common Raven"),
            scientific_name: String::from("Corvus corax"),
            wiki_link: String::from("https://en.wikipedia.org/wiki/Common_raven"),
        },
    ]
}
