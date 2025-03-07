# Honeypot Application

A sophisticated honeypot application written in Rust using Actix-web, designed to detect and log potential cyber attacks.

## Features

### Implemented:

- **Fake Login System**
  - Logs all attempted username/password combinations
  - Responds with convincing error messages for failed logins

- **Simulated Vulnerabilities**
  - Endpoints simulating common vulnerabilities (SQL injection, XSS)
  - Logs all exploitation attempts

- **File Upload Trap**
  - Accepts and logs uploaded files without storing them
  - Helps capture potential malware samples

- **Command Injection Simulator**
  - Endpoint simulating command injection vulnerability
  - Logs attempted commands without execution

- **User-Agent Analysis**
  - Logs and analyzes User-Agent strings
  - Helps identify potential attack tools

- **Banner Grabbing Deception**
  - Provides fake server information in response headers
  - Set to mimic PHP 7.4.3 on Microsoft-IIS/10.0

### To Be Implemented:

- Delayed Responses
- Session Tracking
- Geolocation Logging
- Honeytokens
- Dynamic Content Generation
- Rate Limiting with Logging
- Fake Admin Panel
- API Endpoint Mimicry
- SSL/TLS Downgrade Logging
- Path Traversal Detection

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

This honeypot is for educational and research purposes only. Always ensure you have proper authorization before deploying on any network.
