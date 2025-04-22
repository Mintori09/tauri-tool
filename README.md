# Tauri + Vue 3 + Vite Starter Kit

A modern, production-ready starter kit for building cross-platform desktop applications using Tauri v2, Vue 3, and Vite.

## 🚀 Features

- **Frontend**
  - Vue 3 with Composition API
  - TypeScript support
  - PrimeVue 4 UI Components
  - Vue Router 4 for navigation
  - Pinia 3 for state management
  - SCSS with variables and modular styling
  - ESLint + Prettier for code quality

- **Backend**
  - Tauri v2.2.0
  - Rust
  - File System Access
  - Shell Commands
  - System Tray Support

- **Development**
  - Vite 6 for fast development
  - Hot Module Replacement (HMR)
  - TypeScript configuration
  - ESLint and Prettier configuration
  - SCSS support
  - Modern build tooling

## 📋 Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [pnpm](https://pnpm.io/) (v8 or later)
- [Rust](https://www.rust-lang.org/) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

## 🛠️ Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Mintori09/tauri-starter-kit.git
   cd tauri-starter-kit
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Start the development server:
   ```bash
   pnpm tauri dev
   ```

4. Build the application:
   ```bash
   pnpm tauri build
   ```

## 📁 Project Structure

```
tauri-starter-kit/
├── src/                    # Frontend source code
│   ├── assets/            # Static assets
│   │   └── styles/        # Global styles and variables
│   ├── components/        # Vue components
│   ├── stores/            # Pinia stores
│   ├── views/             # Vue views
│   ├── router/            # Vue Router configuration
│   └── main.ts            # Application entry point
├── src-tauri/             # Tauri backend code
│   ├── src/               # Rust source code
│   └── Cargo.toml         # Rust dependencies
├── public/                # Public assets
└── package.json           # Node.js dependencies
```

## 🚀 Development

### Available Scripts

- `pnpm dev` - Start the development server
- `pnpm build` - Build the application
- `pnpm preview` - Preview the production build
- `pnpm tauri dev` - Start Tauri development
- `pnpm tauri build` - Build Tauri application
- `pnpm lint` - Run ESLint
- `pnpm format` - Format code with Prettier

### Tauri Features

The starter kit includes several Tauri plugins:

- **File System Access**: Read and write files
- **Shell Commands**: Execute system commands
- **System Tray**: Add system tray functionality
- **Native Dialogs**: Show native system dialogs

## 🎨 UI Components

The project uses PrimeVue 4, which provides:

- Modern and responsive UI components
- Theme support
- Accessibility features
- Comprehensive documentation

## 📦 State Management

Pinia 3 is used for state management, providing:

- TypeScript support
- DevTools integration
- Modular stores
- Hot module replacement

## 🔧 Configuration

### TypeScript

The project includes TypeScript configuration for:

- Strict type checking
- Vue 3 support
- Path aliases
- Modern JavaScript features

### ESLint + Prettier

Code quality is ensured through:

- Vue 3 specific rules
- TypeScript integration
- Prettier formatting
- Consistent code style

## 📚 Documentation

- [Vue 3 Documentation](https://v3.vuejs.org/)
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [PrimeVue Documentation](https://primevue.org/documentation)
- [Pinia Documentation](https://pinia.vuejs.org/)
- [Vite Documentation](https://vitejs.dev/)

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri Team](https://tauri.app/)
- [Vue.js Team](https://vuejs.org/)
- [PrimeVue Team](https://primevue.org/)
- [Vite Team](https://vitejs.dev/)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
