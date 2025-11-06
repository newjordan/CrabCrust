// Terminal rendering module
use crate::braille::BrailleGrid;
use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
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
    terminal: Terminal<CrosstermBackend<Stdout>>,
    mode: RenderMode,
    _cleanup: TerminalCleanup,
}

/// RAII guard for terminal cleanup
struct TerminalCleanup {
    mode: RenderMode,
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
            RenderMode::Inline { .. } => {
                // For inline mode, just show cursor
                let _ = execute!(io::stdout(), cursor::Show);
            }
        }
    }
}

impl TerminalRenderer {
    /// Create a new terminal renderer with default fullscreen mode
    pub fn new() -> Result<Self> {
        Self::with_mode(RenderMode::Fullscreen)
    }

    /// Create a terminal renderer with specified mode
    pub fn with_mode(mode: RenderMode) -> Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        match mode {
            RenderMode::Fullscreen => {
                enable_raw_mode()?;
                execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;
            }
            RenderMode::Inline { height } => {
                // For inline mode, reserve space by printing newlines
                let mut stdout = io::stdout();
                execute!(stdout, cursor::Hide)?;
                // Print empty lines to reserve space
                for _ in 0..height {
                    writeln!(stdout)?;
                }
                // Move cursor back up
                execute!(stdout, cursor::MoveUp(height))?;
                stdout.flush()?;
            }
        }

        Ok(Self {
            terminal,
            mode,
            _cleanup: TerminalCleanup { mode },
        })
    }

    /// Get the rendering mode
    pub fn mode(&self) -> RenderMode {
        self.mode
    }

    /// Get terminal size (width, height)
    pub fn size(&self) -> Result<(u16, u16)> {
        let size = self.terminal.size()?;
        match self.mode {
            RenderMode::Fullscreen => Ok((size.width, size.height)),
            RenderMode::Inline { height } => Ok((size.width, height)),
        }
    }

    /// Clear the terminal
    pub fn clear(&mut self) -> Result<()> {
        self.terminal.clear()?;
        Ok(())
    }

    /// Render a BrailleGrid to the terminal
    pub fn render_braille(&mut self, grid: &BrailleGrid) -> Result<()> {
        self.terminal.draw(|frame| {
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

    /// Render text lines to the terminal
    pub fn render_text(&mut self, text: &str) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.area();
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, area);
        })?;
        Ok(())
    }

    /// Render BrailleGrid with text below it
    pub fn render_braille_with_text(&mut self, grid: &BrailleGrid, text: &str) -> Result<()> {
        self.terminal.draw(|frame| {
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

        Ok(())
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create terminal renderer")
    }
}
