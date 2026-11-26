"""GUI-pixel click points of the menu for tools/e2e-local.sh, from the menu's own layout (pvp-ui menu.rs/theme.rs):
the 1391 x 700 dp window, centred, at min(0.9 W / 1391, 0.9 H / 700, 1) physical pixels per dp (at least 0.25).

Usage: python e2e_points.py <horizon-ctl>  → prints: footer_x footer_y body_x body_y edit_hud_x edit_hud_y
"""
import json
import os
import subprocess
import sys
import tempfile

ctl = sys.argv[1]
# The window's size in physical pixels: a screenshot's reply gives it.
shot = os.path.join(tempfile.gettempdir(), "pvp-e2e-size.png")
size = json.loads(subprocess.run([ctl, "screenshot", shot], capture_output=True, text=True).stdout)
W, H = size["width"], size["height"]
ui = json.loads(subprocess.run([ctl, "ui"], capture_output=True, text=True).stdout)
gw, gh = ui["gui_size"]
gui_scale = W / gw
phys = max(0.25, min(0.9 * W / 1391, 0.9 * H / 700, 1.0))
k = phys / gui_scale
ox, oy = (gw - 1391 * k) / 2, (gh - 700 * k) / 2


def gui(x, y):
    return f"{ox + x * k:.1f} {oy + y * k:.1f}"


card_x, card_y = 264 + 25, 19 + 36 + 19 + 32 + 19  # sidebar + gutter; header, chips row
print(
    gui(card_x + 127.5, card_y + 140 - 18),  # the first card's footer
    gui(card_x + 127.5, card_y + 52),  # its body
    gui(19.5 + 112, 700 - 73 - 18 - 33 + 16.5),  # Edit HUD
)
