mod tui_app;
mod ui_util;
mod cli_util;
use tui_app::{draw_ui, Application};
use std::{sync::{Arc, Mutex}, thread};
use rodio::OutputStream;
use inquire::{error::InquireError, ui::{Attributes, Color, RenderConfig, StyleSheet, Styled}, Select};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    inquire::set_global_render_config(get_render_config());
    let [tracks, infos] = get_tracks();

    let app = Application::new(tracks, infos);
    if app.get_matches().subcommand().is_none() {
        app.play()?;
    }

    if let Some(("menu", _)) = app.get_matches().subcommand() {
        let tr = app.tracks.lock().unwrap();
        let inf = app.infos.lock().unwrap();
        open_menu(tr.to_vec(), inf.to_vec())?;
    }
    Ok(())
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default_colored();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightGreen);
    render_config.prompt = StyleSheet::new().with_attr(Attributes::BOLD);
    render_config.highlighted_option_prefix = Styled::new("🎵");
    render_config.selected_option = Some(StyleSheet::new()
        .with_fg(Color::LightMagenta).with_attr(Attributes::ITALIC));
    render_config.option = StyleSheet::new().with_attr(Attributes::ITALIC);
    render_config.help_message = StyleSheet::new().with_fg(Color::LightGreen);
    render_config
}

pub fn get_tracks() -> [Vec<&'static str>; 2] {
    let tracks: Vec<&str> = vec![
        "good-night-160166",
        "lofi-study-112191",
        "the-weekend-117427",
        "sleepy-cat-118974",
        "deep-focus-113552",
        "sax-lofi-vibes-205066",
        "free-lofi-type-beat-evening-glow-209896"
    ];
    let infos: Vec<&str> = vec![r#"
        `good-night-160166` is a mellow lo-fi track that evokes a sense of calm and serenity, perfect for winding down after a long day. 
        The song features a gentle, soothing melody carried by soft, dreamy synths and laid-back beats, creating an ambiance of relaxation. 
        The warm, nostalgic tones and subtle background noises, such as the distant hum of vinyl static or the soft chirping of night insects, add to its comforting atmosphere. 
        The rhythm is unhurried, allowing listeners to drift into a peaceful state, making it an ideal background for studying, reading, or simply unwinding before bed.
        "#,
        r#"
        `lofi-study-112191` is a mellow and focused lo-fi track crafted to enhance concentration and create a serene study environment.
        The song features gentle, soothing melodies layered over smooth, rhythmic beats, providing a calming backdrop that helps maintain focus without distraction. 
        The minimalistic arrangement allows the mind to stay engaged, while the subtle ambient sounds create a peaceful atmosphere conducive to productivity. 
        Whether you're tackling assignments, reading, or working on creative projects, `lofi-study-112191` offers the perfect auditory companion for deep concentration and sustained mental clarity.
        "#,
        r#"
        `the-weekend-117427` is a relaxing lo-fi track that captures the essence of a laid-back weekend. 
        The song features smooth, melodic instrumentals layered with mellow beats, creating a vibe of unwinding and ease. Gentle guitar strums or soft piano notes, combined with the comforting crackle of vinyl, evoke feelings of nostalgia and tranquility. 
        The unhurried tempo and soothing rhythms invite listeners to slow down and enjoy the moment, whether they're lounging at home, sipping coffee, or taking a leisurely stroll. 
        Ideal for enhancing the mood of relaxation and rejuvenation, `the-weekend-117427` serves as the perfect soundtrack for those cherished weekend moments.
        "#,
        r#"
        `sleepy-cat-118974` is a calming lo-fi track that effortlessly creates a peaceful and restful ambiance, making it ideal for relaxation and unwinding. 
        The song features gentle, soothing melodies intertwined with soft, rhythmic beats that evoke a sense of tranquility. 
        The minimalistic arrangement, enhanced by subtle background sounds, provides a serene atmosphere perfect for winding down after a long day. 
        Whether you're looking to relax, meditate, or drift off to sleep, "Sleepy Cat 118974" offers the perfect soothing soundtrack for those quiet moments of rest and relaxation.
        "#,
        r#"
        'deep-focus-113552` is a mesmerizing lo-fi track crafted to enhance concentration and mental clarity during intense work or study sessions. 
        The song envelopes listeners in a cocoon of immersive sound, featuring intricate melodies woven into a tapestry of gentle, rhythmic beats. 
        With its hypnotic cadence and soothing ambiance, `deep-focus-113552` effortlessly guides the mind into a state of deep concentration, fostering productivity and focus. 
        The minimalistic yet captivating arrangement, combined with subtle background textures, creates an atmosphere conducive to sustained attention and creative flow. 
        Whether you're tackling complex tasks, delving into deep thought, or engaging in creative endeavors, this track serves as the perfect companion for achieving a heightened state of mental clarity and productivity.
        "#,
        r#"
        `Sax Lofi Vibes 205066` is a captivating lo-fi track that transports listeners to a realm of mellow tranquility and soulful melodies. 
        At the heart of the song lies the sultry sound of the saxophone, its smooth and expressive notes weaving effortlessly through a backdrop of laid-back beats and soothing rhythms. 
        Each note carries with it a sense of warmth and nostalgia, evoking images of late-night cityscapes and intimate jazz clubs. 
        The fusion of the saxophone's emotive tones with the relaxed ambiance of lo-fi music creates a mesmerizing listening experience, perfect for unwinding after a long day or setting a cozy atmosphere for quiet reflection. 
        Whether you're seeking solace in its melodies or simply looking to bask in its soothing vibes, "sax-lofi-vibes-205066" is sure to envelop you in a blissful aura of musical tranquility.
        "#,
        r#"
        `free-lofi-type-beat-gvening-glow-209896` is a captivating instrumental track that encapsulates the serene ambiance of a tranquil evening. 
        The song unfolds with gentle melodies that paint a picture of the setting sun casting its warm hues across the sky. Soft, rhythmic beats provide a soothing backdrop, evoking a sense of relaxation and contentment. 
        As the music unfolds, listeners are transported to a peaceful realm where time seems to slow down, allowing for moments of reflection and introspection. 
        Whether you're unwinding after a busy day or simply seeking solace in its melodic embrace, `free-lofi-type-beat-gvening-glow-209896` offers a serene and meditative journey through the beauty of twilight.
        "#
    ];
    [tracks, infos]
}

fn open_menu(tracks: Vec<&'static str>, infos: Vec<&'static str>) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let tracks = Arc::new(Mutex::new(tracks));

    let selection: Result<&str, InquireError> = Select::new("What's your pick?", (*tracks.lock().unwrap()).clone()).prompt();


    if let Ok(choice) = selection {

        let handle = thread::spawn(move || {
            let str_index = &tracks.lock().unwrap().iter().position(|n| n == &choice);
            if let Some(value) = str_index {
                tui_app::play_track(&stream_handle, choice, infos[*value], tui_app::MusicMode::Specific);
            }

        });
        draw_ui(handle)?;
    }

    Ok(())
}