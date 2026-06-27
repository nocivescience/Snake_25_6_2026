use macroquad::prelude::*;
use std::collections::VecDeque;

// Configuración de la pantalla
const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 600.0;
const CELL_SIZE: f32 = 30.0;
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
        // Empezamos en el centro
        snake.push_back((GRID_WIDTH / 2, GRID_HEIGHT / 2));
        
        let mut game = Game {
            snake,
            direction: Direction::Right,
            food: (0, 0),
            game_over: false,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        loop {
            // Macroquad tiene su propio módulo rand integrado optimizado para web/nativo
            let new_food = (
                rand::gen_range(0, GRID_WIDTH),
                rand::gen_range(0, GRID_HEIGHT),
            );

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

        // Colisiones con bordes o con su propio cuerpo
        if new_head.0 < 0 || new_head.0 >= GRID_WIDTH || new_head.1 < 0 || new_head.1 >= GRID_HEIGHT || self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Avanzar la cabeza
        self.snake.push_front(new_head);

        // Si come, spawnea nueva comida y no eliminamos la cola (crece)
        if new_head == self.food {
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }
}

fn ventana_config() -> Conf {
    Conf {
        window_title: "Snake Minimalista - Rust (macroquad)".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(ventana_config)]
async fn main() {
    let mut game = Game::new();
    
    // Variables para controlar el tiempo del movimiento (Tick Rate)
    let mut ultimo_tick = get_time();
    let tick_rate = 0.130; // 130 milisegundos en segundos (f64)

    loop {
        // --- 1. ENTRADA DE TECLADO ---
        if is_key_down(KeyCode::Up) && game.direction != Direction::Down { game.direction = Direction::Up; }
        if is_key_down(KeyCode::Down) && game.direction != Direction::Up { game.direction = Direction::Down; }
        if is_key_down(KeyCode::Left) && game.direction != Direction::Right { game.direction = Direction::Left; }
        if is_key_down(KeyCode::Right) && game.direction != Direction::Left { game.direction = Direction::Right; }

        let mut color = GREEN; 

        // Reiniciar juego con Enter si perdiste
        if game.game_over && is_key_down(KeyCode::Enter) {
            game = Game::new();
            ultimo_tick = get_time();
        }

        // --- 2. LÓGICA DEL JUEGO (TICK RATE) ---
        let tiempo_actual = get_time();
        if tiempo_actual - ultimo_tick >= tick_rate {
            game.update();
            ultimo_tick = tiempo_actual;
        }

        // --- 3. RENDERIZADO ---
        if !game.game_over {
            // Fondo Negro
            clear_background(BLACK);

            // Dibujar comida (Rojo)
            draw_rectangle(
                game.food.0 as f32 * CELL_SIZE,
                game.food.1 as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                RED,
            );

            // Dibujar serpiente (Verde) dejando un pequeño margen de 2px
            for segment in &game.snake {
                draw_rectangle(
                    segment.0 as f32 * CELL_SIZE,
                    segment.1 as f32 * CELL_SIZE,
                    CELL_SIZE - 2.0,
                    CELL_SIZE - 2.0,
                    color,
                );
            }
        } else {
            // Pantalla de Game Over: Fondo Gris Oscuro
            clear_background(DARKGRAY);
            
            // Opcional: Dibujar un texto de Game Over en el centro
            draw_text("GAME OVER", WIDTH / 2.0 - 90.0, HEIGHT / 2.0, 40.0, WHITE);
            draw_text("Presiona ENTER para reiniciar", WIDTH / 2.0 - 140.0, HEIGHT / 2.0 + 40.0, 20.0, LIGHTGRAY);
        }

        next_frame().await;
    }
}