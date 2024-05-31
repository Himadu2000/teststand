# Standard Architecture for Web Development

[This project follows OSS guidelines](https://opensource.guide)

## What does this project do?

## Why is this project useful?

## How do I get started?

- Copy src/simple folder
- Rename simple with your project name
- Init a git repo for that folder

## Where can I get more help, if I need it?

Just create an issue. No need to follow a specific format!

Have questions about how to get started? Create an issue!
Want to help a fellow developer? Create an issue!
You get the gist. Create an issue!

If you have a question on how to proceed on certain something,
like "I'm making a library, what rustfmt/clippy/formatting/linting settings I use?"
or "This is my huge project, what folder Structure should I use?"
or "This is my OSS project, there're thousands of issues filed, what to do?"
file an issue, our contributors will help, if needed our technical committee will give a decision.

### Folder Structure

```
.
├── src                      # Application source code
│   ├── book                # Docs
│   │   └── client           # Client side code
│   ├── simple                  # A simple example or your starter template
│   |   ├── src             # Server side code
│   |   |   ├── server             # Server side code
│   |   |   └── website             # Website/Client side code
│   |   └── Cargo.toml             # Your workspace
│   ├── complex                # A complex example for demo purposes
│   |   ├── client             # Client side code
│   |   ├── server             # Server side code
│   |   └── shared             # Universal code
│   └── index.js             # Server application start point
└── process.json             # pm2 process file
```
