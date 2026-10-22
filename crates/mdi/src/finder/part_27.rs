// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_27(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "cloud-outline" => Some(icons::CLOUD_OUTLINE),
        "clipboard-remove" => Some(icons::CLIPBOARD_REMOVE),
        "timer-pause-outline" => Some(icons::TIMER_PAUSE_OUTLINE),
        "watch-vibrate-off" => Some(icons::WATCH_VIBRATE_OFF),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "numeric-6-circle-outline" => Some(icons::NUMERIC_6_CIRCLE_OUTLINE),
        "car-turbocharger" => Some(icons::CAR_TURBOCHARGER),
        "wrench-outline" => Some(icons::WRENCH_OUTLINE),
        "receipt-text-clock" => Some(icons::RECEIPT_TEXT_CLOCK),
        #[allow(deprecated)]
        "cryengine" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cryengine' is deprecated.").print(py);
            }
            Some(icons::CRYENGINE)
        }
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        "cloud-upload" => Some(icons::CLOUD_UPLOAD),
        "barrel-outline" => Some(icons::BARREL_OUTLINE),
        "camera-off" => Some(icons::CAMERA_OFF),
        "metronome-tick" => Some(icons::METRONOME_TICK),
        "delete-sweep" => Some(icons::DELETE_SWEEP),
        "size-s" => Some(icons::SIZE_S),
        "phone-incoming-outgoing-outline" => Some(icons::PHONE_INCOMING_OUTGOING_OUTLINE),
        "printer-off" => Some(icons::PRINTER_OFF),
        "moon-new" => Some(icons::MOON_NEW),
        "sign-caution" => Some(icons::SIGN_CAUTION),
        "cube-unfolded" => Some(icons::CUBE_UNFOLDED),
        "cloud-lock-outline" => Some(icons::CLOUD_LOCK_OUTLINE),
        "arrow-right-drop-circle" => Some(icons::ARROW_RIGHT_DROP_CIRCLE),
        "map-marker-plus-outline" => Some(icons::MAP_MARKER_PLUS_OUTLINE),
        "movie-outline" => Some(icons::MOVIE_OUTLINE),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "pretzel" => Some(icons::PRETZEL),
        "square-small" => Some(icons::SQUARE_SMALL),
        "tooltip-plus-outline" => Some(icons::TOOLTIP_PLUS_OUTLINE),
        "balcony" => Some(icons::BALCONY),
        "account-cog-outline" => Some(icons::ACCOUNT_COG_OUTLINE),
        "receipt-text-send" => Some(icons::RECEIPT_TEXT_SEND),
        "relation-zero-or-many-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_ONE),
        "login-variant" => Some(icons::LOGIN_VARIANT),
        "information-slab-symbol" => Some(icons::INFORMATION_SLAB_SYMBOL),
        "archive-arrow-up" => Some(icons::ARCHIVE_ARROW_UP),
        "pine-tree-box" => Some(icons::PINE_TREE_BOX),
        "monitor-edit" => Some(icons::MONITOR_EDIT),
        "karate" => Some(icons::KARATE),
        "pan-vertical" => Some(icons::PAN_VERTICAL),
        "blinds-open" => Some(icons::BLINDS_OPEN),
        "alert-minus" => Some(icons::ALERT_MINUS),
        "power-off" => Some(icons::POWER_OFF),
        "roman-numeral-3" => Some(icons::ROMAN_NUMERAL_3),
        "credit-card-fast" => Some(icons::CREDIT_CARD_FAST),
        "clipboard-text-outline" => Some(icons::CLIPBOARD_TEXT_OUTLINE),
        "duck" => Some(icons::DUCK),
        "tshirt-crew-outline" => Some(icons::TSHIRT_CREW_OUTLINE),
        "water-off" => Some(icons::WATER_OFF),
        "star-half" => Some(icons::STAR_HALF),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "calendar-outline" => Some(icons::CALENDAR_OUTLINE),
        "book-alert" => Some(icons::BOOK_ALERT),
        "video-minus-outline" => Some(icons::VIDEO_MINUS_OUTLINE),
        "step-backward" => Some(icons::STEP_BACKWARD),
        "view-stream-outline" => Some(icons::VIEW_STREAM_OUTLINE),
        "sun-wireless" => Some(icons::SUN_WIRELESS),
        "border-inside" => Some(icons::BORDER_INSIDE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-low" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-low' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_LOW)
        }
        "snowman" => Some(icons::SNOWMAN),
        "timer-lock-open-outline" => Some(icons::TIMER_LOCK_OPEN_OUTLINE),
        #[allow(deprecated)]
        "simple-icons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'simple-icons' is deprecated.").print(py);
            }
            Some(icons::SIMPLE_ICONS)
        }
        "floor-lamp-torchiere-variant" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT),
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "calendar-check-outline" => Some(icons::CALENDAR_CHECK_OUTLINE),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "decagram-outline" => Some(icons::DECAGRAM_OUTLINE),
        "content-save-move" => Some(icons::CONTENT_SAVE_MOVE),
        "satellite-variant" => Some(icons::SATELLITE_VARIANT),
        "archive-lock-open-outline" => Some(icons::ARCHIVE_LOCK_OPEN_OUTLINE),
        "earth-minus" => Some(icons::EARTH_MINUS),
        "wifi-cog" => Some(icons::WIFI_COG),
        "fridge-off-outline" => Some(icons::FRIDGE_OFF_OUTLINE),
        "calendar-alert-outline" => Some(icons::CALENDAR_ALERT_OUTLINE),
        "clipboard-alert-outline" => Some(icons::CLIPBOARD_ALERT_OUTLINE),
        "heat-pump" => Some(icons::HEAT_PUMP),
        "spa-outline" => Some(icons::SPA_OUTLINE),
        "car" => Some(icons::CAR),
        "thermometer-check" => Some(icons::THERMOMETER_CHECK),
        "microphone" => Some(icons::MICROPHONE),
        "car-side" => Some(icons::CAR_SIDE),
        "lock-off" => Some(icons::LOCK_OFF),
        "format-text-rotation-up" => Some(icons::FORMAT_TEXT_ROTATION_UP),
        "arrow-top-right-bold-box" => Some(icons::ARROW_TOP_RIGHT_BOLD_BOX),
        "track-light-off" => Some(icons::TRACK_LIGHT_OFF),
        "fridge-industrial" => Some(icons::FRIDGE_INDUSTRIAL),
        "wifi-check" => Some(icons::WIFI_CHECK),
        "globe-light-outline" => Some(icons::GLOBE_LIGHT_OUTLINE),
        "wallet-bifold-outline" => Some(icons::WALLET_BIFOLD_OUTLINE),
        "printer-pos-pause-outline" => Some(icons::PRINTER_POS_PAUSE_OUTLINE),
        "radio" => Some(icons::RADIO),
        "broadcast-off" => Some(icons::BROADCAST_OFF),
        "email-heart-outline" => Some(icons::EMAIL_HEART_OUTLINE),
        "star-off" => Some(icons::STAR_OFF),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "battery-50-bluetooth" => Some(icons::BATTERY_50_BLUETOOTH),
        "currency-thb" => Some(icons::CURRENCY_THB),
        "highway" => Some(icons::HIGHWAY),
        "wifi-arrow-right" => Some(icons::WIFI_ARROW_RIGHT),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "panorama-variant" => Some(icons::PANORAMA_VARIANT),
        "invoice-text-check" => Some(icons::INVOICE_TEXT_CHECK),
        "greater-than" => Some(icons::GREATER_THAN),
        "account" => Some(icons::ACCOUNT),
        "send-clock-outline" => Some(icons::SEND_CLOCK_OUTLINE),
        "tag-check" => Some(icons::TAG_CHECK),
        "credit-card-check" => Some(icons::CREDIT_CARD_CHECK),
        "truck-snowflake" => Some(icons::TRUCK_SNOWFLAKE),
        "invoice-plus" => Some(icons::INVOICE_PLUS),
        "car-seat-heater" => Some(icons::CAR_SEAT_HEATER),
        "format-list-checkbox" => Some(icons::FORMAT_LIST_CHECKBOX),
        "laptop-off" => Some(icons::LAPTOP_OFF),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "vector-polygon-variant" => Some(icons::VECTOR_POLYGON_VARIANT),
        "invoice-text-arrow-left-outline" => Some(icons::INVOICE_TEXT_ARROW_LEFT_OUTLINE),
        "square-medium-outline" => Some(icons::SQUARE_MEDIUM_OUTLINE),
        "wheelchair" => Some(icons::WHEELCHAIR),
        "archive-arrow-down" => Some(icons::ARCHIVE_ARROW_DOWN),
        "content-duplicate" => Some(icons::CONTENT_DUPLICATE),
        "contacts-outline" => Some(icons::CONTACTS_OUTLINE),
        "toy-brick-remove" => Some(icons::TOY_BRICK_REMOVE),
        #[allow(deprecated)]
        "language-lua" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-lua' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_LUA)
        }
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "numeric-8-circle-outline" => Some(icons::NUMERIC_8_CIRCLE_OUTLINE),
        "view-dashboard-edit-outline" => Some(icons::VIEW_DASHBOARD_EDIT_OUTLINE),
        "folder-download-outline" => Some(icons::FOLDER_DOWNLOAD_OUTLINE),
        "thermometer-plus" => Some(icons::THERMOMETER_PLUS),
        "crane" => Some(icons::CRANE),
        "cards-playing-diamond-multiple-outline" => {
            Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE_OUTLINE)
        }
        "image-filter-black-white" => Some(icons::IMAGE_FILTER_BLACK_WHITE),
        "cloud-download-outline" => Some(icons::CLOUD_DOWNLOAD_OUTLINE),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "close-thick" => Some(icons::CLOSE_THICK),
        "upload-multiple-outline" => Some(icons::UPLOAD_MULTIPLE_OUTLINE),
        "playlist-remove" => Some(icons::PLAYLIST_REMOVE),
        "angle-acute" => Some(icons::ANGLE_ACUTE),
        "arrow-right-bold-circle" => Some(icons::ARROW_RIGHT_BOLD_CIRCLE),
        "briefcase-minus" => Some(icons::BRIEFCASE_MINUS),
        "transmission-tower-off" => Some(icons::TRANSMISSION_TOWER_OFF),
        "cards-heart" => Some(icons::CARDS_HEART),
        "weather-partly-cloudy" => Some(icons::WEATHER_PARTLY_CLOUDY),
        "file-powerpoint-outline" => Some(icons::FILE_POWERPOINT_OUTLINE),
        "sleep" => Some(icons::SLEEP),
        "math-compass" => Some(icons::MATH_COMPASS),
        "camera-plus" => Some(icons::CAMERA_PLUS),
        "map-plus" => Some(icons::MAP_PLUS),
        "flask" => Some(icons::FLASK),
        "stethoscope" => Some(icons::STETHOSCOPE),
        "calendar-star-outline" => Some(icons::CALENDAR_STAR_OUTLINE),
        "lightbulb-on-90" => Some(icons::LIGHTBULB_ON_90),
        "database-arrow-right" => Some(icons::DATABASE_ARROW_RIGHT),
        "select-multiple-marker" => Some(icons::SELECT_MULTIPLE_MARKER),
        "bank-off-outline" => Some(icons::BANK_OFF_OUTLINE),
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        "timer-minus-outline" => Some(icons::TIMER_MINUS_OUTLINE),
        "fan-speed-2" => Some(icons::FAN_SPEED_2),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "arrow-down-left-bold" => Some(icons::ARROW_DOWN_LEFT_BOLD),
        "vote" => Some(icons::VOTE),
        "picture-in-picture-top-right" => Some(icons::PICTURE_IN_PICTURE_TOP_RIGHT),
        "leaf" => Some(icons::LEAF),
        "format-bold" => Some(icons::FORMAT_BOLD),
        "floor-lamp-outline" => Some(icons::FLOOR_LAMP_OUTLINE),
        "elephant" => Some(icons::ELEPHANT),
        "solid" => Some(icons::SOLID),
        "whistle" => Some(icons::WHISTLE),
        "folder-heart" => Some(icons::FOLDER_HEART),
        "battery-off-outline" => Some(icons::BATTERY_OFF_OUTLINE),
        "invoice-export-outline" => Some(icons::INVOICE_EXPORT_OUTLINE),
        "cannabis" => Some(icons::CANNABIS),
        "restore-alert" => Some(icons::RESTORE_ALERT),
        "escalator" => Some(icons::ESCALATOR),
        "ellipse-outline" => Some(icons::ELLIPSE_OUTLINE),
        "alpha-p-box-outline" => Some(icons::ALPHA_P_BOX_OUTLINE),
        "checkbox-multiple-blank-outline" => Some(icons::CHECKBOX_MULTIPLE_BLANK_OUTLINE),
        "cloud-check-variant-outline" => Some(icons::CLOUD_CHECK_VARIANT_OUTLINE),
        "circle-slice-3" => Some(icons::CIRCLE_SLICE_3),
        "select-inverse" => Some(icons::SELECT_INVERSE),
        "pail-off-outline" => Some(icons::PAIL_OFF_OUTLINE),
        "folder-outline" => Some(icons::FOLDER_OUTLINE),
        "timer-cancel-outline" => Some(icons::TIMER_CANCEL_OUTLINE),
        "stretch-to-page-outline" => Some(icons::STRETCH_TO_PAGE_OUTLINE),
        #[allow(deprecated)]
        "material-design" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-design' is deprecated.")
                    .print(py);
            }
            Some(icons::MATERIAL_DESIGN)
        }
        "view-day-outline" => Some(icons::VIEW_DAY_OUTLINE),
        "bug-stop" => Some(icons::BUG_STOP),
        "seat-recline-normal" => Some(icons::SEAT_RECLINE_NORMAL),
        "sword" => Some(icons::SWORD),
        "fast-forward-30" => Some(icons::FAST_FORWARD_30),
        "lock-smart" => Some(icons::LOCK_SMART),
        "food-drumstick-outline" => Some(icons::FOOD_DRUMSTICK_OUTLINE),
        "dog" => Some(icons::DOG),
        "moon-waning-gibbous" => Some(icons::MOON_WANING_GIBBOUS),
        "table-tennis" => Some(icons::TABLE_TENNIS),
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "calendar-start-outline" => Some(icons::CALENDAR_START_OUTLINE),
        "fuse-off" => Some(icons::FUSE_OFF),
        "flag-variant-remove-outline" => Some(icons::FLAG_VARIANT_REMOVE_OUTLINE),
        "ski-cross-country" => Some(icons::SKI_CROSS_COUNTRY),
        "bus-articulated-front" => Some(icons::BUS_ARTICULATED_FRONT),
        _ => None,
    }
}
