# 🦀 Servidor HTTP con Rust, Actix Web y Actix Files

Este proyecto demuestra cómo implementar un servidor web simple pero robusto en **Rust** utilizando el *framework* **Actix Web** para el enrutamiento y **Actix Files** para servir contenido estático de forma eficiente, como páginas HTML, hojas de estilo (CSS), imágenes y scripts (JS).

---

## 🚀 Requisitos

Asegúrate de tener instalado lo siguiente:

1.  **Rust y Cargo:** La herramienta de construcción y gestor de paquetes de Rust.
    ```bash
    # Instalar Rust (si no lo tienes):
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
2.  **Un navegador web** o la herramienta `curl` para probar el servidor.

---

## 🛠️ Configuración del Proyecto

### 1. Inicializar el Proyecto

Crea un nuevo proyecto de Rust:

```bash
cargo new rust-actix-static-server
cd rust-actix-static-server
