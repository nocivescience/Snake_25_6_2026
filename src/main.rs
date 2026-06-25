use minifb::{Key, Window, WindowOptions};
use rand;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// Configuración de la pantalla
const WIDTH: usize = 900;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 30;
const GRID_WIDTH: i16 = (WIDTH / CELL_SIZE) as i16;
const GRID_HEIGHT: i16 = (HEIGHT / CELL_SIZE) as i16;

#[derive(Clone, Copy, PartialEq)]
enum Direction { Up, Down, Left, Right }

struct Game {
    snake: VecDeque<(i16, i16)>,
    direction: Direction,
    food: (i16, i16),
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((GRID_WIDTH / 2, GRID_HEIGHT / 2));
        
        Game {
            snake,
            direction: Direction::Right,
            food: (5, 5),
            game_over: false,
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let new_food = (rand::random_range(0..GRID_WIDTH), rand::random_range(0..GRID_HEIGHT));
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over { return; }

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        // Colisiones con bordes o consigo misma
        if new_head.0 < 0 || new_head.0 >= GRID_WIDTH || new_head.1 < 0 || new_head.1 >= GRID_HEIGHT || self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }
}

// Dibuja un cuadrado de un color (en formato 0x00RRGGBB) directamente en el buffer de píxeles
fn draw_rect(buffer: &mut Vec<u32>, x: usize, y: usize, size: usize, color: u32) {
    for row in 0..size {
        let screen_y = y + row;
        if screen_y >= HEIGHT { continue; }
        for col in 0..size {
            let screen_x = x + col;
            if screen_x >= WIDTH { continue; }
            buffer[screen_y * WIDTH + screen_x] = color;
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Snake Minimalista - Rust",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| panic!("{}", e));

    // Forzar límite de FPS para el ciclo de eventos de minifb
    window.set_target_fps(60);
    
    let mut game = Game::new();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // El framebuffer de la ventana
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(130); // Velocidad de la serpiente

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Manejo nativo de entradas del teclado sin callbacks engorrosos
        if window.is_key_down(Key::Up) && game.direction != Direction::Down { game.direction = Direction::Up; }
        if window.is_key_down(Key::Down) && game.direction != Direction::Up { game.direction = Direction::Down; }
        if window.is_key_down(Key::Left) && game.direction != Direction::Right { game.direction = Direction::Left; }
        if window.is_key_down(Key::Right) && game.direction != Direction::Left { game.direction = Direction::Right; }

        // Actualizar el estado del juego según el reloj
        if last_tick.elapsed() >= tick_rate {
            game.update();
            last_tick = Instant::now();
        }

        // Si el juego termina, reiniciar automáticamente presionando Enter
        if game.game_over && window.is_key_down(Key::Enter) {
            game = Game::new();
        }

        // --- RENDERIZADO ---
        // Limpiar pantalla a Negro (0x000000)
        buffer.fill(0x000000);

        if !game.game_over {
            // Dibujar comida (Rojo: 0xFF0000)
            draw_rect(&mut buffer, game.food.0 as usize * CELL_SIZE, game.food.1 as usize * CELL_SIZE, CELL_SIZE, 0xFF0000);

            // Dibujar serpiente (Verde: 0x00FF00) con bordes pequeños
            for segment in &game.snake {
                draw_rect(&mut buffer, segment.0 as usize * CELL_SIZE, segment.1 as usize * CELL_SIZE, CELL_SIZE - 2, 0x00FF00);
            }
        } else {
            // Pantalla de Game Over (Fondo Gris oscuro: 0x333333)
            buffer.fill(0x333333);
        }

        // Enviar los píxeles a la ventana de Manjaro
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}