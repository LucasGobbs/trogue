
use trogue::backend::{Backend, TetraBackend};
use trogue::buffer_app::BufferApp;
use trogue::component::*;
use trogue::shape::*;

use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

struct GameState {
 
    buffer_backend: TetraBackend,
    app: BufferApp,
}
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
       // let mut world = World::new();
       // let resources = Resources::new(&mut world);
        let font = Texture::new(ctx, "./examples/resources/terminal.png")?;
        let buffer_backend = TetraBackend::new(font);
       
        Ok(GameState {

            buffer_backend,
            app: BufferApp::new(150,100),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

      
        self.app.buf().clear();
      
   
        let mousex = (input::get_mouse_position(ctx).x / 8.0) as i32;
        let mousey = (input::get_mouse_position(ctx).y / 8.0) as i32;

        self.app.buf().set_char(mousex,mousey,'█',Color::GREEN);
        
        let mut life_txt = TextComponent::new(Component::new(0,0));
        life_txt.add_text("Fala;",Color::WHITE)
                .add_text(" ai mano",Color::RED)
                .generate();
        let mut div = DividerComponent::new(Component::new(1,10).size(11,1));
        div.line_char('#')
           .vertical()
           .corner_char('@')
           .center_char('0')
           .generate();
        
        self.app.buf().c_draw(life_txt);
        self.app.buf().c_draw(div);
        self.app.buf().g_draw(Line::new(50,50,mousex,mousey),'.',Color::BLUE);

        self.buffer_backend.draw(self.app.clone().draw(2), ctx);

        Ok(())
    }
}
fn main() -> tetra::Result {
   
    ContextBuilder::new("Terminal", 150 * 8, 100 * 8)
       // .timestep(Timestep::Fixed(30.0))
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
