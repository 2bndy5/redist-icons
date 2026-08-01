// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_31(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "fire-hydrant-alert" => Some(icons::FIRE_HYDRANT_ALERT),
        #[allow(deprecated)]
        "language-javascript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-javascript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_JAVASCRIPT)
        }
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "battery-off" => Some(icons::BATTERY_OFF),
        "file-find-outline" => Some(icons::FILE_FIND_OUTLINE),
        "tooltip-edit" => Some(icons::TOOLTIP_EDIT),
        "clipboard-play" => Some(icons::CLIPBOARD_PLAY),
        "face-man-outline" => Some(icons::FACE_MAN_OUTLINE),
        "archive" => Some(icons::ARCHIVE),
        "book-lock-outline" => Some(icons::BOOK_LOCK_OUTLINE),
        "headset" => Some(icons::HEADSET),
        "folder-pound-outline" => Some(icons::FOLDER_POUND_OUTLINE),
        "smart-card-reader-outline" => Some(icons::SMART_CARD_READER_OUTLINE),
        "pause-circle-outline" => Some(icons::PAUSE_CIRCLE_OUTLINE),
        "pill-off" => Some(icons::PILL_OFF),
        "movie-open-check" => Some(icons::MOVIE_OPEN_CHECK),
        "lightbulb-question" => Some(icons::LIGHTBULB_QUESTION),
        "layers-plus" => Some(icons::LAYERS_PLUS),
        "alpha-w-box" => Some(icons::ALPHA_W_BOX),
        "selection-off" => Some(icons::SELECTION_OFF),
        "filter-cog-outline" => Some(icons::FILTER_COG_OUTLINE),
        "table-minus" => Some(icons::TABLE_MINUS),
        "forward" => Some(icons::FORWARD),
        "squeegee" => Some(icons::SQUEEGEE),
        "power-sleep" => Some(icons::POWER_SLEEP),
        "package-variant-minus" => Some(icons::PACKAGE_VARIANT_MINUS),
        "battery-charging-wireless-30" => Some(icons::BATTERY_CHARGING_WIRELESS_30),
        "microphone-question-outline" => Some(icons::MICROPHONE_QUESTION_OUTLINE),
        "thermostat-box" => Some(icons::THERMOSTAT_BOX),
        "equalizer-outline" => Some(icons::EQUALIZER_OUTLINE),
        "jeepney" => Some(icons::JEEPNEY),
        "bowling" => Some(icons::BOWLING),
        "credit-card-check" => Some(icons::CREDIT_CARD_CHECK),
        "advertisements-off" => Some(icons::ADVERTISEMENTS_OFF),
        "peanut-off" => Some(icons::PEANUT_OFF),
        "book-plus" => Some(icons::BOOK_PLUS),
        "download-circle" => Some(icons::DOWNLOAD_CIRCLE),
        "dots-grid" => Some(icons::DOTS_GRID),
        "wall-sconce" => Some(icons::WALL_SCONCE),
        "shredder" => Some(icons::SHREDDER),
        "lock-alert" => Some(icons::LOCK_ALERT),
        "repeat-variant" => Some(icons::REPEAT_VARIANT),
        "video-box" => Some(icons::VIDEO_BOX),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "tag-arrow-down" => Some(icons::TAG_ARROW_DOWN),
        "human-male-girl" => Some(icons::HUMAN_MALE_GIRL),
        "folder-home" => Some(icons::FOLDER_HOME),
        "lightbulb-fluorescent-tube" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE),
        "view-grid-compact" => Some(icons::VIEW_GRID_COMPACT),
        "store-minus-outline" => Some(icons::STORE_MINUS_OUTLINE),
        "alpha-p-box" => Some(icons::ALPHA_P_BOX),
        "mouse-right-click" => Some(icons::MOUSE_RIGHT_CLICK),
        "flash" => Some(icons::FLASH),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        "source-repository-multiple" => Some(icons::SOURCE_REPOSITORY_MULTIPLE),
        "book-minus-multiple-outline" => Some(icons::BOOK_MINUS_MULTIPLE_OUTLINE),
        "download" => Some(icons::DOWNLOAD),
        #[allow(deprecated)]
        "waze" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'waze' is deprecated.").print(py);
            }
            Some(icons::WAZE)
        }
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
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
        "slide" => Some(icons::SLIDE),
        "molecule-co2" => Some(icons::MOLECULE_CO2),
        "square-wave" => Some(icons::SQUARE_WAVE),
        "coat-rack" => Some(icons::COAT_RACK),
        "folder-play" => Some(icons::FOLDER_PLAY),
        "calendar-filter" => Some(icons::CALENDAR_FILTER),
        "arrow-right" => Some(icons::ARROW_RIGHT),
        "account-remove-outline" => Some(icons::ACCOUNT_REMOVE_OUTLINE),
        "teddy-bear" => Some(icons::TEDDY_BEAR),
        "vector-square-open" => Some(icons::VECTOR_SQUARE_OPEN),
        "message-text-clock-outline" => Some(icons::MESSAGE_TEXT_CLOCK_OUTLINE),
        "cloud-search" => Some(icons::CLOUD_SEARCH),
        "heart-plus-outline" => Some(icons::HEART_PLUS_OUTLINE),
        "database-export" => Some(icons::DATABASE_EXPORT),
        "chili-hot-outline" => Some(icons::CHILI_HOT_OUTLINE),
        "checkbox-intermediate" => Some(icons::CHECKBOX_INTERMEDIATE),
        "wifi-strength-3-lock-open" => Some(icons::WIFI_STRENGTH_3_LOCK_OPEN),
        #[allow(deprecated)]
        "google-maps" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-maps' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_MAPS)
        }
        "sign-language-outline" => Some(icons::SIGN_LANGUAGE_OUTLINE),
        "tilde" => Some(icons::TILDE),
        "bus-school" => Some(icons::BUS_SCHOOL),
        "candy" => Some(icons::CANDY),
        "alpha-o-circle-outline" => Some(icons::ALPHA_O_CIRCLE_OUTLINE),
        "download-off-outline" => Some(icons::DOWNLOAD_OFF_OUTLINE),
        "receipt-text-minus" => Some(icons::RECEIPT_TEXT_MINUS),
        "receipt-clock" => Some(icons::RECEIPT_CLOCK),
        "server-network-off" => Some(icons::SERVER_NETWORK_OFF),
        "checkbox-multiple-blank-outline" => Some(icons::CHECKBOX_MULTIPLE_BLANK_OUTLINE),
        "yeast" => Some(icons::YEAST),
        "star-half" => Some(icons::STAR_HALF),
        "timeline-alert" => Some(icons::TIMELINE_ALERT),
        "drawing-box" => Some(icons::DRAWING_BOX),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        "help-rhombus" => Some(icons::HELP_RHOMBUS),
        "weather-fog" => Some(icons::WEATHER_FOG),
        "playlist-edit" => Some(icons::PLAYLIST_EDIT),
        "timer-minus" => Some(icons::TIMER_MINUS),
        "receipt-text-outline" => Some(icons::RECEIPT_TEXT_OUTLINE),
        "chevron-down" => Some(icons::CHEVRON_DOWN),
        "file-lock-open-outline" => Some(icons::FILE_LOCK_OPEN_OUTLINE),
        "dice-5-outline" => Some(icons::DICE_5_OUTLINE),
        "file-cog" => Some(icons::FILE_COG),
        "check" => Some(icons::CHECK),
        "diversify" => Some(icons::DIVERSIFY),
        "face-woman-shimmer" => Some(icons::FACE_WOMAN_SHIMMER),
        "human-pregnant" => Some(icons::HUMAN_PREGNANT),
        "ski" => Some(icons::SKI),
        "filter-settings" => Some(icons::FILTER_SETTINGS),
        "bell-alert-outline" => Some(icons::BELL_ALERT_OUTLINE),
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        "restart" => Some(icons::RESTART),
        "arrow-up" => Some(icons::ARROW_UP),
        "checkbox-blank-off-outline" => Some(icons::CHECKBOX_BLANK_OFF_OUTLINE),
        "earth-off" => Some(icons::EARTH_OFF),
        "book-play" => Some(icons::BOOK_PLAY),
        "email-open-multiple-outline" => Some(icons::EMAIL_OPEN_MULTIPLE_OUTLINE),
        "account-injury" => Some(icons::ACCOUNT_INJURY),
        "airballoon" => Some(icons::AIRBALLOON),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "account-cancel-outline" => Some(icons::ACCOUNT_CANCEL_OUTLINE),
        "postage-stamp" => Some(icons::POSTAGE_STAMP),
        "dresser-outline" => Some(icons::DRESSER_OUTLINE),
        "archive-marker-outline" => Some(icons::ARCHIVE_MARKER_OUTLINE),
        "format-strikethrough-variant" => Some(icons::FORMAT_STRIKETHROUGH_VARIANT),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "relation-one-to-one-or-many" => Some(icons::RELATION_ONE_TO_ONE_OR_MANY),
        "table-eye" => Some(icons::TABLE_EYE),
        "paragliding" => Some(icons::PARAGLIDING),
        "calendar-sync" => Some(icons::CALENDAR_SYNC),
        "cellphone-off" => Some(icons::CELLPHONE_OFF),
        "package" => Some(icons::PACKAGE),
        "printer-pos-network" => Some(icons::PRINTER_POS_NETWORK),
        "timer-10" => Some(icons::TIMER_10),
        "hand-clap-off" => Some(icons::HAND_CLAP_OFF),
        "share-off-outline" => Some(icons::SHARE_OFF_OUTLINE),
        "snake" => Some(icons::SNAKE),
        "controller-classic-outline" => Some(icons::CONTROLLER_CLASSIC_OUTLINE),
        "emoticon-sick-outline" => Some(icons::EMOTICON_SICK_OUTLINE),
        "water-boiler" => Some(icons::WATER_BOILER),
        "folder-image" => Some(icons::FOLDER_IMAGE),
        "car-defrost-rear" => Some(icons::CAR_DEFROST_REAR),
        "shape-plus" => Some(icons::SHAPE_PLUS),
        "invoice-text" => Some(icons::INVOICE_TEXT),
        "currency-rupee" => Some(icons::CURRENCY_RUPEE),
        "wall" => Some(icons::WALL),
        "timetable" => Some(icons::TIMETABLE),
        "circle-slice-8" => Some(icons::CIRCLE_SLICE_8),
        "camera-lock-open" => Some(icons::CAMERA_LOCK_OPEN),
        "receipt-text-clock-outline" => Some(icons::RECEIPT_TEXT_CLOCK_OUTLINE),
        "access-point-minus" => Some(icons::ACCESS_POINT_MINUS),
        "forwardburger" => Some(icons::FORWARDBURGER),
        "alpha-r-circle-outline" => Some(icons::ALPHA_R_CIRCLE_OUTLINE),
        "basket-fill" => Some(icons::BASKET_FILL),
        "bottle-tonic-skull-outline" => Some(icons::BOTTLE_TONIC_SKULL_OUTLINE),
        "lan-check" => Some(icons::LAN_CHECK),
        "folder-plus-outline" => Some(icons::FOLDER_PLUS_OUTLINE),
        "satellite-uplink" => Some(icons::SATELLITE_UPLINK),
        "format-color-marker-cancel" => Some(icons::FORMAT_COLOR_MARKER_CANCEL),
        "power-off" => Some(icons::POWER_OFF),
        "texture" => Some(icons::TEXTURE),
        "lightbulb-group" => Some(icons::LIGHTBULB_GROUP),
        "weather-cloudy-arrow-right" => Some(icons::WEATHER_CLOUDY_ARROW_RIGHT),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "pliers" => Some(icons::PLIERS),
        "home-sound-in" => Some(icons::HOME_SOUND_IN),
        "routes" => Some(icons::ROUTES),
        "folder-file" => Some(icons::FOLDER_FILE),
        "temple-hindu-outline" => Some(icons::TEMPLE_HINDU_OUTLINE),
        "tally-mark-3" => Some(icons::TALLY_MARK_3),
        "text-box-outline" => Some(icons::TEXT_BOX_OUTLINE),
        "message-text-fast-outline" => Some(icons::MESSAGE_TEXT_FAST_OUTLINE),
        "note-plus" => Some(icons::NOTE_PLUS),
        "close-circle" => Some(icons::CLOSE_CIRCLE),
        "relation-only-one-to-many" => Some(icons::RELATION_ONLY_ONE_TO_MANY),
        "account-box-minus-outline" => Some(icons::ACCOUNT_BOX_MINUS_OUTLINE),
        "emoticon-confused-outline" => Some(icons::EMOTICON_CONFUSED_OUTLINE),
        "format-header-pound" => Some(icons::FORMAT_HEADER_POUND),
        "phone-sync-outline" => Some(icons::PHONE_SYNC_OUTLINE),
        "resistor" => Some(icons::RESISTOR),
        "magnify-plus-cursor" => Some(icons::MAGNIFY_PLUS_CURSOR),
        "bus-side" => Some(icons::BUS_SIDE),
        "shape-plus-outline" => Some(icons::SHAPE_PLUS_OUTLINE),
        "database-arrow-left" => Some(icons::DATABASE_ARROW_LEFT),
        "alpha-f-circle-outline" => Some(icons::ALPHA_F_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "twitch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitch' is deprecated.").print(py);
            }
            Some(icons::TWITCH)
        }
        "border-vertical" => Some(icons::BORDER_VERTICAL),
        "candy-outline" => Some(icons::CANDY_OUTLINE),
        "playlist-music-outline" => Some(icons::PLAYLIST_MUSIC_OUTLINE),
        "guitar-pick-outline" => Some(icons::GUITAR_PICK_OUTLINE),
        "book-cancel" => Some(icons::BOOK_CANCEL),
        "view-stream-outline" => Some(icons::VIEW_STREAM_OUTLINE),
        "file-star-four-points" => Some(icons::FILE_STAR_FOUR_POINTS),
        "arrow-top-left-thin-circle-outline" => Some(icons::ARROW_TOP_LEFT_THIN_CIRCLE_OUTLINE),
        "plus-minus-box" => Some(icons::PLUS_MINUS_BOX),
        "dice-4-outline" => Some(icons::DICE_4_OUTLINE),
        "numeric-9-plus-circle" => Some(icons::NUMERIC_9_PLUS_CIRCLE),
        "clock-outline" => Some(icons::CLOCK_OUTLINE),
        "view-parallel-outline" => Some(icons::VIEW_PARALLEL_OUTLINE),
        "calendar-text-outline" => Some(icons::CALENDAR_TEXT_OUTLINE),
        "security-network" => Some(icons::SECURITY_NETWORK),
        _ => None,
    }
}
