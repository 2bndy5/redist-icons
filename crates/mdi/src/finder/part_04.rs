// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_4(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "currency-fra" => Some(icons::CURRENCY_FRA),
        "face-woman-outline" => Some(icons::FACE_WOMAN_OUTLINE),
        "caravan" => Some(icons::CARAVAN),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "view-dashboard-variant-outline" => Some(icons::VIEW_DASHBOARD_VARIANT_OUTLINE),
        "database-sync-outline" => Some(icons::DATABASE_SYNC_OUTLINE),
        "offer" => Some(icons::OFFER),
        #[allow(deprecated)]
        "mapbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mapbox' is deprecated.").print(py);
            }
            Some(icons::MAPBOX)
        }
        "printer-pos-plus-outline" => Some(icons::PRINTER_POS_PLUS_OUTLINE),
        "numeric-10-box-outline" => Some(icons::NUMERIC_10_BOX_OUTLINE),
        "cloud-plus" => Some(icons::CLOUD_PLUS),
        "video-2d" => Some(icons::VIDEO_2D),
        "calendar-clock" => Some(icons::CALENDAR_CLOCK),
        "phone-incoming" => Some(icons::PHONE_INCOMING),
        "wifi-strength-lock-outline" => Some(icons::WIFI_STRENGTH_LOCK_OUTLINE),
        "battery-arrow-down" => Some(icons::BATTERY_ARROW_DOWN),
        "glass-stange" => Some(icons::GLASS_STANGE),
        "view-grid-plus-outline" => Some(icons::VIEW_GRID_PLUS_OUTLINE),
        "image-edit" => Some(icons::IMAGE_EDIT),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "firework" => Some(icons::FIREWORK),
        "account-arrow-left" => Some(icons::ACCOUNT_ARROW_LEFT),
        "shield-plus-outline" => Some(icons::SHIELD_PLUS_OUTLINE),
        "cookie-remove-outline" => Some(icons::COOKIE_REMOVE_OUTLINE),
        "heart-half-full" => Some(icons::HEART_HALF_FULL),
        "cellphone-nfc" => Some(icons::CELLPHONE_NFC),
        "phone-sync" => Some(icons::PHONE_SYNC),
        "store-24-hour" => Some(icons::STORE_24_HOUR),
        "comment-eye-outline" => Some(icons::COMMENT_EYE_OUTLINE),
        "lasso" => Some(icons::LASSO),
        "comment-text-multiple-outline" => Some(icons::COMMENT_TEXT_MULTIPLE_OUTLINE),
        "details" => Some(icons::DETAILS),
        "brain" => Some(icons::BRAIN),
        "touch-text-outline" => Some(icons::TOUCH_TEXT_OUTLINE),
        "shield-outline" => Some(icons::SHIELD_OUTLINE),
        #[allow(deprecated)]
        "google-downasaur" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-downasaur' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_DOWNASAUR)
        }
        "shark" => Some(icons::SHARK),
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "order-numeric-descending" => Some(icons::ORDER_NUMERIC_DESCENDING),
        "virtual-reality" => Some(icons::VIRTUAL_REALITY),
        "cloud-arrow-right-outline" => Some(icons::CLOUD_ARROW_RIGHT_OUTLINE),
        "iron-outline" => Some(icons::IRON_OUTLINE),
        "pan-horizontal" => Some(icons::PAN_HORIZONTAL),
        "guy-fawkes-mask" => Some(icons::GUY_FAWKES_MASK),
        "coolant-temperature" => Some(icons::COOLANT_TEMPERATURE),
        "alpha-i-box" => Some(icons::ALPHA_I_BOX),
        "land-plots-circle" => Some(icons::LAND_PLOTS_CIRCLE),
        "weather-sunny-alert" => Some(icons::WEATHER_SUNNY_ALERT),
        "autorenew-off" => Some(icons::AUTORENEW_OFF),
        "shield-remove" => Some(icons::SHIELD_REMOVE),
        "form-dropdown" => Some(icons::FORM_DROPDOWN),
        "power-plug-off-outline" => Some(icons::POWER_PLUG_OFF_OUTLINE),
        "hand-back-right-off-outline" => Some(icons::HAND_BACK_RIGHT_OFF_OUTLINE),
        "car-off" => Some(icons::CAR_OFF),
        #[allow(deprecated)]
        "lumx" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'lumx' is deprecated.").print(py);
            }
            Some(icons::LUMX)
        }
        "globe-light" => Some(icons::GLOBE_LIGHT),
        "format-list-bulleted-type" => Some(icons::FORMAT_LIST_BULLETED_TYPE),
        #[allow(deprecated)]
        "zend" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zend' is deprecated.").print(py);
            }
            Some(icons::ZEND)
        }
        "flag-variant-plus" => Some(icons::FLAG_VARIANT_PLUS),
        "rss-box" => Some(icons::RSS_BOX),
        "rename-box" => Some(icons::RENAME_BOX),
        "fan-plus" => Some(icons::FAN_PLUS),
        "domain-remove" => Some(icons::DOMAIN_REMOVE),
        "wan" => Some(icons::WAN),
        "at" => Some(icons::AT),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "toaster-oven" => Some(icons::TOASTER_OVEN),
        "transit-skip" => Some(icons::TRANSIT_SKIP),
        "stamper" => Some(icons::STAMPER),
        "bookshelf" => Some(icons::BOOKSHELF),
        "bag-suitcase-off-outline" => Some(icons::BAG_SUITCASE_OFF_OUTLINE),
        "sticker-circle-outline" => Some(icons::STICKER_CIRCLE_OUTLINE),
        "arrow-down-right-bold" => Some(icons::ARROW_DOWN_RIGHT_BOLD),
        "satellite-uplink" => Some(icons::SATELLITE_UPLINK),
        "minus-circle-multiple" => Some(icons::MINUS_CIRCLE_MULTIPLE),
        "router-wireless-settings" => Some(icons::ROUTER_WIRELESS_SETTINGS),
        "archive-remove" => Some(icons::ARCHIVE_REMOVE),
        "content-save-outline" => Some(icons::CONTENT_SAVE_OUTLINE),
        "circle-small" => Some(icons::CIRCLE_SMALL),
        "currency-jpy" => Some(icons::CURRENCY_JPY),
        "rickshaw-electric" => Some(icons::RICKSHAW_ELECTRIC),
        "note-alert" => Some(icons::NOTE_ALERT),
        "file-check" => Some(icons::FILE_CHECK),
        "arrow-projectile" => Some(icons::ARROW_PROJECTILE),
        "arrow-left-top-bold" => Some(icons::ARROW_LEFT_TOP_BOLD),
        #[allow(deprecated)]
        "google-cloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cloud' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CLOUD)
        }
        "file-document-minus-outline" => Some(icons::FILE_DOCUMENT_MINUS_OUTLINE),
        "alpha-v-circle-outline" => Some(icons::ALPHA_V_CIRCLE_OUTLINE),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "soccer" => Some(icons::SOCCER),
        "map-marker-right-outline" => Some(icons::MAP_MARKER_RIGHT_OUTLINE),
        "microphone-minus" => Some(icons::MICROPHONE_MINUS),
        "invoice-text" => Some(icons::INVOICE_TEXT),
        "tennis" => Some(icons::TENNIS),
        "account-multiple-check-outline" => Some(icons::ACCOUNT_MULTIPLE_CHECK_OUTLINE),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "alarm-panel-outline" => Some(icons::ALARM_PANEL_OUTLINE),
        "deskphone" => Some(icons::DESKPHONE),
        "lock-open-alert-outline" => Some(icons::LOCK_OPEN_ALERT_OUTLINE),
        "image-lock" => Some(icons::IMAGE_LOCK),
        "door-closed" => Some(icons::DOOR_CLOSED),
        "draw" => Some(icons::DRAW),
        #[allow(deprecated)]
        "microsoft-bing" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-bing' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_BING)
        }
        "wifi-refresh" => Some(icons::WIFI_REFRESH),
        "table-remove" => Some(icons::TABLE_REMOVE),
        "weather-snowy" => Some(icons::WEATHER_SNOWY),
        "web-refresh" => Some(icons::WEB_REFRESH),
        "laser-pointer" => Some(icons::LASER_POINTER),
        "spider" => Some(icons::SPIDER),
        "file-multiple" => Some(icons::FILE_MULTIPLE),
        "tape-measure" => Some(icons::TAPE_MEASURE),
        "apps" => Some(icons::APPS),
        "bathtub-outline" => Some(icons::BATHTUB_OUTLINE),
        "phone-incoming-outgoing" => Some(icons::PHONE_INCOMING_OUTGOING),
        #[allow(deprecated)]
        "apple-icloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-icloud' is deprecated.").print(py);
            }
            Some(icons::APPLE_ICLOUD)
        }
        "human-edit" => Some(icons::HUMAN_EDIT),
        "cash-minus" => Some(icons::CASH_MINUS),
        "map-marker-radius-outline" => Some(icons::MAP_MARKER_RADIUS_OUTLINE),
        "tanker-truck" => Some(icons::TANKER_TRUCK),
        "solar-power-variant" => Some(icons::SOLAR_POWER_VARIANT),
        "cigar-off" => Some(icons::CIGAR_OFF),
        "baby-buggy" => Some(icons::BABY_BUGGY),
        "star-shooting-outline" => Some(icons::STAR_SHOOTING_OUTLINE),
        "invoice-remove" => Some(icons::INVOICE_REMOVE),
        "package-variant-closed" => Some(icons::PACKAGE_VARIANT_CLOSED),
        #[allow(deprecated)]
        "electron-framework" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'electron-framework' is deprecated.")
                    .print(py);
            }
            Some(icons::ELECTRON_FRAMEWORK)
        }
        "movie-filter" => Some(icons::MOVIE_FILTER),
        "view-comfy" => Some(icons::VIEW_COMFY),
        "numeric-2-box-multiple-outline" => Some(icons::NUMERIC_2_BOX_MULTIPLE_OUTLINE),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        "toolbox" => Some(icons::TOOLBOX),
        "motion-play-outline" => Some(icons::MOTION_PLAY_OUTLINE),
        "account-tie-hat" => Some(icons::ACCOUNT_TIE_HAT),
        "thumb-down-outline" => Some(icons::THUMB_DOWN_OUTLINE),
        "clipboard-minus" => Some(icons::CLIPBOARD_MINUS),
        "airballoon-outline" => Some(icons::AIRBALLOON_OUTLINE),
        "roller-shade-closed" => Some(icons::ROLLER_SHADE_CLOSED),
        "account-group-outline" => Some(icons::ACCOUNT_GROUP_OUTLINE),
        "alphabet-latin" => Some(icons::ALPHABET_LATIN),
        "domain" => Some(icons::DOMAIN),
        "comment-check" => Some(icons::COMMENT_CHECK),
        "sun-thermometer-outline" => Some(icons::SUN_THERMOMETER_OUTLINE),
        "vector-circle-variant" => Some(icons::VECTOR_CIRCLE_VARIANT),
        "eject" => Some(icons::EJECT),
        "baby-face-outline" => Some(icons::BABY_FACE_OUTLINE),
        "printer-3d-nozzle-heat-outline" => Some(icons::PRINTER_3D_NOZZLE_HEAT_OUTLINE),
        "cloud-refresh-outline" => Some(icons::CLOUD_REFRESH_OUTLINE),
        "alpha-e" => Some(icons::ALPHA_E),
        "account-edit" => Some(icons::ACCOUNT_EDIT),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "music-accidental-sharp" => Some(icons::MUSIC_ACCIDENTAL_SHARP),
        "hand-wash" => Some(icons::HAND_WASH),
        "math-log" => Some(icons::MATH_LOG),
        "folder-zip-outline" => Some(icons::FOLDER_ZIP_OUTLINE),
        "pig" => Some(icons::PIG),
        "selection-ellipse" => Some(icons::SELECTION_ELLIPSE),
        "selection" => Some(icons::SELECTION),
        "numeric-3-box-multiple" => Some(icons::NUMERIC_3_BOX_MULTIPLE),
        "ray-vertex" => Some(icons::RAY_VERTEX),
        "lightbulb-question" => Some(icons::LIGHTBULB_QUESTION),
        "content-save-plus" => Some(icons::CONTENT_SAVE_PLUS),
        "hub-outline" => Some(icons::HUB_OUTLINE),
        "chevron-right" => Some(icons::CHEVRON_RIGHT),
        "eye-off" => Some(icons::EYE_OFF),
        "bullhorn" => Some(icons::BULLHORN),
        "file-refresh-outline" => Some(icons::FILE_REFRESH_OUTLINE),
        "grease-pencil" => Some(icons::GREASE_PENCIL),
        "human-female" => Some(icons::HUMAN_FEMALE),
        "help-circle" => Some(icons::HELP_CIRCLE),
        "wifi-alert" => Some(icons::WIFI_ALERT),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "arrow-down-thick" => Some(icons::ARROW_DOWN_THICK),
        "store-alert" => Some(icons::STORE_ALERT),
        "eye-lock" => Some(icons::EYE_LOCK),
        "bookmark-remove-outline" => Some(icons::BOOKMARK_REMOVE_OUTLINE),
        "map-marker-remove-outline" => Some(icons::MAP_MARKER_REMOVE_OUTLINE),
        "wifi-strength-2-alert" => Some(icons::WIFI_STRENGTH_2_ALERT),
        "texture" => Some(icons::TEXTURE),
        "blender-outline" => Some(icons::BLENDER_OUTLINE),
        "scale-unbalanced" => Some(icons::SCALE_UNBALANCED),
        "pin-outline" => Some(icons::PIN_OUTLINE),
        "truck-check-outline" => Some(icons::TRUCK_CHECK_OUTLINE),
        "tag-arrow-right" => Some(icons::TAG_ARROW_RIGHT),
        "turnstile" => Some(icons::TURNSTILE),
        "battery-arrow-up" => Some(icons::BATTERY_ARROW_UP),
        "account-cancel" => Some(icons::ACCOUNT_CANCEL),
        "view-list" => Some(icons::VIEW_LIST),
        "barcode-off" => Some(icons::BARCODE_OFF),
        "format-page-split" => Some(icons::FORMAT_PAGE_SPLIT),
        "open-in-new" => Some(icons::OPEN_IN_NEW),
        "filter-variant-remove" => Some(icons::FILTER_VARIANT_REMOVE),
        "head-remove-outline" => Some(icons::HEAD_REMOVE_OUTLINE),
        "diamond-stone" => Some(icons::DIAMOND_STONE),
        "file-refresh" => Some(icons::FILE_REFRESH),
        "printer-off-outline" => Some(icons::PRINTER_OFF_OUTLINE),
        "currency-uah" => Some(icons::CURRENCY_UAH),
        "podium" => Some(icons::PODIUM),
        "tablet-cellphone" => Some(icons::TABLET_CELLPHONE),
        "reorder-horizontal" => Some(icons::REORDER_HORIZONTAL),
        "clipboard-account" => Some(icons::CLIPBOARD_ACCOUNT),
        _ => None,
    }
}
