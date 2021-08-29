# Simple HTTP Checker
Provide a yaml file for configuration. It must be named `config.yml`

> config.yml
```yml
actions:
  slack: 
    sites:
      - https://members.hackrva.org
      - https://hackrva.org
      - https://wiki.hackrva.org
    hook: ""
```

Site will contain a list of urls that you wish to monitor.  The checker will periodically make a request to the URL.  If it doesn't receive an http 200, it will send a message to a slack hook.
