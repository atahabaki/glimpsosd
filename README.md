# glimpsosd

> ⚡ A blazingly fast, customizable OSD that just *glimpses* — crafted for tiling WM nerds 🧠 who crave peace ☮️, performance 🚀, and zero scripting ✨.

## 🧠 Built for the Tiling WM Nerds Who Crave a Bit of DE Comfort

Let’s face it: just because we use tiling window managers doesn’t mean we don’t appreciate the comfy vibe of a full desktop environment. Most DEs come packed with OSDs that show things like:

* 🔠 **Caps Lock state** (yes, we hit it by accident too)
* 🔋 **Power profile updates** when your laptop decides it's sleepy
* 💡 **Keyboard and screen backlight feedback** (a.k.a. “is this thing even working?”)
* 🔊 **Volume level indicators** when you accidentally max out your speakers at 2am

Why should DE folks have all the cozy little niceties?

## 🎛️ Customization That Won’t Fry Your Brain

`glimpsosd` is made for people who like **clean configs**, **zero bloat**, and **no black magic**.

* 🎨 **Stylable/Themable** — Tweak your vibe with CSS. Or don’t. We’re not your parent. (See `examples/style.css` for inspiration.)
* 🧩 **Enable/disable built-in modules** — don’t need volume? Ditch it.
* 🧠 **Scriptable behavior** — Integrates with any WM you throw at it: Sway, Hyprland, i3, bspwm... bring it on.

🧘‍♀️ **Bonus:** it won’t yell at you with Python errors 🐍💥.

## 🛠️ Installation

*Soon*: real package instructions.

For now, clone and build:

```bash
git clone https://github.com/atahabaki/glimpsosd
cd glimpsosd
cargo build --release
```

## 🌀 Hyprland Integration

Running Hyprland? Here’s a no-nonsense snippet to get you glimpsing in style:

```hypr
### Autostart
exec-once = glimpsosd

### Bindings
bindle = , XF86KbdBrightnessUp, exec, glimpsosdctl brightness keyboard increase
bindle = , XF86KbdBrightnessDown, exec, glimpsosdctl brightness keyboard decrease
bindle = , XF86MonBrightnessUp, exec, glimpsosdctl brightness keyboard increase
bindle = , XF86MonBrightnessDown, exec, glimpsosdctl brightness keyboard decrease
# Power profile key (often found on gaming laptops)
bindl = , XF86Launch4, exec, glimpsosdctl power-profile next
```

Stick that in your ~/.config/hypr/hyprland.conf, and you're golden.

## 💖 Contribute, Chill, Repeat

Spotted a bug? Be a hero — squash it for the good of modern civilization. 🦸‍♂️
Need backup? We’re an army. Just airdrop it our way. 😎

Got a wild idea? Want to theme your notifications like a lava lamp?
Open a PR, drop an issue, or start a weird, wonderful conversation. 🙃

## 👀 Screenshots / Demos

Coming soon: animated GIFs of system change euphoria.

