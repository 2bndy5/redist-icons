// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_1(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "underwear-outline" => Some(icons::UNDERWEAR_OUTLINE),
        "battery-40" => Some(icons::BATTERY_40),
        "vector-difference-ab" => Some(icons::VECTOR_DIFFERENCE_AB),
        "alpha-z-box-outline" => Some(icons::ALPHA_Z_BOX_OUTLINE),
        "package-check" => Some(icons::PACKAGE_CHECK),
        "escalator-up" => Some(icons::ESCALATOR_UP),
        "phone-log-outline" => Some(icons::PHONE_LOG_OUTLINE),
        "sleep" => Some(icons::SLEEP),
        "image-off" => Some(icons::IMAGE_OFF),
        "grave-stone" => Some(icons::GRAVE_STONE),
        "keg" => Some(icons::KEG),
        "microphone-message-off" => Some(icons::MICROPHONE_MESSAGE_OFF),
        "phone-lock-outline" => Some(icons::PHONE_LOCK_OUTLINE),
        "microphone-outline" => Some(icons::MICROPHONE_OUTLINE),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "hand-back-left-off" => Some(icons::HAND_BACK_LEFT_OFF),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "window-maximize" => Some(icons::WINDOW_MAXIMIZE),
        #[allow(deprecated)]
        "microsoft-sharepoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-sharepoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_SHAREPOINT)
        }
        "rake" => Some(icons::RAKE),
        "music-circle" => Some(icons::MUSIC_CIRCLE),
        "carabiner" => Some(icons::CARABINER),
        "bag-personal-tag" => Some(icons::BAG_PERSONAL_TAG),
        "calendar-week-begin-outline" => Some(icons::CALENDAR_WEEK_BEGIN_OUTLINE),
        "grid" => Some(icons::GRID),
        "printer-pos-cancel-outline" => Some(icons::PRINTER_POS_CANCEL_OUTLINE),
        "barley" => Some(icons::BARLEY),
        "send-check" => Some(icons::SEND_CHECK),
        "checkbox-multiple-marked-outline" => Some(icons::CHECKBOX_MULTIPLE_MARKED_OUTLINE),
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "format-letter-spacing" => Some(icons::FORMAT_LETTER_SPACING),
        "music-rest-half" => Some(icons::MUSIC_REST_HALF),
        "broom" => Some(icons::BROOM),
        "chili-off-outline" => Some(icons::CHILI_OFF_OUTLINE),
        "arrow-left-bold-box" => Some(icons::ARROW_LEFT_BOLD_BOX),
        "content-save-settings-outline" => Some(icons::CONTENT_SAVE_SETTINGS_OUTLINE),
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "printer-pos-stop" => Some(icons::PRINTER_POS_STOP),
        "home-minus-outline" => Some(icons::HOME_MINUS_OUTLINE),
        "scooter-electric" => Some(icons::SCOOTER_ELECTRIC),
        "camera-outline" => Some(icons::CAMERA_OUTLINE),
        "dots-triangle" => Some(icons::DOTS_TRIANGLE),
        "invert-colors-off" => Some(icons::INVERT_COLORS_OFF),
        "thumbs-up-down" => Some(icons::THUMBS_UP_DOWN),
        "bash" => Some(icons::BASH),
        "generator-mobile" => Some(icons::GENERATOR_MOBILE),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "smart-card" => Some(icons::SMART_CARD),
        "bag-personal-plus-outline" => Some(icons::BAG_PERSONAL_PLUS_OUTLINE),
        "blinds-vertical-closed" => Some(icons::BLINDS_VERTICAL_CLOSED),
        "clock-out" => Some(icons::CLOCK_OUT),
        "drone" => Some(icons::DRONE),
        "movie-minus" => Some(icons::MOVIE_MINUS),
        "tooltip-cellphone" => Some(icons::TOOLTIP_CELLPHONE),
        "flask-empty-minus-outline" => Some(icons::FLASK_EMPTY_MINUS_OUTLINE),
        "car-defrost-front" => Some(icons::CAR_DEFROST_FRONT),
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "shield-crown-outline" => Some(icons::SHIELD_CROWN_OUTLINE),
        "numeric-2-box-multiple-outline" => Some(icons::NUMERIC_2_BOX_MULTIPLE_OUTLINE),
        "credit-card-sync" => Some(icons::CREDIT_CARD_SYNC),
        "waves-arrow-up" => Some(icons::WAVES_ARROW_UP),
        "cash-register" => Some(icons::CASH_REGISTER),
        "content-save-settings" => Some(icons::CONTENT_SAVE_SETTINGS),
        "movie-star" => Some(icons::MOVIE_STAR),
        "view-parallel-outline" => Some(icons::VIEW_PARALLEL_OUTLINE),
        "temperature-kelvin" => Some(icons::TEMPERATURE_KELVIN),
        "weather-dust" => Some(icons::WEATHER_DUST),
        "arrow-down-bold-circle-outline" => Some(icons::ARROW_DOWN_BOLD_CIRCLE_OUTLINE),
        "folder-swap" => Some(icons::FOLDER_SWAP),
        "poll" => Some(icons::POLL),
        "download-lock" => Some(icons::DOWNLOAD_LOCK),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "collapse-all-outline" => Some(icons::COLLAPSE_ALL_OUTLINE),
        "train-car-hopper" => Some(icons::TRAIN_CAR_HOPPER),
        "meter-gas" => Some(icons::METER_GAS),
        "water-check" => Some(icons::WATER_CHECK),
        "shuffle-variant" => Some(icons::SHUFFLE_VARIANT),
        "grease-pencil" => Some(icons::GREASE_PENCIL),
        "wifi-strength-2" => Some(icons::WIFI_STRENGTH_2),
        "briefcase-search-outline" => Some(icons::BRIEFCASE_SEARCH_OUTLINE),
        "marker-cancel" => Some(icons::MARKER_CANCEL),
        "alphabet-piqad" => Some(icons::ALPHABET_PIQAD),
        "invoice-multiple" => Some(icons::INVOICE_MULTIPLE),
        "home-map-marker" => Some(icons::HOME_MAP_MARKER),
        "beaker-alert" => Some(icons::BEAKER_ALERT),
        "package-up" => Some(icons::PACKAGE_UP),
        #[allow(deprecated)]
        "bitcoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitcoin' is deprecated.").print(py);
            }
            Some(icons::BITCOIN)
        }
        "connection" => Some(icons::CONNECTION),
        "post-outline" => Some(icons::POST_OUTLINE),
        "sigma-lower" => Some(icons::SIGMA_LOWER),
        "numeric-8-box-multiple" => Some(icons::NUMERIC_8_BOX_MULTIPLE),
        "shield-edit" => Some(icons::SHIELD_EDIT),
        "billiards-rack" => Some(icons::BILLIARDS_RACK),
        "hand-peace" => Some(icons::HAND_PEACE),
        "flip-vertical" => Some(icons::FLIP_VERTICAL),
        "table-cog" => Some(icons::TABLE_COG),
        "fire-off" => Some(icons::FIRE_OFF),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        "water-pump" => Some(icons::WATER_PUMP),
        "sticker-remove-outline" => Some(icons::STICKER_REMOVE_OUTLINE),
        "table-account" => Some(icons::TABLE_ACCOUNT),
        "bicycle-penny-farthing" => Some(icons::BICYCLE_PENNY_FARTHING),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "hydraulic-oil-temperature" => Some(icons::HYDRAULIC_OIL_TEMPERATURE),
        "case-sensitive-alt" => Some(icons::CASE_SENSITIVE_ALT),
        "fax" => Some(icons::FAX),
        "chevron-left-circle-outline" => Some(icons::CHEVRON_LEFT_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "sort-clock-ascending" => Some(icons::SORT_CLOCK_ASCENDING),
        "tangram" => Some(icons::TANGRAM),
        "set-left" => Some(icons::SET_LEFT),
        "file-restore-outline" => Some(icons::FILE_RESTORE_OUTLINE),
        "wan" => Some(icons::WAN),
        "clipboard-multiple" => Some(icons::CLIPBOARD_MULTIPLE),
        "numeric-2-circle" => Some(icons::NUMERIC_2_CIRCLE),
        "tooltip-image-outline" => Some(icons::TOOLTIP_IMAGE_OUTLINE),
        "home-group-remove" => Some(icons::HOME_GROUP_REMOVE),
        "water-percent" => Some(icons::WATER_PERCENT),
        "alert-minus" => Some(icons::ALERT_MINUS),
        "find-replace" => Some(icons::FIND_REPLACE),
        "camera-lock-open" => Some(icons::CAMERA_LOCK_OPEN),
        "fit-to-page-outline" => Some(icons::FIT_TO_PAGE_OUTLINE),
        "television-classic-off" => Some(icons::TELEVISION_CLASSIC_OFF),
        "content-save-minus" => Some(icons::CONTENT_SAVE_MINUS),
        "bat" => Some(icons::BAT),
        "chart-ppf" => Some(icons::CHART_PPF),
        "tsunami" => Some(icons::TSUNAMI),
        "led-variant-outline" => Some(icons::LED_VARIANT_OUTLINE),
        "airplane-off" => Some(icons::AIRPLANE_OFF),
        "heart-half-outline" => Some(icons::HEART_HALF_OUTLINE),
        "cog-off" => Some(icons::COG_OFF),
        "arrow-up-drop-circle-outline" => Some(icons::ARROW_UP_DROP_CIRCLE_OUTLINE),
        "shield-edit-outline" => Some(icons::SHIELD_EDIT_OUTLINE),
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "size-xxs" => Some(icons::SIZE_XXS),
        "square" => Some(icons::SQUARE),
        "close-network-outline" => Some(icons::CLOSE_NETWORK_OUTLINE),
        "tab" => Some(icons::TAB),
        "fan-chevron-up" => Some(icons::FAN_CHEVRON_UP),
        "network-strength-1" => Some(icons::NETWORK_STRENGTH_1),
        "face-man-shimmer" => Some(icons::FACE_MAN_SHIMMER),
        "car-seat" => Some(icons::CAR_SEAT),
        "dns" => Some(icons::DNS),
        "account-check" => Some(icons::ACCOUNT_CHECK),
        "text-short" => Some(icons::TEXT_SHORT),
        "island-variant" => Some(icons::ISLAND_VARIANT),
        "fridge-variant-off" => Some(icons::FRIDGE_VARIANT_OFF),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "arrow-u-right-bottom" => Some(icons::ARROW_U_RIGHT_BOTTOM),
        "ray-start-end" => Some(icons::RAY_START_END),
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        #[allow(deprecated)]
        "language-rust" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-rust' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUST)
        }
        "pound-box-outline" => Some(icons::POUND_BOX_OUTLINE),
        "call-missed" => Some(icons::CALL_MISSED),
        "weather-sunset-down" => Some(icons::WEATHER_SUNSET_DOWN),
        "email-check" => Some(icons::EMAIL_CHECK),
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "horizontal-rotate-counterclockwise" => Some(icons::HORIZONTAL_ROTATE_COUNTERCLOCKWISE),
        "television-classic" => Some(icons::TELEVISION_CLASSIC),
        "television-off" => Some(icons::TELEVISION_OFF),
        "application-export" => Some(icons::APPLICATION_EXPORT),
        "currency-bdt" => Some(icons::CURRENCY_BDT),
        "table-check" => Some(icons::TABLE_CHECK),
        "weather-hurricane" => Some(icons::WEATHER_HURRICANE),
        "traffic-cone" => Some(icons::TRAFFIC_CONE),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "pencil" => Some(icons::PENCIL),
        "bug-stop-outline" => Some(icons::BUG_STOP_OUTLINE),
        "fridge-alert" => Some(icons::FRIDGE_ALERT),
        "pot-steam-outline" => Some(icons::POT_STEAM_OUTLINE),
        #[allow(deprecated)]
        "bitbucket" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitbucket' is deprecated.").print(py);
            }
            Some(icons::BITBUCKET)
        }
        "vector-polyline-edit" => Some(icons::VECTOR_POLYLINE_EDIT),
        "message-cog-outline" => Some(icons::MESSAGE_COG_OUTLINE),
        "clipboard-text-search-outline" => Some(icons::CLIPBOARD_TEXT_SEARCH_OUTLINE),
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        #[allow(deprecated)]
        "simple-icons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'simple-icons' is deprecated.").print(py);
            }
            Some(icons::SIMPLE_ICONS)
        }
        "hamburger-minus" => Some(icons::HAMBURGER_MINUS),
        "television-guide" => Some(icons::TELEVISION_GUIDE),
        "cradle" => Some(icons::CRADLE),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "email-off-outline" => Some(icons::EMAIL_OFF_OUTLINE),
        "home-thermometer-outline" => Some(icons::HOME_THERMOMETER_OUTLINE),
        "panorama-wide-angle-outline" => Some(icons::PANORAMA_WIDE_ANGLE_OUTLINE),
        "currency-btc" => Some(icons::CURRENCY_BTC),
        "cloud-circle" => Some(icons::CLOUD_CIRCLE),
        "basketball" => Some(icons::BASKETBALL),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
        "send-check-outline" => Some(icons::SEND_CHECK_OUTLINE),
        "filter-plus-outline" => Some(icons::FILTER_PLUS_OUTLINE),
        "microphone" => Some(icons::MICROPHONE),
        "rocket" => Some(icons::ROCKET),
        #[allow(deprecated)]
        "arch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'arch' is deprecated.").print(py);
            }
            Some(icons::ARCH)
        }
        "step-backward" => Some(icons::STEP_BACKWARD),
        "calendar-plus-outline" => Some(icons::CALENDAR_PLUS_OUTLINE),
        "clipboard-plus" => Some(icons::CLIPBOARD_PLUS),
        "folder-arrow-down-outline" => Some(icons::FOLDER_ARROW_DOWN_OUTLINE),
        #[allow(deprecated)]
        "snapchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'snapchat' is deprecated.").print(py);
            }
            Some(icons::SNAPCHAT)
        }
        _ => None,
    }
}
