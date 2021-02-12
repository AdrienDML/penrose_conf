use penrose::{WindowManager, core::{Hook, xconnection::XConn}, draw::widget::Text, draw::{DrawContext, Widget, Result, TextStyle}};

use std::time::SystemTime;

fn get_time() -> String {
    let total_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let mins_tot = total_time / 60;
    let hours_tot = mins_tot / 60;
    let mins = mins_tot - hours_tot*60;
    let hours = hours_tot - (hours_tot /24) * 24;
    format!("[{}:{}]", hours, mins)
}

pub struct Time
{
    text : Text,
}

impl Time {

    pub fn new(style : &TextStyle, is_greedy : bool, right_justified : bool) -> Self {
        let time = get_time();
        Self {
            text : Text::new(
            time,
                style,
                is_greedy,
                right_justified,
            ),
        }
    }

}

impl Widget for Time {
    fn draw(&mut self,
        ctx: &mut dyn DrawContext,
        screen: usize,
        screen_has_focus: bool,
        w: f64,
        h: f64) -> Result<()> {
        self.text.set_text(get_time());
        self.text.draw(ctx, screen, screen_has_focus, w, h)
    }

    fn current_extent(&mut self,
        ctx: &mut dyn DrawContext,
        _h: f64
    ) -> Result<(f64, f64)> {
        self.text.current_extent(ctx, _h)
    }

    fn require_draw(&self) -> bool {
        *self.text.get_text() != get_time() || self.text.require_draw()
    }

    fn is_greedy(&self) -> bool {
        self.text.is_greedy()
    }
}

impl<X : XConn> Hook<X> for Time {
    fn event_handled(&mut self, _ : &mut WindowManager<X>) -> penrose::Result<()> {
        Ok(())
    }
}