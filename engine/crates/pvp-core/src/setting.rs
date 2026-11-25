//! Declarative settings: a module lists its settings once (id, label, kind, default, bounds) and the menu,
//! the profile files and the migrations all read that list.

/// What a setting holds, with its default and bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Toggle {
        default: bool,
    },
    /// A number in `min..=max`, snapped to `step` (0 for no snapping).
    Slider {
        min: f32,
        max: f32,
        step: f32,
        default: f32,
    },
    /// One of `options` (stable ids, stored by id so reordering options keeps players' choices).
    Choice {
        options: &'static [&'static str],
        default: usize,
    },
    /// An `0xAARRGGBB` colour.
    Color {
        default: u32,
    },
    /// A key name as the game spells it (`key.keyboard.r`), or empty for unbound.
    Key {
        default: &'static str,
    },
    /// Free text of at most `max` characters.
    Text {
        default: &'static str,
        max: usize,
    },
}

/// One setting of a module.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Setting {
    /// Stable id: the key in the profile file. Never translated, never reused for another meaning.
    pub id: &'static str,
    /// Translation key of its label.
    pub label: &'static str,
    pub kind: Kind,
}

impl Setting {
    pub const fn toggle(id: &'static str, label: &'static str, default: bool) -> Self {
        Self {
            id,
            label,
            kind: Kind::Toggle { default },
        }
    }
    pub const fn slider(
        id: &'static str,
        label: &'static str,
        min: f32,
        max: f32,
        step: f32,
        default: f32,
    ) -> Self {
        Self {
            id,
            label,
            kind: Kind::Slider {
                min,
                max,
                step,
                default,
            },
        }
    }
    pub const fn choice(
        id: &'static str,
        label: &'static str,
        options: &'static [&'static str],
        default: usize,
    ) -> Self {
        Self {
            id,
            label,
            kind: Kind::Choice { options, default },
        }
    }
    pub const fn color(id: &'static str, label: &'static str, default: u32) -> Self {
        Self {
            id,
            label,
            kind: Kind::Color { default },
        }
    }
    pub const fn key(id: &'static str, label: &'static str, default: &'static str) -> Self {
        Self {
            id,
            label,
            kind: Kind::Key { default },
        }
    }
    pub const fn text(
        id: &'static str,
        label: &'static str,
        default: &'static str,
        max: usize,
    ) -> Self {
        Self {
            id,
            label,
            kind: Kind::Text { default, max },
        }
    }

    /// The value a fresh profile has.
    pub fn default_value(&self) -> Value {
        match self.kind {
            Kind::Toggle { default } => Value::Bool(default),
            Kind::Slider { default, .. } => Value::Num(default),
            Kind::Choice { default, .. } => Value::Choice(default),
            Kind::Color { default } => Value::Color(default),
            Kind::Key { default } => Value::Key(default.to_owned()),
            Kind::Text { default, .. } => Value::Text(default.to_owned()),
        }
    }

    /// `value` made valid for this setting: clamped and snapped, or the default when it is of another kind.
    pub fn sanitize(&self, value: Value) -> Value {
        match (self.kind, value) {
            (Kind::Toggle { .. }, Value::Bool(b)) => Value::Bool(b),
            (
                Kind::Slider {
                    min,
                    max,
                    step,
                    default,
                },
                Value::Num(n),
            ) => {
                if !n.is_finite() {
                    return Value::Num(default);
                }
                let mut n = n.clamp(min, max);
                if step > 0.0 {
                    n = (min + ((n - min) / step).round() * step).clamp(min, max);
                }
                Value::Num(n)
            }
            (Kind::Choice { options, default }, Value::Choice(i)) => {
                Value::Choice(if i < options.len() { i } else { default })
            }
            (Kind::Color { .. }, Value::Color(c)) => Value::Color(c),
            (Kind::Key { .. }, Value::Key(k)) => Value::Key(k),
            (Kind::Text { max, .. }, Value::Text(t)) => Value::Text(t.chars().take(max).collect()),
            _ => self.default_value(),
        }
    }

    /// Reads this setting from its profile form. `None` (keep the default) for a value of the wrong type.
    pub fn from_toml(&self, value: &toml::Value) -> Option<Value> {
        let read = match (self.kind, value) {
            (Kind::Toggle { .. }, toml::Value::Boolean(b)) => Value::Bool(*b),
            (Kind::Slider { .. }, toml::Value::Float(f)) => Value::Num(*f as f32),
            (Kind::Slider { .. }, toml::Value::Integer(i)) => Value::Num(*i as f32),
            (Kind::Choice { options, .. }, toml::Value::String(s)) => {
                Value::Choice(options.iter().position(|o| o == s)?)
            }
            (Kind::Color { .. }, toml::Value::String(s)) => Value::Color(parse_color(s)?),
            (Kind::Key { .. }, toml::Value::String(s)) => Value::Key(s.clone()),
            (Kind::Text { .. }, toml::Value::String(s)) => Value::Text(s.clone()),
            _ => return None,
        };
        Some(self.sanitize(read))
    }

    /// The profile form of `value`.
    pub fn to_toml(&self, value: &Value) -> toml::Value {
        match (self.kind, value) {
            (Kind::Choice { options, default }, Value::Choice(i)) => toml::Value::String(
                options
                    .get(*i)
                    .or(options.get(default))
                    .copied()
                    .unwrap_or("")
                    .to_owned(),
            ),
            (_, Value::Bool(b)) => toml::Value::Boolean(*b),
            (_, Value::Num(n)) => toml::Value::Float(f64::from(*n)),
            (_, Value::Choice(i)) => toml::Value::Integer(*i as i64),
            (_, Value::Color(c)) => toml::Value::String(format!("#{c:08X}")),
            (_, Value::Key(k)) | (_, Value::Text(k)) => toml::Value::String(k.clone()),
        }
    }
}

/// A setting's current value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Num(f32),
    Choice(usize),
    Color(u32),
    Key(String),
    Text(String),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        matches!(self, Value::Bool(true))
    }
    pub fn as_f32(&self) -> f32 {
        match self {
            Value::Num(n) => *n,
            _ => 0.0,
        }
    }
    pub fn as_choice(&self) -> usize {
        match self {
            Value::Choice(i) => *i,
            _ => 0,
        }
    }
    pub fn as_color(&self) -> u32 {
        match self {
            Value::Color(c) => *c,
            _ => 0xFFFF_FFFF,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Value::Key(s) | Value::Text(s) => s,
            _ => "",
        }
    }
}

/// `#AARRGGBB` or `#RRGGBB` (opaque).
pub fn parse_color(s: &str) -> Option<u32> {
    let hex = s.strip_prefix('#')?;
    let v = u32::from_str_radix(hex, 16).ok()?;
    match hex.len() {
        8 => Some(v),
        6 => Some(0xFF00_0000 | v),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sliders_clamp_and_snap_to_their_step() {
        let s = Setting::slider("gap", "x", 0.0, 8.0, 0.5, 2.0);
        assert_eq!(s.sanitize(Value::Num(3.3)), Value::Num(3.5));
        assert_eq!(s.sanitize(Value::Num(99.0)), Value::Num(8.0));
        assert_eq!(s.sanitize(Value::Num(f32::NAN)), Value::Num(2.0));
    }

    #[test]
    fn a_value_of_the_wrong_kind_falls_back_to_the_default() {
        let s = Setting::toggle("on", "x", true);
        assert_eq!(s.sanitize(Value::Num(1.0)), Value::Bool(true));
        assert_eq!(s.from_toml(&toml::Value::String("yes".into())), None);
    }

    #[test]
    fn choices_are_stored_by_id_and_unknown_ids_are_refused() {
        let s = Setting::choice("shape", "x", &["cross", "dot", "circle"], 0);
        assert_eq!(
            s.to_toml(&Value::Choice(2)),
            toml::Value::String("circle".into())
        );
        assert_eq!(
            s.from_toml(&toml::Value::String("dot".into())),
            Some(Value::Choice(1))
        );
        assert_eq!(s.from_toml(&toml::Value::String("star".into())), None);
    }

    #[test]
    fn colours_round_trip_through_their_hex_form() {
        let s = Setting::color("c", "x", 0);
        let v = Value::Color(0x80FF_0010);
        assert_eq!(s.from_toml(&s.to_toml(&v)), Some(v));
        assert_eq!(parse_color("#00FF00"), Some(0xFF00_FF00));
        assert_eq!(parse_color("00FF00"), None);
    }
}
