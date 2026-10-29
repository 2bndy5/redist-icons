// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_6(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "network-off-outline" => Some(icons::NETWORK_OFF_OUTLINE),
        "car-traction-control" => Some(icons::CAR_TRACTION_CONTROL),
        "account-arrow-right" => Some(icons::ACCOUNT_ARROW_RIGHT),
        "battery-charging-medium" => Some(icons::BATTERY_CHARGING_MEDIUM),
        "playlist-star" => Some(icons::PLAYLIST_STAR),
        "firewire" => Some(icons::FIREWIRE),
        "format-letter-ends-with" => Some(icons::FORMAT_LETTER_ENDS_WITH),
        "phone-log" => Some(icons::PHONE_LOG),
        #[allow(deprecated)]
        "microsoft-xbox-controller-menu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-menu' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_MENU)
        }
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "battery-charging-wireless-40" => Some(icons::BATTERY_CHARGING_WIRELESS_40),
        "fast-forward-10" => Some(icons::FAST_FORWARD_10),
        "expand-all" => Some(icons::EXPAND_ALL),
        "camera-enhance" => Some(icons::CAMERA_ENHANCE),
        "brightness-3" => Some(icons::BRIGHTNESS_3),
        "folder-text" => Some(icons::FOLDER_TEXT),
        "timer-cog-outline" => Some(icons::TIMER_COG_OUTLINE),
        "calendar-weekend-outline" => Some(icons::CALENDAR_WEEKEND_OUTLINE),
        #[allow(deprecated)]
        "gatsby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gatsby' is deprecated.").print(py);
            }
            Some(icons::GATSBY)
        }
        "link-lock" => Some(icons::LINK_LOCK),
        "flower-tulip-outline" => Some(icons::FLOWER_TULIP_OUTLINE),
        "bookmark-multiple-outline" => Some(icons::BOOKMARK_MULTIPLE_OUTLINE),
        "exponent" => Some(icons::EXPONENT),
        #[allow(deprecated)]
        "salesforce" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'salesforce' is deprecated.").print(py);
            }
            Some(icons::SALESFORCE)
        }
        "controller" => Some(icons::CONTROLLER),
        "scale-balance" => Some(icons::SCALE_BALANCE),
        "open-in-new" => Some(icons::OPEN_IN_NEW),
        "email-check-outline" => Some(icons::EMAIL_CHECK_OUTLINE),
        "lightbulb-alert-outline" => Some(icons::LIGHTBULB_ALERT_OUTLINE),
        "octagon" => Some(icons::OCTAGON),
        "application-parentheses" => Some(icons::APPLICATION_PARENTHESES),
        "thumbs-up-down-outline" => Some(icons::THUMBS_UP_DOWN_OUTLINE),
        "music-note-sixteenth" => Some(icons::MUSIC_NOTE_SIXTEENTH),
        "windsock" => Some(icons::WINDSOCK),
        "cloud-check-variant-outline" => Some(icons::CLOUD_CHECK_VARIANT_OUTLINE),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        "eyedropper-off" => Some(icons::EYEDROPPER_OFF),
        "table-column-plus-before" => Some(icons::TABLE_COLUMN_PLUS_BEFORE),
        "face-woman" => Some(icons::FACE_WOMAN),
        #[allow(deprecated)]
        "symfony" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'symfony' is deprecated.").print(py);
            }
            Some(icons::SYMFONY)
        }
        "folder-multiple-plus-outline" => Some(icons::FOLDER_MULTIPLE_PLUS_OUTLINE),
        "inbox-outline" => Some(icons::INBOX_OUTLINE),
        "flash-auto" => Some(icons::FLASH_AUTO),
        "script-text-key-outline" => Some(icons::SCRIPT_TEXT_KEY_OUTLINE),
        "clock-alert-outline" => Some(icons::CLOCK_ALERT_OUTLINE),
        "account-edit-outline" => Some(icons::ACCOUNT_EDIT_OUTLINE),
        "keyboard-backspace" => Some(icons::KEYBOARD_BACKSPACE),
        "image-edit" => Some(icons::IMAGE_EDIT),
        "book-refresh" => Some(icons::BOOK_REFRESH),
        "briefcase-off-outline" => Some(icons::BRIEFCASE_OFF_OUTLINE),
        "ev-plug-type2" => Some(icons::EV_PLUG_TYPE2),
        "clipboard-flow-outline" => Some(icons::CLIPBOARD_FLOW_OUTLINE),
        "chili-off" => Some(icons::CHILI_OFF),
        #[allow(deprecated)]
        "vuetify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuetify' is deprecated.").print(py);
            }
            Some(icons::VUETIFY)
        }
        "face-woman-shimmer" => Some(icons::FACE_WOMAN_SHIMMER),
        "bio" => Some(icons::BIO),
        "food-takeout-box-outline" => Some(icons::FOOD_TAKEOUT_BOX_OUTLINE),
        "cookie-outline" => Some(icons::COOKIE_OUTLINE),
        "pencil-ruler-outline" => Some(icons::PENCIL_RULER_OUTLINE),
        "cloud-refresh-outline" => Some(icons::CLOUD_REFRESH_OUTLINE),
        "view-parallel" => Some(icons::VIEW_PARALLEL),
        "angle-right" => Some(icons::ANGLE_RIGHT),
        "car-estate" => Some(icons::CAR_ESTATE),
        "looks" => Some(icons::LOOKS),
        "folder-arrow-right-outline" => Some(icons::FOLDER_ARROW_RIGHT_OUTLINE),
        "application-cog-outline" => Some(icons::APPLICATION_COG_OUTLINE),
        "battery-charging-wireless" => Some(icons::BATTERY_CHARGING_WIRELESS),
        "square-outline" => Some(icons::SQUARE_OUTLINE),
        #[allow(deprecated)]
        "dolby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dolby' is deprecated.").print(py);
            }
            Some(icons::DOLBY)
        }
        "information-symbol" => Some(icons::INFORMATION_SYMBOL),
        "puzzle-edit-outline" => Some(icons::PUZZLE_EDIT_OUTLINE),
        "smoke-detector-off-outline" => Some(icons::SMOKE_DETECTOR_OFF_OUTLINE),
        "transit-transfer" => Some(icons::TRANSIT_TRANSFER),
        "lightbulb-spot-off" => Some(icons::LIGHTBULB_SPOT_OFF),
        "label" => Some(icons::LABEL),
        "robot-vacuum" => Some(icons::ROBOT_VACUUM),
        "shield-account-variant-outline" => Some(icons::SHIELD_ACCOUNT_VARIANT_OUTLINE),
        "invoice-plus" => Some(icons::INVOICE_PLUS),
        "eiffel-tower" => Some(icons::EIFFEL_TOWER),
        "checkbox-marked-circle-minus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_MINUS_OUTLINE),
        "brightness-5" => Some(icons::BRIGHTNESS_5),
        "comma-box-outline" => Some(icons::COMMA_BOX_OUTLINE),
        "calendar-sync-outline" => Some(icons::CALENDAR_SYNC_OUTLINE),
        "diversify" => Some(icons::DIVERSIFY),
        "seat-legroom-normal" => Some(icons::SEAT_LEGROOM_NORMAL),
        "folder-information" => Some(icons::FOLDER_INFORMATION),
        "clock-time-twelve-outline" => Some(icons::CLOCK_TIME_TWELVE_OUTLINE),
        "home-heart" => Some(icons::HOME_HEART),
        "snowflake-melt" => Some(icons::SNOWFLAKE_MELT),
        "move-resize-variant" => Some(icons::MOVE_RESIZE_VARIANT),
        "clipboard-check" => Some(icons::CLIPBOARD_CHECK),
        "toslink" => Some(icons::TOSLINK),
        "inbox-arrow-down" => Some(icons::INBOX_ARROW_DOWN),
        "virtual-reality" => Some(icons::VIRTUAL_REALITY),
        "alpha-d-circle-outline" => Some(icons::ALPHA_D_CIRCLE_OUTLINE),
        "brightness-4" => Some(icons::BRIGHTNESS_4),
        "file-document-check" => Some(icons::FILE_DOCUMENT_CHECK),
        "content-save-plus-outline" => Some(icons::CONTENT_SAVE_PLUS_OUTLINE),
        "egg-off" => Some(icons::EGG_OFF),
        "alpha-s-circle" => Some(icons::ALPHA_S_CIRCLE),
        "brush" => Some(icons::BRUSH),
        "shopping-outline" => Some(icons::SHOPPING_OUTLINE),
        "vector-arrange-below" => Some(icons::VECTOR_ARRANGE_BELOW),
        "calendar-refresh" => Some(icons::CALENDAR_REFRESH),
        "code-equal" => Some(icons::CODE_EQUAL),
        "select-arrow-down" => Some(icons::SELECT_ARROW_DOWN),
        #[allow(deprecated)]
        "svg" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'svg' is deprecated.").print(py);
            }
            Some(icons::SVG)
        }
        "heart-minus-outline" => Some(icons::HEART_MINUS_OUTLINE),
        "reflect-horizontal" => Some(icons::REFLECT_HORIZONTAL),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "wall-sconce-outline" => Some(icons::WALL_SCONCE_OUTLINE),
        "file-powerpoint-outline" => Some(icons::FILE_POWERPOINT_OUTLINE),
        "upload" => Some(icons::UPLOAD),
        "flower-tulip" => Some(icons::FLOWER_TULIP),
        "content-save-check-outline" => Some(icons::CONTENT_SAVE_CHECK_OUTLINE),
        "triangle-outline" => Some(icons::TRIANGLE_OUTLINE),
        "home-clock" => Some(icons::HOME_CLOCK),
        #[allow(deprecated)]
        "kodi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kodi' is deprecated.").print(py);
            }
            Some(icons::KODI)
        }
        "credit-card-clock-outline" => Some(icons::CREDIT_CARD_CLOCK_OUTLINE),
        "book-account" => Some(icons::BOOK_ACCOUNT),
        "soy-sauce" => Some(icons::SOY_SAUCE),
        "smoke-detector" => Some(icons::SMOKE_DETECTOR),
        "format-overline" => Some(icons::FORMAT_OVERLINE),
        "cookie-check" => Some(icons::COOKIE_CHECK),
        "book-cog" => Some(icons::BOOK_COG),
        "checkbox-blank-off" => Some(icons::CHECKBOX_BLANK_OFF),
        "printer-pos-remove-outline" => Some(icons::PRINTER_POS_REMOVE_OUTLINE),
        "router-wireless-off" => Some(icons::ROUTER_WIRELESS_OFF),
        "arrow-right-bold-outline" => Some(icons::ARROW_RIGHT_BOLD_OUTLINE),
        "email-open-heart-outline" => Some(icons::EMAIL_OPEN_HEART_OUTLINE),
        "closed-caption-outline" => Some(icons::CLOSED_CAPTION_OUTLINE),
        "solar-panel-large" => Some(icons::SOLAR_PANEL_LARGE),
        "wifi-star" => Some(icons::WIFI_STAR),
        "page-previous" => Some(icons::PAGE_PREVIOUS),
        "airplane-takeoff" => Some(icons::AIRPLANE_TAKEOFF),
        "bottle-tonic-skull-outline" => Some(icons::BOTTLE_TONIC_SKULL_OUTLINE),
        "briefcase-account" => Some(icons::BRIEFCASE_ACCOUNT),
        "toy-brick-marker" => Some(icons::TOY_BRICK_MARKER),
        "chevron-up-circle-outline" => Some(icons::CHEVRON_UP_CIRCLE_OUTLINE),
        "social-distance-2-meters" => Some(icons::SOCIAL_DISTANCE_2_METERS),
        "book-play-outline" => Some(icons::BOOK_PLAY_OUTLINE),
        "surround-sound-5-1-2" => Some(icons::SURROUND_SOUND_5_1_2),
        "snowflake" => Some(icons::SNOWFLAKE),
        "sort-bool-descending-variant" => Some(icons::SORT_BOOL_DESCENDING_VARIANT),
        "ev-plug-chademo" => Some(icons::EV_PLUG_CHADEMO),
        "chart-scatter-plot-hexbin" => Some(icons::CHART_SCATTER_PLOT_HEXBIN),
        "monitor-cellphone-star" => Some(icons::MONITOR_CELLPHONE_STAR),
        "dishwasher-alert" => Some(icons::DISHWASHER_ALERT),
        "video-switch-outline" => Some(icons::VIDEO_SWITCH_OUTLINE),
        "format-paragraph-spacing" => Some(icons::FORMAT_PARAGRAPH_SPACING),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "fruit-grapes-outline" => Some(icons::FRUIT_GRAPES_OUTLINE),
        "turtle" => Some(icons::TURTLE),
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "resize-bottom-right" => Some(icons::RESIZE_BOTTOM_RIGHT),
        "dishwasher-off" => Some(icons::DISHWASHER_OFF),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "border-top" => Some(icons::BORDER_TOP),
        "account-supervisor-circle-outline" => Some(icons::ACCOUNT_SUPERVISOR_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "linkedin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linkedin' is deprecated.").print(py);
            }
            Some(icons::LINKEDIN)
        }
        "table-large-plus" => Some(icons::TABLE_LARGE_PLUS),
        "account" => Some(icons::ACCOUNT),
        "water-minus-outline" => Some(icons::WATER_MINUS_OUTLINE),
        "signal-variant" => Some(icons::SIGNAL_VARIANT),
        "film" => Some(icons::FILM),
        "phone-incoming" => Some(icons::PHONE_INCOMING),
        "chevron-right-box" => Some(icons::CHEVRON_RIGHT_BOX),
        "mushroom-off" => Some(icons::MUSHROOM_OFF),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "hamburger-check" => Some(icons::HAMBURGER_CHECK),
        "numeric-8-box-outline" => Some(icons::NUMERIC_8_BOX_OUTLINE),
        "sail-boat-sink" => Some(icons::SAIL_BOAT_SINK),
        "flask-empty" => Some(icons::FLASK_EMPTY),
        "cellphone-dock" => Some(icons::CELLPHONE_DOCK),
        "calendar-outline" => Some(icons::CALENDAR_OUTLINE),
        "truck-off-road-off" => Some(icons::TRUCK_OFF_ROAD_OFF),
        "lock-percent-open-variant" => Some(icons::LOCK_PERCENT_OPEN_VARIANT),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "credit-card-settings-outline" => Some(icons::CREDIT_CARD_SETTINGS_OUTLINE),
        "printer-search" => Some(icons::PRINTER_SEARCH),
        "account-alert-outline" => Some(icons::ACCOUNT_ALERT_OUTLINE),
        "whistle" => Some(icons::WHISTLE),
        "tray-minus" => Some(icons::TRAY_MINUS),
        "format-text" => Some(icons::FORMAT_TEXT),
        "currency-sign" => Some(icons::CURRENCY_SIGN),
        "flask-plus-outline" => Some(icons::FLASK_PLUS_OUTLINE),
        "thermometer" => Some(icons::THERMOMETER),
        "air-horn" => Some(icons::AIR_HORN),
        "account-supervisor" => Some(icons::ACCOUNT_SUPERVISOR),
        "currency-rupee" => Some(icons::CURRENCY_RUPEE),
        "text-box-search" => Some(icons::TEXT_BOX_SEARCH),
        "archive-star" => Some(icons::ARCHIVE_STAR),
        "file-document-refresh-outline" => Some(icons::FILE_DOCUMENT_REFRESH_OUTLINE),
        "cake" => Some(icons::CAKE),
        "numeric-off" => Some(icons::NUMERIC_OFF),
        "angle-obtuse" => Some(icons::ANGLE_OBTUSE),
        "clock-star-four-points-outline" => Some(icons::CLOCK_STAR_FOUR_POINTS_OUTLINE),
        "projector-screen-variant-off-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF_OUTLINE),
        "arrow-left-drop-circle" => Some(icons::ARROW_LEFT_DROP_CIRCLE),
        "tag-arrow-up-outline" => Some(icons::TAG_ARROW_UP_OUTLINE),
        _ => None,
    }
}
