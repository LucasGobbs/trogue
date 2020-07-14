
use trogue::backend::{Backend, TetraBackend};
use trogue::trogue_app::TrogueApp;
use trogue::component::*;
use trogue::shape::*;
use trogue::buffer::*;

use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

struct GameState {
 
    buffer_backend: TetraBackend,
    app: TrogueApp,
}
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
       // let mut world = World::new();
       // let resources = Resources::new(&mut world);
        let font = Texture::new(ctx, "./examples/resources/terminal.png")?;
        let buffer_backend = TetraBackend::new(font);
       
        Ok(GameState {

            buffer_backend,
            app: TrogueApp::new(75,75),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.app.set_mouse_pos((input::get_mouse_position(ctx).x / 8.0) as i32, 
                               (input::get_mouse_position(ctx).y / 8.0) as i32);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

      
        self.app.buf().clear();
      
   
        let mousex = self.app.mouse_pos.0;
        let mousey = self.app.mouse_pos.1;

        self.app.buf().set_char(mousex,mousey,'█',Color::GREEN);
        
        let mut life_txt = TextComponent::new(Component::new(mousex,
                                                                                   mousey + 1));
                                                        
        life_txt.centered()
                .add_text(format!("({},",mousex).as_str(),Color::GREEN)
                .add_text(format!("{})",mousey).as_str(),Color::GREEN)
                .generate();

        let mut div = DividerComponent::new(Component::new(1,10)
                                                                        .size(11,1));
        div.line_char('#')
           .vertical()
           .corner_char('@')
           .center_char('0')
           .generate();
        
        
        self.app.buf().c_draw(div);
        
        self.app.buf().g_draw(Line::new(0,0,mousex,mousey),'.',Color::rgb(0.6,0.1,0.8));
        self.app.buf().g_draw(Rect::new(0,0,75,75,false),'#',Color::rgb(0.6,0.1,0.8));
        self.app.buf().c_draw(life_txt);
        self.app.buf().set_char(mousex, mousey, 'x', Color::WHITE);
        self.buffer_backend.draw(self.app.clone().draw(), ctx);

        self.buffer_backend.test(Buffer::new(15,15),ctx,"./examples/resources/RobotY.ttf");
        Ok(())
    }
}
fn main() -> tetra::Result {
   
    ContextBuilder::new("Terminal", 75 * 8 , 75 * 8)
       // .timestep(Timestep::Fixed(30.0))
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
