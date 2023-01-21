use ncurses;

fn main() {
    /* If your locale env is unicode, you should use `setlocale`. */
    // let locale_conf = LcCategory::all;
    // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

    /* Start ncurses. */
    ncurses::initscr();

    /* Print to the back buffer. */
    ncurses::addstr("Hello, world!");

    /* Print some unicode(Chinese) string. */
    // addstr("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Update the screen. */
    ncurses::refresh();

    /* Wait for a key press. */
    ncurses::getch();

    /* Terminate ncurses. */
    ncurses::endwin();
}
