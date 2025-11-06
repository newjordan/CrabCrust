// Terminal rendering module
use crate::braille::BrailleGrid;
use anyhow::Result;
use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use std::io::{self, Stdout, Write};

/// Rendering mode for animations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    /// Fullscreen mode - takes over entire terminal (for demos)
    Fullscreen,
    /// Inline mode - renders in a fixed-height frame without clearing history
    Inline { height: u16 },
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Fullscreen
    }
}

/// Terminal renderer with panic-safe cleanup
pub struct TerminalRenderer {
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    mode: RenderMode,
    inline_start_row: u16,
    _cleanup: TerminalCleanup,
}

/// RAII guard for terminal cleanup
struct TerminalCleanup {
    mode: RenderMode,
    inline_start_row: u16,
}

impl Drop for TerminalCleanup {
    fn drop(&mut self) {
        match self.mode {
            RenderMode::Fullscreen => {
                let _ = disable_raw_mode();
                let _ = execute!(
                    io::stdout(),
                    LeaveAlternateScreen,
                    cursor::Show
                );
            }
            RenderMode::Inline { height } => {
                // Clear the inline animation area
                let _ = Self::clear_inline_area(self.inline_start_row, height);
                let _ = execute!(io::stdout(), cursor::Show);
            }
        }
    }
}

impl TerminalCleanup {
    /// Clear the inline animation area
    fn clear_inline_area(start_row: u16, height: u16) -> Result<()> {
        let mut stdout = io::stdout();

        // Move to start of animation area
        execute!(stdout, cursor::MoveTo(0, start_row))?;

        // Clear each line
        for _ in 0..height {
            queue!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))?;
            queue!(stdout, cursor::MoveDown(1))?;
        }

        // Move cursor back to start position
        execute!(stdout, cursor::MoveTo(0, start_row))?;

        stdout.flush()?;
        Ok(())
    }
}

impl TerminalRenderer {
    /// Create a new terminal renderer with default fullscreen mode
    pub fn new() -> Result<Self> {
        Self::with_mode(RenderMode::Fullscreen)
    }

    /// Create a terminal renderer with specified mode
    pub fn with_mode(mode: RenderMode) -> Result<Self> {
        match mode {
            RenderMode::Fullscreen => {
                enable_raw_mode()?;
                execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;

                let stdout = io::stdout();
                let backend = CrosstermBackend::new(stdout);
                let terminal = Terminal::new(backend)?;

                Ok(Self {
                    terminal: Some(terminal),
                    mode,
                    inline_start_row: 0,
                    _cleanup: TerminalCleanup { mode, inline_start_row: 0 },
                })
            }
            RenderMode::Inline { height } => {
                // For inline mode, don't use ratatui's Terminal at all
                // Just reserve space in the current terminal
                let mut stdout = io::stdout();

                // Save current cursor position
                let (_, start_row) = cursor::position()?;

                // Print empty lines to reserve space
                for _ in 0..height {
                    writeln!(stdout)?;
                }

                stdout.flush()?;

                Ok(Self {
                    terminal: None,
                    mode,
                    inline_start_row: start_row,
                    _cleanup: TerminalCleanup { mode, inline_start_row: start_row },
                })
            }
        }
    }

    /// Get the rendering mode
    pub fn mode(&self) -> RenderMode {
        self.mode
    }

    /// Get terminal size (width, height)
    pub fn size(&self) -> Result<(u16, u16)> {
        match self.mode {
            RenderMode::Fullscreen => {
                let size = self.terminal.as_ref().unwrap().size()?;
                Ok((size.width, size.height))
            }
            RenderMode::Inline { height } => {
                let (width, _) = crossterm::terminal::size()?;
                Ok((width, height))
            }
        }
    }

    /// Clear the terminal
    pub fn clear(&mut self) -> Result<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal.clear()?;
        }
        Ok(())
    }

    /// Manually clear the inline animation area (usually automatic on drop)
    pub fn clear_inline_frame(&self) -> Result<()> {
        if let RenderMode::Inline { height } = self.mode {
            TerminalCleanup::clear_inline_area(self.inline_start_row, height)?;
        }
        Ok(())
    }

    /// Render a BrailleGrid to the terminal
    pub fn render_braille(&mut self, grid: &BrailleGrid) -> Result<()> {
        match self.mode {
            RenderMode::Fullscreen => self.render_braille_fullscreen(grid),
            RenderMode::Inline { .. } => self.render_braille_inline(grid),
        }
    }

    /// Render braille in fullscreen mode (using ratatui)
    fn render_braille_fullscreen(&mut self, grid: &BrailleGrid) -> Result<()> {
        let terminal = self.terminal.as_mut().unwrap();
        terminal.draw(|frame| {
            let area = frame.area();

            // Build lines from braille grid
            let mut lines = Vec::new();
            for y in 0..grid.height().min(area.height as usize) {
                let mut spans = Vec::new();
                for x in 0..grid.width().min(area.width as usize) {
                    let ch = grid.get_char(x, y);
                    let color = grid.get_color(x, y);

                    let style = if let Some(c) = color {
                        Style::default().fg(ratatui::style::Color::Rgb(c.r, c.g, c.b))
                    } else {
                        Style::default()
                    };

                    spans.push(Span::styled(ch.to_string(), style));
                }
                lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, area);
        })?;

        Ok(())
    }

    /// Render braille in inline mode (direct stdout writing)
    fn render_braille_inline(&mut self, grid: &BrailleGrid) -> Result<()> {
        let mut stdout = io::stdout();

        // Move to start position
        execute!(stdout, cursor::MoveTo(0, self.inline_start_row))?;

        // Render each line of the grid directly
        for y in 0..grid.height() {
            // Position at the start of the line
            queue!(stdout, cursor::MoveTo(0, self.inline_start_row + y as u16))?;

            for x in 0..grid.width() {
                let ch = grid.get_char(x, y);
                let color = grid.get_color(x, y);

                if let Some(c) = color {
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Rgb {
                            r: c.r,
                            g: c.g,
                            b: c.b
                        }),
                        Print(ch),
                        ResetColor
                    )?;
                } else {
                    queue!(stdout, Print(ch))?;
                }
            }
        }

        // Move cursor to after the animation area
        let (_, height) = self.size()?;
        execute!(stdout, cursor::MoveTo(0, self.inline_start_row + height))?;

        stdout.flush()?;
        Ok(())
    }

    /// Render text lines to the terminal (fullscreen mode only)
    pub fn render_text(&mut self, text: &str) -> Result<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal.draw(|frame| {
                let area = frame.area();
                let paragraph = Paragraph::new(text);
                frame.render_widget(paragraph, area);
            })?;
        }
        Ok(())
    }

    /// Render BrailleGrid with text below it (fullscreen mode only)
    pub fn render_braille_with_text(&mut self, grid: &BrailleGrid, text: &str) -> Result<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal.draw(|frame| {
            let area = frame.area();

            // Split area: top for braille, bottom for text
            let braille_height = grid.height().min((area.height as usize).saturating_sub(5)) as u16;
            let text_height = area.height.saturating_sub(braille_height);

            // Render braille in top area
            let braille_area = Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: braille_height,
            };

            let mut braille_lines = Vec::new();
            for y in 0..grid.height().min(braille_height as usize) {
                let mut spans = Vec::new();
                for x in 0..grid.width().min(area.width as usize) {
                    let ch = grid.get_char(x, y);
                    let color = grid.get_color(x, y);

                    let style = if let Some(c) = color {
                        Style::default().fg(ratatui::style::Color::Rgb(c.r, c.g, c.b))
                    } else {
                        Style::default()
                    };

                    spans.push(Span::styled(ch.to_string(), style));
                }
                braille_lines.push(Line::from(spans));
            }

            let braille_paragraph = Paragraph::new(braille_lines);
            frame.render_widget(braille_paragraph, braille_area);

            // Render text in bottom area
            let text_area = Rect {
                x: area.x,
                y: area.y + braille_height,
                width: area.width,
                height: text_height,
            };

            let text_paragraph = Paragraph::new(text);
            frame.render_widget(text_paragraph, text_area);
            })?;
        }

        Ok(())
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create terminal renderer")
    }
}
