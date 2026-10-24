//! trogue showcase demo — everything drawn with the public API:
//! 3D wireframe objects (cube + octahedron) projected by hand and drawn
//! with Line, orbiting triangles, letter-shaped particles, pulsing circles
//! and a HUD built from DividerComponent / TextComponent.
//!
//! run: cargo run --example demo

use std::f32::consts::PI;

use trogue::backend::{Backend, TetraBackend};
use trogue::trogue_app::TrogueApp;
use trogue::component::*;
use trogue::shape::*;
use trogue::buffer::Buffer;

use tetra::graphics::{self, Color};
use tetra::math::{Mat4, Vec3};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

const W: i32 = 75;
const H: i32 = 60;

// ---------------------------------------------------------------- 3D math --

#[derive(Clone, Copy)]
struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

impl V3 {
    fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }
    fn rotated(&self, yaw: f32, pitch: f32) -> V3 {
        // yaw around Y, then pitch around X
        let (cy, sy) = (yaw.cos(), yaw.sin());
        let (cp, sp) = (pitch.cos(), pitch.sin());
        let x1 = self.x * cy + self.z * sy;
        let z1 = -self.x * sy + self.z * cy;
        let y1 = self.y * cp - z1 * sp;
        let z2 = self.y * sp + z1 * cp;
        V3::new(x1, y1, z2)
    }
    fn project(&self, scale: f32, cx: i32, cy: i32) -> (i32, i32) {
        // weak perspective: bigger z = farther
        let persp = 1.0 / (1.0 + self.z * 0.06);
        (
            cx + (self.x * scale * persp) as i32,
            cy + (self.y * scale * persp) as i32,
        )
    }
}

struct WireObj {
    verts: Vec<V3>,
    edges: Vec<(usize, usize)>,
    glyph: char,
    color: Color,
    center: V3,
    scale: f32,
    speed: f32,
    orbit_radius: f32,
    cy_off: i32,
}

impl WireObj {
    fn draw(&self, t: f32, app_buf: &mut Buffer) {
        let yaw = t * self.speed;
        let pitch = t * self.speed * 0.6;
        let cx = (W / 2) as i32;
        let cy = (H / 2 - 6 + self.cy_off) as i32;
        let (ocx, ocy) = if self.orbit_radius > 0.0 {
            let a = t * self.speed * 1.7;
            (cx + (self.orbit_radius * a.cos()) as i32, cy + (self.orbit_radius * 0.5 * a.sin()) as i32)
        } else {
            (cx, cy)
        };
        let pts: Vec<(i32, i32)> = self
            .verts
            .iter()
            .map(|v| {
                let rv = v.rotated(yaw, pitch);
                let r = V3::new(rv.x + self.center.x, rv.y + self.center.y, rv.z + self.center.z);
                r.project(self.scale, ocx, ocy)
            })
            .collect();
        for (a, b) in &self.edges {
            let (ax, ay) = pts[*a];
            let (bx, by) = pts[*b];
            app_buf.g_draw(
                Line::new(ax, ay, bx, by),
                self.glyph,
                self.color,
            );
        }
    }
}

fn cube() -> WireObj {
    let v = [
        V3::new(-1.0, -1.0, -1.0), V3::new(1.0, -1.0, -1.0),
        V3::new(1.0, 1.0, -1.0),   V3::new(-1.0, 1.0, -1.0),
        V3::new(-1.0, -1.0, 1.0),  V3::new(1.0, -1.0, 1.0),
        V3::new(1.0, 1.0, 1.0),    V3::new(-1.0, 1.0, 1.0),
    ];
    WireObj {
        verts: v.to_vec(),
        edges: vec![
            (0, 1), (1, 2), (2, 3), (3, 0), // back face
            (4, 5), (5, 6), (6, 7), (7, 4), // front face
            (0, 4), (1, 5), (2, 6), (3, 7), // connectors
        ],
        glyph: '#',
        color: Color::rgb(0.35, 0.8, 1.0),
        center: V3::new(0.0, 0.0, 0.0),
        scale: 10.0,
        speed: 0.8,
        orbit_radius: 0.0,
        cy_off: 0,
    }
}

fn cube_small() -> WireObj {
    let v = [
        V3::new(-1.0, -1.0, -1.0), V3::new(1.0, -1.0, -1.0),
        V3::new(1.0, 1.0, -1.0),   V3::new(-1.0, 1.0, -1.0),
        V3::new(-1.0, -1.0, 1.0),  V3::new(1.0, -1.0, 1.0),
        V3::new(1.0, 1.0, 1.0),    V3::new(-1.0, 1.0, 1.0),
    ];
    WireObj {
        verts: v.to_vec(),
        edges: vec![
            (0, 1), (1, 2), (2, 3), (3, 0),
            (4, 5), (5, 6), (6, 7), (7, 4),
            (0, 4), (1, 5), (2, 6), (3, 7),
        ],
        glyph: 'ù',
        color: Color::rgb(0.95, 0.85, 0.2),
        center: V3::new(0.0, 0.0, 0.0),
        scale: 2.5,
        speed: 1.4,
        orbit_radius: 0.0,
        cy_off: -14,
    }
}

fn octahedron() -> WireObj {
    let v = [
        V3::new(0.0, -1.3, 0.0), V3::new(0.0, 1.3, 0.0),
        V3::new(1.0, 0.0, 0.0),  V3::new(-1.0, 0.0, 0.0),
        V3::new(0.0, 0.0, 1.0),  V3::new(0.0, 0.0, -1.0),
    ];
    WireObj {
        verts: v.to_vec(),
        edges: vec![
            (0, 2), (0, 3), (0, 4), (0, 5),
            (1, 2), (1, 3), (1, 4), (1, 5),
            (2, 4), (4, 3), (3, 5), (5, 2), // equator
        ],
        glyph: '%',
        color: Color::rgb(1.0, 0.6, 0.2),
        center: V3::new(0.0, 0.0, 0.0),
        scale: 9.0,
        speed: -0.5,
        orbit_radius: 24.0,
        cy_off: 0,
    }
}

// letter-shaped particle cannon: each particle is a character that orbits
// the center on its own ellipse while slowly falling
struct LetterParticle {
    glyph: char,
    color: Color,
    angle: f32,
    radius: f32,
    speed: f32,
    phase: f32,
}

fn spawn_particles(t: f32) -> Vec<LetterParticle> {
    const LETTERS: [char; 6] = ['t', 'r', 'o', 'g', 'u', 'e'];
    let mut out = Vec::new();
    for i in 0..48 {
        let angle = t * 0.9 + i as f32 * (PI * 2.0 / 48.0);
        out.push(LetterParticle {
            glyph: LETTERS[i % LETTERS.len()],
            color: if i % 3 == 0 {
                Color::rgb(0.55, 1.0, 0.6)
            } else if i % 3 == 1 {
                Color::rgb(0.35, 0.8, 1.0)
            } else {
                Color::rgb(0.9, 0.4, 0.9)
            },
            angle,
            radius: 14.0 + 6.0 * (t * 0.7 + i as f32 * 0.4).sin(),
            speed: 0.0,
            phase: i as f32 * 0.35,
        });
    }
    out
}

// ------------------------------------------------------------------ state --

struct GameState {
    backend: TetraBackend,
    app: TrogueApp,
    t: f32,
    title_scene: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let font = tetra::graphics::Texture::new(ctx, "./examples/resources/terminal.png")?;
        let backend = TetraBackend::new(font);
        Ok(GameState {
            backend,
            app: TrogueApp::new(W, H),
            t: 0.0,
            title_scene: std::env::args().any(|a| a == "title"),
        })
    }

    fn draw_scene(&mut self) {
        if self.title_scene {
            self.draw_title();
        } else {
            self.draw_main();
        }
    }
    // showcase scene for the README screenshot: big legible TROGUE banner
    fn draw_title(&mut self) {
        let t = self.t;
        let buf = self.app.buf();
        let cyan = Color::rgb(0.35, 0.8, 1.0);
        let orange = Color::rgb(1.0, 0.6, 0.2);
        let dim = Color::rgb(0.30, 0.30, 0.42);

        // double frame
        buf.g_draw(Rect::new(0, 0, W - 1, H - 1, false), 'Í', dim);

        // block-art TROGUE, 5 rows per letter, 1 space between letters
        const T: [&str; 5] = [
            "█████",
            "  █  ",
            "  █  ",
            "  █  ",
            "  █  ",
        ];
        const R: [&str; 5] = [
            "████ ",
            "█   █",
            "████ ",
            "█  █ ",
            "█   █",
        ];
        const O: [&str; 5] = [
            " ███ ",
            "█   █",
            "█   █",
            "█   █",
            " ███ ",
        ];
        const G: [&str; 5] = [
            " ████",
            "█    ",
            "█  ██",
            "█   █",
            " ███ ",
        ];
        const U: [&str; 5] = [
            "█   █",
            "█   █",
            "█   █",
            "█   █",
            " ███ ",
        ];
        const E: [&str; 5] = [
            "█████",
            "█    ",
            "████ ",
            "█    ",
            "█████",
        ];
        let letters = [&T, &R, &O, &G, &U, &E];
        // each letter 5 wide + 1 gap = 6 * 6 - 1 = 35 columns
        let mut lx = (W / 2 - 35 / 2) as i32;
        let ly = (H / 2 - 8) as i32;
        for (li, letter) in letters.iter().enumerate() {
            for (ry, row) in letter.iter().enumerate() {
                for (rx, ch) in row.chars().enumerate() {
                    if ch == '█' {
                        // shimmer: columns light up in waves
                        let wave = ((t * 2.0 + (lx + rx as i32) as f32 * 0.35).sin() + 1.0) / 2.0;
                        let color = if li % 2 == 0 { cyan } else { orange };
                        let c = Color::rgb(
                            color.r * (0.6 + 0.4 * wave),
                            color.g * (0.6 + 0.4 * wave),
                            color.b * (0.6 + 0.4 * wave),
                        );
                        buf.set_char(lx + rx as i32, ly + ry as i32, '\u{00db}', c);
                    }
                }
            }
            lx += 6;
        }

        // small rotating wireframe cube orbiting above the wordmark
        cube_small().draw(t, buf);

        // subtitle
        let mut sub = TextComponent::new(Component::new(W / 2, ly + 7));
        sub.centered()
            .add_text("graphics-agnostic terminal UI", Color::rgb(0.55, 0.75, 0.9))
            .generate();
        buf.c_draw(sub);

        // orbiting letter particles under the wordmark
        for p in spawn_particles(t) {
            let x = (W / 2) as i32 + (p.radius * p.angle.cos()) as i32;
            let y = (H / 2 + 12) as i32
                + (p.radius * 0.4 * p.angle.sin()) as i32;
            buf.set_char(x, y, p.glyph, p.color);
        }

        // footer
        let mut foot = TextComponent::new(Component::new(W / 2, H - 3));
        foot.centered()
            .add_text("tab: scenes   esc: quit", dim)
            .generate();
        buf.c_draw(foot);
    }

    fn draw_main(&mut self) {
        let t = self.t;
        let mx = self.app.mouse_pos.0;
        let my = self.app.mouse_pos.1;
        let buf = self.app.buf();

        // --- frame ---
        buf.g_draw(Rect::new(0, 0, W - 1, H - 1, false), '²', Color::rgb(0.25, 0.25, 0.35));

        // --- 3D wireframes ---
        cube().draw(t, buf);
        octahedron().draw(t, buf);

        // --- triangles orbiting the 3D scene, reacting to the mouse ---
        let cx = (W / 2) as i32;
        let cy = (H / 2 - 6) as i32;
        for i in 0..3 {
            let a = t * (0.6 + 0.3 * i as f32) + i as f32 * (PI * 2.0 / 3.0);
            let px = cx + (26.0 * a.cos()) as i32;
            let py = cy + (13.0 * a.sin()) as i32;
            // each triangle points toward the mouse
            let dx = (mx - px) as f32;
            let dy = (my - py) as f32;
            let ang = dy.atan2(dx);
            let nose = 5.0;
            let wing = 3.5;
            buf.g_draw(
                Triangle::new(
                    px + (nose * ang.cos()) as i32, py + (nose * ang.sin()) as i32,
                    px + (wing * (ang + 2.5).cos()) as i32, py + (wing * (ang + 2.5).sin()) as i32,
                    px + (wing * (ang - 2.5).cos()) as i32, py + (wing * (ang - 2.5).sin()) as i32,
                ),
                '►',
                Color::rgb(0.95, 0.85, 0.2),
            );
        }

        // --- letter particles ---
        for p in spawn_particles(t) {
            let x = cx + (p.radius * p.angle.cos()) as i32;
            let y = cy + (p.radius * 0.5 * p.angle.sin()) as i32 + ((t + p.phase) * 3.0).sin() as i32;
            buf.set_char(x, y, p.glyph, p.color);
        }

        // --- pulsing circles anchored to the mouse ---
        let pulse = ((t * 2.0).sin() * 3.0) as i32 + 4;
        buf.g_draw(Circle::new(mx, my, pulse), 'ú', Color::rgb(0.2, 1.0, 0.5));
        buf.g_draw(Circle::new(mx, my, pulse + 4), 'ù', Color::rgb(0.9, 0.3, 0.5));
        buf.set_char(mx, my, 'þ', Color::WHITE);

        // --- HUD ---
        let mut title = TextComponent::new(Component::new(cx, 1));
        title.centered()
            .add_text("- TROGUE DEMO -", Color::rgb(0.35, 0.8, 1.0))
            .generate();
        buf.c_draw(title);

        let mut div = DividerComponent::new(Component::new(1, 3).size(W - 2, 1));
        div.line_char('Ä').center_char('Á').generate();
        buf.c_draw(div);

        let mut hud = TextComponent::new(Component::new(2, H - 2));
        hud.add_text(format!("mouse ({:03},{:03})", mx, my).as_str(), Color::rgb(0.6, 0.6, 0.7))
            .add_text("  │  ", Color::rgb(0.3, 0.3, 0.4))
            .add_text(format!("t = {:>5.1}s", t).as_str(), Color::rgb(0.6, 0.6, 0.7))
            .add_text("  │  ", Color::rgb(0.3, 0.3, 0.4))
            .add_text("esc quits", Color::rgb(0.4, 0.4, 0.5))
            .generate();
        buf.c_draw(hud);

        let mut hint = TextComponent::new(Component::new(W - 2, H - 2));
        hint.centered()
            .add_text("wireframe: cube + octahedron", Color::rgb(0.35, 0.35, 0.5))
            .generate();
        buf.c_draw(hint);
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.t += 1.0 / 60.0;
        self.app.set_mouse_pos(
            (input::get_mouse_position(ctx).x / 12.0) as i32,
            (input::get_mouse_position(ctx).y / 12.0) as i32,
        );
        if input::is_key_pressed(ctx, Key::Tab) {
            self.title_scene = !self.title_scene;
        }
        if input::is_key_down(ctx, Key::Escape) {
            tetra::window::quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        self.app.buf().clear();
        self.draw_scene();
        graphics::set_transform_matrix(ctx, Mat4::scaling_3d(Vec3::new(1.5, 1.5, 1.0)));
        self.backend.draw(self.app.clone().draw(), ctx);
        graphics::reset_transform_matrix(ctx);
        Ok(())
    }
}
fn main() -> tetra::Result {
    ContextBuilder::new("trogue demo", W * 12, H * 12)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
