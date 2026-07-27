//! `cueforge-renderer`
//!
//! Reference ASCII and text visual renderer for simulation state inspection.

use cueforge_physics::World;
use cueforge_table::TableSpec;

/// Render an ASCII representation of the pool table bed and ball locations.
pub fn render_ascii_table(world: &World, table: &TableSpec, cols: usize, rows: usize) -> String {
    let half_w = table.width.0 / 2.0;
    let half_l = table.length.0 / 2.0;

    let mut grid = vec![vec!['.'; cols]; rows];

    // Render table boundary frame
    for item in grid[0].iter_mut().take(cols) {
        *item = '#';
    }
    for item in grid[rows - 1].iter_mut().take(cols) {
        *item = '#';
    }
    for row in grid.iter_mut().take(rows) {
        row[0] = '#';
        row[cols - 1] = '#';
    }

    // Render pockets ('O')
    for pocket in &table.pockets {
        let norm_x = (pocket.position.x + half_w) / table.width.0;
        let norm_y = (pocket.position.y + half_l) / table.length.0;

        let col = ((norm_x * (cols as f64 - 3.0)) as usize + 1).clamp(1, cols - 2);
        let row = ((norm_y * (rows as f64 - 3.0)) as usize + 1).clamp(1, rows - 2);
        grid[row][col] = 'O';
    }

    // Render balls ('0'..'9', 'C')
    for ball in &world.balls {
        if !ball.is_active() {
            continue;
        }

        let norm_x = (ball.position.x + half_w) / table.width.0;
        let norm_y = (ball.position.y + half_l) / table.length.0;

        let col = ((norm_x * (cols as f64 - 3.0)) as usize + 1).clamp(1, cols - 2);
        let row = ((norm_y * (rows as f64 - 3.0)) as usize + 1).clamp(1, rows - 2);

        let symbol = match ball.id.0 {
            0 => 'C', // Cue ball
            id if id < 10 => (b'0' + id as u8) as char,
            _ => '*',
        };
        grid[row][col] = symbol;
    }

    let mut output = String::new();
    for row in grid {
        let line: String = row.into_iter().collect();
        output.push_str(&line);
        output.push('\n');
    }
    output
}
