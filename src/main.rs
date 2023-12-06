use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui_counter_app::{app, event::{EventHandler, Event}, tui::Tui, update::update};

fn main() -> Result<()> {
    // Create an application.
    let mut app 
        = app::App::new();

    // Initialize the terminal user interface.
    let backend 
        = CrosstermBackend::new(std::io::stderr());

    let terminal 
        = Terminal::new(backend)?;

    let events 
        = EventHandler::new(250);

    let mut tui 
        = Tui::new(terminal, events);

    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        tui.draw(&mut app)?; // Render the user interface.

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }


    tui.exit()?;     // Exit the user interface.

    Ok(())
}
