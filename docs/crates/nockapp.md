# NockApp

## Overview

The [`nockapp`](../../crates/nockapp/) crate provides the application framework for building applications on the [Nockchain](../../) platform. It serves as the primary runtime and application container for all Nockchain apps.

## Directory Structure

Located in [`crates/nockapp/`](../../crates/nockapp/).

### Key Subdirectories

- [`apps/`](../../crates/nockapp/apps/): Example and core applications
  - [`choo/`](../../crates/nockapp/apps/choo/): Main application and Hoon compiler
  - [`hoon/`](../../crates/nockapp/apps/hoon/): Hoon integration and runtime support
  - [`http-app/`](../../crates/nockapp/apps/http-app/): HTTP interface for external API access
  - [`test-app/`](../../crates/nockapp/apps/test-app/): Testing application for framework validation
- [`crown/`](../../crates/nockapp/crown/): Runtime environment
  - [`src/drivers/`](../../crates/nockapp/crown/src/drivers/): System drivers for hardware and OS interfaces
  - [`src/kernel/`](../../crates/nockapp/crown/src/kernel/): Kernel interfaces for system-level access
  - [`src/nockapp/`](../../crates/nockapp/crown/src/nockapp/): Application runtime and lifecycle management

## Core Concepts

- **NockApp**: The central application container and runtime
- **Drivers**: System-level interfaces for I/O and hardware access
- **Wire**: Communication channels between applications and components
- **Event Loop**: Processes messages and maintains application state
- **Crown**: The runtime environment that hosts applications

## Components

- Application lifecycle management (initialization, termination, updates)
- State management with persistent storage
- Inter-application communication through a message-passing system
- System resource access with controlled permissions
- Asynchronous event processing

## Technical Features

- **Poke System**: Allows applications to send messages to each other
- **Scry System**: Read-only queries for application state
- **Subscription Model**: Long-lived connections for state updates
- **Driver Framework**: Modular system for hardware/OS interfaces
- **Hot Reload**: Application updates without system restart

## Usage

The `nockapp` crate is used to:

- Define application structure and interfaces through a consistent API
- Manage application state with automatic persistence
- Interface with the blockchain for consensus and verification
- Access system resources in a controlled, secure manner
- Communicate between applications through typed messages

## Development

To develop applications with NockApp:

1. Define an application using the NockApp framework
2. Implement handlers for system events and messages
3. Manage state using the provided persistence APIs
4. Use drivers for system resource access
5. Package the application for deployment

## Related Crates

- [sword](./sword.md): Nock interpreter and runtime
- [kernels](./kernels.md): Kernel implementations 