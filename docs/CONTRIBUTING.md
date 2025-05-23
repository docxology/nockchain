# Contributing to Nockchain

Thank you for your interest in contributing to Nockchain, a lightweight blockchain for heavyweight verifiable applications. This guide will help you understand the contribution process and how you can effectively participate in the project's development.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Pull Request Process](#pull-request-process)
5. [Coding Standards](#coding-standards)
6. [Documentation Guidelines](#documentation-guidelines)
7. [Testing Guidelines](#testing-guidelines)
8. [Communication Channels](#communication-channels)

## Code of Conduct

We are committed to providing a friendly, safe, and welcoming environment for all contributors. Please adhere to the following principles:

- Be respectful and inclusive
- Value different viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

To contribute to Nockchain, you'll need:

1. Rust development environment with `rustup` installed (see [https://rustup.rs/](https://rustup.rs/))
2. Hoon compiler (`choo`) installed via `make install-choo`
3. Git for version control
4. Familiarity with blockchain concepts, zero-knowledge proofs, and/or Nock/Hoon (depending on the area you wish to contribute to)

### Setting Up the Development Environment

1. Fork the Nockchain repository on GitHub
2. Clone your fork locally:
   ```
   git clone https://github.com/YOUR_USERNAME/nockchain.git
   cd nockchain
   ```
3. Add the upstream repository as a remote:
   ```
   git remote add upstream https://github.com/tetra/nockchain.git
   ```
4. Build the project:
   ```
   make build-hoon-all
   make build
   ```
5. Run tests to ensure everything is working properly:
   ```
   make test
   ```

## Development Workflow

1. **Choose an Issue**: Start by looking at open issues in the issue tracker and find one that interests you.

2. **Create a Branch**: Create a branch for your work based on the `develop` branch:
   ```
   git checkout develop
   git pull upstream develop
   git checkout -b feature/your-feature-name
   ```
   Use descriptive branch names like `feature/new-validator` or `fix/mempool-race-condition`.

3. **Make Your Changes**: Implement your changes, following the coding standards and ensuring tests are included.

4. **Commit Your Changes**: Make focused, logical commits with clear messages:
   ```
   git commit -m "feat: add new validator mechanism for ZK proofs"
   ```
   Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.

5. **Keep Your Branch Updated**: Regularly rebase your branch on the latest `develop`:
   ```
   git fetch upstream
   git rebase upstream/develop
   ```

6. **Push to Your Fork**:
   ```
   git push origin feature/your-feature-name
   ```

## Pull Request Process

1. **Create a Pull Request**: Open a pull request from your feature branch to the `develop` branch of the main repository.

2. **Describe Your Changes**: Provide a clear description of the changes, referencing any related issues.

3. **Complete the Pull Request Template**: Fill out all sections of the PR template, including:
   - Purpose of the change
   - How it was implemented
   - Testing performed
   - Documentation updates

4. **Request Reviews**: Request reviews from appropriate maintainers.

5. **Address Feedback**: Respond to any feedback and make necessary changes.

6. **Passing Checks**: Ensure all CI checks are passing before requesting a merge.

7. **Merging**: A maintainer will merge your PR once it's approved.

## Coding Standards

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code
- Run `clippy` to catch common mistakes and improve your code
- Maintain backward compatibility whenever possible
- Write clear and concise documentation for all public APIs

### Hoon Code

- Follow established Hoon coding conventions
- Prioritize clarity over cleverness
- Include comments for complex operations
- Organize gate arms in logical groups
- Use conventional rune spacing and formatting

## Documentation Guidelines

Good documentation is critical to the success of Nockchain:

- Update documentation when changing functionality
- Use clear and concise language
- Include examples for complex features
- Add Mermaid diagrams for visualizing architecture and processes
- Link related documentation sections together
- Document public APIs thoroughly
- Keep the README.md file up-to-date

## Testing Guidelines

All code should be thoroughly tested:

- Write unit tests for new functionality
- Include integration tests for system components
- Maintain existing tests when modifying code
- Test edge cases and error conditions
- Document test scenarios and expectations
- Run the full test suite locally before submitting PRs

## Communication Channels

- **GitHub Issues**: For bug reports, feature requests, and task tracking
- **Pull Requests**: For code review and detailed technical discussions
- **Community Forums**: For general discussions and questions

Thank you for contributing to Nockchain! Your efforts help build a robust platform for trustless settlement of heavyweight verifiable computation. 