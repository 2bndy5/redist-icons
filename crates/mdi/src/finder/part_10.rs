// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_10(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "folder-heart" => Some(icons::FOLDER_HEART),
        "playlist-plus" => Some(icons::PLAYLIST_PLUS),
        "cupboard" => Some(icons::CUPBOARD),
        "package-variant-closed-minus" => Some(icons::PACKAGE_VARIANT_CLOSED_MINUS),
        "vector-radius" => Some(icons::VECTOR_RADIUS),
        "printer-pos-sync-outline" => Some(icons::PRINTER_POS_SYNC_OUTLINE),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "alpha-b" => Some(icons::ALPHA_B),
        "home-floor-1" => Some(icons::HOME_FLOOR_1),
        "vector-polyline-minus" => Some(icons::VECTOR_POLYLINE_MINUS),
        "death-star" => Some(icons::DEATH_STAR),
        "seed" => Some(icons::SEED),
        "zodiac-capricorn" => Some(icons::ZODIAC_CAPRICORN),
        "omega" => Some(icons::OMEGA),
        "account-check-outline" => Some(icons::ACCOUNT_CHECK_OUTLINE),
        "pan-bottom-right" => Some(icons::PAN_BOTTOM_RIGHT),
        "fruit-watermelon" => Some(icons::FRUIT_WATERMELON),
        "seat-recline-extra" => Some(icons::SEAT_RECLINE_EXTRA),
        "bolt" => Some(icons::BOLT),
        "brightness-1" => Some(icons::BRIGHTNESS_1),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        "emoticon-devil" => Some(icons::EMOTICON_DEVIL),
        "forklift" => Some(icons::FORKLIFT),
        "coffee-maker-check-outline" => Some(icons::COFFEE_MAKER_CHECK_OUTLINE),
        "bike-pedal" => Some(icons::BIKE_PEDAL),
        "curling" => Some(icons::CURLING),
        "wind-power-outline" => Some(icons::WIND_POWER_OUTLINE),
        "tram" => Some(icons::TRAM),
        "flash-red-eye" => Some(icons::FLASH_RED_EYE),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        "numeric-6-box-outline" => Some(icons::NUMERIC_6_BOX_OUTLINE),
        "export-variant" => Some(icons::EXPORT_VARIANT),
        "invoice-arrow-left" => Some(icons::INVOICE_ARROW_LEFT),
        "format-color-highlight" => Some(icons::FORMAT_COLOR_HIGHLIGHT),
        "shredder" => Some(icons::SHREDDER),
        "layers-off" => Some(icons::LAYERS_OFF),
        "relation-one-to-one" => Some(icons::RELATION_ONE_TO_ONE),
        "clipboard-off" => Some(icons::CLIPBOARD_OFF),
        "lead-pencil" => Some(icons::LEAD_PENCIL),
        "bed-double" => Some(icons::BED_DOUBLE),
        "watering-can-outline" => Some(icons::WATERING_CAN_OUTLINE),
        "decimal-decrease" => Some(icons::DECIMAL_DECREASE),
        "phone-check-outline" => Some(icons::PHONE_CHECK_OUTLINE),
        "power-plug-off" => Some(icons::POWER_PLUG_OFF),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "bug-pause-outline" => Some(icons::BUG_PAUSE_OUTLINE),
        "file-document-arrow-right" => Some(icons::FILE_DOCUMENT_ARROW_RIGHT),
        "train-car-passenger-door-open" => Some(icons::TRAIN_CAR_PASSENGER_DOOR_OPEN),
        "spoon-sugar" => Some(icons::SPOON_SUGAR),
        "home-plus" => Some(icons::HOME_PLUS),
        "account-circle" => Some(icons::ACCOUNT_CIRCLE),
        "eye-arrow-right" => Some(icons::EYE_ARROW_RIGHT),
        "alpha-p-box-outline" => Some(icons::ALPHA_P_BOX_OUTLINE),
        "message-lock" => Some(icons::MESSAGE_LOCK),
        "road" => Some(icons::ROAD),
        #[allow(deprecated)]
        "microsoft-azure-devops" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure-devops' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE_DEVOPS)
        }
        "circle-box" => Some(icons::CIRCLE_BOX),
        "trackpad" => Some(icons::TRACKPAD),
        "vanish-quarter" => Some(icons::VANISH_QUARTER),
        "tray" => Some(icons::TRAY),
        "size-s" => Some(icons::SIZE_S),
        "select-multiple-marker" => Some(icons::SELECT_MULTIPLE_MARKER),
        "deskphone" => Some(icons::DESKPHONE),
        "printer-pos-remove" => Some(icons::PRINTER_POS_REMOVE),
        "egg-off-outline" => Some(icons::EGG_OFF_OUTLINE),
        "human-male-male" => Some(icons::HUMAN_MALE_MALE),
        "weather-cloudy-arrow-right" => Some(icons::WEATHER_CLOUDY_ARROW_RIGHT),
        #[allow(deprecated)]
        "laravel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'laravel' is deprecated.").print(py);
            }
            Some(icons::LARAVEL)
        }
        "credit-card-plus" => Some(icons::CREDIT_CARD_PLUS),
        "clipboard-text-outline" => Some(icons::CLIPBOARD_TEXT_OUTLINE),
        "record-rec" => Some(icons::RECORD_REC),
        "call-merge" => Some(icons::CALL_MERGE),
        "tractor-variant" => Some(icons::TRACTOR_VARIANT),
        "card-account-details" => Some(icons::CARD_ACCOUNT_DETAILS),
        "ear-hearing" => Some(icons::EAR_HEARING),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        "gender-male-female" => Some(icons::GENDER_MALE_FEMALE),
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        "garage-variant-lock" => Some(icons::GARAGE_VARIANT_LOCK),
        "mustache" => Some(icons::MUSTACHE),
        "safe-square" => Some(icons::SAFE_SQUARE),
        "lock-smart" => Some(icons::LOCK_SMART),
        "water-alert" => Some(icons::WATER_ALERT),
        "filter-multiple-outline" => Some(icons::FILTER_MULTIPLE_OUTLINE),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "glass-mug" => Some(icons::GLASS_MUG),
        "arrow-horizontal-lock" => Some(icons::ARROW_HORIZONTAL_LOCK),
        "monitor-account" => Some(icons::MONITOR_ACCOUNT),
        "folder-star-multiple-outline" => Some(icons::FOLDER_STAR_MULTIPLE_OUTLINE),
        "bell-minus" => Some(icons::BELL_MINUS),
        "progress-star" => Some(icons::PROGRESS_STAR),
        "camera-document-off" => Some(icons::CAMERA_DOCUMENT_OFF),
        "metronome-tick" => Some(icons::METRONOME_TICK),
        "format-page-split" => Some(icons::FORMAT_PAGE_SPLIT),
        #[allow(deprecated)]
        "origin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'origin' is deprecated.").print(py);
            }
            Some(icons::ORIGIN)
        }
        "restart" => Some(icons::RESTART),
        "beaker-plus-outline" => Some(icons::BEAKER_PLUS_OUTLINE),
        "account-switch-outline" => Some(icons::ACCOUNT_SWITCH_OUTLINE),
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        #[allow(deprecated)]
        "ubuntu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubuntu' is deprecated.").print(py);
            }
            Some(icons::UBUNTU)
        }
        "video-account" => Some(icons::VIDEO_ACCOUNT),
        "sail-boat" => Some(icons::SAIL_BOAT),
        "calendar-remove-outline" => Some(icons::CALENDAR_REMOVE_OUTLINE),
        "arrow-left-thick" => Some(icons::ARROW_LEFT_THICK),
        "timeline" => Some(icons::TIMELINE),
        "pencil-remove-outline" => Some(icons::PENCIL_REMOVE_OUTLINE),
        "alpha-t-box-outline" => Some(icons::ALPHA_T_BOX_OUTLINE),
        "pliers" => Some(icons::PLIERS),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "format-bold" => Some(icons::FORMAT_BOLD),
        "invoice-text-check-outline" => Some(icons::INVOICE_TEXT_CHECK_OUTLINE),
        "bacteria" => Some(icons::BACTERIA),
        "keyboard-return" => Some(icons::KEYBOARD_RETURN),
        "pencil-remove" => Some(icons::PENCIL_REMOVE),
        "cards-club-outline" => Some(icons::CARDS_CLUB_OUTLINE),
        "flag-variant-minus-outline" => Some(icons::FLAG_VARIANT_MINUS_OUTLINE),
        "sun-compass" => Some(icons::SUN_COMPASS),
        "alarm-light" => Some(icons::ALARM_LIGHT),
        "human-male-board" => Some(icons::HUMAN_MALE_BOARD),
        "box-shadow" => Some(icons::BOX_SHADOW),
        "alpha-n-box" => Some(icons::ALPHA_N_BOX),
        "wifi-refresh" => Some(icons::WIFI_REFRESH),
        "bell-plus-outline" => Some(icons::BELL_PLUS_OUTLINE),
        "credit-card-edit" => Some(icons::CREDIT_CARD_EDIT),
        "arrow-left-bold-hexagon-outline" => Some(icons::ARROW_LEFT_BOLD_HEXAGON_OUTLINE),
        "account-settings-outline" => Some(icons::ACCOUNT_SETTINGS_OUTLINE),
        "credit-card-edit-outline" => Some(icons::CREDIT_CARD_EDIT_OUTLINE),
        "format-horizontal-align-center" => Some(icons::FORMAT_HORIZONTAL_ALIGN_CENTER),
        "hair-dryer-outline" => Some(icons::HAIR_DRYER_OUTLINE),
        "account-file-outline" => Some(icons::ACCOUNT_FILE_OUTLINE),
        "movie-open-star" => Some(icons::MOVIE_OPEN_STAR),
        "wifi-remove" => Some(icons::WIFI_REMOVE),
        "timer" => Some(icons::TIMER),
        "comment-minus" => Some(icons::COMMENT_MINUS),
        "information-variant-circle-outline" => Some(icons::INFORMATION_VARIANT_CIRCLE_OUTLINE),
        "open-in-app" => Some(icons::OPEN_IN_APP),
        "help-rhombus-outline" => Some(icons::HELP_RHOMBUS_OUTLINE),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "archive-clock" => Some(icons::ARCHIVE_CLOCK),
        "battery-charging-90" => Some(icons::BATTERY_CHARGING_90),
        "arrow-u-left-bottom" => Some(icons::ARROW_U_LEFT_BOTTOM),
        "cards-playing-spade-multiple-outline" => Some(icons::CARDS_PLAYING_SPADE_MULTIPLE_OUTLINE),
        "map-minus" => Some(icons::MAP_MINUS),
        "sticker-emoji" => Some(icons::STICKER_EMOJI),
        "book-marker-outline" => Some(icons::BOOK_MARKER_OUTLINE),
        "checkbox-marked-circle-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "wordpress" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wordpress' is deprecated.").print(py);
            }
            Some(icons::WORDPRESS)
        }
        "video-2d" => Some(icons::VIDEO_2D),
        "export" => Some(icons::EXPORT),
        #[allow(deprecated)]
        "humble-bundle" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'humble-bundle' is deprecated.").print(py);
            }
            Some(icons::HUMBLE_BUNDLE)
        }
        "wifi-arrow-left" => Some(icons::WIFI_ARROW_LEFT),
        "image-outline" => Some(icons::IMAGE_OUTLINE),
        "gamepad" => Some(icons::GAMEPAD),
        "timer-settings" => Some(icons::TIMER_SETTINGS),
        "arrow-left-circle" => Some(icons::ARROW_LEFT_CIRCLE),
        "forum-outline" => Some(icons::FORUM_OUTLINE),
        "file-image-minus" => Some(icons::FILE_IMAGE_MINUS),
        "monitor-shimmer" => Some(icons::MONITOR_SHIMMER),
        "barcode" => Some(icons::BARCODE),
        "dock-top" => Some(icons::DOCK_TOP),
        "cellphone-sound" => Some(icons::CELLPHONE_SOUND),
        "recycle" => Some(icons::RECYCLE),
        "briefcase-download-outline" => Some(icons::BRIEFCASE_DOWNLOAD_OUTLINE),
        "form-textbox-lock" => Some(icons::FORM_TEXTBOX_LOCK),
        "thermostat" => Some(icons::THERMOSTAT),
        "file-image-outline" => Some(icons::FILE_IMAGE_OUTLINE),
        #[allow(deprecated)]
        "nix" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nix' is deprecated.").print(py);
            }
            Some(icons::NIX)
        }
        "shape-circle-plus" => Some(icons::SHAPE_CIRCLE_PLUS),
        "pencil-box-multiple" => Some(icons::PENCIL_BOX_MULTIPLE),
        "playlist-minus" => Some(icons::PLAYLIST_MINUS),
        "cards-playing-diamond-multiple-outline" => {
            Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE_OUTLINE)
        }
        "fan-off" => Some(icons::FAN_OFF),
        "folder-refresh-outline" => Some(icons::FOLDER_REFRESH_OUTLINE),
        "timeline-clock" => Some(icons::TIMELINE_CLOCK),
        "format-float-right" => Some(icons::FORMAT_FLOAT_RIGHT),
        "airplane-remove" => Some(icons::AIRPLANE_REMOVE),
        "text-box-edit-outline" => Some(icons::TEXT_BOX_EDIT_OUTLINE),
        "comment-processing-outline" => Some(icons::COMMENT_PROCESSING_OUTLINE),
        "chevron-up" => Some(icons::CHEVRON_UP),
        "office-building-remove" => Some(icons::OFFICE_BUILDING_REMOVE),
        "image-filter-center-focus-strong" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG),
        "gift-open-outline" => Some(icons::GIFT_OPEN_OUTLINE),
        "folder-lock" => Some(icons::FOLDER_LOCK),
        "flask-empty-remove" => Some(icons::FLASK_EMPTY_REMOVE),
        "folder-network-outline" => Some(icons::FOLDER_NETWORK_OUTLINE),
        "application" => Some(icons::APPLICATION),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "water-plus-outline" => Some(icons::WATER_PLUS_OUTLINE),
        #[allow(deprecated)]
        "cryengine" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cryengine' is deprecated.").print(py);
            }
            Some(icons::CRYENGINE)
        }
        "hammer-wrench" => Some(icons::HAMMER_WRENCH),
        "database-remove-outline" => Some(icons::DATABASE_REMOVE_OUTLINE),
        "phone-refresh" => Some(icons::PHONE_REFRESH),
        "sledding" => Some(icons::SLEDDING),
        "printer-pos-refresh" => Some(icons::PRINTER_POS_REFRESH),
        "smoke-detector-outline" => Some(icons::SMOKE_DETECTOR_OUTLINE),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "cloud-minus-outline" => Some(icons::CLOUD_MINUS_OUTLINE),
        "table-large-remove" => Some(icons::TABLE_LARGE_REMOVE),
        "book-education-outline" => Some(icons::BOOK_EDUCATION_OUTLINE),
        _ => None,
    }
}
