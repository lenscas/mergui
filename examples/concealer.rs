//This example shows the concealer. The concealer is a widget that either hides or shows a layer when the user clicks on it.
//For this example we are going to make a concealer that hides a button.

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image, VectorFont},
    Result, Timer, {run, Input, Settings, Window},
};

use mergui::{
    channels::Concealer,
    widgets::{ButtonConfig, ConcealerConfig},
    Context, FontStyle, MFont,
};

fn main() {
    run(
        Settings {
            size: Vector::new(1366., 768.),
            title: "Concealer manager",
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

    //we need an image for the concealers/buttons
    let button = Image::load(&gfx, "button.png").await?;

    //next up, we need to create the context for the GUI parts.
    //The context is used for the following things
    //1: Decide the render order.
    //2: Decide what the cursor needs to be, based on its location
    //3: Decide which widget currently has focus (if any)
    //4: Decide which widget gets which events (if any).

    let mut context = Context::new();

    //we then construct a layer. A layer is used to group and control multiple widgets together.
    let mut layer = context.add_layer();

    //we need to create a layer which only the concealer is able to control.
    //it is pretty much the same as every other layer, except that this id doesn't implement clone.
    let hidden_layer = context.add_singular_layer();

    let concealer = ConcealerConfig {
        //the button the user can click on. See the button example to see what the fields are about.
        button: ButtonConfig {
            background: button.clone(),
            background_location: Rectangle::new(Vector::new(100., 105.), Vector::new(100., 50.)),
            blend_color: Some(Color::GREEN),
            hover_color: Some(Color::RED),
            font_style: FontStyle {
                font: font.clone(),
                location: Vector::new(30., 30.),
                color: Color::WHITE,
            },
            text: "First".into(),
        },
        //the layer that the concealer is in control off.
        layer: hidden_layer,
    };

    //now, add the conceiler to the layer.
    let mut concealer_return = layer.add_widget(concealer);
    //now, time to add our button
    let mut button = concealer_return.channel.add_widget(ButtonConfig {
        background: button.clone(),
        background_location: Rectangle::new(Vector::new(205., 105.), Vector::new(100., 50.)),
        blend_color: Some(Color::GREEN),
        hover_color: Some(Color::RED),
        font_style: FontStyle {
            font: font.clone(),
            location: Vector::new(20., 30.),
            color: Color::WHITE,
        },
        text: "To Cyan".into(),
    });

    gfx.clear(Color::WHITE);
    context.render(&mut gfx, &window)?;
    gfx.present(&window)?;
    //add a timer to make sure we draw at 60FPS and not faster.
    let mut render_timer = Timer::time_per_second(60.0);
    //this variable is used to change the background color when one of the hidden buttons gets clicked
    let mut background_color = Color::WHITE;
    loop {
        while let Some(e) = inputs.next_event().await {
            //whenever we are given an event, we pass it to context so it can update the widget.
            context.event(&e, &window);

            if button.channel.has_clicked() {
                background_color = if background_color == Color::CYAN {
                    Color::WHITE
                } else {
                    Color::CYAN
                }
            }
        }
        //limit the FPS to 60
        if render_timer.exhaust().is_some() {
            //check if we need to draw a white_background or not
            gfx.clear(background_color);
            //render the widgets
            context.render(&mut gfx, &window)?;
            //paint to the screen
            gfx.present(&window)?;
        }
    }
}
