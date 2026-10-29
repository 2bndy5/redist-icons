// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_30(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "abugida-thai" => Some(icons::ABUGIDA_THAI),
        "water-percent-alert" => Some(icons::WATER_PERCENT_ALERT),
        "emoticon" => Some(icons::EMOTICON),
        "phone-cancel-outline" => Some(icons::PHONE_CANCEL_OUTLINE),
        "tooth-outline" => Some(icons::TOOTH_OUTLINE),
        "human-male-child" => Some(icons::HUMAN_MALE_CHILD),
        "elevator-passenger-outline" => Some(icons::ELEVATOR_PASSENGER_OUTLINE),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        #[allow(deprecated)]
        "trello" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'trello' is deprecated.").print(py);
            }
            Some(icons::TRELLO)
        }
        "cog-play-outline" => Some(icons::COG_PLAY_OUTLINE),
        "lock-open-variant-outline" => Some(icons::LOCK_OPEN_VARIANT_OUTLINE),
        "office-building-plus-outline" => Some(icons::OFFICE_BUILDING_PLUS_OUTLINE),
        "format-letter-matches" => Some(icons::FORMAT_LETTER_MATCHES),
        #[allow(deprecated)]
        "cordova" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cordova' is deprecated.").print(py);
            }
            Some(icons::CORDOVA)
        }
        "format-pilcrow-arrow-left" => Some(icons::FORMAT_PILCROW_ARROW_LEFT),
        "weather-partly-rainy" => Some(icons::WEATHER_PARTLY_RAINY),
        "axis-arrow-info" => Some(icons::AXIS_ARROW_INFO),
        "clipboard" => Some(icons::CLIPBOARD),
        "wifi-plus" => Some(icons::WIFI_PLUS),
        "table-row-plus-before" => Some(icons::TABLE_ROW_PLUS_BEFORE),
        "calendar-alert-outline" => Some(icons::CALENDAR_ALERT_OUTLINE),
        "target" => Some(icons::TARGET),
        "wallet-bifold-outline" => Some(icons::WALLET_BIFOLD_OUTLINE),
        "sticker-plus" => Some(icons::STICKER_PLUS),
        "alpha-k-box" => Some(icons::ALPHA_K_BOX),
        #[allow(deprecated)]
        "ethereum" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ethereum' is deprecated.").print(py);
            }
            Some(icons::ETHEREUM)
        }
        "lightbulb-group-off" => Some(icons::LIGHTBULB_GROUP_OFF),
        #[allow(deprecated)]
        "evernote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'evernote' is deprecated.").print(py);
            }
            Some(icons::EVERNOTE)
        }
        "movie-open-edit" => Some(icons::MOVIE_OPEN_EDIT),
        "sim" => Some(icons::SIM),
        "pirate" => Some(icons::PIRATE),
        "application-cog" => Some(icons::APPLICATION_COG),
        "copyleft" => Some(icons::COPYLEFT),
        "panorama-sphere-outline" => Some(icons::PANORAMA_SPHERE_OUTLINE),
        "calendar-export-outline" => Some(icons::CALENDAR_EXPORT_OUTLINE),
        "sort-variant-off" => Some(icons::SORT_VARIANT_OFF),
        "flask" => Some(icons::FLASK),
        "check-decagram" => Some(icons::CHECK_DECAGRAM),
        "arrow-expand-vertical" => Some(icons::ARROW_EXPAND_VERTICAL),
        #[allow(deprecated)]
        "microsoft-onenote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onenote' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONENOTE)
        }
        "qrcode-edit" => Some(icons::QRCODE_EDIT),
        "send-outline" => Some(icons::SEND_OUTLINE),
        "card-minus" => Some(icons::CARD_MINUS),
        "cupcake" => Some(icons::CUPCAKE),
        "gamepad-round-down" => Some(icons::GAMEPAD_ROUND_DOWN),
        "car-tire-alert" => Some(icons::CAR_TIRE_ALERT),
        "alpha-t-box" => Some(icons::ALPHA_T_BOX),
        #[allow(deprecated)]
        "microsoft-teams" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-teams' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_TEAMS)
        }
        "gamepad-circle-up" => Some(icons::GAMEPAD_CIRCLE_UP),
        "calendar-filter" => Some(icons::CALENDAR_FILTER),
        "server-remove" => Some(icons::SERVER_REMOVE),
        "select-arrow-up" => Some(icons::SELECT_ARROW_UP),
        "camera-plus" => Some(icons::CAMERA_PLUS),
        "home-group" => Some(icons::HOME_GROUP),
        "dns-outline" => Some(icons::DNS_OUTLINE),
        "home-analytics" => Some(icons::HOME_ANALYTICS),
        "fan-speed-3" => Some(icons::FAN_SPEED_3),
        "clipboard-text-off" => Some(icons::CLIPBOARD_TEXT_OFF),
        "arrow-left-bold-circle-outline" => Some(icons::ARROW_LEFT_BOLD_CIRCLE_OUTLINE),
        "cradle-outline" => Some(icons::CRADLE_OUTLINE),
        "cookie-edit-outline" => Some(icons::COOKIE_EDIT_OUTLINE),
        #[allow(deprecated)]
        "pi-hole" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pi-hole' is deprecated.").print(py);
            }
            Some(icons::PI_HOLE)
        }
        "calendar-badge" => Some(icons::CALENDAR_BADGE),
        "hours-12" => Some(icons::HOURS_12),
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        "format-header-pound" => Some(icons::FORMAT_HEADER_POUND),
        "closed-caption" => Some(icons::CLOSED_CAPTION),
        "filter-variant" => Some(icons::FILTER_VARIANT),
        "head-minus" => Some(icons::HEAD_MINUS),
        "amplifier-off" => Some(icons::AMPLIFIER_OFF),
        "chart-donut-variant" => Some(icons::CHART_DONUT_VARIANT),
        "passport-remove" => Some(icons::PASSPORT_REMOVE),
        "dice-d12-outline" => Some(icons::DICE_D12_OUTLINE),
        "pencil-off-outline" => Some(icons::PENCIL_OFF_OUTLINE),
        "chess-pawn" => Some(icons::CHESS_PAWN),
        "alpha-l-circle-outline" => Some(icons::ALPHA_L_CIRCLE_OUTLINE),
        "server-off" => Some(icons::SERVER_OFF),
        "network-strength-off-outline" => Some(icons::NETWORK_STRENGTH_OFF_OUTLINE),
        "television-pause" => Some(icons::TELEVISION_PAUSE),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        "candelabra-fire" => Some(icons::CANDELABRA_FIRE),
        "clock-time-six-outline" => Some(icons::CLOCK_TIME_SIX_OUTLINE),
        "sd" => Some(icons::SD),
        "checkbox-marked-circle-plus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_PLUS_OUTLINE),
        "script-outline" => Some(icons::SCRIPT_OUTLINE),
        "timer-remove-outline" => Some(icons::TIMER_REMOVE_OUTLINE),
        "alphabet-tengwar" => Some(icons::ALPHABET_TENGWAR),
        "chart-box-outline" => Some(icons::CHART_BOX_OUTLINE),
        "wheelchair" => Some(icons::WHEELCHAIR),
        "email-multiple" => Some(icons::EMAIL_MULTIPLE),
        "tab-search" => Some(icons::TAB_SEARCH),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "glass-tulip" => Some(icons::GLASS_TULIP),
        "mosque" => Some(icons::MOSQUE),
        "racing-helmet" => Some(icons::RACING_HELMET),
        "valve" => Some(icons::VALVE),
        "run" => Some(icons::RUN),
        "truck-check-outline" => Some(icons::TRUCK_CHECK_OUTLINE),
        "animation" => Some(icons::ANIMATION),
        "bullhorn-variant" => Some(icons::BULLHORN_VARIANT),
        "hand-wash" => Some(icons::HAND_WASH),
        "timer-3" => Some(icons::TIMER_3),
        "format-header-3" => Some(icons::FORMAT_HEADER_3),
        "robot-off-outline" => Some(icons::ROBOT_OFF_OUTLINE),
        "bottle-tonic-outline" => Some(icons::BOTTLE_TONIC_OUTLINE),
        "abjad-hebrew" => Some(icons::ABJAD_HEBREW),
        "book-check" => Some(icons::BOOK_CHECK),
        "book-play" => Some(icons::BOOK_PLAY),
        "eye-arrow-left-outline" => Some(icons::EYE_ARROW_LEFT_OUTLINE),
        "bike-pedal-mountain" => Some(icons::BIKE_PEDAL_MOUNTAIN),
        "blur-radial" => Some(icons::BLUR_RADIAL),
        "newspaper-plus" => Some(icons::NEWSPAPER_PLUS),
        "book-settings-outline" => Some(icons::BOOK_SETTINGS_OUTLINE),
        "tow-truck" => Some(icons::TOW_TRUCK),
        "color-helper" => Some(icons::COLOR_HELPER),
        "diving-snorkel" => Some(icons::DIVING_SNORKEL),
        "panorama-variant" => Some(icons::PANORAMA_VARIANT),
        "numeric-4-circle-outline" => Some(icons::NUMERIC_4_CIRCLE_OUTLINE),
        "map-marker-question" => Some(icons::MAP_MARKER_QUESTION),
        "smoking" => Some(icons::SMOKING),
        "pine-tree-variant" => Some(icons::PINE_TREE_VARIANT),
        "alpha-i-box" => Some(icons::ALPHA_I_BOX),
        "video-standard-definition" => Some(icons::VIDEO_STANDARD_DEFINITION),
        "file-plus-outline" => Some(icons::FILE_PLUS_OUTLINE),
        "package-variant" => Some(icons::PACKAGE_VARIANT),
        "file-outline" => Some(icons::FILE_OUTLINE),
        "swap-horizontal-circle-outline" => Some(icons::SWAP_HORIZONTAL_CIRCLE_OUTLINE),
        "printer-3d-nozzle-off-outline" => Some(icons::PRINTER_3D_NOZZLE_OFF_OUTLINE),
        "phone-plus-outline" => Some(icons::PHONE_PLUS_OUTLINE),
        "view-dashboard" => Some(icons::VIEW_DASHBOARD),
        "shield-account" => Some(icons::SHIELD_ACCOUNT),
        "tag-minus" => Some(icons::TAG_MINUS),
        "food-croissant" => Some(icons::FOOD_CROISSANT),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "cast-education" => Some(icons::CAST_EDUCATION),
        "engine-outline" => Some(icons::ENGINE_OUTLINE),
        "bow-tie" => Some(icons::BOW_TIE),
        "chevron-left-circle" => Some(icons::CHEVRON_LEFT_CIRCLE),
        "format-align-justify" => Some(icons::FORMAT_ALIGN_JUSTIFY),
        "arrow-bottom-left-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_LEFT_THIN_CIRCLE_OUTLINE)
        }
        "truck-minus-outline" => Some(icons::TRUCK_MINUS_OUTLINE),
        "home-floor-2" => Some(icons::HOME_FLOOR_2),
        "message-image-outline" => Some(icons::MESSAGE_IMAGE_OUTLINE),
        "guy-fawkes-mask" => Some(icons::GUY_FAWKES_MASK),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        "battery-40-bluetooth" => Some(icons::BATTERY_40_BLUETOOTH),
        "database-arrow-right-outline" => Some(icons::DATABASE_ARROW_RIGHT_OUTLINE),
        "bookshelf" => Some(icons::BOOKSHELF),
        "phone-hangup" => Some(icons::PHONE_HANGUP),
        "crop-portrait" => Some(icons::CROP_PORTRAIT),
        "calendar-start" => Some(icons::CALENDAR_START),
        "compass-outline" => Some(icons::COMPASS_OUTLINE),
        "transcribe" => Some(icons::TRANSCRIBE),
        "format-quote-close-outline" => Some(icons::FORMAT_QUOTE_CLOSE_OUTLINE),
        "card-bulleted" => Some(icons::CARD_BULLETED),
        "alarm-snooze" => Some(icons::ALARM_SNOOZE),
        "filmstrip-off" => Some(icons::FILMSTRIP_OFF),
        "home-roof" => Some(icons::HOME_ROOF),
        "bomb-off" => Some(icons::BOMB_OFF),
        "play-outline" => Some(icons::PLAY_OUTLINE),
        "alert" => Some(icons::ALERT),
        "cookie-plus" => Some(icons::COOKIE_PLUS),
        "badge-account" => Some(icons::BADGE_ACCOUNT),
        "database-eye-outline" => Some(icons::DATABASE_EYE_OUTLINE),
        "clipboard-arrow-up-outline" => Some(icons::CLIPBOARD_ARROW_UP_OUTLINE),
        "timer-remove" => Some(icons::TIMER_REMOVE),
        "headphones-box" => Some(icons::HEADPHONES_BOX),
        "phone" => Some(icons::PHONE),
        "file-cog" => Some(icons::FILE_COG),
        "home-group-plus" => Some(icons::HOME_GROUP_PLUS),
        "card" => Some(icons::CARD),
        "baby-bottle" => Some(icons::BABY_BOTTLE),
        "escalator" => Some(icons::ESCALATOR),
        "format-text-wrapping-overflow" => Some(icons::FORMAT_TEXT_WRAPPING_OVERFLOW),
        "table-column-width" => Some(icons::TABLE_COLUMN_WIDTH),
        "tab-remove" => Some(icons::TAB_REMOVE),
        "wifi-marker" => Some(icons::WIFI_MARKER),
        "collage" => Some(icons::COLLAGE),
        "store-24-hour" => Some(icons::STORE_24_HOUR),
        "briefcase-upload" => Some(icons::BRIEFCASE_UPLOAD),
        "file-chart-check" => Some(icons::FILE_CHART_CHECK),
        "arrow-collapse-up" => Some(icons::ARROW_COLLAPSE_UP),
        "head-lightbulb-outline" => Some(icons::HEAD_LIGHTBULB_OUTLINE),
        "folder-cog-outline" => Some(icons::FOLDER_COG_OUTLINE),
        "washing-machine-alert" => Some(icons::WASHING_MACHINE_ALERT),
        "car-key" => Some(icons::CAR_KEY),
        "credit-card" => Some(icons::CREDIT_CARD),
        "emoticon-remove-outline" => Some(icons::EMOTICON_REMOVE_OUTLINE),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "stairs" => Some(icons::STAIRS),
        #[allow(deprecated)]
        "dlna" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dlna' is deprecated.").print(py);
            }
            Some(icons::DLNA)
        }
        "store-clock-outline" => Some(icons::STORE_CLOCK_OUTLINE),
        "battery-charging-wireless-20" => Some(icons::BATTERY_CHARGING_WIRELESS_20),
        "share-variant-outline" => Some(icons::SHARE_VARIANT_OUTLINE),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "form-textbox" => Some(icons::FORM_TEXTBOX),
        "battery-charging-100" => Some(icons::BATTERY_CHARGING_100),
        "head-plus-outline" => Some(icons::HEAD_PLUS_OUTLINE),
        "folder-file" => Some(icons::FOLDER_FILE),
        "circle-medium" => Some(icons::CIRCLE_MEDIUM),
        _ => None,
    }
}
