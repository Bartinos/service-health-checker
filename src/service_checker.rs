use std::fmt::format;

use crate::mail_controller::{self, MailController};
use crate::services::{Checkable, WebService};
use curl::easy::Easy;

pub struct ServiceChecker {
    mail_controller: MailController,
    services_to_check: Vec<Box<dyn Checkable>>,
    curl: Easy,
}

impl ServiceChecker {
    pub fn new(
        mail_controller: MailController,
        services_to_check: Vec<Box<dyn Checkable>>,
    ) -> Self {
        let easy_curl = Easy::new();

        ServiceChecker {
            mail_controller,
            services_to_check,
            curl: easy_curl,
        }
    }

    pub fn start(&mut self) {
        //loop {
        let mut error_reports: Vec<String> = Vec::new();
        for checkable in &self.services_to_check {
            let check_result = checkable.check_service(&mut self.curl);
            
            match check_result {
                Ok(success_report) => {
                    println!("{}", success_report);
                }
                Err(error_report) => {
                    println!("{}", error_report);
                    error_reports.push(error_report);
                }
            }
        }

        println!("");
        if error_reports.len() > 0 {
            for error in &error_reports {
                //println!("{}", error);
            }
            let reports_body = format!("{} \n------------------------------------------------------", error_reports.join("\n"));
            //println!("{}", reports_body);
            self.mail_controller.send_mail(reports_body)
        }
        //}
    }
}
