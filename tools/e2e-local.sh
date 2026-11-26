#!/usr/bin/env bash
# Local end-to-end check of the client in a real game, driven by horizon-ctl (no GitHub Actions).
# Needs the prepared resources (engine/.local/horizon-client/26.3) and a built client:
#   cd engine && cargo build -p pvp-client && cargo build -p horizon-client-lib --bin horizon-ctl
# Uses its own game folder (engine/.local/e2e-game) and starts from a fresh profile. Steps:
#   world loads → Right Shift opens the menu → the FPS card's footer turns the module on → the card opens its
#   page → Edit HUD → dragging the FPS element moves it → Escape → the profile on disk has the module on and
#   the element moved. Screenshots go to engine/.local/e2e-game/shots/. Exit status 0 when every check passed.
set -uo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root/engine"
ctl=./target/debug/horizon-ctl.exe; [[ -x $ctl ]] || ctl=./target/debug/horizon-ctl
game=$(pwd)/target/debug/pvp-client.exe; [[ -x $game ]] || game=$(pwd)/target/debug/pvp-client
dir=$(pwd)/.local/e2e-game
profile=$dir/pvp-client/profiles/default.toml
mkdir -p "$dir/shots"
rm -rf "$dir/pvp-client"
fail=0
# `quit` answers before the process ends: wait until no client is left (at most 60 s).
stop_game() {
  $ctl quit >/dev/null 2>&1 || true
  for _ in $(seq 60); do
    if command -v tasklist >/dev/null; then
      tasklist 2>/dev/null | grep -qi "pvp-client" || return 0
    else
      pgrep -f pvp-client >/dev/null || return 0
    fi
    sleep 1
  done
}
check() { if eval "$2"; then echo "ok   $1"; else echo "FAIL $1"; fail=1; fi; }
stop_game
$ctl launch --world PvpTest --creative --game "$game" --wait \
  --resources "$(pwd)/.local/horizon-client/26.3" --game-dir "$dir" > "$dir/launch.json" 2>&1
check "world loads" 'grep -q "\"screen\":\"playing\"" "$dir/launch.json"'
# Points are read while playing: the state has no renderer section while a screen is open.
read -r fx fy bx by hx hy < <(python "$root/tools/e2e_points.py" "$ctl")
$ctl key rshift tap >/dev/null; $ctl wait 20 >/dev/null
check "Right Shift opens the menu" '[[ $($ctl state | python -c "import json,sys; print(json.load(sys.stdin)[\"screen\"])") == mod ]]'
$ctl screenshot "$dir/shots/menu.png" >/dev/null
# The first card's footer and body and the sidebar's Edit HUD, in GUI pixels (tools/e2e_points.py).
$ctl click-at "$fx" "$fy" >/dev/null; $ctl wait 10 >/dev/null
$ctl click-at "$bx" "$by" >/dev/null; $ctl wait 10 >/dev/null
$ctl screenshot "$dir/shots/module-page.png" >/dev/null
$ctl click-at "$hx" "$hy" >/dev/null; $ctl wait 10 >/dev/null
$ctl drag-at 20 8 60 40 120 80 160 110 >/dev/null; $ctl wait 10 >/dev/null
$ctl screenshot "$dir/shots/hud-editor.png" >/dev/null
$ctl key esc tap >/dev/null; $ctl wait 30 >/dev/null
check "the FPS module is on in the profile" 'grep -A1 "^\[modules.fps\]" "$profile" | grep -q "enabled = true"'
check "the FPS element moved" '! grep -A4 "hud.counter" "$profile" | grep -q "^x = 4.0"'
stop_game
echo "e2e-local $(git rev-parse --short HEAD): $([[ $fail == 0 ]] && echo PASS || echo FAIL)"
exit $fail
