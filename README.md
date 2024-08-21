# Service Health Checker
Takes `MAIL_FROM`, `MAIL_TO` and `MAIL_KEY` as env variables.

## Docker

Build with:  
`docker build . -t service-health-checker`

Run with:  
`docker run --env-file .env -v ./services.csv:/app/services.csv service-health-checker`
