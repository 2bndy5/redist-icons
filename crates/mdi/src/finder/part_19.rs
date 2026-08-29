// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_19(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "feature-search" => Some(icons::FEATURE_SEARCH),
        "cart" => Some(icons::CART),
        "numeric-7-circle-outline" => Some(icons::NUMERIC_7_CIRCLE_OUTLINE),
        "square-rounded" => Some(icons::SQUARE_ROUNDED),
        "movie-minus-outline" => Some(icons::MOVIE_MINUS_OUTLINE),
        "furigana-vertical" => Some(icons::FURIGANA_VERTICAL),
        "view-comfy" => Some(icons::VIEW_COMFY),
        "octagram-edit-outline" => Some(icons::OCTAGRAM_EDIT_OUTLINE),
        "broom" => Some(icons::BROOM),
        "arrow-collapse-right" => Some(icons::ARROW_COLLAPSE_RIGHT),
        "database-arrow-left-outline" => Some(icons::DATABASE_ARROW_LEFT_OUTLINE),
        "translate-variant" => Some(icons::TRANSLATE_VARIANT),
        "human-handsup" => Some(icons::HUMAN_HANDSUP),
        "molecule-co" => Some(icons::MOLECULE_CO),
        "format-line-height" => Some(icons::FORMAT_LINE_HEIGHT),
        #[allow(deprecated)]
        "kubernetes" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kubernetes' is deprecated.").print(py);
            }
            Some(icons::KUBERNETES)
        }
        "home-minus" => Some(icons::HOME_MINUS),
        "horse" => Some(icons::HORSE),
        "waves" => Some(icons::WAVES),
        "monitor-cellphone-star" => Some(icons::MONITOR_CELLPHONE_STAR),
        "alert-plus" => Some(icons::ALERT_PLUS),
        "eye-off-outline" => Some(icons::EYE_OFF_OUTLINE),
        "floor-lamp-outline" => Some(icons::FLOOR_LAMP_OUTLINE),
        "hand-front-left-outline" => Some(icons::HAND_FRONT_LEFT_OUTLINE),
        "bike-fast" => Some(icons::BIKE_FAST),
        "message" => Some(icons::MESSAGE),
        "apple-keyboard-shift" => Some(icons::APPLE_KEYBOARD_SHIFT),
        "fire-station" => Some(icons::FIRE_STATION),
        "page-first" => Some(icons::PAGE_FIRST),
        "et" => Some(icons::ET),
        "book-refresh-outline" => Some(icons::BOOK_REFRESH_OUTLINE),
        "checkbox-multiple-outline" => Some(icons::CHECKBOX_MULTIPLE_OUTLINE),
        "fingerprint-off" => Some(icons::FINGERPRINT_OFF),
        "undo" => Some(icons::UNDO),
        "movie-open-settings-outline" => Some(icons::MOVIE_OPEN_SETTINGS_OUTLINE),
        "border-all-variant" => Some(icons::BORDER_ALL_VARIANT),
        "silverware-clean" => Some(icons::SILVERWARE_CLEAN),
        "shore" => Some(icons::SHORE),
        "folder-play" => Some(icons::FOLDER_PLAY),
        "map-marker" => Some(icons::MAP_MARKER),
        "account-lock-open-outline" => Some(icons::ACCOUNT_LOCK_OPEN_OUTLINE),
        "music-note-outline" => Some(icons::MUSIC_NOTE_OUTLINE),
        "comment-eye" => Some(icons::COMMENT_EYE),
        "delete-empty" => Some(icons::DELETE_EMPTY),
        "weather-night" => Some(icons::WEATHER_NIGHT),
        "phone-sync" => Some(icons::PHONE_SYNC),
        "format-list-numbered" => Some(icons::FORMAT_LIST_NUMBERED),
        #[allow(deprecated)]
        "bitbucket" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitbucket' is deprecated.").print(py);
            }
            Some(icons::BITBUCKET)
        }
        "check-decagram-outline" => Some(icons::CHECK_DECAGRAM_OUTLINE),
        "speaker-stop" => Some(icons::SPEAKER_STOP),
        "lock-open-minus" => Some(icons::LOCK_OPEN_MINUS),
        "credit-card-clock" => Some(icons::CREDIT_CARD_CLOCK),
        "format-header-increase" => Some(icons::FORMAT_HEADER_INCREASE),
        "counter" => Some(icons::COUNTER),
        "candy-off-outline" => Some(icons::CANDY_OFF_OUTLINE),
        "folder-search-outline" => Some(icons::FOLDER_SEARCH_OUTLINE),
        "water-boiler-alert" => Some(icons::WATER_BOILER_ALERT),
        "lock" => Some(icons::LOCK),
        "source-branch-sync" => Some(icons::SOURCE_BRANCH_SYNC),
        "phone-missed-outline" => Some(icons::PHONE_MISSED_OUTLINE),
        "eye-lock-outline" => Some(icons::EYE_LOCK_OUTLINE),
        "video-plus" => Some(icons::VIDEO_PLUS),
        "menorah-fire" => Some(icons::MENORAH_FIRE),
        "patio-heater" => Some(icons::PATIO_HEATER),
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "card-account-details-star-outline" => Some(icons::CARD_ACCOUNT_DETAILS_STAR_OUTLINE),
        "card-search" => Some(icons::CARD_SEARCH),
        "mosque-outline" => Some(icons::MOSQUE_OUTLINE),
        #[allow(deprecated)]
        "symfony" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'symfony' is deprecated.").print(py);
            }
            Some(icons::SYMFONY)
        }
        "window-closed" => Some(icons::WINDOW_CLOSED),
        "movie-play" => Some(icons::MOVIE_PLAY),
        "close-circle-multiple-outline" => Some(icons::CLOSE_CIRCLE_MULTIPLE_OUTLINE),
        "camera-metering-matrix" => Some(icons::CAMERA_METERING_MATRIX),
        "border-left-variant" => Some(icons::BORDER_LEFT_VARIANT),
        "solar-power-variant" => Some(icons::SOLAR_POWER_VARIANT),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "water-circle" => Some(icons::WATER_CIRCLE),
        "text-recognition" => Some(icons::TEXT_RECOGNITION),
        "drone" => Some(icons::DRONE),
        "card-minus" => Some(icons::CARD_MINUS),
        "cart-outline" => Some(icons::CART_OUTLINE),
        "heart-remove-outline" => Some(icons::HEART_REMOVE_OUTLINE),
        "content-save-plus" => Some(icons::CONTENT_SAVE_PLUS),
        "playlist-check" => Some(icons::PLAYLIST_CHECK),
        "rice" => Some(icons::RICE),
        "credit-card-check" => Some(icons::CREDIT_CARD_CHECK),
        "book-alphabet" => Some(icons::BOOK_ALPHABET),
        "checkbox-marked-circle-plus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_PLUS_OUTLINE),
        "land-plots-circle" => Some(icons::LAND_PLOTS_CIRCLE),
        "message-star-outline" => Some(icons::MESSAGE_STAR_OUTLINE),
        "account-outline" => Some(icons::ACCOUNT_OUTLINE),
        "arrow-u-left-top" => Some(icons::ARROW_U_LEFT_TOP),
        "human-male-girl" => Some(icons::HUMAN_MALE_GIRL),
        "wheelchair" => Some(icons::WHEELCHAIR),
        "sun-thermometer" => Some(icons::SUN_THERMOMETER),
        "home-sound-out-outline" => Some(icons::HOME_SOUND_OUT_OUTLINE),
        "folder-upload" => Some(icons::FOLDER_UPLOAD),
        "tablet" => Some(icons::TABLET),
        "umbrella" => Some(icons::UMBRELLA),
        "account-box-plus-outline" => Some(icons::ACCOUNT_BOX_PLUS_OUTLINE),
        "fridge-variant-outline" => Some(icons::FRIDGE_VARIANT_OUTLINE),
        "printer-3d-nozzle" => Some(icons::PRINTER_3D_NOZZLE),
        "gift-open-outline" => Some(icons::GIFT_OPEN_OUTLINE),
        "camera-marker" => Some(icons::CAMERA_MARKER),
        "call-split" => Some(icons::CALL_SPLIT),
        "symbol" => Some(icons::SYMBOL),
        "nas" => Some(icons::NAS),
        "numeric-9-plus" => Some(icons::NUMERIC_9_PLUS),
        "speaker-multiple" => Some(icons::SPEAKER_MULTIPLE),
        "butterfly" => Some(icons::BUTTERFLY),
        "folder-eye" => Some(icons::FOLDER_EYE),
        "glass-stange" => Some(icons::GLASS_STANGE),
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        "sitemap" => Some(icons::SITEMAP),
        "truck-fast" => Some(icons::TRUCK_FAST),
        "lock-remove" => Some(icons::LOCK_REMOVE),
        "border-left" => Some(icons::BORDER_LEFT),
        "car-windshield-outline" => Some(icons::CAR_WINDSHIELD_OUTLINE),
        "sigma-lower" => Some(icons::SIGMA_LOWER),
        "database-import" => Some(icons::DATABASE_IMPORT),
        "picture-in-picture-bottom-right" => Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT),
        "calendar-week-outline" => Some(icons::CALENDAR_WEEK_OUTLINE),
        "molecule-co2" => Some(icons::MOLECULE_CO2),
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "arrow-top-left-thick" => Some(icons::ARROW_TOP_LEFT_THICK),
        "image-filter-drama-outline" => Some(icons::IMAGE_FILTER_DRAMA_OUTLINE),
        #[allow(deprecated)]
        "arch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'arch' is deprecated.").print(py);
            }
            Some(icons::ARCH)
        }
        "arrow-left-drop-circle" => Some(icons::ARROW_LEFT_DROP_CIRCLE),
        "emoticon-neutral" => Some(icons::EMOTICON_NEUTRAL),
        "sofa" => Some(icons::SOFA),
        "network-strength-1-alert" => Some(icons::NETWORK_STRENGTH_1_ALERT),
        #[allow(deprecated)]
        "nix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nix' is deprecated.").print(py);
            }
            Some(icons::NIX)
        }
        "tooltip-text" => Some(icons::TOOLTIP_TEXT),
        "disc-player" => Some(icons::DISC_PLAYER),
        "frequently-asked-questions" => Some(icons::FREQUENTLY_ASKED_QUESTIONS),
        "numeric-2-box-multiple" => Some(icons::NUMERIC_2_BOX_MULTIPLE),
        #[allow(deprecated)]
        "google-maps" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-maps' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_MAPS)
        }
        "thermometer-probe" => Some(icons::THERMOMETER_PROBE),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "radius-outline" => Some(icons::RADIUS_OUTLINE),
        "pail-outline" => Some(icons::PAIL_OUTLINE),
        "nature-people-outline" => Some(icons::NATURE_PEOPLE_OUTLINE),
        "panorama-variant-outline" => Some(icons::PANORAMA_VARIANT_OUTLINE),
        "cash-off" => Some(icons::CASH_OFF),
        "image-filter-hdr" => Some(icons::IMAGE_FILTER_HDR),
        "checkbox-multiple-blank" => Some(icons::CHECKBOX_MULTIPLE_BLANK),
        "bug-play" => Some(icons::BUG_PLAY),
        "horizontal-rotate-counterclockwise" => Some(icons::HORIZONTAL_ROTATE_COUNTERCLOCKWISE),
        "format-header-decrease" => Some(icons::FORMAT_HEADER_DECREASE),
        "transit-transfer" => Some(icons::TRANSIT_TRANSFER),
        "fast-forward-15" => Some(icons::FAST_FORWARD_15),
        "star" => Some(icons::STAR),
        "map-marker-radius-outline" => Some(icons::MAP_MARKER_RADIUS_OUTLINE),
        "map-plus" => Some(icons::MAP_PLUS),
        "movie-open-minus" => Some(icons::MOVIE_OPEN_MINUS),
        "video-input-component" => Some(icons::VIDEO_INPUT_COMPONENT),
        "cloud-off-outline" => Some(icons::CLOUD_OFF_OUTLINE),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "folder-cog" => Some(icons::FOLDER_COG),
        "image-off" => Some(icons::IMAGE_OFF),
        "numeric-7" => Some(icons::NUMERIC_7),
        "music-circle-outline" => Some(icons::MUSIC_CIRCLE_OUTLINE),
        "checkbox-outline" => Some(icons::CHECKBOX_OUTLINE),
        "sign-direction-remove" => Some(icons::SIGN_DIRECTION_REMOVE),
        "email-minus" => Some(icons::EMAIL_MINUS),
        "printer-wireless" => Some(icons::PRINTER_WIRELESS),
        "camera-party-mode" => Some(icons::CAMERA_PARTY_MODE),
        "lectern" => Some(icons::LECTERN),
        "flashlight" => Some(icons::FLASHLIGHT),
        "distribute-horizontal-left" => Some(icons::DISTRIBUTE_HORIZONTAL_LEFT),
        "gauge" => Some(icons::GAUGE),
        "sort-alphabetical-variant" => Some(icons::SORT_ALPHABETICAL_VARIANT),
        "instrument-triangle" => Some(icons::INSTRUMENT_TRIANGLE),
        "equalizer-outline" => Some(icons::EQUALIZER_OUTLINE),
        #[allow(deprecated)]
        "dev-to" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dev-to' is deprecated.").print(py);
            }
            Some(icons::DEV_TO)
        }
        "receipt-outline" => Some(icons::RECEIPT_OUTLINE),
        "pen" => Some(icons::PEN),
        "emoticon-wink" => Some(icons::EMOTICON_WINK),
        "dock-top" => Some(icons::DOCK_TOP),
        "human-dolly" => Some(icons::HUMAN_DOLLY),
        "decagram" => Some(icons::DECAGRAM),
        "database-export-outline" => Some(icons::DATABASE_EXPORT_OUTLINE),
        "drawing" => Some(icons::DRAWING),
        "alpha-m-circle-outline" => Some(icons::ALPHA_M_CIRCLE_OUTLINE),
        "mushroom-off-outline" => Some(icons::MUSHROOM_OFF_OUTLINE),
        "email-off" => Some(icons::EMAIL_OFF),
        "table-remove" => Some(icons::TABLE_REMOVE),
        "head-minus-outline" => Some(icons::HEAD_MINUS_OUTLINE),
        "axis-arrow-info" => Some(icons::AXIS_ARROW_INFO),
        "book-lock-open" => Some(icons::BOOK_LOCK_OPEN),
        "parking" => Some(icons::PARKING),
        "kabaddi" => Some(icons::KABADDI),
        "transmission-tower-import" => Some(icons::TRANSMISSION_TOWER_IMPORT),
        "account-cowboy-hat" => Some(icons::ACCOUNT_COWBOY_HAT),
        "bookmark-off-outline" => Some(icons::BOOKMARK_OFF_OUTLINE),
        "timer" => Some(icons::TIMER),
        "music-note-half" => Some(icons::MUSIC_NOTE_HALF),
        "wifi-lock" => Some(icons::WIFI_LOCK),
        #[allow(deprecated)]
        "xamarin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xamarin' is deprecated.").print(py);
            }
            Some(icons::XAMARIN)
        }
        "shield-plus-outline" => Some(icons::SHIELD_PLUS_OUTLINE),
        _ => None,
    }
}
