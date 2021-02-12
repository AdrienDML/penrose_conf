#[macro_use]
extern crate penrose;

#[macro_use]
extern crate penrose_conf;

use std::convert::TryFrom;

use penrose::{
    core::{
        config::Config,
        helpers::index_selectors,
        layout::{side_stack, bottom_stack, floating, Layout, LayoutConf},
        hooks::Hooks,
    },
    draw::{
        TextStyle, Color, Position,
        StatusBar,
        widget::{
            Workspaces,
            CurrentLayout,
            ActiveWindowName,
            RootWindowName,
        }        
    },
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager, XcbDraw}, XcbConnection,
    Backward, Forward, Less, More, Selector,
};

use penrose_conf::{
    consts::*,
    widget::Time,
};

use simplelog::{LevelFilter, SimpleLogger};


// Replace these with your preferred terminal and program launcher



fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let floating_classes = vec![
        "peek",
        FLOAT_CLASS,
    ];
    
    let config = Config::default().builder()
        .workspaces(vec!["1", "2", "3", "4","5"])
        .layouts(vec![
            layout!("[side]", side_stack), 
            layout!("[botom]", bottom_stack),
            layout!("[floating]", floating),
        ])
        .gap_px(0)
        .focused_border(0x458588)
        .floating_classes(floating_classes)
        .build().unwrap();
            
    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 11,
        fg: Color::try_from(WHITE)?,
        bg: Some(Color::try_from(BLACK)?),
        padding: (2.0, 2.0),
    };
    let hooks :  Hooks<XcbConnection> = vec![
        Box::new(StatusBar::try_new(
            XcbDraw::new()?,
            Position::Top,
            SB_HEIGHT,
            Color::try_from(BLACK)?,
            &[&FONT.to_string()],
            vec![
                Box::new(Workspaces::new(
                    &config.workspaces().clone(),
                    &style,
                    Color::try_from(BLUE)?,
                    Color::try_from(GREY)?
                )),
                Box::new(CurrentLayout::new(&style)),
                Box::new(ActiveWindowName::new(
                    &TextStyle {
                        bg: Some(Color::try_from(BLUE)?),
                        padding: (6.0, 4.0),
                        ..style.clone()
                    },
                    80,
                    true,
                    false,
                )),
                Box::new(
                    Time::new(&style, false, true)
                ),
                Box::new(RootWindowName::new(
                    &TextStyle {
                        padding: (4.0, 2.0),
                        ..style.clone()
                    },
                    false,
                    true,
                )),
            ],
        )?)
    ];

    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-n" => run_external!(LAUNCHER);
        "M-t" => run_external!(TERMINAL);
        "Print" => run_external!("maim -s ~/Image/ScreenShot/$(date +%s).png");
        // Exit Penrose (important to remember this one!)
        "M-Escape" => run_internal!(exit);
        
        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-q" => run_internal!(kill_client);
        //"M-h" => run_internal!(hide_client, wm.focused_client_id().unwrap());
        
        
        // workspace management
        "M-w" => run_internal!(toggle_workspace);
        "M-Tab" => run_internal!(cycle_workspace, Forward);
        "M-S-Tab" => run_internal!(cycle_workspace, Backward);
        
        // Layout management
        "M-l" => run_internal!(cycle_layout, Forward);
        "M-S-l" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);
        
        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces().len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces().len()) ];
        };
    };
    
    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}