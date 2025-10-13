use std::{f32::consts::PI};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::{self, Point, Rect}, render::Canvas, video::Window};

// constants and Map
const WW: u32 = 512*2;
const WH: u32 = 512;

const MAP_X: usize = 8;
const MAP_Y: usize = 8;
const MAP_S: i32 = 64; // cube size

const MAP: [i32; MAP_X * MAP_Y] = [
    1,1,1,1,1,1,1,1,
    1,0,1,0,0,0,0,1,
    1,0,1,0,0,0,0,1,
    1,0,1,0,0,0,0,1,
    1,0,0,0,0,0,0,1,
    1,0,0,0,0,1,0,1,
    1,0,0,0,0,0,0,1,
    1,1,1,1,1,1,1,1,
];

const TWOPI: f32 = 2. * PI;
const DR: f32 = 0.0174533;
fn deg_to_rad(deg: f32) -> f32 {
    (deg * PI) / 180.0
}


struct Player {
    px: f32,
    py: f32, 
    pa: f32,
    pdx: f32,
    pdy: f32
}

impl Player {
    fn new(px: f32, py: f32, pa: f32) -> Self {
        let mut player = Player {px, py, pa, pdx: 0., pdy: 0.};
        player.update_delta();
        player
    }

    fn update_delta(&mut self) {
        let angle_rad = deg_to_rad(self.pa);
        self.pdx = angle_rad.cos();
        self.pdy = -angle_rad.sin();
    }

    fn button_handle(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::W =>{
                self.px += self.pdx * 5.;
                self.py += self.pdy * 5.;
            }, 
            Keycode::S =>{
                self.px -= self.pdx * 5.;
                self.py -= self.pdy * 5.;
            }, 
            Keycode::A => {
                self.pa += 5.;
                self.fix_angle();
                self.update_delta();
            },
            Keycode::D => {
                self.pa -= 5.;
                self.fix_angle();
                self.update_delta();
            }
            _ => {}
        }
    }
    fn fix_angle(&mut self) {
        while self.pa > 359. {
            self.pa -= 360.;
        }
        while self.pa < 0. {
            self.pa += 360.;
        }
    }
}

fn draw_map(canvas: &mut Canvas<Window>) -> Result<(), String> {
    for y in 0..MAP_Y {
        for x in 0..MAP_X {
            let i = y * MAP_X + x;
            if MAP[i] == 1 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }

            let xo = (x as i32) * MAP_S;
            let yo = (y as i32) * MAP_S;

            // Draw a rectangle
            canvas.fill_rect(rect::Rect::new(
                xo + 1, 
                yo + 1, 
                (MAP_S - 2) as u32, 
                (MAP_S - 2) as u32
            ))?;
        }
    }
    Ok(())
}

fn draw_player_2d(canvas: &mut Canvas<Window>, player: &Player) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 0)); // Yellow
    // no PointSize in SDL
    let p_x = player.px as i32;
    let p_y = player.py as i32;
    let point_rect = sdl2::rect::Rect::new(p_x - 4, p_y  - 4, 8, 8);
    canvas.fill_rect(point_rect)?;

    // line
    canvas.draw_line(
        Point::new(p_x, p_y), 
        Point::new((player.px + player.pdx * 20.) as i32, (player.py + player.pdy * 20.) as i32)
    )?;

    Ok(())
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, ang: f32) -> f32 {
    ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt()

}

fn draw_rays_3d(canvas: &mut Canvas<Window>, player: &Player) -> Result<(), String> {
    // raycasting logic
    let mut mx: i32;
    let mut my: i32;
    let mut mp: i32;
    let mut dof: i32;
    let mut rx: f32 = 0.;
    let mut ry: f32 = 0.;
    let mut ra: f32 = deg_to_rad(player.pa);
    let mut xo: f32 = 64.;
    let mut yo: f32 = 64.;
    let mut disV: f32 = 10000000000.;
    let mut disH: f32 = 10000000000.;
    let mut disT: f32 = 0.;
    let mut hx = 0.;
    let mut hy = 0.;
    let mut vx = 0.;
    let mut vy = 0.;

    ra += DR * 30.;
    if ra < 0. { ra += TWOPI;}
    if ra > TWOPI { ra -= TWOPI;}
    for r in 0..60 {
        disV = 10000000000.;
        disH = 10000000000.;
        dof = 0;
        let tan = ra.tan();
        let sin = ra.sin();
        let cos = ra.cos();
        // Check vertical
        // Left 
        if  cos < -0.001 {
            rx = (((player.px as i32) >> 6) << 6) as f32 - 0.0001;
            ry = (player.px - rx)*tan + player.py;
            xo = -64.;
            yo = -xo * tan;
        }
        // Right *right*
        else if cos > 0.001  {
            rx = (((player.px as i32) >> 6) << 6) as f32 + 64.;
            ry = (player.px - rx)*tan + player.py;
            xo = 64.;
            yo = -xo * tan;
        } else {
            rx = player.px;
            ry = player.py;
            dof = 8;
        }
        while dof < 8 {
            mx = rx as i32 >> 6;
            my = ry as i32 >> 6;
            mp = my*(MAP_X as i32) + mx;
            if mp > 0 && mp < MAP_X as i32 * MAP_Y as i32 && MAP[mp as usize] == 1 {
                dof = 8; // hit a wall
                vx = rx;
                vy = ry;
                disV = dist(player.px, player.py, vx, vy, ra);
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }

        vx = rx;
        vy = ry;

        // check horizontal
        // if ray is looking up
        let tan = 1.0 / tan;
        dof = 0;
        if(sin) > 0.001{
            ry = ((player.py as i32 >> 6) << 6) as f32 - 0.0001;
            rx = (player.py-ry)*tan + player.px;
            yo = -64.;
            xo = -yo*tan;
        }
        // sin is down
        else if(sin) < -0.001{
            ry = ((player.py as i32 >> 6) << 6) as f32 + 64.;
            rx = (player.py-ry)*tan + player.px;
            yo = 64.;
            xo = -yo*tan;
        } else {
            rx = player.px;
            ry = player.py;
            dof = 8;
        }

        while dof < 8{
            mx = (rx as i32) >> 6;
            my = (ry as i32) >> 6;
            mp = my*(MAP_X as i32) + mx;
            if mp > 0 && mp < MAP_X as i32 * MAP_Y as i32 && MAP[mp as usize] == 1 { // wall is hit
                dof = 8;
                hx = rx;
                hy = ry;
                disH = dist(player.px, player.py, hx, hy, ra);
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
        }
        if disV < disH {
            rx = vx;
            ry = vy;
            disT = disV;
        } else {
            disT = disH;
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(
            Point::new(player.px as i32, player.py as i32), 
            Point::new(rx as i32, ry as i32), 
        )?;
        let mut lineH = (MAP_S * 320) as f32 / disT;
        if lineH > 320. {lineH = 320.;}
        // canvas.draw_line(Point::new(r*8+530, 0), Point::new(r*8+530, lineH as i32))?;
        canvas.draw_rect(Rect::new(r*8 + 530, lineH as i32, 8, lineH as u32))?;
        ra -= DR;
        if ra < 0. { ra += TWOPI}
        if ra > TWOPI { ra -= TWOPI}
    }


    Ok(())
}

fn main() -> Result<(), String> {
    // 1. Init SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // 2. Window & Canvas
    let window = video_subsystem.window("Raycaster", WW, WH)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync()
        .build().map_err(|e| e.to_string())?;

    // 3. Player & event
    let mut player = Player::new(150., 400., 90.);

    let mut event_pump = sdl_context.event_pump()?;

    // Main loop
    'running: loop {
        // Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    player.button_handle(key);
                }
                _ => {}
            }
        }

        // Drawing
        canvas.set_draw_color(Color::RGB(77, 77, 77));
        canvas.clear();

        draw_map(&mut canvas)?;
        draw_rays_3d(&mut canvas, &player)?;
        draw_player_2d(&mut canvas, &player)?;

        // Present
        canvas.present();
    }

    Ok(())
}
