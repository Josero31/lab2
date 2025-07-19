use minifb::{Key, Window, WindowOptions};


const WIDTH: usize = 800;
const HEIGHT: usize = 600;
// Tamaño del framebuffer lógico (juego)
const FB_WIDTH: usize = 100;
const FB_HEIGHT: usize = 100;


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    // Framebuffer lógico para el juego (celdas vivas/muertas)
    let mut fb = vec![false; FB_WIDTH * FB_HEIGHT];
    let mut next_fb = vec![false; FB_WIDTH * FB_HEIGHT];

    // Inicializar patrón creativo
    init_pattern(&mut fb);

    let mut window = Window::new(
        "Conway's Game of Life - Lab 2",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    // Limitar a 10 FPS para mejor visualización
    window.set_target_fps(10);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // 1. Renderizar el estado actual del juego en el framebuffer de la ventana
        render(&mut buffer, &fb);

        // 2. Actualizar el estado del juego
        update_life(&fb, &mut next_fb);
        std::mem::swap(&mut fb, &mut next_fb);

        // 3. Mostrar en pantalla
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}


// Dibuja un punto (celda) en el framebuffer de la ventana, escalando desde el lógico
fn point(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    // Escalado para que el juego se vea grande
    let scale_x = WIDTH / FB_WIDTH;
    let scale_y = HEIGHT / FB_HEIGHT;
    for dy in 0..scale_y {
        for dx in 0..scale_x {
            let px = x * scale_x + dx;
            let py = y * scale_y + dy;
            if px < WIDTH && py < HEIGHT {
                buffer[py * WIDTH + px] = color;
            }
        }
    }
}

// Devuelve el color de una celda según su estado
fn get_color(alive: bool) -> u32 {
    if alive {
        0xFFFFFFFF // Blanco
    } else {
        0xFF000000 // Negro
    }
}


// Renderiza el estado del juego en el framebuffer de la ventana
fn render(buffer: &mut [u32], fb: &[bool]) {
    // Fondo azul oscuro
    for pixel in buffer.iter_mut() {
        *pixel = 0xFF323264;
    }
    for y in 0..FB_HEIGHT {
        for x in 0..FB_WIDTH {
            let idx = y * FB_WIDTH + x;
            let color = get_color(fb[idx]);
            point(buffer, x, y, color);
        }
    }
}


// Actualiza el estado del juego de la vida
fn update_life(current: &[bool], next: &mut [bool]) {
    for y in 0..FB_HEIGHT {
        for x in 0..FB_WIDTH {
            let idx = y * FB_WIDTH + x;
            let alive = current[idx];
            let mut neighbors = 0;
            // Contar vecinos vivos (con bordes tipo "loop")
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dx == 0 && dy == 0 { continue; }
                    let nx = (x as isize + dx + FB_WIDTH as isize) % FB_WIDTH as isize;
                    let ny = (y as isize + dy + FB_HEIGHT as isize) % FB_HEIGHT as isize;
                    if current[ny as usize * FB_WIDTH + nx as usize] {
                        neighbors += 1;
                    }
                }
            }
            // Reglas de Conway
            next[idx] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true, // Sobrevive
                (false, 3) => true,            // Nace
                _ => false,                    // Muere
            };
        }
    }
}

// Inicializa un patrón creativo con varios organismos clásicos
fn init_pattern(fb: &mut [bool]) {
    // Glider invertido (espejo)
    let glider2 = [ (1,0), (0,1), (2,1), (0,2), (1,2) ];
    for (dx,dy) in glider2 {
        let x = 70 + dx;
        let y = 5 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Glider diagonal abajo-izquierda
    let glider3 = [ (0,0), (1,1), (2,1), (0,2), (1,2) ];
    for (dx,dy) in glider3 {
        let x = 5 + dx;
        let y = 70 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // LWSS invertido
    let lwss2 = [ (0,0),(1,0),(2,0),(3,0),(4,1),(0,2),(4,2),(1,3),(2,3),(3,3) ];
    for (dx,dy) in lwss2 {
        let x = 60 + dx;
        let y = 60 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Gosper Glider Gun (parcial, solo la parte central para no saturar)
    let gun = [
        (1,5),(1,6),(2,5),(2,6),
        (13,3),(14,3),(12,4),(16,4),(11,5),(17,5),(11,6),(15,6),(17,6),(18,6),(11,7),(17,7),(12,8),(16,8),(13,9),(14,9),
        (25,1),(23,2),(25,2),(21,3),(22,3),(21,4),(22,4),(21,5),(22,5),(23,6),(25,6),(25,7),
        (35,3),(36,3),(35,4),(36,4)
    ];
    for (dx,dy) in gun {
        let x = 10 + dx;
        let y = 10 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Más bloques y osciladores distribuidos
    let block2 = [ (0,0),(1,0),(0,1),(1,1) ];
    for (dx,dy) in block2 {
        let x = 95 + dx;
        let y = 95 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }
    for (dx,dy) in block2 {
        let x = 50 + dx;
        let y = 90 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }
    // Oscilador "toad" extra
    let toad2 = [ (1,0),(2,0),(3,0),(0,1),(1,1),(2,1) ];
    for (dx,dy) in toad2 {
        let x = 80 + dx;
        let y = 50 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }
    // Oscilador "blinker" extra
    let blinker2 = [ (0,0),(1,0),(2,0) ];
    for (dx,dy) in blinker2 {
        let x = 90 + dx;
        let y = 10 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }
    // Pentadecathlon extra
    let pentadecathlon2 = [ (2,0),(2,1),(2,2),(1,3),(3,3),(2,4),(2,5),(2,6),(1,7),(3,7),(2,8),(2,9) ];
    for (dx,dy) in pentadecathlon2 {
        let x = 70 + dx;
        let y = 70 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }
    // Glider
    let glider = [ (1,0), (2,1), (0,2), (1,2), (2,2) ];
    for (dx,dy) in glider {
        let x = 1 + dx;
        let y = 1 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // LWSS (Light-weight spaceship)
    let lwss = [ (1,0),(4,0),(0,1),(0,2),(4,2),(0,3),(1,3),(2,3),(3,3),(4,3) ];
    for (dx,dy) in lwss {
        let x = 20 + dx;
        let y = 20 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Pulsar (period 3)
    let pulsar = [
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12)
    ];
    for (dx,dy) in pulsar {
        let x = 50 + dx;
        let y = 50 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Beacon
    let beacon = [ (0,0),(1,0),(0,1),(1,1),(2,2),(3,2),(2,3),(3,3) ];
    for (dx,dy) in beacon {
        let x = 80 + dx;
        let y = 80 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Blinker
    let blinker = [ (0,0),(1,0),(2,0) ];
    for (dx,dy) in blinker {
        let x = 10 + dx;
        let y = 40 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Toad
    let toad = [ (1,0),(2,0),(3,0),(0,1),(1,1),(2,1) ];
    for (dx,dy) in toad {
        let x = 30 + dx;
        let y = 60 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Block
    let block = [ (0,0),(1,0),(0,1),(1,1) ];
    for (dx,dy) in block {
        let x = 60 + dx;
        let y = 10 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Tub
    let tub = [ (1,0),(0,1),(2,1),(1,2) ];
    for (dx,dy) in tub {
        let x = 90 + dx;
        let y = 90 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Boat
    let boat = [ (0,0),(1,0),(0,1),(2,1),(1,2) ];
    for (dx,dy) in boat {
        let x = 80 + dx;
        let y = 20 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Loaf
    let loaf = [ (1,0),(2,0),(0,1),(3,1),(1,2),(3,2),(2,3) ];
    for (dx,dy) in loaf {
        let x = 10 + dx;
        let y = 80 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }

    // Pentadecathlon
    let pentadecathlon = [ (2,0),(2,1),(2,2),(1,3),(3,3),(2,4),(2,5),(2,6),(1,7),(3,7),(2,8),(2,9) ];
    for (dx,dy) in pentadecathlon {
        let x = 40 + dx;
        let y = 40 + dy;
        if x < FB_WIDTH && y < FB_HEIGHT {
            fb[y * FB_WIDTH + x] = true;
        }
    }
}

