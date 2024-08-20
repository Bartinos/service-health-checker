mod mail_controller;
mod service_checker;
mod services;
use dotenv::dotenv;
use services::{read_services, Checkable};

fn main() {
    dotenv().ok();
    println!("Initializing Service Health Checker");

    let mail_controller = mail_controller::MailController::new();
    let checkable_services: Vec<Box<dyn Checkable>> = read_services().unwrap();

    let mut service_checker =
        service_checker::ServiceChecker::new(mail_controller, checkable_services);

    service_checker.start();
}
