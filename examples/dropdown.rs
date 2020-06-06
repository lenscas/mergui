//This example shows how to create a dropdown and how to get the current selected option
//It does so by changing the background to whatever color was selected

use mergui::widgets::DropDownConfig;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image, VectorFont},
    Result, Timer, {run, Input, Settings, Window},
};

use mergui::{Context, FontStyle, MFont};
use std::marker::PhantomData;

fn main() {
    run(
        Settings {
            size: Vector::new(1366., 768.),
            title: "Dropdown",
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

    //now, we make a font style, which tells our widgets how the text should be drawn.
    let basic_font_style = FontStyle {
        font: font.clone(),
        location: Vector::new(100., 20.),
        color: Color::BLACK,
    };

    //next up, we need to create the context for the GUI parts.
    //The context is used for the following things
    //1: Decide the render order.
    //2: Decide what the cursor needs to be, based on its location
    //3: Decide which widget currently has focus (if any)
    //4: Decide which widget gets which events (if any).

    let mut context = Context::new();

    let button = Image::load(&gfx, "button.png").await?;

    //we then construct a layer. A layer is used to group and control multiple widgets together.
    let mut layer = context.add_layer();

    //This gets a list of options and allows the user to select 1 of them.
    let conf = DropDownConfig {
        //the options that can be selected. It accepts either a DropDownValueConfig or something that supports .into() for it.
        values: vec![
            (
                Color::WHITE,   //the value once selected
                "White".into(), //The text that the user sees
                //the style the text uses
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 30.0)?,
                    location: Vector::new(20., 55.),
                    ..basic_font_style.clone()
                },
            ),
            (
                Color::CYAN,
                "Cyan".into(),
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 35.0)?,
                    location: Vector::new(20., 55.),
                    ..basic_font_style.clone()
                },
            ),
            (
                Color::PURPLE,
                "Purple".into(),
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 35.0)?,
                    location: Vector::new(20., 55.),
                    ..basic_font_style.clone()
                },
            ),
        ],
        //where the dropdown should be rendered
        location: Rectangle::new(Vector::new(100., 50.), Vector::new(160., 50.)),
        //how height every option should be
        option_height: 50.0,
        //the picture for the button that can open/close the dropdown
        open_button: button.clone(),
        //how big this button should be
        open_button_size: (100.0, 50.0).into(),
        //what the default selected is (if any)
        selected: Some(0),
        //the color for the dividers from every option
        divider_color: Color::BLACK,
        //how big this divider should be
        divider_size: 5.0,
        t: PhantomData, //generic sadness :(
    };
    let dropdown = layer.add_widget(conf);

    gfx.clear(Color::WHITE);
    context.render(&mut gfx, &window)?;
    gfx.present(&window)?;
    //add a timer to make sure we draw at 60FPS and not faster.
    let mut render_timer = Timer::time_per_second(60.0);

    loop {
        while let Some(e) = inputs.next_event().await {
            //whenever we are given an event, we pass it to context so it can update the widget.
            context.event(&e, &window);
        }
        //limit the FPS to 60
        if render_timer.exhaust().is_some() {
            //get the color we need to draw
            gfx.clear(dropdown.channel.get_value().unwrap_or(Color::WHITE));
            //render the widgets
            context.render(&mut gfx, &window)?;
            //paint to the screen
            gfx.present(&window)?;
        }
    }
}
