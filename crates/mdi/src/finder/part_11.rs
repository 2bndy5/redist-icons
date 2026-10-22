// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_11(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "alpha-x-box-outline" => Some(icons::ALPHA_X_BOX_OUTLINE),
        "skip-forward" => Some(icons::SKIP_FORWARD),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        #[allow(deprecated)]
        "react" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'react' is deprecated.").print(py);
            }
            Some(icons::REACT)
        }
        "export-variant" => Some(icons::EXPORT_VARIANT),
        "human-cane" => Some(icons::HUMAN_CANE),
        #[allow(deprecated)]
        "jira" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jira' is deprecated.").print(py);
            }
            Some(icons::JIRA)
        }
        "beaker-minus-outline" => Some(icons::BEAKER_MINUS_OUTLINE),
        "airplane-alert" => Some(icons::AIRPLANE_ALERT),
        "seat-legroom-normal" => Some(icons::SEAT_LEGROOM_NORMAL),
        "silverware-clean" => Some(icons::SILVERWARE_CLEAN),
        "gas-cylinder" => Some(icons::GAS_CYLINDER),
        "weather-partly-snowy" => Some(icons::WEATHER_PARTLY_SNOWY),
        "autorenew" => Some(icons::AUTORENEW),
        "office-building-plus" => Some(icons::OFFICE_BUILDING_PLUS),
        "glass-cocktail" => Some(icons::GLASS_COCKTAIL),
        "keyboard-f2" => Some(icons::KEYBOARD_F2),
        "swap-horizontal-circle-outline" => Some(icons::SWAP_HORIZONTAL_CIRCLE_OUTLINE),
        "phone-outgoing-outline" => Some(icons::PHONE_OUTGOING_OUTLINE),
        "seed-off-outline" => Some(icons::SEED_OFF_OUTLINE),
        "valve-open" => Some(icons::VALVE_OPEN),
        "atm" => Some(icons::ATM),
        "heart-box" => Some(icons::HEART_BOX),
        "tag-off-outline" => Some(icons::TAG_OFF_OUTLINE),
        "book-outline" => Some(icons::BOOK_OUTLINE),
        "invoice-text-outline" => Some(icons::INVOICE_TEXT_OUTLINE),
        "circle-slice-8" => Some(icons::CIRCLE_SLICE_8),
        "star-face" => Some(icons::STAR_FACE),
        "card-text" => Some(icons::CARD_TEXT),
        "phone-refresh-outline" => Some(icons::PHONE_REFRESH_OUTLINE),
        "card-account-mail" => Some(icons::CARD_ACCOUNT_MAIL),
        "human-greeting" => Some(icons::HUMAN_GREETING),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "mouse-off" => Some(icons::MOUSE_OFF),
        "chili-alert-outline" => Some(icons::CHILI_ALERT_OUTLINE),
        "toy-brick" => Some(icons::TOY_BRICK),
        "invoice-list" => Some(icons::INVOICE_LIST),
        "glass-wine" => Some(icons::GLASS_WINE),
        "alpha-t-circle-outline" => Some(icons::ALPHA_T_CIRCLE_OUTLINE),
        "fridge-industrial-alert-outline" => Some(icons::FRIDGE_INDUSTRIAL_ALERT_OUTLINE),
        "file-cloud" => Some(icons::FILE_CLOUD),
        "lock-percent-outline" => Some(icons::LOCK_PERCENT_OUTLINE),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "phone-alert" => Some(icons::PHONE_ALERT),
        "zodiac-taurus" => Some(icons::ZODIAC_TAURUS),
        "pencil-box-multiple-outline" => Some(icons::PENCIL_BOX_MULTIPLE_OUTLINE),
        "vacuum-outline" => Some(icons::VACUUM_OUTLINE),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "temple-hindu" => Some(icons::TEMPLE_HINDU),
        "note-edit-outline" => Some(icons::NOTE_EDIT_OUTLINE),
        "pumpkin" => Some(icons::PUMPKIN),
        "diameter-outline" => Some(icons::DIAMETER_OUTLINE),
        "account-cog" => Some(icons::ACCOUNT_COG),
        "railroad-light" => Some(icons::RAILROAD_LIGHT),
        "dice-d8" => Some(icons::DICE_D8),
        "rocket-launch-outline" => Some(icons::ROCKET_LAUNCH_OUTLINE),
        "unfold-less-horizontal" => Some(icons::UNFOLD_LESS_HORIZONTAL),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "ski" => Some(icons::SKI),
        "ip-network" => Some(icons::IP_NETWORK),
        "inbox-arrow-down-outline" => Some(icons::INBOX_ARROW_DOWN_OUTLINE),
        #[allow(deprecated)]
        "arch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'arch' is deprecated.").print(py);
            }
            Some(icons::ARCH)
        }
        "clipboard-pulse-outline" => Some(icons::CLIPBOARD_PULSE_OUTLINE),
        "layers" => Some(icons::LAYERS),
        "curtains-closed" => Some(icons::CURTAINS_CLOSED),
        "star-four-points-outline" => Some(icons::STAR_FOUR_POINTS_OUTLINE),
        #[allow(deprecated)]
        "blender-software" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'blender-software' is deprecated.")
                    .print(py);
            }
            Some(icons::BLENDER_SOFTWARE)
        }
        "shield-plus" => Some(icons::SHIELD_PLUS),
        "plus-thick" => Some(icons::PLUS_THICK),
        "heart-minus" => Some(icons::HEART_MINUS),
        "ribbon" => Some(icons::RIBBON),
        "medical-bag" => Some(icons::MEDICAL_BAG),
        "phone-minus" => Some(icons::PHONE_MINUS),
        "domain-off" => Some(icons::DOMAIN_OFF),
        "compare-horizontal" => Some(icons::COMPARE_HORIZONTAL),
        "abacus" => Some(icons::ABACUS),
        "book-cancel" => Some(icons::BOOK_CANCEL),
        "tray-arrow-down" => Some(icons::TRAY_ARROW_DOWN),
        "currency-uah" => Some(icons::CURRENCY_UAH),
        #[allow(deprecated)]
        "google-translate" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-translate' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_TRANSLATE)
        }
        "alpha-v" => Some(icons::ALPHA_V),
        "dog-side-off" => Some(icons::DOG_SIDE_OFF),
        "bell-cancel-outline" => Some(icons::BELL_CANCEL_OUTLINE),
        "surround-sound-5-1" => Some(icons::SURROUND_SOUND_5_1),
        "poker-chip" => Some(icons::POKER_CHIP),
        "content-save-off" => Some(icons::CONTENT_SAVE_OFF),
        "format-text-rotation-vertical" => Some(icons::FORMAT_TEXT_ROTATION_VERTICAL),
        "view-stream" => Some(icons::VIEW_STREAM),
        "power-plug-off-outline" => Some(icons::POWER_PLUG_OFF_OUTLINE),
        "filmstrip" => Some(icons::FILMSTRIP),
        "calendar-plus" => Some(icons::CALENDAR_PLUS),
        "candy-off-outline" => Some(icons::CANDY_OFF_OUTLINE),
        "content-save-check" => Some(icons::CONTENT_SAVE_CHECK),
        "book-cancel-outline" => Some(icons::BOOK_CANCEL_OUTLINE),
        "database-remove-outline" => Some(icons::DATABASE_REMOVE_OUTLINE),
        "zodiac-capricorn" => Some(icons::ZODIAC_CAPRICORN),
        "format-wrap-inline" => Some(icons::FORMAT_WRAP_INLINE),
        "baseball-bat" => Some(icons::BASEBALL_BAT),
        #[allow(deprecated)]
        "facebook-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-gaming' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_GAMING)
        }
        "perspective-less" => Some(icons::PERSPECTIVE_LESS),
        "arrow-collapse-horizontal" => Some(icons::ARROW_COLLAPSE_HORIZONTAL),
        "circle-half" => Some(icons::CIRCLE_HALF),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        "usb-flash-drive-outline" => Some(icons::USB_FLASH_DRIVE_OUTLINE),
        "handshake" => Some(icons::HANDSHAKE),
        "web-cancel" => Some(icons::WEB_CANCEL),
        "puzzle" => Some(icons::PUZZLE),
        "archive-plus" => Some(icons::ARCHIVE_PLUS),
        "sticker-circle-outline" => Some(icons::STICKER_CIRCLE_OUTLINE),
        "bell-alert" => Some(icons::BELL_ALERT),
        "clock-digital" => Some(icons::CLOCK_DIGITAL),
        "bed-outline" => Some(icons::BED_OUTLINE),
        "chart-histogram" => Some(icons::CHART_HISTOGRAM),
        "radius-outline" => Some(icons::RADIUS_OUTLINE),
        "chart-sankey-variant" => Some(icons::CHART_SANKEY_VARIANT),
        "printer-pos-refresh-outline" => Some(icons::PRINTER_POS_REFRESH_OUTLINE),
        "television-ambient-light" => Some(icons::TELEVISION_AMBIENT_LIGHT),
        "phone-forward-outline" => Some(icons::PHONE_FORWARD_OUTLINE),
        #[allow(deprecated)]
        "xmpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xmpp' is deprecated.").print(py);
            }
            Some(icons::XMPP)
        }
        "border-right" => Some(icons::BORDER_RIGHT),
        "bag-personal" => Some(icons::BAG_PERSONAL),
        "math-norm" => Some(icons::MATH_NORM),
        #[allow(deprecated)]
        "rollupjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'rollupjs' is deprecated.").print(py);
            }
            Some(icons::ROLLUPJS)
        }
        "hours-24" => Some(icons::HOURS_24),
        "zodiac-virgo" => Some(icons::ZODIAC_VIRGO),
        "lock-open-remove" => Some(icons::LOCK_OPEN_REMOVE),
        "powershell" => Some(icons::POWERSHELL),
        "message-alert-outline" => Some(icons::MESSAGE_ALERT_OUTLINE),
        "phone-off-outline" => Some(icons::PHONE_OFF_OUTLINE),
        "filter-settings-outline" => Some(icons::FILTER_SETTINGS_OUTLINE),
        "weather-hail" => Some(icons::WEATHER_HAIL),
        "checkbox-multiple-marked-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_MARKED_CIRCLE_OUTLINE)
        }
        "smoke-detector-variant-alert" => Some(icons::SMOKE_DETECTOR_VARIANT_ALERT),
        "printer-3d" => Some(icons::PRINTER_3D),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "table-cancel" => Some(icons::TABLE_CANCEL),
        "file-music" => Some(icons::FILE_MUSIC),
        "receipt" => Some(icons::RECEIPT),
        "skew-more" => Some(icons::SKEW_MORE),
        "download-lock-outline" => Some(icons::DOWNLOAD_LOCK_OUTLINE),
        "chevron-right-box-outline" => Some(icons::CHEVRON_RIGHT_BOX_OUTLINE),
        "beaker-remove-outline" => Some(icons::BEAKER_REMOVE_OUTLINE),
        "shield-airplane-outline" => Some(icons::SHIELD_AIRPLANE_OUTLINE),
        "file-cad-box" => Some(icons::FILE_CAD_BOX),
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "bomb" => Some(icons::BOMB),
        "phone-settings-outline" => Some(icons::PHONE_SETTINGS_OUTLINE),
        "bell-circle" => Some(icons::BELL_CIRCLE),
        "table-remove" => Some(icons::TABLE_REMOVE),
        "cookie-cog-outline" => Some(icons::COOKIE_COG_OUTLINE),
        "hub-outline" => Some(icons::HUB_OUTLINE),
        "tag-hidden" => Some(icons::TAG_HIDDEN),
        "alpha-y-circle" => Some(icons::ALPHA_Y_CIRCLE),
        "traffic-light-outline" => Some(icons::TRAFFIC_LIGHT_OUTLINE),
        "map-check-outline" => Some(icons::MAP_CHECK_OUTLINE),
        "briefcase" => Some(icons::BRIEFCASE),
        "sun-thermometer-outline" => Some(icons::SUN_THERMOMETER_OUTLINE),
        "text-search-variant" => Some(icons::TEXT_SEARCH_VARIANT),
        #[allow(deprecated)]
        "language-c" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-c' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_C)
        }
        "sun-wireless-outline" => Some(icons::SUN_WIRELESS_OUTLINE),
        "download-network" => Some(icons::DOWNLOAD_NETWORK),
        #[allow(deprecated)]
        "teamviewer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'teamviewer' is deprecated.").print(py);
            }
            Some(icons::TEAMVIEWER)
        }
        "roller-shade-closed" => Some(icons::ROLLER_SHADE_CLOSED),
        "tag-search-outline" => Some(icons::TAG_SEARCH_OUTLINE),
        "axis-z-arrow-lock" => Some(icons::AXIS_Z_ARROW_LOCK),
        #[allow(deprecated)]
        "minecraft" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'minecraft' is deprecated.").print(py);
            }
            Some(icons::MINECRAFT)
        }
        "et" => Some(icons::ET),
        "movie-open-edit-outline" => Some(icons::MOVIE_OPEN_EDIT_OUTLINE),
        "timer-lock-open" => Some(icons::TIMER_LOCK_OPEN),
        "human-capacity-increase" => Some(icons::HUMAN_CAPACITY_INCREASE),
        "numeric-4" => Some(icons::NUMERIC_4),
        "invoice-text-minus" => Some(icons::INVOICE_TEXT_MINUS),
        "blood-bag" => Some(icons::BLOOD_BAG),
        "car-off" => Some(icons::CAR_OFF),
        "donkey" => Some(icons::DONKEY),
        "clock-time-one" => Some(icons::CLOCK_TIME_ONE),
        "roman-numeral-8" => Some(icons::ROMAN_NUMERAL_8),
        "image-minus-outline" => Some(icons::IMAGE_MINUS_OUTLINE),
        "emoticon-frown" => Some(icons::EMOTICON_FROWN),
        "shield-sun-outline" => Some(icons::SHIELD_SUN_OUTLINE),
        "floppy" => Some(icons::FLOPPY),
        "puzzle-remove" => Some(icons::PUZZLE_REMOVE),
        "database-eye-off-outline" => Some(icons::DATABASE_EYE_OFF_OUTLINE),
        "diving-scuba" => Some(icons::DIVING_SCUBA),
        "movie-open-check" => Some(icons::MOVIE_OPEN_CHECK),
        "octagram" => Some(icons::OCTAGRAM),
        "message-lock" => Some(icons::MESSAGE_LOCK),
        "garage-lock" => Some(icons::GARAGE_LOCK),
        "database-marker-outline" => Some(icons::DATABASE_MARKER_OUTLINE),
        "candelabra" => Some(icons::CANDELABRA),
        "rivet" => Some(icons::RIVET),
        "case-sensitive-alt" => Some(icons::CASE_SENSITIVE_ALT),
        "bus-stop-uncovered" => Some(icons::BUS_STOP_UNCOVERED),
        "align-horizontal-left" => Some(icons::ALIGN_HORIZONTAL_LEFT),
        "map-marker-minus" => Some(icons::MAP_MARKER_MINUS),
        "key-alert-outline" => Some(icons::KEY_ALERT_OUTLINE),
        "cards-playing-diamond-outline" => Some(icons::CARDS_PLAYING_DIAMOND_OUTLINE),
        _ => None,
    }
}
