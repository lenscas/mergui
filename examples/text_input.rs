//This example shows how to create a text input field and how to interact with it
//It does so by rendering the inserted text somewhere else on the screen as well.

use mergui::widgets::InputConfig;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, VectorFont},
    Result, Timer, {run, Input, Settings, Window},
};

use mergui::{Context, FontStyle, MFont};

fn main() {
    run(
        Settings {
            size: Vector::new(1366., 768.),
            title: "Button",
            resizable: false,
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut inputs: Input) -> Result<()> {
    // Pretty much every widget needs a font. However we need to wrap it before we can use it.

    //first, load the font like normal.
    let base_font = VectorFont::load("font.ttf").await?;

    //next up, we need to create the context for the GUI parts.
    //The context is used for the following things
    //1: Decide the render order.
    //2: Decide what the cursor needs to be, based on its location
    //3: Decide which widget currently has focus (if any)
    //4: Decide which widget gets which events (if any).

    let mut context = Context::new();

    //we then construct a layer. A layer is used to group and control multiple widgets together.
    let mut layer = context.add_layer();

    //Now, we get to the input widget. This is a widget that allows the user to insert some text.
    let var_name = InputConfig {
        font: FontStyle {
            font: MFont::from_font(&base_font, &gfx, 30.0)?,
            location: Vector::new(100., 20.),
            color: Color::BLACK,
        },
        placeholder: None,
        location: Rectangle::new(Vector::new(100., 50.), Vector::new(260., 50.)),
        start_value: Some(String::from("Text box")),
        cursor_config: Default::default(),
    };
    let config = var_name;
    let text_input = layer.add_widget(config);

    // Time to render the button
    //First, render something to the screen. We do this out of the loop so we don't have to wait for the timers to draw the first frame.
    gfx.clear(Color::WHITE);
    context.render(&mut gfx, &window)?;
    gfx.present(&window)?;
    //add a timer to make sure we draw at 60FPS and not faster.
    let mut render_timer = Timer::time_per_second(60.0);

    let example_font = MFont::from_font(&base_font, &gfx, 30.0)?;

    loop {
        while let Some(e) = inputs.next_event().await {
            //whenever we are given an event, we pass it to context so it can update the widget.
            context.event(&e, &window);
        }
        //limit the FPS to 60
        if render_timer.exhaust().is_some() {
            //check if we need to draw a white_background or not
            gfx.clear(Color::WHITE);

            example_font.draw(
                &mut gfx,
                &text_input.channel.get(),
                Color::BLACK,
                Vector::new(100., 135.),
            )?;
            //render the widgets
            context.render(&mut gfx, &window)?;
            //paint to the screen
            gfx.present(&window)?;
        }
    }
}
