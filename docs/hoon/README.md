# Hoon Components

This directory contains technical documentation for the [Hoon](../../hoon/) components of [Nockchain](../../).

## What is Hoon?

Hoon is a functional programming language specifically designed for the Urbit ecosystem. In Nockchain, Hoon is used to implement core components that run on the Nock virtual machine, providing a higher-level programming environment above the Nock abstraction layer.

## Structure

- [Apps](./apps.md): Documentation for [Hoon applications](../../hoon/apps/)
- [Common](./common.md): Documentation for [common Hoon libraries and utilities](../../hoon/common/)
- [Test Jams](./test-jams.md): Documentation for [test jam files](../../hoon/test-jams/) for testing Hoon components

## The Role of Hoon in Nockchain

Hoon serves several critical functions in the Nockchain ecosystem:

1. **Application Layer**: Hoon applications provide user-facing functionality
2. **Core Components**: Key blockchain components are implemented in Hoon
3. **Code Generation**: Hoon is used for generating optimized Nock code
4. **Standard Library**: Provides reusable utilities and abstractions
5. **Zero-Knowledge Integration**: Connects to zero-knowledge proof systems

## Relationship with Nock

Hoon compiles to Nock, which is the low-level functional computation model that forms the foundation of Nockchain. The Sword crate provides the runtime for executing this Nock code. 