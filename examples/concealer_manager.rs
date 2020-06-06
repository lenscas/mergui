//This example shows the concealer manager in action
//it will show 2 concealers that are mananged, each with a single button to change the background color.

use mergui::widgets::{ButtonConfig, ConcealerConfig, ConcealerManagerConfig};

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

    //lets make the first concealer. We start with a layer that it will control
    let mut first_layer = context.add_singular_layer();
    let mut to_cyan_button = first_layer.add_widget(ButtonConfig {
        background: button.clone(), //the image for the button
        background_location: Rectangle::new(Vector::new(205., 105.), Vector::new(100., 50.)), //where we need to draw it
        blend_color: Some(Color::GREEN), //used to blend a color to the button when it gets drawn
        hover_color: Some(Color::RED), //used to blend a color to the butten when it gets drawn while the mouse is hovering over it
        font_style: FontStyle {
            font: font.clone(),
            location: Vector::new(20., 30.), //The font location is relative to the location given to background_location
            color: Color::WHITE,
        },
        text: "To Cyan".into(), //the text
    });
    //we then make the first concealer config. We will add it to the manager later.
    //First, lets make another concealer.
    let first_concealer = ConcealerConfig {
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
        layer: first_layer,
    };

    let mut second_layer = context.add_singular_layer();
    let mut to_purple_button = second_layer.add_widget(ButtonConfig {
        background: button.clone(), //the image for the button
        background_location: Rectangle::new(Vector::new(205., 160.), Vector::new(100., 50.)), //where we need to draw it
        blend_color: Some(Color::GREEN), //used to blend a color to the button when it gets drawn
        hover_color: Some(Color::RED), //used to blend a color to the butten when it gets drawn while the mouse is hovering over it
        font_style: FontStyle {
            font: font.clone(),
            location: Vector::new(20., 30.), //The font location is relative to the location given to background_location
            color: Color::WHITE,
        },
        text: "To Purple".into(), //the text
    });
    let second_concealer = ConcealerConfig {
        button: ButtonConfig {
            background: button.clone(),
            background_location: Rectangle::new(Vector::new(100., 160.), Vector::new(100., 50.)),
            blend_color: Some(Color::GREEN),
            hover_color: Some(Color::RED),
            font_style: FontStyle {
                font: font.clone(),
                location: Vector::new(25., 30.),
                color: Color::WHITE,
            },
            text: "Second".into(),
        },
        layer: second_layer,
    };
    //now, with 2 concealers made. Lets add them to a ConcealerManager
    //first, make the manager config
    let manager_config = ConcealerManagerConfig {
        concealers: vec![first_concealer, second_concealer],
    };
    //now, add the manager to the layer.
    let _concealer_manager_channel = layer.add_widget(manager_config);
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

            if to_cyan_button.channel.has_clicked() {
                background_color = if background_color == Color::CYAN {
                    Color::WHITE
                } else {
                    Color::CYAN
                }
            }
            if to_purple_button.channel.has_clicked() {
                background_color = if background_color == Color::PURPLE {
                    Color::WHITE
                } else {
                    Color::PURPLE
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
