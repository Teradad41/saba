use crate::alloc::string::ToString;
use alloc::format;
use alloc::rc::Rc;
use core::cell::RefCell;
use noli::error::Result as OsResult;
use noli::window::{StringSize, Window};
use saba_core::browser::Browser;
use saba_core::constants::*;
use saba_core::error::Error;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap(),
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;

        self.run_app()?;

        Ok(())
    }

    fn run_app(&mut self) -> Result<(), Error> {
        // 後ほど実装
        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        if let Err(error) = self.setup_toolbar() {
            return Err(Error::InvalidUI(format!(
                "failed to initialize a toolbar with error: {}",
                error
            )));
        }

        // 画面を更新する
        self.window.flush();
        Ok(())
    }

    fn setup_toolbar(&mut self) -> OsResult<()> {
        self.window
            .fill_rect(LIGHT_GRAY, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)?;

        self.window
            .draw_line(GRAY, 0, 0, WINDOW_WIDTH, TOOLBAR_HEIGHT)?;

        self.window.draw_line(
            DARK_GRAY,
            0,
            TOOLBAR_HEIGHT + 1,
            WINDOW_WIDTH - 1,
            TOOLBAR_HEIGHT,
        )?;

        self.window
            .draw_string(BLACK, 5, 5, "Address", StringSize::Medium, false)?;

        self.window
            .fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, 2 + ADDRESS_BAR_HEIGHT)?;

        self.window.draw_line(GRAY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window.draw_line(GRAY, 70, 2, 70, ADDRESS_BAR_HEIGHT)?;
        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;
        self.window
            .draw_line(GRAY, 71, 3, 71, 1 + ADDRESS_BAR_HEIGHT)?;

        Ok(())
    }
}
