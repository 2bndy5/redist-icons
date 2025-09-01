// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_37(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "windsock" => Some(icons::WINDSOCK),
        "wiper-wash-alert" => Some(icons::WIPER_WASH_ALERT),
        "wiper-wash" => Some(icons::WIPER_WASH),
        "wiper" => Some(icons::WIPER),
        "wizard-hat" => Some(icons::WIZARD_HAT),
        #[allow(deprecated)]
        "wordpress" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wordpress' is deprecated.").print(py);
            }
            Some(icons::WORDPRESS)
        }
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "wrap" => Some(icons::WRAP),
        "wrench-check-outline" => Some(icons::WRENCH_CHECK_OUTLINE),
        "wrench-check" => Some(icons::WRENCH_CHECK),
        "wrench-clock-outline" => Some(icons::WRENCH_CLOCK_OUTLINE),
        "wrench-clock" => Some(icons::WRENCH_CLOCK),
        "wrench-cog-outline" => Some(icons::WRENCH_COG_OUTLINE),
        "wrench-cog" => Some(icons::WRENCH_COG),
        "wrench-outline" => Some(icons::WRENCH_OUTLINE),
        "wrench" => Some(icons::WRENCH),
        #[allow(deprecated)]
        "xamarin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xamarin' is deprecated.").print(py);
            }
            Some(icons::XAMARIN)
        }
        "xml" => Some(icons::XML),
        #[allow(deprecated)]
        "xmpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xmpp' is deprecated.").print(py);
            }
            Some(icons::XMPP)
        }
        #[allow(deprecated)]
        "yahoo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'yahoo' is deprecated.").print(py);
            }
            Some(icons::YAHOO)
        }
        "yeast" => Some(icons::YEAST),
        "yin-yang" => Some(icons::YIN_YANG),
        "yoga" => Some(icons::YOGA),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        #[allow(deprecated)]
        "youtube-subscription" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-subscription' is deprecated.")
                    .print(py);
            }
            Some(icons::YOUTUBE_SUBSCRIPTION)
        }
        #[allow(deprecated)]
        "youtube-tv" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-tv' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_TV)
        }
        #[allow(deprecated)]
        "youtube" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE)
        }
        "yurt" => Some(icons::YURT),
        #[allow(deprecated)]
        "z-wave" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'z-wave' is deprecated.").print(py);
            }
            Some(icons::Z_WAVE)
        }
        #[allow(deprecated)]
        "zend" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zend' is deprecated.").print(py);
            }
            Some(icons::ZEND)
        }
        #[allow(deprecated)]
        "zigbee" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zigbee' is deprecated.").print(py);
            }
            Some(icons::ZIGBEE)
        }
        "zip-box-outline" => Some(icons::ZIP_BOX_OUTLINE),
        "zip-box" => Some(icons::ZIP_BOX),
        "zip-disk" => Some(icons::ZIP_DISK),
        "zodiac-aquarius" => Some(icons::ZODIAC_AQUARIUS),
        "zodiac-aries" => Some(icons::ZODIAC_ARIES),
        "zodiac-cancer" => Some(icons::ZODIAC_CANCER),
        "zodiac-capricorn" => Some(icons::ZODIAC_CAPRICORN),
        "zodiac-gemini" => Some(icons::ZODIAC_GEMINI),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "zodiac-pisces" => Some(icons::ZODIAC_PISCES),
        "zodiac-sagittarius" => Some(icons::ZODIAC_SAGITTARIUS),
        "zodiac-scorpio" => Some(icons::ZODIAC_SCORPIO),
        "zodiac-taurus" => Some(icons::ZODIAC_TAURUS),
        "zodiac-virgo" => Some(icons::ZODIAC_VIRGO),
        _ => None,
    }
}
