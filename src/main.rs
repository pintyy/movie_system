fn main() {
    let mut database: Vec<Movie> = Vec::new();
    register_movie("Dexter".to_string(), "Michael C Hall".to_string(), MovieCategories::Criminal, 2006, 106, &mut database);
    get_movie_by_name("Dexter".to_string(), &mut database);
}

#[derive(Debug, Clone)]
struct Movie {
    name: String,
    actor: String,
    category: MovieCategories,
    publish_year: u32,
    episode: u32,
}

#[derive(Debug, Clone)]
enum MovieCategories {
    Sitcom,
    Action,
    Romantic,
    Criminal,
}

fn register_movie(name: String, actor: String, category: MovieCategories, publish_year: u32, episode: u32, database: &mut Vec<Movie>) {
    let movie = Movie {
        name,
        actor,
        category,
        publish_year,
        episode,
    };
    database.push(movie);
}

fn get_movie_by_name(name: String, database: &Vec<Movie>) {
    let mut found = false; 
    for data in database {
        if name == data.name {
            println!("Movie information: {:#?}", data);
            found = true; 
            break;
        }
    }
    if !found {
        println!("Couldn't find the movie: {}", name);
    }
}
