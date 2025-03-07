# hotspot
Honeypot

    DONE: 
    - Fake Login System:

        Implement a fake login page that logs all attempted username/password combinations.

        Respond with convincing error messages for failed logins.
   
    - Simulated Vulnerabilities:

        Create endpoints that appear vulnerable to common attacks (e.g., SQL injection, XSS).

        Log all attempts to exploit these "vulnerabilities".

    
    - File Upload Trap:

        Add a file upload feature that accepts and logs all uploaded files without actually storing them.

        This can help capture potential malware samples.
    
    - Command Injection Simulator:

        Create an endpoint that appears to allow command injection.

        Log all attempted commands without actually executing them.
    
    - User-Agent Analysis:

        Log and analyze User-Agent strings to identify potential attack tools.
    -   Banner Grabbing:

        Provide fake server information in response headers to mislead attackers about your system.
        (set: x-powered-by: PHP/7.4.3
        server: Microsoft-IIS/10.0 )

TODO:

    - Delayed Responses:

        Implement artificial delays in responses to slow down automated scanning tools.


    - Session Tracking:

        Implement session tracking to monitor attacker behavior across multiple requests.

    - Geolocation Logging:

        Log the geographical origin of incoming requests.
    
  
    - Honeytokens:

        Place fake sensitive data (like API keys) in easily discoverable locations and monitor their usage.

    - Dynamic Content Generation:

        Generate fake dynamic content to make the honeypot appear more like a real application.

    - Rate Limiting with Logging:

        Implement rate limiting and log when limits are exceeded, which could indicate automated attacks.

    - Fake Admin Panel:

        Create a fake admin panel that logs all access attempts.

    - API Endpoint Mimicry:

        Mimic common API endpoints (e.g., /api/users) and log interactions with them.

    - SSL/TLS Downgrade Logging:

        Log attempts to downgrade SSL/TLS connections.
    
    - Path traversal