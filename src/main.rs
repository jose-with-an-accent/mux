use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    action: String,
    #[arg(short)]
    project_name: String
    
}
#[derive(Debug)]
struct Project {
    name: String,
    template_id: String
}
#[derive(Debug)]
struct TemplateInfo { 
    name: String,
    author: String,
    template_url: String,
    
}
#[derive(Debug)] 
struct Language {
    name: String,
    country: String,
    short_code: String
}
fn get_all_templates() -> Vec<TemplateInfo> {
    vec![
        TemplateInfo {
            name: String::from("Astra Theme 1"),
            author: String::from("Astra Theme Dev"),
            template_url: String::from("this_does_not_exist")
        },
        TemplateInfo {
            name: String::from("Astra Theme 2"),
            author: String::from("Astra Developers"),
            template_url: String::from("template_url://")
        }
    ]
}
fn load_template_settings() -> TemplateInfo {
    TemplateInfo {
        name: String::from("Astra Theme 1"),
        author: String::from("Jose Sanchez"),
        template_url: String::from("https://github.com/jose-with-an-accent/base_theme")
    }
}
fn new_project(name: &String) {
    let template_location: String = String::from("template/"); // default project location
    let output_location = String::from("build/"); // default output location
    

    println!("Creating new directory at {} with name {}", output_location, name);

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
fn update_i18n() {
    
}
fn view_projects() {
    println!("All Projects");
    let projects: Vec<Project> = vec![];

    for project in projects {
        println!("{} - Last updated {}", &project.name, "November 7, 2022")
    }
}
fn main() {
    
    let args = Args::parse();

    match args.action.as_str() {
        "projects" => view_projects(),
        "new" => {
            new_project(&args.project_name);
        }
        "deploy" => {
            // this will connect to a certain backend and deploy the project from there
            deploy_project();
        }
        "i18n" => {
            update_i18n();
        }
        &_ => panic!("unknown value for argument")
        
    };
}
