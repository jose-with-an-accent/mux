use clap::Parser;
use reqwest::blocking;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    action: String,
    #[arg(short)]
    project_name: String,
}
#[derive(Debug)]
struct Project {
    name: String,
}
struct Translation {}
#[derive(Debug)]
struct TemplateInfo {
    name: String,
    author: String,
    template_url: String,
}
#[derive(Debug)]
struct Language {
    name: String,
    country_code: String,
    language_code: String,
}
fn get_all_templates() -> Vec<TemplateInfo> {
    vec![
        TemplateInfo {
            name: String::from("Astra Theme 1"),
            author: String::from("Astra Theme Dev"),
            template_url: String::from("this_does_not_exist"),
        },
        TemplateInfo {
            name: String::from("Astra Theme 2"),
            author: String::from("Astra Developers"),
            template_url: String::from("template_url://"),
        },
    ]
}

fn new_project(name: &String) {
    let mut template_location = env::current_dir().unwrap();
    println!("{:?}", &template_location);
    template_location.push(format!("/{}", name));

    match fs::create_dir(&template_location) {
        Ok(..) => {
            println!("Ok")
        }
        Err(error) => {
            println!("There was an error creating the directory: {}", error)
        }
    }
    init_project(&mut template_location);
}

fn init_project(path: &mut PathBuf) {
    let i18n_enabled = true;

    if i18n_enabled {
        let i18n_file_path = path.clone();
        path.push("/i18n");

        let languages = vec!["en", "es"];

        for language in languages {
            let mut language_file_path = i18n_file_path.clone();
            language_file_path.push(format!("/{}.json", language));
            println!("{:?}", &language_file_path);

            fs::write(language_file_path, "{}");
        }
    }

    let available_templates = get_all_templates();

    println!("All templates:");
    for (i, template) in available_templates.iter().enumerate() {
        println!("#{}: {}", i, &template.name);
    }
    let chosen = 1; // replace this by code to determine chosen one

    let chosen_template = &available_templates[1];

    println!("{:?}", &chosen_template);
}
fn deploy_project() {
    println!("Select a service to deploy")
}
fn update_i18n() {}
fn view_projects() {
    let current_project = Project {
        name: String::from("Default Project"),
    };

    println!("Current project: {}", current_project.name);
    // let last_updated_cloud = get_last_updated_cloud()
    println!("Last updated: ")
}
fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        "project" => view_projects(),
        "init" => {
            let mut path = env::current_dir().unwrap();
            init_project(&mut path);
        }
        "new" => new_project(&args.project_name),
        "deploy" => deploy_project(),
        "i18n" => update_i18n(),
        &_ => panic!("unknown value for argument"),
    };
}
