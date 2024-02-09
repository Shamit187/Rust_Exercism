# Exercism Rust Track Readme

Welcome to the Exercism Rust track! This README.md file will guide you through getting started with solving exercises on Exercism using Rust.

## Installing Rust Locally

To solve exercises locally, you'll need to have Rust installed on your machine. Follow the [official Rust installation instructions](https://www.rust-lang.org/tools/install) for your platform.

IDE support can significantly enhance your productivity. If you're using Visual Studio Code, check out its documentation for Rust support. We also recommend enabling linting with Clippy for better code quality.

## Learning Rust

If you're new to Rust, here are some resources to help you get started:

- [The Rust Programming Language](https://doc.rust-lang.org/book/) is an excellent resource for beginners and advanced users alike.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) provides practical examples of Rust concepts.
- [Into_rust()](https://www.youtube.com/c/ChasingLogic) offers screencasts for learning Rust.
- [Rustlings](https://github.com/rust-lang/rustlings) provides small exercises to familiarize yourself with Rust syntax and concepts.
- The [Rust User Forum](https://users.rust-lang.org/) is a great place to ask questions and learn from others.

## Getting Started with Exercism

Here's how to start solving exercises on Exercism:

1. Choose a programming language track from [the tracks page](https://exercism.org/tracks).
2. Join the track of your choice and start solving exercises.
3. Complete exercises to unlock more challenges and improve your fluency in the language.
4. Request help from mentors if you get stuck on an exercise.

## Mentoring Students

Interested in mentoring others? Mentoring is a rewarding way to reinforce your own learning while helping others improve their skills. Check out the [mentoring page](https://exercism.org/become-a-mentor) for instructions on becoming a mentor.

## Building Exercism

Exercism is an open-source project, and we welcome contributions from all backgrounds. If you'd like to help build Exercism, check out the [contributing page](https://github.com/exercism/docs/blob/main/CONTRIBUTING.md) for tasks you can work on.

---
# Working Locally

Here's how to solve exercises on your local machine:

1. Install the Exercism Command Line Interface (CLI) by following [these instructions](https://exercism.org/cli-walkthrough).
2. Configure the CLI with your API token using `exercism configure --token=<your-api-token>`.
3. Download exercises to your local machine using `exercism download --exercise=<exercise-slug> --track=<track-slug>`.
4. Solve the exercise locally, following the instructions in the `README.md` file.
5. Submit your solution using `exercism submit <implementation_file_paths>`.

For troubleshooting and more CLI functionality, refer to the relevant sections in the exercise README files or run `exercism help`.

If you encounter any issues, feel free to open an issue on the [exercism/cli](https://github.com/exercism/cli) repository or seek help from the community. Make sure to check the [Interactive Walkthrough](https://exercism.org/tracks/rust/troubleshooting) before opening any issues.

## Install Exercism

### Download the Archive

If you know your processor architecture (x86 (32-bit) vs x86_64 (64-bit) vs ARM), download the appropriate archive from the [releases page](https://github.com/exercism/cli/releases).

### Extract from the Archive

Once you have the archive downloaded you need to extract the executable from it.

- If you’re using a graphic interface (e.g., GNOME or Unity on Ubuntu): go to your downloads directory, right-click on the downloaded file, and select Extract Here.

- If you’re using the command line (use the right archive file name for your architecture):

    ```bash
    tar -xf exercism-linux-64bit.tgz
    ```

### Moving the Executable to ~/bin

Once you download and extract the archive, make it available in your $PATH.

First, let’s make sure the directory exists:

```bash
mkdir -p ~/bin
```

Next, let’s move the exercism executable there:

```bash
mv exercism ~/bin
```

Make sure everything worked:

```bash
~/bin/exercism
```

The above should output something like the below:

```
A command-line interface for the v3 redesign of Exercism.

Download exercises and submit your solutions.

Usage:
   [command]
```

### Adding ~/bin to $PATH in Bash

Note: If you’re not running Bash try to adjust the below to your shell or Talk to a Volunteer.

To have the exercism executable available everywhere on the command line you need to make sure ~/bin is in your $PATH.

There is a chance it’s there already; let’s see whether it is:

```bash
[[ ":$PATH:" == *":$HOME/bin:"* || ":$PATH:" == *":~/bin:"* ]] && echo "~/bin is in PATH" || echo "~/bin is not in PATH"
```

If the above prints ~/bin is not in PATH let’s add ~/bin to $PATH and reload Bash configuration:

```bash
echo 'export PATH=~/bin:$PATH' >> ~/.bash_profile
source ~/.bash_profile
```

To check whether this worked, try to run exercism without providing the path:

```bash
exercism
```

The above should output something like the below:

```
A command-line interface for the v3 redesign of Exercism.

Download exercises and submit your solutions.

Usage:
   [command]
```

### Configuring the CLI

In order to configure the CLI, paste in the following text into your terminal:

```bash
exercism configure --token=<token_here>
```

You can find your token on your settings page.

After typing in the command, hit the Enter key.

After hitting the Enter key, you should see a notification from the CLI that a configuration file has been written.

**Important:** The token above should be treated like a password and not be shared with anyone!

When asking for help that involves a command, remove your token before sharing the command.

Here is the entire instruction: [Exercism CLI Walkthrough](https://exercism.org/cli-walkthrough)
