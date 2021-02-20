use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = FullMode::new()?;
    let mut input = NcInput::new_empty();
    let mut stdplane = nc.stdplane();

    let welcome = format!("Welcome to lartem!

You, {} are a lowly {} toiling away in the basement of of a nameless, faceless, heartless corporation.

One day it all just gets too much, and so you rise from your Aeron, take up your trusty keyboard, and set about realigning the attitudes of those around you.

Press the ANY key to continue...
",
        "<name>",
        "<role>",
    );

    // no wrapper method for puttext yet
    unsafe { ncplane_puttext(stdplane, 0, NCALIGN_LEFT, cstring![welcome], core::ptr::null_mut()) };

    nc.render()?;
    nc.getc_blocking(None)?;

    stdplane.erase();

    let (ydim, xdim) = stdplane.dim_yx();

    stdplane.putstr_aligned(0, NCALIGN_CENTER, "lartem2")?;
    // there are also ncplane_vprintf functions but rust's format is probably safer
    stdplane.putstr_aligned(1, NCALIGN_RIGHT, &format!("notcurses {}", FullMode::version()))?;
    stdplane.putstr_aligned(2, NCALIGN_RIGHT, &format!("stdplane {}x{}", ydim, xdim))?;

    // TODO: Define planes representing the main game board, a status display, a message buffer, and maybe a
    // minimap if we decide that the board is scrollable.  Arrange them in an efficient and aesthetically
    // pleasing manner according to the dimensions of stdplane.  Maybe some of them will be overlaid
    // translucently onto others because we can.  It might be convenient to have a wider 'frame' plane
    // underneath each to display a border without it interfering with coordinates.
    let board = NcPlane::with_options_bound(&mut stdplane, NcPlaneOptions::new_aligned(3, NCALIGN_CENTER, 24, 80))?;

    // Temp hack to add a background
    // I haven't bothered to read up on borrowing yet so I don't know why we need an intermediate variable here
    {
        let cell = NcCell::with_char('.', board);
        board.set_base_cell(&cell)?;
    }

    board.perimeter_rounded(NCSTYLE_NONE, 0, 0)?;

    let mut player_cell = NcCell::with_char('@', board);
    board.on_styles(NCSTYLE_BOLD);
    player_cell.on_styles(NCSTYLE_BOLD);
    board.putc_yx(12, 40, &player_cell)?;

    let messages = NcPlane::new_bound(&mut stdplane, 27, 0, 5, 80)?;
    messages.set_scrolling(true);

    messages.putstrln("Welcome to lartem!")?;

    loop {
        nc.render()?;

        let key = nc.getc_blocking(Some(&mut input))?;

        match key {
            NCKEY_RESIZE => (),  // TODO handle resize
            'l' if input.ctrl => { nc.refresh()?; }
            'q' => break,
            _ => (),
        }

    }

    Ok(())
}
