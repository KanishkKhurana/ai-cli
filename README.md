# User Documentation
### Overview
This CLI tool helps automate interactions with the Fleek platform and AI-based code generation. The tool allows users to:

- Log in to Fleek.
- Create and deploy serverless functions on Fleek.
- Redeploy existing serverless functions.
- Generate JavaScript functions based on prompts  using an AI API.

#### please ensure you have installed the [Fleek CLI](https://www.npmjs.com/package/@fleek-platform/cli) before running this project

### Installation
1. Clone the Repository:
```
git clone https://github.com/KanishkKhurana/ai-cli.git
cd ai-cli
```
2. Build the Project:

Ensure you have Rust installed. You can install Rust from [rust-lang.org](https://rust-lang.org).
```
cargo build --release
```

3. Run the CLI:

After building, you can start the CLI flow:
```
cargo run -- login
```


### Project Structure
- ./src/main.rs: This is the entry point of the application. It parses the CLI arguments and triggers the appropriate functions.

- Functions:

  -   create_file(content: &str): Saves the generated JavaScript code to ./src/myfile.js.

  -   fleek_login(): Executes the fleek login command.
  -   get_ai_resp(prompt: String, key: &str) -> std::result::Result<(), Error>: Sends a request to the AI API with the provided prompt and API key, then saves the generated JavaScript code.
  -   create_and_deploy(name: &str, path: &str): Creates and deploys a serverless function using the fleek CLI.
  -   redeploy(name: &str, path: &str): Redeploys an existing serverless function using the fleek CLI.


### Commands

####  Login to Fleek:

Use the following command to log in to Fleek:

```
cargo run -- login
```

#### Create and Deploy a Function:

To create and deploy a serverless function on Fleek, use:
```
cargo run --  create_function
```
You will be prompted to enter the function name and the path to the function's code.

#### Redeploy an Existing Function:

To redeploy an existing function:
```
cargo run -- redeploy
```
You will be prompted to enter the function name and the path to the function's code.

#### Generate a JavaScript Function using AI:

To generate a JavaScript function based on a prompt:
```
cargo run -- create
```
You will be prompted to enter the prompt and your API key.
The generated JavaScript function will be saved in ./src/myfile.js.

### Error Handling
If the CLI encounters an error, it will output a relevant message. Ensure that all inputs are correct and try again.



### Dependencies
1. clap: Used for parsing command-line arguments.
2. reqwest: Used for making HTTP requests to the AI API.
3. serde_json: Used for handling JSON data.
4. tokio: Used for asynchronous execution.

### Adding New Features
- New Commands: To add a new command, update the Args struct and the match statement in main().
- New Functions: Define new functions in main.rs and call them from the match block based on the parsed command.

