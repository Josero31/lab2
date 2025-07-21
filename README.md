# Conway's Game of Life - Lab 2

Este proyecto es una implementación en Rust del famoso Juego de la Vida de Conway, con renderizado en tiempo real usando la librería minifb. El juego utiliza un framebuffer lógico de 100x100 celdas y una ventana de 800x600 píxeles, con un estilo visual tipo arcade (células amarillas y fondo azul oscuro).

## Características
- Simulación del Juego de la Vida de Conway.
- Renderizado en tiempo real con minifb.
- Framebuffer lógico escalado a la ventana.
- Patrones iniciales creativos: gliders, guns, blinkers, toads, blocks, loafs, pentadecathlons y más.
- Colores vivos tipo arcade.
- FPS limitado para mejor visualización.

## Requisitos
- Rust (recomendado instalar desde [rustup.rs](https://rustup.rs/))
- Cargo

## Instalación
1. Clona el repositorio:
   ```sh
   git clone <URL_DEL_REPO>
   cd lab2
   ```
2. Instala las dependencias:
   ```sh
   cargo build
   ```

## Ejecución
1. Ejecuta el juego:
   ```sh
   cargo run
   ```
2. Se abrirá una ventana mostrando la simulación. Presiona `ESC` para cerrar.

## Personalización
- Puedes modificar los patrones iniciales en la función `init_pattern` dentro de `src/main.rs`.
- Cambia la resolución lógica o de ventana editando las constantes `FB_WIDTH`, `FB_HEIGHT`, `WIDTH`, `HEIGHT`.
- Los colores se pueden ajustar en la función `get_color` y en el fondo de la función `render`.

## Ejemplo visual
![Ejemplo de simulación](demo.gif)

## Autor
Josero31

---

