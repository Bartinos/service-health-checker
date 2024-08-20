use crate::mail_controller::MailController;
use crate::services::WebService;

mod mail_controller;
mod service_checker;
mod services;
use dotenv::dotenv;
use services::Checkable;

fn main() {
    dotenv().ok();
    println!("Initializing Service Health Checker");

    let mail_controller = mail_controller::MailController::new();
    let checkable_services: Vec<Box<dyn Checkable>> = vec![
    ];

    let mut service_checker =
        service_checker::ServiceChecker::new(mail_controller, checkable_services);

    service_checker.start();
}
