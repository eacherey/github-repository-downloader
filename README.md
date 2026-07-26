# Project github-dl

## About the project

A CLI utility for mass cloning all repositories of a GitHub user or organization. It supports unauthenticated downloads (rate-limited to 60 requests/hour) or authenticated requests using a stored token (rate-limited to 5000 requests/hour), with an option to temporarily override the stored token via a CLI flag for that specific run. It also allows specifying a custom destination directory, where each repository is cloned into its own dedicated subdirectory.

## Usage

```sh
github-dl repo eacherey

github-dl repo eacherey --token <your_token>

github-dl repo eacherey --output ./my_backups

github-dl repo --org eacherey_org
```

## License

[MIT](LICENSE)