// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_5(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "dice-d6-outline" => Some(icons::DICE_D6_OUTLINE),
        "clipboard-arrow-right" => Some(icons::CLIPBOARD_ARROW_RIGHT),
        "credit-card-wireless-off-outline" => Some(icons::CREDIT_CARD_WIRELESS_OFF_OUTLINE),
        "minus-circle-off-outline" => Some(icons::MINUS_CIRCLE_OFF_OUTLINE),
        "home-edit-outline" => Some(icons::HOME_EDIT_OUTLINE),
        "scoreboard-outline" => Some(icons::SCOREBOARD_OUTLINE),
        "keyboard-off" => Some(icons::KEYBOARD_OFF),
        "bookmark-music-outline" => Some(icons::BOOKMARK_MUSIC_OUTLINE),
        "alpha-t-circle-outline" => Some(icons::ALPHA_T_CIRCLE_OUTLINE),
        "invoice-list" => Some(icons::INVOICE_LIST),
        "broom" => Some(icons::BROOM),
        #[allow(deprecated)]
        "ansible" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ansible' is deprecated.").print(py);
            }
            Some(icons::ANSIBLE)
        }
        "alpha-k" => Some(icons::ALPHA_K),
        "shower" => Some(icons::SHOWER),
        "face-woman-outline" => Some(icons::FACE_WOMAN_OUTLINE),
        "cookie-check-outline" => Some(icons::COOKIE_CHECK_OUTLINE),
        "weather-night" => Some(icons::WEATHER_NIGHT),
        "clock-star-four-points-outline" => Some(icons::CLOCK_STAR_FOUR_POINTS_OUTLINE),
        "folder-lock-open" => Some(icons::FOLDER_LOCK_OPEN),
        "gender-female" => Some(icons::GENDER_FEMALE),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "store-outline" => Some(icons::STORE_OUTLINE),
        "file-image-remove" => Some(icons::FILE_IMAGE_REMOVE),
        "table-plus" => Some(icons::TABLE_PLUS),
        "clover" => Some(icons::CLOVER),
        "alpha-e-circle-outline" => Some(icons::ALPHA_E_CIRCLE_OUTLINE),
        "web" => Some(icons::WEB),
        "checkbox-marked-circle-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_OUTLINE),
        "regex" => Some(icons::REGEX),
        "axis-arrow-info" => Some(icons::AXIS_ARROW_INFO),
        "image-filter-none" => Some(icons::IMAGE_FILTER_NONE),
        "logout" => Some(icons::LOGOUT),
        "briefcase-off-outline" => Some(icons::BRIEFCASE_OFF_OUTLINE),
        "train-car-flatbed-tank" => Some(icons::TRAIN_CAR_FLATBED_TANK),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "bookmark-multiple-outline" => Some(icons::BOOKMARK_MULTIPLE_OUTLINE),
        "airplane-off" => Some(icons::AIRPLANE_OFF),
        "abjad-arabic" => Some(icons::ABJAD_ARABIC),
        "alarm-light" => Some(icons::ALARM_LIGHT),
        "puzzle-star-outline" => Some(icons::PUZZLE_STAR_OUTLINE),
        "folder-remove" => Some(icons::FOLDER_REMOVE),
        "gauge-full" => Some(icons::GAUGE_FULL),
        "phone-off-outline" => Some(icons::PHONE_OFF_OUTLINE),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "earth-plus" => Some(icons::EARTH_PLUS),
        "star-four-points-box" => Some(icons::STAR_FOUR_POINTS_BOX),
        "pumpkin" => Some(icons::PUMPKIN),
        "earth" => Some(icons::EARTH),
        "television-pause" => Some(icons::TELEVISION_PAUSE),
        "shield-sync-outline" => Some(icons::SHIELD_SYNC_OUTLINE),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "file-check" => Some(icons::FILE_CHECK),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "barcode-scan" => Some(icons::BARCODE_SCAN),
        "umbrella-beach" => Some(icons::UMBRELLA_BEACH),
        "smart-card-off" => Some(icons::SMART_CARD_OFF),
        "monitor-share" => Some(icons::MONITOR_SHARE),
        "download-network-outline" => Some(icons::DOWNLOAD_NETWORK_OUTLINE),
        "hammer-screwdriver" => Some(icons::HAMMER_SCREWDRIVER),
        "music-note-off-outline" => Some(icons::MUSIC_NOTE_OFF_OUTLINE),
        "arrow-collapse-left" => Some(icons::ARROW_COLLAPSE_LEFT),
        "numeric-9-plus-box-multiple" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE),
        "baby-bottle" => Some(icons::BABY_BOTTLE),
        "invoice-text-edit" => Some(icons::INVOICE_TEXT_EDIT),
        "delete-circle" => Some(icons::DELETE_CIRCLE),
        "phone-alert" => Some(icons::PHONE_ALERT),
        "fish" => Some(icons::FISH),
        "bell-plus" => Some(icons::BELL_PLUS),
        "dots-horizontal-circle-outline" => Some(icons::DOTS_HORIZONTAL_CIRCLE_OUTLINE),
        "database-remove-outline" => Some(icons::DATABASE_REMOVE_OUTLINE),
        "size-xl" => Some(icons::SIZE_XL),
        "folder-star-multiple-outline" => Some(icons::FOLDER_STAR_MULTIPLE_OUTLINE),
        "heart-outline" => Some(icons::HEART_OUTLINE),
        "receipt-text-clock" => Some(icons::RECEIPT_TEXT_CLOCK),
        "movie-filter" => Some(icons::MOVIE_FILTER),
        "biohazard" => Some(icons::BIOHAZARD),
        "movie-open-plus-outline" => Some(icons::MOVIE_OPEN_PLUS_OUTLINE),
        "folder-text" => Some(icons::FOLDER_TEXT),
        "zodiac-virgo" => Some(icons::ZODIAC_VIRGO),
        #[allow(deprecated)]
        "microsoft-xbox-controller-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-view' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_VIEW)
        }
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "phone-refresh-outline" => Some(icons::PHONE_REFRESH_OUTLINE),
        "checkbox-marked-circle-minus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_MINUS_OUTLINE),
        "view-dashboard-variant-outline" => Some(icons::VIEW_DASHBOARD_VARIANT_OUTLINE),
        "cards-playing-spade" => Some(icons::CARDS_PLAYING_SPADE),
        "web-refresh" => Some(icons::WEB_REFRESH),
        "account-box" => Some(icons::ACCOUNT_BOX),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "expansion-card" => Some(icons::EXPANSION_CARD),
        "sack-outline" => Some(icons::SACK_OUTLINE),
        "account-music" => Some(icons::ACCOUNT_MUSIC),
        "alpha-q-box-outline" => Some(icons::ALPHA_Q_BOX_OUTLINE),
        "checkbook-arrow-left" => Some(icons::CHECKBOOK_ARROW_LEFT),
        "chevron-double-right" => Some(icons::CHEVRON_DOUBLE_RIGHT),
        "share-off" => Some(icons::SHARE_OFF),
        "phone-settings" => Some(icons::PHONE_SETTINGS),
        "face-man" => Some(icons::FACE_MAN),
        "alpha-c-circle" => Some(icons::ALPHA_C_CIRCLE),
        "shield-lock-open-outline" => Some(icons::SHIELD_LOCK_OPEN_OUTLINE),
        "radio" => Some(icons::RADIO),
        "progress-tag" => Some(icons::PROGRESS_TAG),
        "refresh-auto" => Some(icons::REFRESH_AUTO),
        "liquid-spot" => Some(icons::LIQUID_SPOT),
        "car-search" => Some(icons::CAR_SEARCH),
        "chevron-triple-down" => Some(icons::CHEVRON_TRIPLE_DOWN),
        "star-box-multiple" => Some(icons::STAR_BOX_MULTIPLE),
        "alpha-p-circle" => Some(icons::ALPHA_P_CIRCLE),
        "coffee-off-outline" => Some(icons::COFFEE_OFF_OUTLINE),
        "emoticon-sad-outline" => Some(icons::EMOTICON_SAD_OUTLINE),
        "crosshairs-gps" => Some(icons::CROSSHAIRS_GPS),
        "pen" => Some(icons::PEN),
        "card-bulleted" => Some(icons::CARD_BULLETED),
        "hand-cycle" => Some(icons::HAND_CYCLE),
        "glass-wine" => Some(icons::GLASS_WINE),
        "sort-bool-ascending-variant" => Some(icons::SORT_BOOL_ASCENDING_VARIANT),
        "flag-variant-minus" => Some(icons::FLAG_VARIANT_MINUS),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "human-child" => Some(icons::HUMAN_CHILD),
        "image-area" => Some(icons::IMAGE_AREA),
        "sticker-minus-outline" => Some(icons::STICKER_MINUS_OUTLINE),
        "microphone-outline" => Some(icons::MICROPHONE_OUTLINE),
        "home-variant-outline" => Some(icons::HOME_VARIANT_OUTLINE),
        "crosshairs-question" => Some(icons::CROSSHAIRS_QUESTION),
        "mushroom-off-outline" => Some(icons::MUSHROOM_OFF_OUTLINE),
        "seed-off-outline" => Some(icons::SEED_OFF_OUTLINE),
        "shield-off-outline" => Some(icons::SHIELD_OFF_OUTLINE),
        #[allow(deprecated)]
        "google-earth" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-earth' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_EARTH)
        }
        "timer" => Some(icons::TIMER),
        "map-plus" => Some(icons::MAP_PLUS),
        "thermostat-box" => Some(icons::THERMOSTAT_BOX),
        "message-processing-outline" => Some(icons::MESSAGE_PROCESSING_OUTLINE),
        "folder-star-multiple" => Some(icons::FOLDER_STAR_MULTIPLE),
        "surround-sound-5-1" => Some(icons::SURROUND_SOUND_5_1),
        "store" => Some(icons::STORE),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "panorama" => Some(icons::PANORAMA),
        "order-bool-descending-variant" => Some(icons::ORDER_BOOL_DESCENDING_VARIANT),
        "account-question-outline" => Some(icons::ACCOUNT_QUESTION_OUTLINE),
        "cart-check" => Some(icons::CART_CHECK),
        "chili-mild-outline" => Some(icons::CHILI_MILD_OUTLINE),
        "knob" => Some(icons::KNOB),
        "garage-open-variant" => Some(icons::GARAGE_OPEN_VARIANT),
        "file-eye-outline" => Some(icons::FILE_EYE_OUTLINE),
        "transit-connection-variant" => Some(icons::TRANSIT_CONNECTION_VARIANT),
        "database-search-outline" => Some(icons::DATABASE_SEARCH_OUTLINE),
        #[allow(deprecated)]
        "mastodon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mastodon' is deprecated.").print(py);
            }
            Some(icons::MASTODON)
        }
        "email-off-outline" => Some(icons::EMAIL_OFF_OUTLINE),
        "slot-machine-outline" => Some(icons::SLOT_MACHINE_OUTLINE),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        "seat-legroom-extra" => Some(icons::SEAT_LEGROOM_EXTRA),
        "bottle-tonic-outline" => Some(icons::BOTTLE_TONIC_OUTLINE),
        "file-delimited" => Some(icons::FILE_DELIMITED),
        "view-dashboard-edit" => Some(icons::VIEW_DASHBOARD_EDIT),
        "hexagon-slice-1" => Some(icons::HEXAGON_SLICE_1),
        "window-shutter-alert" => Some(icons::WINDOW_SHUTTER_ALERT),
        "incognito-off" => Some(icons::INCOGNITO_OFF),
        "meditation" => Some(icons::MEDITATION),
        "swim" => Some(icons::SWIM),
        "tshirt-v" => Some(icons::TSHIRT_V),
        "white-balance-iridescent" => Some(icons::WHITE_BALANCE_IRIDESCENT),
        "server-minus-outline" => Some(icons::SERVER_MINUS_OUTLINE),
        "arrow-up-thick" => Some(icons::ARROW_UP_THICK),
        "alpha-q" => Some(icons::ALPHA_Q),
        "cube-unfolded" => Some(icons::CUBE_UNFOLDED),
        "numeric-7-circle" => Some(icons::NUMERIC_7_CIRCLE),
        "beaker-minus" => Some(icons::BEAKER_MINUS),
        "matrix" => Some(icons::MATRIX),
        "account-clock" => Some(icons::ACCOUNT_CLOCK),
        "video-2d" => Some(icons::VIDEO_2D),
        "border-left" => Some(icons::BORDER_LEFT),
        #[allow(deprecated)]
        "nuxt" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nuxt' is deprecated.").print(py);
            }
            Some(icons::NUXT)
        }
        "bank-circle-outline" => Some(icons::BANK_CIRCLE_OUTLINE),
        "arrow-down-left-bold" => Some(icons::ARROW_DOWN_LEFT_BOLD),
        "folder-multiple-plus-outline" => Some(icons::FOLDER_MULTIPLE_PLUS_OUTLINE),
        "arrow-up-down-bold" => Some(icons::ARROW_UP_DOWN_BOLD),
        "home-account" => Some(icons::HOME_ACCOUNT),
        "checkbox-marked" => Some(icons::CHECKBOX_MARKED),
        "television-classic" => Some(icons::TELEVISION_CLASSIC),
        "fire" => Some(icons::FIRE),
        "heart-broken" => Some(icons::HEART_BROKEN),
        "beaker-alert" => Some(icons::BEAKER_ALERT),
        "fast-forward-15" => Some(icons::FAST_FORWARD_15),
        "garage-open" => Some(icons::GARAGE_OPEN),
        "microscope" => Some(icons::MICROSCOPE),
        #[allow(deprecated)]
        "language-typescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-typescript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_TYPESCRIPT)
        }
        "ornament" => Some(icons::ORNAMENT),
        "format-list-numbered" => Some(icons::FORMAT_LIST_NUMBERED),
        #[allow(deprecated)]
        "humble-bundle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'humble-bundle' is deprecated.").print(py);
            }
            Some(icons::HUMBLE_BUNDLE)
        }
        "database-search" => Some(icons::DATABASE_SEARCH),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "minus-box-multiple" => Some(icons::MINUS_BOX_MULTIPLE),
        "alphabetical-off" => Some(icons::ALPHABETICAL_OFF),
        "shield-home" => Some(icons::SHIELD_HOME),
        "temperature-fahrenheit" => Some(icons::TEMPERATURE_FAHRENHEIT),
        "video-marker-outline" => Some(icons::VIDEO_MARKER_OUTLINE),
        "account-tie-voice-off" => Some(icons::ACCOUNT_TIE_VOICE_OFF),
        "vote" => Some(icons::VOTE),
        "arrow-up-bold-circle-outline" => Some(icons::ARROW_UP_BOLD_CIRCLE_OUTLINE),
        "scale" => Some(icons::SCALE),
        "timer-edit" => Some(icons::TIMER_EDIT),
        _ => None,
    }
}
