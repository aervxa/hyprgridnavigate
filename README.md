# *hypr*gridnavigate

I use hyprexpo for my workspace overview, I like a 3x3 and I wanted horizontal slide animation when changing workspace horizontally, but a vertical slide (slidevert) when changing workspaces horizontally (`hyprctl dispatch workspace ±3` gives the top/bottom effect).

So I wrote a script in Rust to basically do exactly that via IPC communication with Hyprland.

Usage

```bash
hyprgridnavigate left|top|right|bottom <true>
```

- first arg is for which side to move to; the script won't ever let you go out of the 3x3 grid.
- second arg is optional, it's for moving the active window as you move (movetoworkspace).
