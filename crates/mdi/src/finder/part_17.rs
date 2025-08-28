// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_17(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "fruit-grapes" => Some(icons::FRUIT_GRAPES),
        "fruit-pear" => Some(icons::FRUIT_PEAR),
        "fruit-pineapple" => Some(icons::FRUIT_PINEAPPLE),
        "fruit-watermelon" => Some(icons::FRUIT_WATERMELON),
        "fuel-cell" => Some(icons::FUEL_CELL),
        "fuel" => Some(icons::FUEL),
        "fullscreen-exit" => Some(icons::FULLSCREEN_EXIT),
        "fullscreen" => Some(icons::FULLSCREEN),
        "function-variant" => Some(icons::FUNCTION_VARIANT),
        "function" => Some(icons::FUNCTION),
        "furigana-horizontal" => Some(icons::FURIGANA_HORIZONTAL),
        "furigana-vertical" => Some(icons::FURIGANA_VERTICAL),
        "fuse-alert" => Some(icons::FUSE_ALERT),
        "fuse-blade" => Some(icons::FUSE_BLADE),
        "fuse-off" => Some(icons::FUSE_OFF),
        "fuse" => Some(icons::FUSE),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "gamepad-circle-left" => Some(icons::GAMEPAD_CIRCLE_LEFT),
        "gamepad-circle-outline" => Some(icons::GAMEPAD_CIRCLE_OUTLINE),
        "gamepad-circle-right" => Some(icons::GAMEPAD_CIRCLE_RIGHT),
        "gamepad-circle-up" => Some(icons::GAMEPAD_CIRCLE_UP),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "gamepad-down" => Some(icons::GAMEPAD_DOWN),
        "gamepad-left" => Some(icons::GAMEPAD_LEFT),
        "gamepad-outline" => Some(icons::GAMEPAD_OUTLINE),
        "gamepad-right" => Some(icons::GAMEPAD_RIGHT),
        "gamepad-round-down" => Some(icons::GAMEPAD_ROUND_DOWN),
        "gamepad-round-left" => Some(icons::GAMEPAD_ROUND_LEFT),
        "gamepad-round-outline" => Some(icons::GAMEPAD_ROUND_OUTLINE),
        "gamepad-round-right" => Some(icons::GAMEPAD_ROUND_RIGHT),
        "gamepad-round-up" => Some(icons::GAMEPAD_ROUND_UP),
        "gamepad-round" => Some(icons::GAMEPAD_ROUND),
        "gamepad-square-outline" => Some(icons::GAMEPAD_SQUARE_OUTLINE),
        "gamepad-square" => Some(icons::GAMEPAD_SQUARE),
        "gamepad-up" => Some(icons::GAMEPAD_UP),
        "gamepad-variant-outline" => Some(icons::GAMEPAD_VARIANT_OUTLINE),
        "gamepad-variant" => Some(icons::GAMEPAD_VARIANT),
        "gamepad" => Some(icons::GAMEPAD),
        "gamma" => Some(icons::GAMMA),
        "gantry-crane" => Some(icons::GANTRY_CRANE),
        "garage-alert-variant" => Some(icons::GARAGE_ALERT_VARIANT),
        "garage-alert" => Some(icons::GARAGE_ALERT),
        "garage-lock" => Some(icons::GARAGE_LOCK),
        "garage-open-variant" => Some(icons::GARAGE_OPEN_VARIANT),
        "garage-open" => Some(icons::GARAGE_OPEN),
        "garage-variant-lock" => Some(icons::GARAGE_VARIANT_LOCK),
        "garage-variant" => Some(icons::GARAGE_VARIANT),
        "garage" => Some(icons::GARAGE),
        "gas-burner" => Some(icons::GAS_BURNER),
        "gas-cylinder" => Some(icons::GAS_CYLINDER),
        "gas-station-in-use-outline" => Some(icons::GAS_STATION_IN_USE_OUTLINE),
        "gas-station-in-use" => Some(icons::GAS_STATION_IN_USE),
        "gas-station-off-outline" => Some(icons::GAS_STATION_OFF_OUTLINE),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "gas-station-outline" => Some(icons::GAS_STATION_OUTLINE),
        "gas-station" => Some(icons::GAS_STATION),
        "gate-alert" => Some(icons::GATE_ALERT),
        "gate-and" => Some(icons::GATE_AND),
        "gate-arrow-left" => Some(icons::GATE_ARROW_LEFT),
        "gate-arrow-right" => Some(icons::GATE_ARROW_RIGHT),
        "gate-buffer" => Some(icons::GATE_BUFFER),
        "gate-nand" => Some(icons::GATE_NAND),
        "gate-nor" => Some(icons::GATE_NOR),
        "gate-not" => Some(icons::GATE_NOT),
        "gate-open" => Some(icons::GATE_OPEN),
        "gate-or" => Some(icons::GATE_OR),
        "gate-xnor" => Some(icons::GATE_XNOR),
        "gate-xor" => Some(icons::GATE_XOR),
        "gate" => Some(icons::GATE),
        #[allow(deprecated)]
        "gatsby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gatsby' is deprecated.").print(py);
            }
            Some(icons::GATSBY)
        }
        "gauge-empty" => Some(icons::GAUGE_EMPTY),
        "gauge-full" => Some(icons::GAUGE_FULL),
        "gauge-low" => Some(icons::GAUGE_LOW),
        "gauge" => Some(icons::GAUGE),
        "gavel" => Some(icons::GAVEL),
        "gender-female" => Some(icons::GENDER_FEMALE),
        "gender-male-female-variant" => Some(icons::GENDER_MALE_FEMALE_VARIANT),
        "gender-male-female" => Some(icons::GENDER_MALE_FEMALE),
        "gender-male" => Some(icons::GENDER_MALE),
        "gender-non-binary" => Some(icons::GENDER_NON_BINARY),
        "gender-transgender" => Some(icons::GENDER_TRANSGENDER),
        "generator-mobile" => Some(icons::GENERATOR_MOBILE),
        "generator-portable" => Some(icons::GENERATOR_PORTABLE),
        "generator-stationary" => Some(icons::GENERATOR_STATIONARY),
        #[allow(deprecated)]
        "gentoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gentoo' is deprecated.").print(py);
            }
            Some(icons::GENTOO)
        }
        "gesture-double-tap" => Some(icons::GESTURE_DOUBLE_TAP),
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "gesture-spread" => Some(icons::GESTURE_SPREAD),
        "gesture-swipe-down" => Some(icons::GESTURE_SWIPE_DOWN),
        "gesture-swipe-horizontal" => Some(icons::GESTURE_SWIPE_HORIZONTAL),
        "gesture-swipe-left" => Some(icons::GESTURE_SWIPE_LEFT),
        "gesture-swipe-right" => Some(icons::GESTURE_SWIPE_RIGHT),
        "gesture-swipe-up" => Some(icons::GESTURE_SWIPE_UP),
        "gesture-swipe-vertical" => Some(icons::GESTURE_SWIPE_VERTICAL),
        "gesture-swipe" => Some(icons::GESTURE_SWIPE),
        "gesture-tap-box" => Some(icons::GESTURE_TAP_BOX),
        "gesture-tap-button" => Some(icons::GESTURE_TAP_BUTTON),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "gesture-tap" => Some(icons::GESTURE_TAP),
        "gesture-two-double-tap" => Some(icons::GESTURE_TWO_DOUBLE_TAP),
        "gesture-two-tap" => Some(icons::GESTURE_TWO_TAP),
        "gesture" => Some(icons::GESTURE),
        "ghost-off-outline" => Some(icons::GHOST_OFF_OUTLINE),
        "ghost-off" => Some(icons::GHOST_OFF),
        "ghost-outline" => Some(icons::GHOST_OUTLINE),
        "ghost" => Some(icons::GHOST),
        "gift-off-outline" => Some(icons::GIFT_OFF_OUTLINE),
        "gift-off" => Some(icons::GIFT_OFF),
        "gift-open-outline" => Some(icons::GIFT_OPEN_OUTLINE),
        "gift-open" => Some(icons::GIFT_OPEN),
        "gift-outline" => Some(icons::GIFT_OUTLINE),
        "gift" => Some(icons::GIFT),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        #[allow(deprecated)]
        "github" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'github' is deprecated.").print(py);
            }
            Some(icons::GITHUB)
        }
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "glass-cocktail" => Some(icons::GLASS_COCKTAIL),
        "glass-flute" => Some(icons::GLASS_FLUTE),
        "glass-fragile" => Some(icons::GLASS_FRAGILE),
        "glass-mug-off" => Some(icons::GLASS_MUG_OFF),
        "glass-mug-variant-off" => Some(icons::GLASS_MUG_VARIANT_OFF),
        "glass-mug-variant" => Some(icons::GLASS_MUG_VARIANT),
        "glass-mug" => Some(icons::GLASS_MUG),
        "glass-pint-outline" => Some(icons::GLASS_PINT_OUTLINE),
        "glass-stange" => Some(icons::GLASS_STANGE),
        "glass-tulip" => Some(icons::GLASS_TULIP),
        "glass-wine" => Some(icons::GLASS_WINE),
        "glasses" => Some(icons::GLASSES),
        "globe-light-outline" => Some(icons::GLOBE_LIGHT_OUTLINE),
        "globe-light" => Some(icons::GLOBE_LIGHT),
        "globe-model" => Some(icons::GLOBE_MODEL),
        #[allow(deprecated)]
        "gmail" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gmail' is deprecated.").print(py);
            }
            Some(icons::GMAIL)
        }
        #[allow(deprecated)]
        "gnome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gnome' is deprecated.").print(py);
            }
            Some(icons::GNOME)
        }
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "go-kart" => Some(icons::GO_KART),
        #[allow(deprecated)]
        "gog" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gog' is deprecated.").print(py);
            }
            Some(icons::GOG)
        }
        "gold" => Some(icons::GOLD),
        "golf-cart" => Some(icons::GOLF_CART),
        "golf-tee" => Some(icons::GOLF_TEE),
        "golf" => Some(icons::GOLF),
        "gondola" => Some(icons::GONDOLA),
        #[allow(deprecated)]
        "goodreads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'goodreads' is deprecated.").print(py);
            }
            Some(icons::GOODREADS)
        }
        #[allow(deprecated)]
        "google-ads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-ads' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_ADS)
        }
        #[allow(deprecated)]
        "google-analytics" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-analytics' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ANALYTICS)
        }
        #[allow(deprecated)]
        "google-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-assistant' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ASSISTANT)
        }
        #[allow(deprecated)]
        "google-cardboard" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cardboard' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CARDBOARD)
        }
        #[allow(deprecated)]
        "google-chrome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-chrome' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CHROME)
        }
        #[allow(deprecated)]
        "google-circles-communities" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'google-circles-communities' is deprecated.",
                )
                .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_COMMUNITIES)
        }
        #[allow(deprecated)]
        "google-circles-extended" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-extended' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_EXTENDED)
        }
        #[allow(deprecated)]
        "google-circles-group" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles-group' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CIRCLES_GROUP)
        }
        #[allow(deprecated)]
        "google-circles" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-circles' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CIRCLES)
        }
        #[allow(deprecated)]
        "google-classroom" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-classroom' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CLASSROOM)
        }
        #[allow(deprecated)]
        "google-cloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cloud' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CLOUD)
        }
        #[allow(deprecated)]
        "google-downasaur" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-downasaur' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_DOWNASAUR)
        }
        #[allow(deprecated)]
        "google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-drive' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_DRIVE)
        }
        #[allow(deprecated)]
        "google-earth" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-earth' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_EARTH)
        }
        #[allow(deprecated)]
        "google-fit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-fit' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_FIT)
        }
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        #[allow(deprecated)]
        "google-lens" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-lens' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_LENS)
        }
        #[allow(deprecated)]
        "google-maps" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-maps' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_MAPS)
        }
        #[allow(deprecated)]
        "google-my-business" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-my-business' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_MY_BUSINESS)
        }
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        #[allow(deprecated)]
        "google-play" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-play' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLAY)
        }
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        #[allow(deprecated)]
        "google-podcast" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-podcast' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PODCAST)
        }
        #[allow(deprecated)]
        "google-spreadsheet" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-spreadsheet' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_SPREADSHEET)
        }
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        #[allow(deprecated)]
        "google-translate" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-translate' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_TRANSLATE)
        }
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "gradient-horizontal" => Some(icons::GRADIENT_HORIZONTAL),
        "gradient-vertical" => Some(icons::GRADIENT_VERTICAL),
        "grain" => Some(icons::GRAIN),
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "graph" => Some(icons::GRAPH),
        #[allow(deprecated)]
        "graphql" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'graphql' is deprecated.").print(py);
            }
            Some(icons::GRAPHQL)
        }
        "grass" => Some(icons::GRASS),
        "grave-stone" => Some(icons::GRAVE_STONE),
        "grease-pencil" => Some(icons::GREASE_PENCIL),
        "greater-than-or-equal" => Some(icons::GREATER_THAN_OR_EQUAL),
        "greater-than" => Some(icons::GREATER_THAN),
        "greenhouse" => Some(icons::GREENHOUSE),
        "grid-large" => Some(icons::GRID_LARGE),
        "grid-off" => Some(icons::GRID_OFF),
        "grid" => Some(icons::GRID),
        "grill-outline" => Some(icons::GRILL_OUTLINE),
        "grill" => Some(icons::GRILL),
        "group" => Some(icons::GROUP),
        "guitar-acoustic" => Some(icons::GUITAR_ACOUSTIC),
        "guitar-electric" => Some(icons::GUITAR_ELECTRIC),
        "guitar-pick-outline" => Some(icons::GUITAR_PICK_OUTLINE),
        "guitar-pick" => Some(icons::GUITAR_PICK),
        "guy-fawkes-mask" => Some(icons::GUY_FAWKES_MASK),
        "gymnastics" => Some(icons::GYMNASTICS),
        "hail" => Some(icons::HAIL),
        "hair-dryer-outline" => Some(icons::HAIR_DRYER_OUTLINE),
        "hair-dryer" => Some(icons::HAIR_DRYER),
        "halloween" => Some(icons::HALLOWEEN),
        "hamburger-check" => Some(icons::HAMBURGER_CHECK),
        _ => None,
    }
}
