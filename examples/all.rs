//This example goes over every widget and shows how to construct it.
//some widgets are not shown here. This is because they are either:
//Broken since the move to quicksilver 0.4: core::TextButton
//don't have much new to add in how they are used: core::TextButton, core::ImageButton
//are a wrapper over an already shown widgets: ConcealerManager.

//there is a short explanation at the end of this file to explain what they do though.

use mergui::widgets::{input::InputConfig, ButtonConfig, ConcealerConfig, DropDownConfig};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image, VectorFont},
    Result, Timer, {run, Input, Settings, Window},
};

use mergui::{core::Text, Context, FontStyle, MFont};
use std::marker::PhantomData;

fn main() {
    run(
        Settings {
            size: Vector::new(1366., 768.),
            title: "Every widget",
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

    //at this point in time, we don't have access to the location of the cursor. Setting it to 0,0 is however good enough for now.
    let mut context = Context::new((0., 0.).into());

    //we then construct a layer. A layer is used to group and control multiple widgets together.
    //You probably want 1 layer per screen/menu.
    //Whenever a layer gets dropped. The widgets that belong to that layer will also be removed.
    let layer = context.add_layer();

    //Now, our first widget. A simple piece of text.
    let conf = Text {
        text: "This is just a piece of text. Nothing fancy".into(),
        font_style: basic_font_style.clone(), //the fontstyle is cloned because we can use it later
    };

    //here, we add the widget to the context so it can be drawn and receive events.
    //in return we get an Response<()> back
    //Dropping the Response will remove the widget, which is why we store it as _t
    //its other uses will be explained later.
    let _t = context.add_widget(conf, &layer).unwrap();

    //for the next one (and a few others) we also need an image. Lets load it now
    let button = Image::load(&gfx, "button.png").await?;

    //now, time for the first interactable widget. A button!
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

    //in order to explain how to work with responses we are going to make it so the background changes color whenever you click the button
    let mut button_response = context.add_widget(conf, &layer).unwrap();

    //The next widget is a concealer. This is a widget that automatically hides/shows a list of widgets whenever it gets clicked.
    //A good use for it could be a collapsible menu.
    //In this case, we simply render another button .
    let conf = ConcealerConfig {
        button: ButtonConfig {
            background: button.clone(),
            background_location: Rectangle::new(Vector::new(100., 105.), Vector::new(100., 50.)),
            blend_color: Some(Color::GREEN),
            hover_color: Some(Color::RED),
            font_style: FontStyle {
                font: font.clone(),
                location: Vector::new(15., 30.),
                color: Color::WHITE,
            },
            text: "Concealer".into(),
        },
        hidden_widgets: vec![(
            0,
            ButtonConfig {
                background: button.clone(),
                background_location: Rectangle::new(
                    Vector::new(210., 105.),
                    Vector::new(100., 50.),
                ),
                blend_color: Some(Color::GREEN),
                hover_color: Some(Color::RED),
                font_style: FontStyle {
                    font: font.clone(),
                    location: Vector::new(15., 30.),
                    color: Color::WHITE,
                },
                text: "Hidden".into(),
            },
        )],
        to_widget: PhantomData,
        to_result: PhantomData,
    };
    let _concealer = context.add_widget(conf, &layer).unwrap();

    //Now, we get to the input widget. This is a widget that allows the user to insert some text.
    let config = InputConfig {
        font: FontStyle {
            font: MFont::from_font(&base_font, &gfx, 30.0)?,
            ..basic_font_style.clone()
        },
        placeholder: None,
        location: Rectangle::new(Vector::new(100., 160.), Vector::new(260., 50.)),
        start_value: Some(String::from("Text box")),
        cursor_config: Default::default(),
    };
    let _text_input = context.add_widget(config, &layer).unwrap();

    //The dropdown widget. This gets a list of options and allows the user to select 1 of them.
    let conf = DropDownConfig {
        values: vec![
            (
                "Dropdown",
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 30.0)?,
                    location: Vector::new(10., 55.),
                    ..basic_font_style.clone()
                },
            ),
            (
                "Second",
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 35.0)?,
                    location: Vector::new(15., 55.),
                    ..basic_font_style.clone()
                },
            ),
        ],
        location: Rectangle::new(Vector::new(100., 215.), Vector::new(160., 50.)),
        option_height: 50.0,
        open_button: button.clone(),
        open_button_size: (100.0, 50.0).into(),
        selected: Some(0),
        divider_color: Color::BLACK,
        divider_size: 5.0,
        t: PhantomData,
    };
    let _dropdown = context.add_widget(conf, &layer).unwrap();

    //Ever widget is added. Time to render them and give them events.
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

//as for the earlier mentioned widgets that didn't make it into the example
//core::image_button. It is the image side of a widgets::Button.
//core::text_button. It is the text side of a widgets::Button.

//widgets::ConcealerManager. This is a wrapper over a group of concealers. It makes sure that there is at most 1 active at any given time.
