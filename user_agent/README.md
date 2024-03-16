# User Agent
### user_agent_basic
This makes a request to a specified URL with a specified User_Agent  
Usage : cargo run <user-agent> <url>  

### user_agent_evolved
Creates a proxy on localhost:8080 that adds the specified user_agent on incoming requests  
Usage : cargo run <user-agent>  
Poc : curl -x http://localhost:8080   

### user_agent_legion 
Spams the specified URL with the specified User_Agent  
Usage : cargo run <user-agent> <url>  
