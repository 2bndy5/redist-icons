// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_14(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "phone-off-outline" => Some(icons::PHONE_OFF_OUTLINE),
        "invoice-text-send" => Some(icons::INVOICE_TEXT_SEND),
        "archive-alert-outline" => Some(icons::ARCHIVE_ALERT_OUTLINE),
        "diameter-outline" => Some(icons::DIAMETER_OUTLINE),
        "file-export" => Some(icons::FILE_EXPORT),
        "movie-open-outline" => Some(icons::MOVIE_OPEN_OUTLINE),
        "lock-alert" => Some(icons::LOCK_ALERT),
        "water-remove-outline" => Some(icons::WATER_REMOVE_OUTLINE),
        "chili-mild-outline" => Some(icons::CHILI_MILD_OUTLINE),
        "cart-minus" => Some(icons::CART_MINUS),
        "emoticon-plus" => Some(icons::EMOTICON_PLUS),
        "arrow-left-thin" => Some(icons::ARROW_LEFT_THIN),
        "clipboard-plus" => Some(icons::CLIPBOARD_PLUS),
        "wallet-bifold-outline" => Some(icons::WALLET_BIFOLD_OUTLINE),
        "send-check-outline" => Some(icons::SEND_CHECK_OUTLINE),
        "dice-multiple" => Some(icons::DICE_MULTIPLE),
        "crosshairs-gps" => Some(icons::CROSSHAIRS_GPS),
        "bookmark-music" => Some(icons::BOOKMARK_MUSIC),
        "phone" => Some(icons::PHONE),
        "home-lock-open" => Some(icons::HOME_LOCK_OPEN),
        "book-minus-outline" => Some(icons::BOOK_MINUS_OUTLINE),
        "arrow-left-thick" => Some(icons::ARROW_LEFT_THICK),
        "seed-plus-outline" => Some(icons::SEED_PLUS_OUTLINE),
        "calendar-star" => Some(icons::CALENDAR_STAR),
        "gas-station-off-outline" => Some(icons::GAS_STATION_OFF_OUTLINE),
        "vhs" => Some(icons::VHS),
        "video-input-svideo" => Some(icons::VIDEO_INPUT_SVIDEO),
        "equal-box" => Some(icons::EQUAL_BOX),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "briefcase-arrow-up-down-outline" => Some(icons::BRIEFCASE_ARROW_UP_DOWN_OUTLINE),
        "alpha-y-circle-outline" => Some(icons::ALPHA_Y_CIRCLE_OUTLINE),
        "cellphone-off" => Some(icons::CELLPHONE_OFF),
        "clipboard-check-outline" => Some(icons::CLIPBOARD_CHECK_OUTLINE),
        "hexagon-slice-1" => Some(icons::HEXAGON_SLICE_1),
        "selection-marker" => Some(icons::SELECTION_MARKER),
        "checkbox-multiple-blank-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_BLANK_CIRCLE_OUTLINE)
        }
        "wrench-outline" => Some(icons::WRENCH_OUTLINE),
        "call-received" => Some(icons::CALL_RECEIVED),
        "calendar-today" => Some(icons::CALENDAR_TODAY),
        "mortar-pestle-plus" => Some(icons::MORTAR_PESTLE_PLUS),
        "weather-partly-rainy" => Some(icons::WEATHER_PARTLY_RAINY),
        "bell-remove" => Some(icons::BELL_REMOVE),
        "bug-play-outline" => Some(icons::BUG_PLAY_OUTLINE),
        "wifi-arrow-left" => Some(icons::WIFI_ARROW_LEFT),
        "undo-variant" => Some(icons::UNDO_VARIANT),
        "wind-turbine-check" => Some(icons::WIND_TURBINE_CHECK),
        "not-equal-variant" => Some(icons::NOT_EQUAL_VARIANT),
        "baseball-diamond" => Some(icons::BASEBALL_DIAMOND),
        "music-off" => Some(icons::MUSIC_OFF),
        "database-marker" => Some(icons::DATABASE_MARKER),
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        "shield-lock-open" => Some(icons::SHIELD_LOCK_OPEN),
        "shield-lock-open-outline" => Some(icons::SHIELD_LOCK_OPEN_OUTLINE),
        "clipboard-text-play-outline" => Some(icons::CLIPBOARD_TEXT_PLAY_OUTLINE),
        "slot-machine-outline" => Some(icons::SLOT_MACHINE_OUTLINE),
        "alpha-r-box-outline" => Some(icons::ALPHA_R_BOX_OUTLINE),
        #[allow(deprecated)]
        "skype" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'skype' is deprecated.").print(py);
            }
            Some(icons::SKYPE)
        }
        "block-helper" => Some(icons::BLOCK_HELPER),
        "home-thermometer-outline" => Some(icons::HOME_THERMOMETER_OUTLINE),
        "fleur-de-lis" => Some(icons::FLEUR_DE_LIS),
        #[allow(deprecated)]
        "semantic-web" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'semantic-web' is deprecated.").print(py);
            }
            Some(icons::SEMANTIC_WEB)
        }
        "resistor-nodes" => Some(icons::RESISTOR_NODES),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "ideogram-cjk-variant" => Some(icons::IDEOGRAM_CJK_VARIANT),
        "label-off-outline" => Some(icons::LABEL_OFF_OUTLINE),
        "briefcase-check-outline" => Some(icons::BRIEFCASE_CHECK_OUTLINE),
        "window-minimize" => Some(icons::WINDOW_MINIMIZE),
        "file-code-outline" => Some(icons::FILE_CODE_OUTLINE),
        "arrow-down-bold" => Some(icons::ARROW_DOWN_BOLD),
        "upload-off" => Some(icons::UPLOAD_OFF),
        "view-dashboard-outline" => Some(icons::VIEW_DASHBOARD_OUTLINE),
        "routes-clock" => Some(icons::ROUTES_CLOCK),
        "door" => Some(icons::DOOR),
        "crown-outline" => Some(icons::CROWN_OUTLINE),
        "emoticon-remove-outline" => Some(icons::EMOTICON_REMOVE_OUTLINE),
        "face-man" => Some(icons::FACE_MAN),
        "alpha-o-circle-outline" => Some(icons::ALPHA_O_CIRCLE_OUTLINE),
        "file-remove-outline" => Some(icons::FILE_REMOVE_OUTLINE),
        "table-arrow-right" => Some(icons::TABLE_ARROW_RIGHT),
        "tag-off" => Some(icons::TAG_OFF),
        "rotate-3d" => Some(icons::ROTATE_3D),
        "comment-arrow-right" => Some(icons::COMMENT_ARROW_RIGHT),
        "download-box-outline" => Some(icons::DOWNLOAD_BOX_OUTLINE),
        "bell-alert" => Some(icons::BELL_ALERT),
        "folder-arrow-right-outline" => Some(icons::FOLDER_ARROW_RIGHT_OUTLINE),
        "egg-fried" => Some(icons::EGG_FRIED),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "cloud-key-outline" => Some(icons::CLOUD_KEY_OUTLINE),
        "stretch-to-page" => Some(icons::STRETCH_TO_PAGE),
        "phone-refresh-outline" => Some(icons::PHONE_REFRESH_OUTLINE),
        #[allow(deprecated)]
        "microsoft-azure" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE)
        }
        "crop-square" => Some(icons::CROP_SQUARE),
        "movie-open-plus" => Some(icons::MOVIE_OPEN_PLUS),
        "bomb" => Some(icons::BOMB),
        "library-outline" => Some(icons::LIBRARY_OUTLINE),
        "traffic-light" => Some(icons::TRAFFIC_LIGHT),
        "wiper" => Some(icons::WIPER),
        "currency-mnt" => Some(icons::CURRENCY_MNT),
        "invoice-clock-outline" => Some(icons::INVOICE_CLOCK_OUTLINE),
        "snail" => Some(icons::SNAIL),
        "scoreboard" => Some(icons::SCOREBOARD),
        "silverware-fork" => Some(icons::SILVERWARE_FORK),
        #[allow(deprecated)]
        "midi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'midi' is deprecated.").print(py);
            }
            Some(icons::MIDI)
        }
        "parachute" => Some(icons::PARACHUTE),
        "download-box" => Some(icons::DOWNLOAD_BOX),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        "projector-screen-variant-off-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF_OUTLINE),
        "relation-zero-or-many-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_ONE),
        "format-text-rotation-up" => Some(icons::FORMAT_TEXT_ROTATION_UP),
        "spellcheck" => Some(icons::SPELLCHECK),
        "filter-off-outline" => Some(icons::FILTER_OFF_OUTLINE),
        "circle-expand" => Some(icons::CIRCLE_EXPAND),
        "numeric-1-box-multiple" => Some(icons::NUMERIC_1_BOX_MULTIPLE),
        "eye-plus" => Some(icons::EYE_PLUS),
        "chart-timeline-variant" => Some(icons::CHART_TIMELINE_VARIANT),
        "account-badge-outline" => Some(icons::ACCOUNT_BADGE_OUTLINE),
        "cradle" => Some(icons::CRADLE),
        "bus-articulated-front" => Some(icons::BUS_ARTICULATED_FRONT),
        "timeline-question" => Some(icons::TIMELINE_QUESTION),
        "email-arrow-right" => Some(icons::EMAIL_ARROW_RIGHT),
        "cloud-lock-open" => Some(icons::CLOUD_LOCK_OPEN),
        "cursor-move" => Some(icons::CURSOR_MOVE),
        "kite-outline" => Some(icons::KITE_OUTLINE),
        "message-minus" => Some(icons::MESSAGE_MINUS),
        "animation-play-outline" => Some(icons::ANIMATION_PLAY_OUTLINE),
        "slot-machine" => Some(icons::SLOT_MACHINE),
        "card-multiple" => Some(icons::CARD_MULTIPLE),
        #[allow(deprecated)]
        "microsoft-bing" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-bing' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_BING)
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
        "numeric-2-circle-outline" => Some(icons::NUMERIC_2_CIRCLE_OUTLINE),
        "flag-plus-outline" => Some(icons::FLAG_PLUS_OUTLINE),
        "earth" => Some(icons::EARTH),
        #[allow(deprecated)]
        "qqchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qqchat' is deprecated.").print(py);
            }
            Some(icons::QQCHAT)
        }
        "message-plus-outline" => Some(icons::MESSAGE_PLUS_OUTLINE),
        "vector-arrange-below" => Some(icons::VECTOR_ARRANGE_BELOW),
        "currency-jpy" => Some(icons::CURRENCY_JPY),
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "file-image-outline" => Some(icons::FILE_IMAGE_OUTLINE),
        "paragliding" => Some(icons::PARAGLIDING),
        "cloud-circle" => Some(icons::CLOUD_CIRCLE),
        "calendar-month" => Some(icons::CALENDAR_MONTH),
        "pentagon-outline" => Some(icons::PENTAGON_OUTLINE),
        "cube-scan" => Some(icons::CUBE_SCAN),
        "opacity" => Some(icons::OPACITY),
        "image-remove" => Some(icons::IMAGE_REMOVE),
        "application-cog-outline" => Some(icons::APPLICATION_COG_OUTLINE),
        "weather-dust" => Some(icons::WEATHER_DUST),
        "receipt-text-arrow-left-outline" => Some(icons::RECEIPT_TEXT_ARROW_LEFT_OUTLINE),
        "speaker" => Some(icons::SPEAKER),
        "tag-outline" => Some(icons::TAG_OUTLINE),
        "watch-import" => Some(icons::WATCH_IMPORT),
        "gesture-swipe-down" => Some(icons::GESTURE_SWIPE_DOWN),
        "calendar-week" => Some(icons::CALENDAR_WEEK),
        "router-wireless-settings" => Some(icons::ROUTER_WIRELESS_SETTINGS),
        "layers-search" => Some(icons::LAYERS_SEARCH),
        "silverware-spoon" => Some(icons::SILVERWARE_SPOON),
        "marker-check" => Some(icons::MARKER_CHECK),
        "order-alphabetical-ascending" => Some(icons::ORDER_ALPHABETICAL_ASCENDING),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "printer-pos-star" => Some(icons::PRINTER_POS_STAR),
        "food-drumstick" => Some(icons::FOOD_DRUMSTICK),
        #[allow(deprecated)]
        "language-kotlin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-kotlin' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_KOTLIN)
        }
        "medal-outline" => Some(icons::MEDAL_OUTLINE),
        "home-flood" => Some(icons::HOME_FLOOD),
        "circle-box" => Some(icons::CIRCLE_BOX),
        "battery-charging-wireless-40" => Some(icons::BATTERY_CHARGING_WIRELESS_40),
        "heat-pump-outline" => Some(icons::HEAT_PUMP_OUTLINE),
        "note" => Some(icons::NOTE),
        "middleware-outline" => Some(icons::MIDDLEWARE_OUTLINE),
        "table-column" => Some(icons::TABLE_COLUMN),
        "sword-cross" => Some(icons::SWORD_CROSS),
        "currency-kzt" => Some(icons::CURRENCY_KZT),
        #[allow(deprecated)]
        "bootstrap" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bootstrap' is deprecated.").print(py);
            }
            Some(icons::BOOTSTRAP)
        }
        "file-download" => Some(icons::FILE_DOWNLOAD),
        "hexagon-multiple" => Some(icons::HEXAGON_MULTIPLE),
        "fast-forward-30" => Some(icons::FAST_FORWARD_30),
        "beaker-check" => Some(icons::BEAKER_CHECK),
        "file-move" => Some(icons::FILE_MOVE),
        "location-enter" => Some(icons::LOCATION_ENTER),
        "radio-am" => Some(icons::RADIO_AM),
        "poll" => Some(icons::POLL),
        "source-commit-end-local" => Some(icons::SOURCE_COMMIT_END_LOCAL),
        "fire-alert" => Some(icons::FIRE_ALERT),
        "cog-sync" => Some(icons::COG_SYNC),
        "pan-vertical" => Some(icons::PAN_VERTICAL),
        "pencil-box-outline" => Some(icons::PENCIL_BOX_OUTLINE),
        "wifi-strength-alert-outline" => Some(icons::WIFI_STRENGTH_ALERT_OUTLINE),
        "hand-pointing-right" => Some(icons::HAND_POINTING_RIGHT),
        "feather" => Some(icons::FEATHER),
        "watch-export" => Some(icons::WATCH_EXPORT),
        "storefront-minus-outline" => Some(icons::STOREFRONT_MINUS_OUTLINE),
        "skip-next-outline" => Some(icons::SKIP_NEXT_OUTLINE),
        "home-roof" => Some(icons::HOME_ROOF),
        "border-bottom-variant" => Some(icons::BORDER_BOTTOM_VARIANT),
        "bottle-soda-classic" => Some(icons::BOTTLE_SODA_CLASSIC),
        "format-header-pound" => Some(icons::FORMAT_HEADER_POUND),
        "led-strip-variant" => Some(icons::LED_STRIP_VARIANT),
        "circular-saw" => Some(icons::CIRCULAR_SAW),
        "invert-colors" => Some(icons::INVERT_COLORS),
        "human-female-dance" => Some(icons::HUMAN_FEMALE_DANCE),
        _ => None,
    }
}
