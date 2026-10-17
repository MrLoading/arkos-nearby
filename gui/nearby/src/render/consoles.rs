use mirui::prelude::*;
use mirui::ui::widgets::Image;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Platform {
    Unknown,
    Favorites,
    Nes,
    Fds,
    Snes,
    Megadrive,
    Mastersystem,
    Segacd,
    Sega32X,
    Pcengine,
    Arcade,
    Neogeo,
    Cps1,
    Cps2,
    Cps3,
    Psx,
    Gb,
    Gbc,
    Gba,
}
impl Platform {
    pub(super) fn from_id(id: &str) -> Self {
        match id {
            "favorites" => Self::Favorites,
            "nes" => Self::Nes,
            "fds" => Self::Fds,
            "snes" => Self::Snes,
            "megadrive" => Self::Megadrive,
            "mastersystem" => Self::Mastersystem,
            "segacd" => Self::Segacd,
            "sega32x" => Self::Sega32X,
            "pcengine" => Self::Pcengine,
            "arcade" => Self::Arcade,
            "neogeo" => Self::Neogeo,
            "cps1" => Self::Cps1,
            "cps2" => Self::Cps2,
            "cps3" => Self::Cps3,
            "psx" => Self::Psx,
            "gb" => Self::Gb,
            "gbc" => Self::Gbc,
            "gba" => Self::Gba,
            _ => Self::Unknown,
        }
    }
}

#[compose]
pub(super) fn console(platform: Platform) -> Entity {
    ui! {
        View (grow: 1.0, min_height: 0, min_width: 0) {
            match platform {
                Platform :: Favorites => {
                    Image (
                        src: path!(M 27 15 H 103 V 94 H 27 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(137, 146, 120)
                    )
                    Image (
                        src: path!(M 37 23 H 113 V 102 H 37 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(77, 101, 78)
                    )
                    Image (
                        src: path!(M 47 31 H 123 V 110 H 47 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(176, 184, 155)
                    )
                    Image (
                        src: path!(M 53 38 H 117 V 84 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 66, 52)
                    )
                    Image (
                        src: path!(M 61 45 H 109 V 77 H 61 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(204, 148, 66)
                    )
                    Image (
                        src: path!(M 58 91 H 113 V 101 H 58 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(58, 83, 65)
                    )
                    Image (
                        src: path!(M 62 95 H 109 V 101 H 62 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(212, 181, 110)
                    )
                    Image (
                        src: path!(
                            M 85 48 V 55 H 78 V 62 H 71 V 69 H 80 V 78 H 91 V 69 H 100 V 62 H 93 V 55 H 86 V
                            48 Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(239, 232, 201)
                    )
                }
                Platform :: Nes => {
                    Image (
                        src: path!(M 19 30 H 142 V 89 H 19 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 21 27 H 140 V 78 H 21 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 21 68 H 140 V 82 H 21 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 42 27 H 119 V 63 H 42 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(179, 179, 154)
                    )
                    Image (
                        src: path!(M 47 32 H 114 V 57 H 47 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 24 49 H 36 V 59 H 24 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 122 49 H 134 V 59 H 122 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 44 86 H 69 V 91 H 44 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 101 86 H 126 V 91 H 101 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Fds => {
                    Image (
                        src: path!(M 20 38 H 140 V 85 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 23 35 H 137 V 81 H 23 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 28 40 H 132 V 53 H 28 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 39 59 H 119 V 73 H 39 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 44 61 H 114 V 64 H 44 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 119 65 H 128 V 74 H 119 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 63 19 H 98 V 46 H 63 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 68 22 H 93 V 35 H 68 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 70 37 H 90 V 44 H 70 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                }
                Platform :: Snes => {
                    Image (
                        src: path!(M 20 33 H 140 V 87 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 23 27 H 137 V 84 H 23 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 35 28 H 126 V 50 H 35 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 45 32 H 117 V 40 H 45 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 44 57 H 62 V 72 H 44 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(139, 119, 161)
                    )
                    Image (
                        src: path!(M 99 57 H 117 V 72 H 99 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(139, 119, 161)
                    )
                    Image (
                        src: path!(M 66 65 H 93 V 68 H 66 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 35 79 H 62 V 85 H 35 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 100 79 H 127 V 85 H 100 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Megadrive => {
                    Image (
                        src: path!(M 20 34 H 139 V 81 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 24 30 H 135 V 74 H 24 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(65, 79, 69)
                    )
                    Image (
                        src: path!(
                            M 105 51 L 103 61 L 98 69 L 90 74 L 80 76 L 70 74 L 62 69 L 57 61 L 55 51 L 57 41
                            L 62 33 L 70 28 L 80 26 L 90 28 L 98 33 L 103 41 Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 48 41 H 115 V 49 H 48 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 27 59 H 40 V 65 H 27 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 35 72 H 62 V 79 H 35 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 97 72 H 124 V 79 H 97 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Mastersystem => {
                    Image (
                        src: path!(M 17 37 H 143 V 85 H 17 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 20 34 H 140 V 75 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(66, 82, 71)
                    )
                    Image (
                        src: path!(M 24 39 H 136 V 47 H 24 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 51 48 H 113 V 56 H 51 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 27 64 H 50 V 70 H 27 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 67 74 H 96 V 82 H 67 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 106 74 H 135 V 82 H 106 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Segacd => {
                    Image (
                        src: path!(M 12 38 H 151 V 98 H 12 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 16 41 H 147 V 94 H 16 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(86, 98, 86)
                    )
                    Image (
                        src: path!(M 18 83 H 99 V 88 H 18 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 121 84 H 139 V 88 H 121 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 16 34 H 135 V 81 H 16 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 20 30 H 131 V 74 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(65, 79, 69)
                    )
                    Image (
                        src: path!(
                            M 105 51 L 103 61 L 98 69 L 90 74 L 80 76 L 70 74 L 62 69 L 57 61 L 55 51 L 57 41
                            L 62 33 L 70 28 L 80 26 L 90 28 L 98 33 L 103 41 Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 48 41 H 115 V 49 H 48 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 27 59 H 40 V 65 H 27 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 35 72 H 62 V 79 H 35 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 97 72 H 124 V 79 H 97 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Sega32X => {
                    Image (
                        src: path!(M 20 34 H 139 V 81 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 24 30 H 135 V 74 H 24 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(65, 79, 69)
                    )
                    Image (
                        src: path!(
                            M 105 51 L 103 61 L 98 69 L 90 74 L 80 76 L 70 74 L 62 69 L 57 61 L 55 51 L 57 41
                            L 62 33 L 70 28 L 80 26 L 90 28 L 98 33 L 103 41 Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 48 41 H 115 V 49 H 48 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 27 59 H 40 V 65 H 27 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 35 72 H 62 V 79 H 35 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 97 72 H 124 V 79 H 97 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 49 17 H 110 V 45 H 49 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 54 13 H 105 V 41 H 54 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(89, 103, 90)
                    )
                    Image (
                        src: path!(M 62 16 H 97 V 23 H 62 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 58 33 H 102 V 37 H 58 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                }
                Platform :: Pcengine => {
                    Image (
                        src: path!(M 39 26 H 121 V 92 H 39 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 41 24 H 119 V 86 H 41 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 48 28 H 112 V 61 H 48 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 54 32 H 106 V 60 H 54 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 58 34 H 102 V 56 H 58 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 49 68 H 93 V 73 H 49 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 101 67 H 112 V 76 H 101 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(164, 118, 73)
                    )
                    Image (
                        src: path!(M 68 82 H 92 V 88 H 68 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Arcade => {
                    Image (
                        src: path!(M 42 8 H 117 V 26 H 42 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 46 12 H 113 V 21 H 46 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 47 26 H 110 V 77 H 47 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 54 30 H 102 V 62 H 54 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 58 34 H 98 V 58 H 58 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(88, 118, 107)
                    )
                    Image (
                        src: path!(M 47 65 H 110 L 123 80 V 89 H 34 V 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 40 89 H 117 V 114 H 40 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 47 91 H 110 V 110 H 47 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(82, 105, 88)
                    )
                    Image (
                        src: path!(M 53 72 H 56 V 81 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 50 69 H 59 V 74 H 50 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 88 75 H 95 V 79 H 88 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 101 75 H 108 V 79 H 101 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 64 97 H 91 V 101 H 64 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Neogeo => {
                    Image (
                        src: path!(M 15 38 H 147 V 87 H 15 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 20 32 H 142 V 80 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(71, 86, 74)
                    )
                    Image (
                        src: path!(M 42 30 H 120 V 50 H 42 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 50 34 H 114 V 41 H 50 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 23 63 H 40 V 70 H 23 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 113 64 H 132 V 71 H 113 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 64 H 99 V 68 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                }
                Platform :: Cps1 => {
                    Image (
                        src: path!(M 16 28 H 146 V 92 H 16 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 19 25 H 143 V 86 H 19 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(82, 128, 91)
                    )
                    Image (
                        src: path!(M 23 82 H 139 V 89 H 23 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 26 32 H 45 V 42 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 34 H 41 V 36 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 32 H 72 V 42 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 34 H 68 V 36 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 32 H 99 V 42 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 34 H 95 V 36 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 32 H 126 V 42 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 34 H 122 V 36 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 26 48 H 45 V 58 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 50 H 41 V 52 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 48 H 72 V 58 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 50 H 68 V 52 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 48 H 99 V 58 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 50 H 95 V 52 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 48 H 126 V 58 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 50 H 122 V 52 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 26 64 H 45 V 74 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 66 H 41 V 68 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 64 H 72 V 74 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 66 H 68 V 68 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 64 H 99 V 74 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 66 H 95 V 68 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 64 H 126 V 74 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 66 H 122 V 68 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 108 53 H 137 V 77 H 108 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 112 57 H 133 V 73 H 112 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 114 60 H 117 V 68 H 114 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                }
                Platform :: Cps2 => {
                    Image (
                        src: path!(M 16 28 H 146 V 92 H 16 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 19 25 H 143 V 86 H 19 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(100, 125, 154)
                    )
                    Image (
                        src: path!(M 23 82 H 139 V 89 H 23 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 26 32 H 45 V 42 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 34 H 41 V 36 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 32 H 72 V 42 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 34 H 68 V 36 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 32 H 99 V 42 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 34 H 95 V 36 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 32 H 126 V 42 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 34 H 122 V 36 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 26 48 H 45 V 58 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 50 H 41 V 52 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 48 H 72 V 58 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 50 H 68 V 52 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 48 H 99 V 58 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 50 H 95 V 52 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 48 H 126 V 58 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 50 H 122 V 52 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 26 64 H 45 V 74 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 66 H 41 V 68 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 64 H 72 V 74 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 66 H 68 V 68 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 64 H 99 V 74 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 66 H 95 V 68 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 64 H 126 V 74 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 66 H 122 V 68 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 108 53 H 137 V 77 H 108 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 112 57 H 133 V 73 H 112 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 114 60 H 117 V 68 H 114 Z M 120 60 H 123 V 68 H 120 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                }
                Platform :: Cps3 => {
                    Image (
                        src: path!(M 16 28 H 146 V 92 H 16 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 19 25 H 143 V 86 H 19 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(108, 152, 123)
                    )
                    Image (
                        src: path!(M 23 82 H 139 V 89 H 23 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(210, 165, 79)
                    )
                    Image (
                        src: path!(M 26 32 H 45 V 42 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 34 H 41 V 36 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 32 H 72 V 42 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 34 H 68 V 36 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 32 H 99 V 42 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 34 H 95 V 36 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 32 H 126 V 42 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 34 H 122 V 36 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 26 48 H 45 V 58 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 50 H 41 V 52 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 48 H 72 V 58 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 50 H 68 V 52 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 48 H 99 V 58 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 50 H 95 V 52 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 48 H 126 V 58 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 50 H 122 V 52 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 26 64 H 45 V 74 H 26 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 29 66 H 41 V 68 H 29 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 53 64 H 72 V 74 H 53 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 66 H 68 V 68 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 80 64 H 99 V 74 H 80 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 83 66 H 95 V 68 H 83 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 107 64 H 126 V 74 H 107 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 110 66 H 122 V 68 H 110 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 108 53 H 137 V 77 H 108 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 112 57 H 133 V 73 H 112 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(
                            M 114 60 H 117 V 68 H 114 Z M 120 60 H 123 V 68 H 120 Z M 126 60 H 129 V 68 H 126
                            Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                }
                Platform :: Psx => {
                    Image (
                        src: path!(M 18 32 H 142 V 85 H 18 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 20 29 H 140 V 81 H 20 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(M 39 36 H 121 V 74 H 39 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(
                            M 102 55 L 100 63 L 96 71 L 88 75 L 80 77 L 72 75 L 64 71 L 60 63 L 58 55 L 60 47
                            L 64 39 L 72 35 L 80 33 L 88 35 L 96 39 L 100 47 Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(217, 217, 189)
                    )
                    Image (
                        src: path!(
                            M 83 55 L 83 56 L 82 57 L 81 58 L 80 58 L 79 58 L 78 57 L 77 56 L 77 55 L 77 54 L
                            78 53 L 79 52 L 80 52 L 81 52 L 82 53 L 83 54 Z
                        ),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 25 40 H 34 V 48 H 25 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(132, 167, 111)
                    )
                    Image (
                        src: path!(M 126 40 H 134 V 48 H 126 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 37 79 H 64 V 84 H 37 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 95 79 H 122 V 84 H 95 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Gb => {
                    Image (
                        src: path!(M 47 7 H 112 V 105 H 47 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 50 7 H 109 V 101 H 50 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(202, 205, 184)
                    )
                    Image (
                        src: path!(M 54 17 H 105 V 60 H 54 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 58 21 H 101 V 54 H 58 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 61 24 H 98 V 51 H 61 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(166, 183, 118)
                    )
                    Image (
                        src: path!(M 61 69 H 66 V 84 H 61 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 74 H 71 V 79 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 88 72 H 95 V 79 H 88 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 98 65 H 105 V 72 H 98 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 73 88 H 81 V 91 H 73 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 85 88 H 93 V 91 H 85 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 92 97 H 106 V 99 H 92 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Gbc => {
                    Image (
                        src: path!(M 47 7 H 112 V 105 H 47 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 50 7 H 109 V 101 H 50 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(138, 101, 152)
                    )
                    Image (
                        src: path!(M 54 17 H 105 V 60 H 54 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(168, 175, 152)
                    )
                    Image (
                        src: path!(M 58 21 H 101 V 54 H 58 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 61 24 H 98 V 51 H 61 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(166, 183, 118)
                    )
                    Image (
                        src: path!(M 61 69 H 66 V 84 H 61 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 56 74 H 71 V 79 H 56 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 88 72 H 95 V 79 H 88 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 98 65 H 105 V 72 H 98 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 73 88 H 81 V 91 H 73 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 85 88 H 93 V 91 H 85 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 92 97 H 106 V 99 H 92 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                Platform :: Gba => {
                    Image (
                        src: path!(M 12 35 H 148 V 92 H 12 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 8 42 H 152 V 84 H 8 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 12 39 H 148 V 84 H 12 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(116, 100, 151)
                    )
                    Image (
                        src: path!(M 39 39 H 121 V 83 H 39 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(172, 164, 182)
                    )
                    Image (
                        src: path!(M 44 43 H 116 V 78 H 44 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 48 46 H 112 V 75 H 48 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(177, 185, 135)
                    )
                    Image (
                        src: path!(M 22 51 H 27 V 66 H 22 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 17 56 H 32 V 61 H 17 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                    Image (
                        src: path!(M 124 56 H 131 V 63 H 124 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 135 49 H 142 V 56 H 135 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(165, 76, 59)
                    )
                    Image (
                        src: path!(M 22 76 H 30 V 78 H 22 Z),
                        viewbox: 160,
                        position: Position::Absolute,
                        width: Dimension::percent(100),
                        height: Dimension::percent(100),
                        color: Color::rgb(41, 60, 51)
                    )
                }
                _ => {
                }
            }
        }
    }
}
