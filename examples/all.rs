use mergui::widgets::ButtonConfig;
use quicksilver::graphics::blend::{
    BlendChannel, BlendFactor, BlendFunction, BlendInput, BlendMode,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

use mergui::{core::Text, Context, FontStyle, MFont};

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Image Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the Font, just like loading any other asset
    let font = MFont::load_ttf(&gfx, "font.ttf").await?;
    let mut context = Context::new([0.0, 0.0].into());
    let layer = context.add_layer();
    let conf = Text {
        text: "Some awesome piece of text".into(),
        font_style: FontStyle {
            font: font.clone(),
            size: 15.0,
            location: Vector::new(20, 20),
            color: Color::BLACK,
            max_width: None,
        },
    };
    let _t = context.add_widget(conf, &layer).unwrap();
    let button = Image::load(&gfx, "button.png").await?;
    //*
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
        background: button,
        background_location: Rectangle::new((100, 50), (200, 100)),
        blend_color: Some(Color::GREEN),
        hover_color: Some(Color::RED),
        font_style: FontStyle {
            font: font.clone(),
            size: 20.0,
            location: Vector::new(30, 55),
            color: Color::BLUE,
            max_width: None,
        },
        text: "Some text".into(),
    };
    let _button = context.add_widget(conf, &layer).unwrap();

    gfx.clear(Color::WHITE);
    context.render(&mut gfx);

    gfx.present(&window)?;

    loop {
        while let Some(e) = events.next_event().await {
            context.event(&e, &window);
        }
        gfx.clear(Color::WHITE);
        context.render(&mut gfx);
        gfx.present(&window)?;
    }
}
