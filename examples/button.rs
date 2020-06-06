//This example shows how to create a button and how to see if someone clicked on it.
//It does so by changing the background whenever you click on the button

use mergui::widgets::ButtonConfig;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image, VectorFont},
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
    //then, turn it into a MFont. This is done so we can clone the font, which is something we need.
    let font = MFont::from_font(&base_font, &gfx, 15.0)?;

    //next up, we need to create the context for the GUI parts.
    //The context is used for the following things
    //1: Decide the render order.
    //2: Decide what the cursor needs to be, based on its location
    //3: Decide which widget currently has focus (if any)
    //4: Decide which widget gets which events (if any).

    let mut context = Context::new();

    //we then construct a layer. A layer is used to group and control multiple widgets together.
    let mut layer = context.add_layer();

    //for the next one (and a few others) we also need an image. Lets load it now
    let button = Image::load(&gfx, "button.png").await?;

    //now, time to setup the button.
    let conf = ButtonConfig {
        background: button.clone(), //the image for the button
        background_location: Rectangle::new(Vector::new(100., 50.), Vector::new(100., 50.)), //where we need to draw it
        blend_color: Some(Color::GREEN), //used to blend a color to the button when it gets drawn
        hover_color: Some(Color::RED), //used to blend a color to the butten when it gets drawn while the mouse is hovering over it
        font_style: FontStyle {
            font: font.clone(),
            location: Vector::new(20., 30.), //The font location is relative to the location given to background_location
            color: Color::WHITE,
        },
        text: "A Button".into(), //the text
    };

    //add the button to the layer so it can be rendered and receive events
    let mut button_response = layer.add_widget(conf);

    // Time to render the button
    //First, render something to the screen. We do this out of the loop so we don't have to wait for the timers to draw the first frame.
    gfx.clear(Color::WHITE);
    context.render(&mut gfx, &window)?;
    gfx.present(&window)?;
    //add a timer to make sure we draw at 60FPS and not faster.
    let mut render_timer = Timer::time_per_second(60.0);
    //this variable is used to change the background whenever the user clicks on the button
    let mut white_background = true;
    loop {
        while let Some(e) = inputs.next_event().await {
            //whenever we are given an event, we pass it to context so it can update the widget.
            context.event(&e, &window);
            //after this is done, we can read the new state of the button
            //in order to comunicate with the widget, we need the channel field from our Response.
            //This an object that the widget uses to comunicate back. In our case, it is of type BasicClickable.
            //This has 1 method called has_clicked() that simply returns true if the user clicked on the button since last time we executed has_clicked()
            //Lets change the background whenever it got clicked.
            if button_response.channel.has_clicked() {
                white_background = !white_background
            }
        }
        //limit the FPS to 60
        if render_timer.exhaust().is_some() {
            //check if we need to draw a white_background or not
            gfx.clear(if white_background {
                Color::WHITE
            } else {
                Color::CYAN
            });
            //render the widgets
            context.render(&mut gfx, &window)?;
            //paint to the screen
            gfx.present(&window)?;
        }
    }
}
