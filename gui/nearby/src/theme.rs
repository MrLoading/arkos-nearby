#[cfg(any(target_os = "linux", test))]
use crate::model::{Page, View};
use mirui::prelude::*;
use mirui::ui::theme::{Theme, ThemeInfo};
#[cfg(any(target_os = "linux", test))]
use std::path::Path;

#[cfg(target_os = "linux")]
pub const SETTINGS: &str = "/opt/arkos-nearby/ui-theme";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Palette {
    #[default]
    Forest,
    Sunset,
    Night,
}

impl Palette {
    pub const ALL: [Self; 3] = [Self::Forest, Self::Sunset, Self::Night];

    pub fn id(self) -> &'static str {
        match self {
            Self::Forest => "forest",
            Self::Sunset => "sunset",
            Self::Night => "night",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|theme| theme.id() == value.trim())
    }

    #[cfg(any(target_os = "linux", test))]
    pub fn next(self) -> Self {
        match self {
            Self::Forest => Self::Sunset,
            Self::Sunset => Self::Night,
            Self::Night => Self::Forest,
        }
    }

    #[cfg(any(target_os = "linux", test))]
    pub fn can_switch(view: &View) -> bool {
        matches!(view.page, Page::Mode | Page::Choice) && !view.busy
    }

    #[cfg(any(target_os = "linux", test))]
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|value| Self::parse(&value))
            .unwrap_or_default()
    }

    #[cfg(any(target_os = "linux", test))]
    pub fn save(self, path: &Path) -> std::io::Result<()> {
        use std::io::Write;
        let temporary = path.with_extension("tmp");
        let mut file = std::fs::File::create(&temporary)?;
        writeln!(file, "{}", self.id())?;
        file.sync_all()?;
        std::fs::rename(temporary, path)?;
        if let Some(parent) = path.parent() {
            std::fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }

    pub fn theme(self) -> Theme {
        use ColorToken::*;
        let values = match self {
            Self::Forest => [
                0xf2eddb, 0x293a31, 0x42664c, 0xe6e7d3, 0xbdc6a9, 0x63705b, 0xc68c3d, 0xad573b,
                0xd6dfc7, 0xaabda0, 0x243f33, 0xf2eddb,
            ],
            Self::Sunset => [
                0xf5e9d4, 0x482f29, 0xa74e37, 0xecddc4, 0xcebaa0, 0x82614f, 0xc18a32, 0x70835c,
                0xe8cdb0, 0xc49e81, 0x623b30, 0xf5e9d4,
            ],
            Self::Night => [
                0x283933, 0xf2e8c9, 0xd2aa65, 0x344b40, 0x58705b, 0xb5c6a4, 0xdcaa56, 0xde8760,
                0x3c5547, 0x577058, 0x1c2e26, 0x1c2e26,
            ],
        };
        let tokens = [
            Surface,
            OnSurface,
            Primary,
            SurfaceVariant,
            Outline,
            OnSurfaceVariant,
            Custom("gold"),
            Custom("clay"),
            Custom("sky"),
            Custom("hill"),
            Custom("deep"),
            OnPrimary,
        ];
        let mut theme = Theme::light().with_info(ThemeInfo::new(self.id(), self.id(), ""));
        for (token, rgb) in tokens.into_iter().zip(values) {
            theme.set(
                token,
                Color::rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8),
            );
        }
        theme.set(Custom("header-text"), Color::rgb(243, 236, 216));
        theme.set(Custom("header-detail"), Color::rgb(199, 208, 181));
        theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preferences_round_trip_and_unknown_values_use_default() {
        let path = std::env::temp_dir().join(format!("nearby-theme-{}", std::process::id()));
        assert_eq!(Palette::load(&path), Palette::Forest);
        for palette in Palette::ALL {
            palette.save(&path).unwrap();
            assert_eq!(Palette::load(&path), palette);
        }
        std::fs::write(&path, "unknown").unwrap();
        assert_eq!(Palette::load(&path), Palette::Forest);
        std::fs::remove_file(path).unwrap();
    }
}
