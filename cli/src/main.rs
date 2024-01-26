use cli::birdme::Bird;
use cli::config;
use std::io;

fn main() {
    println!("Welcome to birdme, I hope you enjoy learning about some of your local birds!");

    let config = match config::get_config() {
        Some(conf) => conf,
        None => config::Config::new(),
    };
    // TODO put the setting of the region onto the user
    let birds = fetch_birds(&config.region.expect("region must be set"));

    println!("Please select a bird below to learn more:");
    for (i, bird) in birds.iter().enumerate() {
        println!("{}. {} ({})", i + 1, bird.name, bird.scientific_name);
    }

    let mut selection = String::new();

    read_input(&mut selection);

    let index = selection.trim().parse::<usize>();

    match index {
        Ok(i) => {
            let bird = &birds[i - 1];
            println!();
            println!("{} ({})", bird.name, bird.scientific_name);
            println!("{}", bird.blurb);
            println!("To learn more visit {}", bird.link);
        }
        Err(err) => println!("couldnt parse index: {:?}", err),
    }
}

fn read_input(s: &mut String) {
    io::stdin().read_line(s).expect("Failed to read line");
}

fn fetch_birds(region: &str) -> Vec<Bird> {
    vec![
        Bird {
            name: String::from("American Robin"),
            scientific_name: String::from("Turdus migratorius"),
            link: String::from("https://en.wikipedia.org/wiki/American_robin"),
            blurb: String::from("Eats a ton of worms every day..."),
        },
        Bird {
            name: String::from("Barn Swallow"),
            scientific_name: String::from("Hirundo rustica"),
            link: String::from("https://en.wikipedia.org/wiki/Barn_swallow"),
            blurb: String::from("Likes to nest under bridges..."),
        },
        Bird {
            name: String::from("Barred Owl"),
            scientific_name: String::from("Strix varia"),
            link: String::from("https://en.wikipedia.org/wiki/Barred_owl"),
            blurb: String::from("Very majestic in the winter..."),
        },
        Bird {
            name: String::from("Blue Jay"),
            scientific_name: String::from("Cyanocitta cristata"),
            link: String::from("https://en.wikipedia.org/wiki/Blue_jay"),
            blurb: String::from("Loud and rude to others..."),
        },
        Bird {
            name: String::from("Common Raven"),
            scientific_name: String::from("Corvus corax"),
            link: String::from("https://en.wikipedia.org/wiki/Common_raven"),
            blurb: String::from("Found across America..."),
        },
    ]
}
