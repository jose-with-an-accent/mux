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
    name: Option<String>,
    #[arg(short)]
    template_sub_action: Option<String>

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

fn cli_new_project(args: Args) {
    let name = &args.name.unwrap();
    let mut template_location = env::current_dir().unwrap();
    println!("{:?}", &template_location);
    template_location.push(format!("{}", name));

    fs::create_dir(&template_location).unwrap();

    init_project(&mut template_location);
}

fn init_project(path: &mut PathBuf) {
    let i18n_enabled = true;

    if i18n_enabled {
        let mut i18n_file_path = path.clone();
        i18n_file_path.push("i18n");

        fs::create_dir(i18n_file_path);

        let languages = vec!["en", "es"];

        // for language in languages {
        //     let mut language_file_path = &i18n_file_path.clone();
        //     language_file_path.push(format!("/{}.json", language));
        //     println!("{:?}", &language_file_path);

        //     fs::write(language_file_path, "{}");
        // }
    }

    let available_templates = get_all_templates();

    println!("All templates:");
    for (i, template) in available_templates.iter().enumerate() {
        println!("#{}: {}", i, &template.name);
    }
    let chosen = 1; // replace this by code to determine chosen one

    let chosen_template = &available_templates[chosen];

    println!("{:?}", &chosen_template);
}
fn deploy_project() {
    println!("Select a service to deploy")
}
fn update_i18n() {
    todo!()
    // This function is meant to handle internationalization
}
fn view_template_demo() {
    // 
    // launch a basic web server that 
}
fn view_project() {
    println!("Project Configuration");
    
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
        "project" => view_project(),
        "init" => {
            let mut path = env::current_dir().unwrap();
            init_project(&mut path);
        }
        "new" => cli_new_project(args),
        "templates" => {
            let templates = get_all_templates();
            println!("Current templates: ");
                match args.template_sub_action.unwrap().as_str() {
                    "view" => {
                        for (i, template) in templates.iter().enumerate() {
                            println!("#{}: {} //", i, &template.name);

                    }
                }
                &_ => panic!("unknown value for template sub-argument")

            }
        },
        "deploy" => deploy_project(),
        "i18n" => update_i18n(),
        "debug" => {
            let mut base_path = env::current_dir().unwrap().clone();
            base_path.push("project");
            println!("Current directory: {:?}", base_path);

            let mut i18n_path = base_path.clone();
            i18n_path.push("i18n");
            let i18n_folder = fs::read_dir(&i18n_path).unwrap();

            let mut project_config_file = base_path.clone();
            project_config_file.push("project.toml");

            let project_config = fs::read_to_string(project_config_file).unwrap();

            println!("Contents of project.toml file: \n{}", project_config);

            for file in i18n_folder {
                println!("{}", file.unwrap().file_name().to_str().unwrap())
            }
        }
        &_ => panic!("unknown value for argument"),
    };
}
