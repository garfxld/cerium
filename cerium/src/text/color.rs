use serde::{Deserialize, Serialize, Serializer};

// ===== Rgb =====

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn of(hex: u32) -> Self {
        let [_, r, g, b] = hex.to_be_bytes();
        Self { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let r = self.r();
        let g = self.g();
        let b = self.b();

        let s = format!("#{r:02x}{g:02x}{b:02x}");
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;

        if let Ok(color) = NamedColor::try_from(s.as_str()) {
            return Ok(color.into());
        }

        // not a NamedColor, therefore the prefix is required.
        let hex = s.strip_prefix('#').unwrap();

        if hex.len() != 6 {
            return Err(D::Error::custom(format!("invalid hex color: {}", s)));
        }

        let rgb = u32::from_str_radix(hex, 16)
            .map_err(|_| D::Error::custom(format!("invalid hex color: {}", s)))?;

        let r = ((rgb >> 16) & 0xFF) as u8;
        let g = ((rgb >> 8) & 0xFF) as u8;
        let b = (rgb & 0xFF) as u8;
        Ok(Rgb { r, g, b })
    }
}

impl From<Rgba> for Rgb {
    fn from(value: Rgba) -> Self {
        Self {
            r: value.r(),
            g: value.g(),
            b: value.b(),
        }
    }
}

// ===== Rgba =====

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn of(hex: u32) -> Self {
        let [a, r, g, b] = hex.to_be_bytes();
        Self { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

impl Serialize for Rgba {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let r = self.r();
        let g = self.g();
        let b = self.b();
        let a = self.a();

        let s = format!("#{r:02x}{g:02x}{b:02x}{a:02x}");
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;

        // the prefix is required.
        let hex = s.strip_prefix('#').unwrap();

        if hex.len() != 8 {
            return Err(D::Error::custom(format!("invalid hex color: {}", s)));
        }

        let rgba = u32::from_str_radix(hex, 16)
            .map_err(|_| D::Error::custom(format!("invalid hex color: {}", s)))?;

        let r = ((rgba >> 24) & 0xFF) as u8;
        let g = ((rgba >> 16) & 0xFF) as u8;
        let b = ((rgba >> 8) & 0xFF) as u8;
        let a = (rgba & 0xFF) as u8;
        Ok(Rgba { r, g, b, a })
    }
}

impl From<Rgb> for Rgba {
    fn from(value: Rgb) -> Self {
        Self {
            r: value.r(),
            g: value.g(),
            b: value.b(),
            a: u8::MAX,
        }
    }
}

// ===== NamedColor =====

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl From<NamedColor> for Rgb {
    fn from(value: NamedColor) -> Self {
        match value {
            NamedColor::Black => Rgb::of(0x000000),
            NamedColor::DarkBlue => Rgb::of(0x0000AA),
            NamedColor::DarkGreen => Rgb::of(0x00AA00),
            NamedColor::DarkAqua => Rgb::of(0x00AAAA),
            NamedColor::DarkRed => Rgb::of(0xAA0000),
            NamedColor::DarkPurple => Rgb::of(0xAA00AA),
            NamedColor::Gold => Rgb::of(0xFFAA00),
            NamedColor::Gray => Rgb::of(0xAAAAAA),
            NamedColor::DarkGray => Rgb::of(0x555555),
            NamedColor::Blue => Rgb::of(0x5555FF),
            NamedColor::Green => Rgb::of(0x55FF55),
            NamedColor::Aqua => Rgb::of(0x55FFFF),
            NamedColor::Red => Rgb::of(0xFF5555),
            NamedColor::LightPurple => Rgb::of(0xFF55FF),
            NamedColor::Yellow => Rgb::of(0xFFFF55),
            NamedColor::White => Rgb::of(0xFFFFFF),
        }
    }
}

impl From<NamedColor> for Rgba {
    fn from(value: NamedColor) -> Self {
        Rgb::from(value).into()
    }
}

impl TryFrom<&str> for NamedColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "black" => Ok(NamedColor::Black),
            "dark_blue" => Ok(NamedColor::DarkBlue),
            "dark_green" => Ok(NamedColor::DarkGreen),
            "dark_aqua" => Ok(NamedColor::DarkAqua),
            "dark_red" => Ok(NamedColor::DarkRed),
            "dark_purple" => Ok(NamedColor::DarkPurple),
            "gold" => Ok(NamedColor::Gold),
            "gray" => Ok(NamedColor::Gray),
            "dark_gray" => Ok(NamedColor::DarkGray),
            "blue" => Ok(NamedColor::Blue),
            "green" => Ok(NamedColor::Green),
            "aqua" => Ok(NamedColor::Aqua),
            "red" => Ok(NamedColor::Red),
            "light_purple" => Ok(NamedColor::LightPurple),
            "yellow" => Ok(NamedColor::Yellow),
            "white" => Ok(NamedColor::White),
            _ => Err(()),
        }
    }
}
