use crate::schema::user_location::UserLocationContent;
use std::collections::HashMap;

macro_rules! user_map {
    ($($key:expr => $value:expr),*) => {{
        let mut map: HashMap<String, Option<String>> = HashMap::new();
        $(map.insert($key.to_string(), $value);)*
        map
    }};
}

fn string_option_to_string(option: Option<String>) -> String {
    option.map(|x| x.to_string()).unwrap_or("Unknown".to_string())
}

fn number_option_to_string_option(option: Option<i32>) -> Option<String> {
    option.map(|x| x.to_string())
}

pub fn replace_magic_string(value: String, content: &UserLocationContent) -> String {
    let user = content.user.clone().unwrap();
    let world = content.world.clone().unwrap();

    let fields = user_map! {
        "user.bio" => user.bio,
        "user.date_joined" => user.date_joined,
        "user.developer_type" => user.developer_type,
        "user.display_name" => user.display_name,
        "user.last_platform" => user.last_platform,
        "user.note" => user.note,
        "user.state" => user.state,
        "user.status" => user.status,
        "user.status_description" => user.status_description,
        "user.travelling_to_instance" => user.traveling_to_instance,
        "user.travelling_to_location" => user.traveling_to_location,
        "user.travelling_to_world" => user.traveling_to_world,
        "world.author_id" => world.author_id,
        "world.author_name" => world.author_name,
        "world.capacity" => number_option_to_string_option(world.capacity),
        "world.description" => world.description,
        "world.favorites" => number_option_to_string_option(world.favorites),
        "world.heat" => number_option_to_string_option(world.heat),
        "world.id" => world.id,
        "world.name" => world.name,
        "world.namespace" => world.namespace,
        "world.occupants" => number_option_to_string_option(world.occupants),
        "world.organization" => world.organization,
        "world.popularity" => number_option_to_string_option(world.popularity),
        "world.release_status" => world.release_status,
        "world.tags" => world.tags.map(|x| x.join(", ")),
        "world.unity_packages" => world.unity_packages.map(|x| x.iter().map(|x| x.asset_url.clone().unwrap_or("Unknown".to_string())).collect::<Vec<String>>().join(", ")),
        "world.visits" => number_option_to_string_option(world.visits)
    };

    let mut new_value = value.clone();
    for (key, value) in fields {
        new_value = new_value.replace(&format!("{{{{{}}}}}", key), &string_option_to_string(value));
    }
    
    new_value
}