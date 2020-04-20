use mergui::widgets::{input::InputConfig, ButtonConfig, ConcealerConfig, DropDownConfig};
use quicksilver::graphics::blend::{
    BlendChannel, BlendFactor, BlendFunction, BlendInput, BlendMode,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image, VectorFont},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

use mergui::{core::Text, Context, FontStyle, MFont};
use std::marker::PhantomData;

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Image Example",
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the Font, just like loading any other asset
    let base_font = VectorFont::load("font.ttf").await?;
    let font = MFont::from_font(&base_font, &gfx, 15.0)?;
    let mut context = Context::new([0.0, 0.0].into());
    let layer = context.add_layer();

    let basic_font_style = FontStyle {
        font: font.clone(),
        location: Vector::new(20, 20),
        color: Color::BLACK,
    };

    let conf = Text {
        text: "Some awesome piece of text".into(),
        font_style: basic_font_style.clone(),
    };
    let _t = context.add_widget(conf, &layer).unwrap();
    let button = Image::load(&gfx, "button.png").await?;
    gfx.set_blend_mode(Some(BlendMode {
        equation: Default::default(),
        function: BlendFunction::Same {
            source: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: false,
            },
            destination: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: true,
            },
        },
        global_color: [0.0; 4],
    }));

    let conf = ButtonConfig {
        background: button.clone(),
        background_location: Rectangle::new((100, 50), (200, 100)),
        blend_color: Some(Color::GREEN),
        hover_color: Some(Color::RED),
        font_style: FontStyle {
            font: MFont::from_font(&base_font, &gfx, 20.0)?,
            location: Vector::new(30, 55),
            color: Color::BLUE,
        },
        text: "Some text".into(),
    };
    let _button = context.add_widget(conf, &layer).unwrap();
    let conf = ConcealerConfig {
        button: ButtonConfig {
            background: button.clone(),
            background_location: Rectangle::new((100, 155), (200, 100)),
            blend_color: Some(Color::GREEN),
            hover_color: Some(Color::RED),
            font_style: FontStyle {
                font: MFont::from_font(&base_font, &gfx, 20.0)?,
                location: Vector::new(30, 55),
                color: Color::BLUE,
            },
            text: "Concealer".into(),
        },
        hidden_widgets: vec![(
            0,
            ButtonConfig {
                background: button.clone(),
                background_location: Rectangle::new((310, 155), (200, 100)),
                blend_color: Some(Color::GREEN),
                hover_color: Some(Color::RED),
                font_style: FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 20.0)?,
                    location: Vector::new(30, 55),
                    color: Color::BLUE,
                },
                text: "Hidden".into(),
            },
        )],
        to_widget: PhantomData,
        to_result: PhantomData,
    };
    let _concealer = context.add_widget(conf, &layer).unwrap();

    let conf = DropDownConfig {
        values: vec![
            (
                "a̐éö̲",
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 30.0)?,
                    location: Vector::new(10, 55),
                    ..basic_font_style.clone()
                },
            ),
            (
                "a̐éö̲",
                FontStyle {
                    font: MFont::from_font(&base_font, &gfx, 35.0)?,
                    location: Vector::new(15, 55),
                    ..basic_font_style.clone()
                },
            ),
        ],
        location: Rectangle::new((100, 300), (160, 50)),
        option_height: 50.0,
        open_button: button.clone(),
        open_button_size: [100.0, 50.0].into(),
        selected: Some(0),
        divider_color: Color::BLACK,
        divider_size: 5.0,
        t: PhantomData,
    };
    let _dropdown = context.add_widget(conf, &layer).unwrap();

    let config = InputConfig {
        font: FontStyle {
            font: MFont::from_font(&base_font, &gfx, 40.0)?,
            ..basic_font_style.clone()
        },
        placeholder: None, //Option<PlaceholderConfig>,
        location: Rectangle::new((100, 355), (160, 50)),
        start_value: Some(String::from("a̐éö̲")),
    };
    let _text_input = context.add_widget(config, &layer).unwrap();
    gfx.clear(Color::BLACK);
    context.render(&mut gfx)?;

    gfx.present(&window)?;

    loop {
        while let Some(e) = events.next_event().await {
            context.event(&e, &window);
        }
        gfx.clear(Color::RED);
        context.render(&mut gfx)?;
        gfx.present(&window)?;
    }
}
