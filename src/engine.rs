use windows::Win32::Foundation::RECT;

/// Pure functional coordinate math for Ichi snaps.
/// Takes current window RECT, monitor work area RECT, and snap parameters.
pub fn calculate_snap(key: u32, cycle: usize, _current: RECT, work_area: RECT) -> RECT {
    let mw = work_area.right - work_area.left;
    let mh = work_area.bottom - work_area.top;

    // Multi-tap cycle ratios (1/2, 1/3, 2/3)
    let grid_ratio = match cycle % 3 {
        0 => 0.5,
        1 => 1.0 / 3.0,
        2 => 2.0 / 3.0,
        _ => 0.5,
    };

    let (nx, ny, nw, nh) = match key {
        1 => { // Bottom Left
            let nw = (mw as f64 * grid_ratio) as i32;
            let nh = (mh as f64 * grid_ratio) as i32;
            (work_area.left, work_area.bottom - nh, nw, nh)
        }
        2 => { // Bottom Center
            let nh = (mh as f64 * grid_ratio) as i32;
            (work_area.left, work_area.bottom - nh, mw, nh)
        }
        3 => { // Bottom Right
            let nw = (mw as f64 * grid_ratio) as i32;
            let nh = (mh as f64 * grid_ratio) as i32;
            (work_area.right - nw, work_area.bottom - nh, nw, nh)
        }
        4 => { // Left
            let nw = (mw as f64 * grid_ratio) as i32;
            (work_area.left, work_area.top, nw, mh)
        }
        5 => { // Center / Maximize (100, 80, 60 ratios)
            let ratio = match cycle % 3 {
                0 => 1.0,
                1 => 0.8,
                2 => 0.6,
                _ => 1.0,
            };
            let nw = (mw as f64 * ratio) as i32;
            let nh = (mh as f64 * ratio) as i32;
            (work_area.left + (mw - nw) / 2, work_area.top + (mh - nh) / 2, nw, nh)
        }
        6 => { // Right
            let nw = (mw as f64 * grid_ratio) as i32;
            (work_area.right - nw, work_area.top, nw, mh)
        }
        7 => { // Top Left
            let nw = (mw as f64 * grid_ratio) as i32;
            let nh = (mh as f64 * grid_ratio) as i32;
            (work_area.left, work_area.top, nw, nh)
        }
        8 => { // Top Center
            let nh = (mh as f64 * grid_ratio) as i32;
            (work_area.left, work_area.top, mw, nh)
        }
        9 => { // Top Right
            let nw = (mw as f64 * grid_ratio) as i32;
            let nh = (mh as f64 * grid_ratio) as i32;
            (work_area.right - nw, work_area.top, nw, nh)
        }
        _ => (work_area.left, work_area.top, mw, mh),
    };

    RECT {
        left: nx,
        top: ny,
        right: nx + nw,
        bottom: ny + nh,
    }
}
