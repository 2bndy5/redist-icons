// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_31(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "shape-circle-plus" => Some(icons::SHAPE_CIRCLE_PLUS),
        "source-merge" => Some(icons::SOURCE_MERGE),
        "pause-box" => Some(icons::PAUSE_BOX),
        "phone-forward" => Some(icons::PHONE_FORWARD),
        "audio-input-rca" => Some(icons::AUDIO_INPUT_RCA),
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "numeric-7-box-multiple" => Some(icons::NUMERIC_7_BOX_MULTIPLE),
        #[allow(deprecated)]
        "stack-exchange" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-exchange' is deprecated.").print(py);
            }
            Some(icons::STACK_EXCHANGE)
        }
        "wifi-strength-3-lock-open" => Some(icons::WIFI_STRENGTH_3_LOCK_OPEN),
        "clipboard-clock-outline" => Some(icons::CLIPBOARD_CLOCK_OUTLINE),
        "tray-arrow-down" => Some(icons::TRAY_ARROW_DOWN),
        "calendar-lock-open-outline" => Some(icons::CALENDAR_LOCK_OPEN_OUTLINE),
        "shield-outline" => Some(icons::SHIELD_OUTLINE),
        "thought-bubble" => Some(icons::THOUGHT_BUBBLE),
        #[allow(deprecated)]
        "language-ruby-on-rails" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby-on-rails' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_RUBY_ON_RAILS)
        }
        "book-music-outline" => Some(icons::BOOK_MUSIC_OUTLINE),
        "wallet-membership" => Some(icons::WALLET_MEMBERSHIP),
        "smart-card-off-outline" => Some(icons::SMART_CARD_OFF_OUTLINE),
        "store-edit" => Some(icons::STORE_EDIT),
        "database-eye" => Some(icons::DATABASE_EYE),
        "hexagon-slice-4" => Some(icons::HEXAGON_SLICE_4),
        "keyboard-f6" => Some(icons::KEYBOARD_F6),
        "delete-clock" => Some(icons::DELETE_CLOCK),
        "checkbox-blank-outline" => Some(icons::CHECKBOX_BLANK_OUTLINE),
        "face-woman-profile" => Some(icons::FACE_WOMAN_PROFILE),
        "arrow-down-left" => Some(icons::ARROW_DOWN_LEFT),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "account-convert" => Some(icons::ACCOUNT_CONVERT),
        "toslink" => Some(icons::TOSLINK),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "cash-multiple" => Some(icons::CASH_MULTIPLE),
        "sack-outline" => Some(icons::SACK_OUTLINE),
        "polaroid" => Some(icons::POLAROID),
        "bow-tie" => Some(icons::BOW_TIE),
        "note-alert" => Some(icons::NOTE_ALERT),
        "ip-network" => Some(icons::IP_NETWORK),
        "arm-flex" => Some(icons::ARM_FLEX),
        "tooltip-text" => Some(icons::TOOLTIP_TEXT),
        "curtains-closed" => Some(icons::CURTAINS_CLOSED),
        "card-search" => Some(icons::CARD_SEARCH),
        "calendar-minus-outline" => Some(icons::CALENDAR_MINUS_OUTLINE),
        "pause" => Some(icons::PAUSE),
        "arrow-down-bold-circle" => Some(icons::ARROW_DOWN_BOLD_CIRCLE),
        "receipt-text-remove-outline" => Some(icons::RECEIPT_TEXT_REMOVE_OUTLINE),
        "account-arrow-down" => Some(icons::ACCOUNT_ARROW_DOWN),
        "percent-circle-outline" => Some(icons::PERCENT_CIRCLE_OUTLINE),
        "factory" => Some(icons::FACTORY),
        "airplane-off" => Some(icons::AIRPLANE_OFF),
        "bridge" => Some(icons::BRIDGE),
        "window-shutter-alert" => Some(icons::WINDOW_SHUTTER_ALERT),
        "store-24-hour" => Some(icons::STORE_24_HOUR),
        "cellphone-text" => Some(icons::CELLPHONE_TEXT),
        "coffee" => Some(icons::COFFEE),
        "xml" => Some(icons::XML),
        "dots-triangle" => Some(icons::DOTS_TRIANGLE),
        "format-paragraph" => Some(icons::FORMAT_PARAGRAPH),
        #[allow(deprecated)]
        "microsoft-dynamics-365" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-dynamics-365' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_DYNAMICS_365)
        }
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "file-remove" => Some(icons::FILE_REMOVE),
        "numeric-9" => Some(icons::NUMERIC_9),
        "briefcase-account-outline" => Some(icons::BRIEFCASE_ACCOUNT_OUTLINE),
        "video-minus" => Some(icons::VIDEO_MINUS),
        "wallet-outline" => Some(icons::WALLET_OUTLINE),
        "file-presentation-box" => Some(icons::FILE_PRESENTATION_BOX),
        "kite" => Some(icons::KITE),
        "airplane-landing" => Some(icons::AIRPLANE_LANDING),
        "application-array" => Some(icons::APPLICATION_ARRAY),
        "information-slab-symbol" => Some(icons::INFORMATION_SLAB_SYMBOL),
        "ip-outline" => Some(icons::IP_OUTLINE),
        "hamburger-minus" => Some(icons::HAMBURGER_MINUS),
        "furigana-vertical" => Some(icons::FURIGANA_VERTICAL),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        "landslide" => Some(icons::LANDSLIDE),
        "gamepad-square-outline" => Some(icons::GAMEPAD_SQUARE_OUTLINE),
        "message-star-outline" => Some(icons::MESSAGE_STAR_OUTLINE),
        "account-child-circle" => Some(icons::ACCOUNT_CHILD_CIRCLE),
        "contactless-payment-circle" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE),
        "queue-first-in-last-out" => Some(icons::QUEUE_FIRST_IN_LAST_OUT),
        "forum-minus" => Some(icons::FORUM_MINUS),
        "monitor-arrow-down-variant" => Some(icons::MONITOR_ARROW_DOWN_VARIANT),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "current-dc" => Some(icons::CURRENT_DC),
        "database-arrow-up" => Some(icons::DATABASE_ARROW_UP),
        "movie-settings-outline" => Some(icons::MOVIE_SETTINGS_OUTLINE),
        "water" => Some(icons::WATER),
        "flower-tulip" => Some(icons::FLOWER_TULIP),
        "power-cycle" => Some(icons::POWER_CYCLE),
        "delete-empty" => Some(icons::DELETE_EMPTY),
        "format-letter-case" => Some(icons::FORMAT_LETTER_CASE),
        "book-lock-open" => Some(icons::BOOK_LOCK_OPEN),
        "calendar-plus" => Some(icons::CALENDAR_PLUS),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "calendar-arrow-left" => Some(icons::CALENDAR_ARROW_LEFT),
        "hand-peace" => Some(icons::HAND_PEACE),
        "book-open" => Some(icons::BOOK_OPEN),
        "file-swap-outline" => Some(icons::FILE_SWAP_OUTLINE),
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "underwear-outline" => Some(icons::UNDERWEAR_OUTLINE),
        "toggle-switch" => Some(icons::TOGGLE_SWITCH),
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "weather-snowy-rainy" => Some(icons::WEATHER_SNOWY_RAINY),
        "water-minus-outline" => Some(icons::WATER_MINUS_OUTLINE),
        "thumb-up-outline" => Some(icons::THUMB_UP_OUTLINE),
        "image-lock-outline" => Some(icons::IMAGE_LOCK_OUTLINE),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "car-seat" => Some(icons::CAR_SEAT),
        "hand-back-right-off-outline" => Some(icons::HAND_BACK_RIGHT_OFF_OUTLINE),
        "glasses" => Some(icons::GLASSES),
        "message-text-fast" => Some(icons::MESSAGE_TEXT_FAST),
        "tractor" => Some(icons::TRACTOR),
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        "home-export-outline" => Some(icons::HOME_EXPORT_OUTLINE),
        "sword-cross" => Some(icons::SWORD_CROSS),
        "eraser-variant" => Some(icons::ERASER_VARIANT),
        "application-cog-outline" => Some(icons::APPLICATION_COG_OUTLINE),
        "alert-box-outline" => Some(icons::ALERT_BOX_OUTLINE),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "home-floor-g" => Some(icons::HOME_FLOOR_G),
        #[allow(deprecated)]
        "firebase" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firebase' is deprecated.").print(py);
            }
            Some(icons::FIREBASE)
        }
        "download-circle" => Some(icons::DOWNLOAD_CIRCLE),
        "arrow-down-bold-outline" => Some(icons::ARROW_DOWN_BOLD_OUTLINE),
        "head-question" => Some(icons::HEAD_QUESTION),
        "kangaroo" => Some(icons::KANGAROO),
        "download-network-outline" => Some(icons::DOWNLOAD_NETWORK_OUTLINE),
        "arrow-expand-down" => Some(icons::ARROW_EXPAND_DOWN),
        "toggle-switch-variant" => Some(icons::TOGGLE_SWITCH_VARIANT),
        "credit-card-wireless-off-outline" => Some(icons::CREDIT_CARD_WIRELESS_OFF_OUTLINE),
        #[allow(deprecated)]
        "font-awesome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'font-awesome' is deprecated.").print(py);
            }
            Some(icons::FONT_AWESOME)
        }
        "fish" => Some(icons::FISH),
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "monitor-dashboard" => Some(icons::MONITOR_DASHBOARD),
        "sawtooth-wave" => Some(icons::SAWTOOTH_WAVE),
        #[allow(deprecated)]
        "docker" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'docker' is deprecated.").print(py);
            }
            Some(icons::DOCKER)
        }
        "numeric-0" => Some(icons::NUMERIC_0),
        "clock-check" => Some(icons::CLOCK_CHECK),
        "folder-multiple-plus" => Some(icons::FOLDER_MULTIPLE_PLUS),
        "pen" => Some(icons::PEN),
        "dice-2-outline" => Some(icons::DICE_2_OUTLINE),
        "account-key" => Some(icons::ACCOUNT_KEY),
        "spider-web" => Some(icons::SPIDER_WEB),
        "account-tie-voice-off" => Some(icons::ACCOUNT_TIE_VOICE_OFF),
        "baby-face-outline" => Some(icons::BABY_FACE_OUTLINE),
        "minus-box" => Some(icons::MINUS_BOX),
        "checkbox-marked" => Some(icons::CHECKBOX_MARKED),
        "meditation" => Some(icons::MEDITATION),
        "sack" => Some(icons::SACK),
        "message-lock" => Some(icons::MESSAGE_LOCK),
        "car-brake-fluid-level" => Some(icons::CAR_BRAKE_FLUID_LEVEL),
        "cross-outline" => Some(icons::CROSS_OUTLINE),
        #[allow(deprecated)]
        "electron-framework" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'electron-framework' is deprecated.")
                    .print(py);
            }
            Some(icons::ELECTRON_FRAMEWORK)
        }
        "water-outline" => Some(icons::WATER_OUTLINE),
        "axis-x-arrow" => Some(icons::AXIS_X_ARROW),
        "table-remove" => Some(icons::TABLE_REMOVE),
        "currency-usd" => Some(icons::CURRENCY_USD),
        "diving-flippers" => Some(icons::DIVING_FLIPPERS),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "truck-off-road-off" => Some(icons::TRUCK_OFF_ROAD_OFF),
        "tunnel-outline" => Some(icons::TUNNEL_OUTLINE),
        "camera-off" => Some(icons::CAMERA_OFF),
        "file-tree" => Some(icons::FILE_TREE),
        "signal-2g" => Some(icons::SIGNAL_2G),
        "atom-variant" => Some(icons::ATOM_VARIANT),
        "light-recessed" => Some(icons::LIGHT_RECESSED),
        "file-cloud-outline" => Some(icons::FILE_CLOUD_OUTLINE),
        "gamepad-round-left" => Some(icons::GAMEPAD_ROUND_LEFT),
        "checkbox-blank-circle" => Some(icons::CHECKBOX_BLANK_CIRCLE),
        "emoticon-poop-outline" => Some(icons::EMOTICON_POOP_OUTLINE),
        "message-image-outline" => Some(icons::MESSAGE_IMAGE_OUTLINE),
        "toolbox" => Some(icons::TOOLBOX),
        "list-box-outline" => Some(icons::LIST_BOX_OUTLINE),
        "cloud-clock" => Some(icons::CLOUD_CLOCK),
        "database-import" => Some(icons::DATABASE_IMPORT),
        "chat-plus" => Some(icons::CHAT_PLUS),
        "microphone-variant" => Some(icons::MICROPHONE_VARIANT),
        "lock-open-variant" => Some(icons::LOCK_OPEN_VARIANT),
        "sim-alert-outline" => Some(icons::SIM_ALERT_OUTLINE),
        "source-repository-multiple" => Some(icons::SOURCE_REPOSITORY_MULTIPLE),
        "book-multiple-outline" => Some(icons::BOOK_MULTIPLE_OUTLINE),
        "bus-clock" => Some(icons::BUS_CLOCK),
        "glass-fragile" => Some(icons::GLASS_FRAGILE),
        "tally-mark-1" => Some(icons::TALLY_MARK_1),
        "chart-scatter-plot" => Some(icons::CHART_SCATTER_PLOT),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "numeric-6-box-multiple" => Some(icons::NUMERIC_6_BOX_MULTIPLE),
        "horse-variant-fast" => Some(icons::HORSE_VARIANT_FAST),
        "battery-remove-outline" => Some(icons::BATTERY_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "language-xaml" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-xaml' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_XAML)
        }
        "shape-rectangle-plus" => Some(icons::SHAPE_RECTANGLE_PLUS),
        "card-account-details-outline" => Some(icons::CARD_ACCOUNT_DETAILS_OUTLINE),
        "passport" => Some(icons::PASSPORT),
        "wrap" => Some(icons::WRAP),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "eiffel-tower" => Some(icons::EIFFEL_TOWER),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "puzzle-plus-outline" => Some(icons::PUZZLE_PLUS_OUTLINE),
        "network-strength-2" => Some(icons::NETWORK_STRENGTH_2),
        "translate" => Some(icons::TRANSLATE),
        "podium-bronze" => Some(icons::PODIUM_BRONZE),
        "hexagon-slice-3" => Some(icons::HEXAGON_SLICE_3),
        "lock-remove-outline" => Some(icons::LOCK_REMOVE_OUTLINE),
        _ => None,
    }
}
