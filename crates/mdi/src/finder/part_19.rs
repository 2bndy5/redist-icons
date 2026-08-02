// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_19(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "microphone-message-off" => Some(icons::MICROPHONE_MESSAGE_OFF),
        "currency-eur-off" => Some(icons::CURRENCY_EUR_OFF),
        "currency-rial" => Some(icons::CURRENCY_RIAL),
        "iron" => Some(icons::IRON),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "mapbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mapbox' is deprecated.").print(py);
            }
            Some(icons::MAPBOX)
        }
        "archive-cancel" => Some(icons::ARCHIVE_CANCEL),
        "phone-rotate-landscape" => Some(icons::PHONE_ROTATE_LANDSCAPE),
        "comment-off" => Some(icons::COMMENT_OFF),
        "roller-shade" => Some(icons::ROLLER_SHADE),
        "account-box-multiple" => Some(icons::ACCOUNT_BOX_MULTIPLE),
        "timeline-plus" => Some(icons::TIMELINE_PLUS),
        "file-video-outline" => Some(icons::FILE_VIDEO_OUTLINE),
        "file-restore" => Some(icons::FILE_RESTORE),
        "book-education" => Some(icons::BOOK_EDUCATION),
        "database-clock" => Some(icons::DATABASE_CLOCK),
        "bank" => Some(icons::BANK),
        "battery-outline" => Some(icons::BATTERY_OUTLINE),
        "scan-helper" => Some(icons::SCAN_HELPER),
        "size-xs" => Some(icons::SIZE_XS),
        "border-bottom" => Some(icons::BORDER_BOTTOM),
        "dump-truck" => Some(icons::DUMP_TRUCK),
        "receipt-text-plus-outline" => Some(icons::RECEIPT_TEXT_PLUS_OUTLINE),
        "gauge-full" => Some(icons::GAUGE_FULL),
        "blur-off" => Some(icons::BLUR_OFF),
        "store-remove-outline" => Some(icons::STORE_REMOVE_OUTLINE),
        "garage" => Some(icons::GARAGE),
        "star" => Some(icons::STAR),
        "relation-zero-or-many-to-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE),
        "office-building-marker" => Some(icons::OFFICE_BUILDING_MARKER),
        "timer-off" => Some(icons::TIMER_OFF),
        "eyedropper-remove" => Some(icons::EYEDROPPER_REMOVE),
        "folder-music" => Some(icons::FOLDER_MUSIC),
        "square-rounded-badge-outline" => Some(icons::SQUARE_ROUNDED_BADGE_OUTLINE),
        "tray-arrow-up" => Some(icons::TRAY_ARROW_UP),
        "shield-bug-outline" => Some(icons::SHIELD_BUG_OUTLINE),
        "battery-charging-wireless-90" => Some(icons::BATTERY_CHARGING_WIRELESS_90),
        "file-lock-outline" => Some(icons::FILE_LOCK_OUTLINE),
        "content-save-all" => Some(icons::CONTENT_SAVE_ALL),
        #[allow(deprecated)]
        "language-r" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-r' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_R)
        }
        "relation-one-or-many-to-zero-or-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ZERO_OR_ONE),
        "battery-charging-40" => Some(icons::BATTERY_CHARGING_40),
        "arrow-left-bottom-bold" => Some(icons::ARROW_LEFT_BOTTOM_BOLD),
        #[allow(deprecated)]
        "google-fit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-fit' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_FIT)
        }
        #[allow(deprecated)]
        "youtube-subscription" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-subscription' is deprecated.")
                    .print(py);
            }
            Some(icons::YOUTUBE_SUBSCRIPTION)
        }
        "book-edit" => Some(icons::BOOK_EDIT),
        "numeric-3-circle" => Some(icons::NUMERIC_3_CIRCLE),
        "invoice-arrow-left-outline" => Some(icons::INVOICE_ARROW_LEFT_OUTLINE),
        "oil" => Some(icons::OIL),
        "television-classic-off" => Some(icons::TELEVISION_CLASSIC_OFF),
        "thermometer-bluetooth" => Some(icons::THERMOMETER_BLUETOOTH),
        "multicast" => Some(icons::MULTICAST),
        "pac-man" => Some(icons::PAC_MAN),
        #[allow(deprecated)]
        "microsoft-xbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-xbox' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_XBOX)
        }
        "cloud-plus-outline" => Some(icons::CLOUD_PLUS_OUTLINE),
        "phone-paused" => Some(icons::PHONE_PAUSED),
        "scoreboard" => Some(icons::SCOREBOARD),
        "cards-playing-heart-multiple" => Some(icons::CARDS_PLAYING_HEART_MULTIPLE),
        "content-save-check" => Some(icons::CONTENT_SAVE_CHECK),
        "numeric-0-box" => Some(icons::NUMERIC_0_BOX),
        "zodiac-sagittarius" => Some(icons::ZODIAC_SAGITTARIUS),
        "wifi-settings" => Some(icons::WIFI_SETTINGS),
        "database-refresh-outline" => Some(icons::DATABASE_REFRESH_OUTLINE),
        "office-building-plus-outline" => Some(icons::OFFICE_BUILDING_PLUS_OUTLINE),
        "lightbulb" => Some(icons::LIGHTBULB),
        "awning-outline" => Some(icons::AWNING_OUTLINE),
        "numeric-9-plus-box-multiple-outline" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "black-mesa" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'black-mesa' is deprecated.").print(py);
            }
            Some(icons::BLACK_MESA)
        }
        "printer-pos" => Some(icons::PRINTER_POS),
        "card-text" => Some(icons::CARD_TEXT),
        #[allow(deprecated)]
        "patreon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'patreon' is deprecated.").print(py);
            }
            Some(icons::PATREON)
        }
        "trophy-broken" => Some(icons::TROPHY_BROKEN),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "dharmachakra" => Some(icons::DHARMACHAKRA),
        "roller-skate" => Some(icons::ROLLER_SKATE),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "subdirectory-arrow-left" => Some(icons::SUBDIRECTORY_ARROW_LEFT),
        "storefront" => Some(icons::STOREFRONT),
        "gate-xnor" => Some(icons::GATE_XNOR),
        "archive-edit-outline" => Some(icons::ARCHIVE_EDIT_OUTLINE),
        "account-remove" => Some(icons::ACCOUNT_REMOVE),
        "desktop-classic" => Some(icons::DESKTOP_CLASSIC),
        "relation-one-to-one" => Some(icons::RELATION_ONE_TO_ONE),
        "play-circle" => Some(icons::PLAY_CIRCLE),
        "selection-ellipse-arrow-inside" => Some(icons::SELECTION_ELLIPSE_ARROW_INSIDE),
        "toy-brick-plus-outline" => Some(icons::TOY_BRICK_PLUS_OUTLINE),
        "calendar-today" => Some(icons::CALENDAR_TODAY),
        "send-variant" => Some(icons::SEND_VARIANT),
        "medication" => Some(icons::MEDICATION),
        "tag-text" => Some(icons::TAG_TEXT),
        "ticket-account" => Some(icons::TICKET_ACCOUNT),
        "video-plus" => Some(icons::VIDEO_PLUS),
        "format-wrap-square" => Some(icons::FORMAT_WRAP_SQUARE),
        "align-horizontal-distribute" => Some(icons::ALIGN_HORIZONTAL_DISTRIBUTE),
        "bottle-tonic-plus-outline" => Some(icons::BOTTLE_TONIC_PLUS_OUTLINE),
        #[allow(deprecated)]
        "language-markdown-outline" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'language-markdown-outline' is deprecated.",
                )
                .print(py);
            }
            Some(icons::LANGUAGE_MARKDOWN_OUTLINE)
        }
        "widgets-outline" => Some(icons::WIDGETS_OUTLINE),
        "ballot-recount-outline" => Some(icons::BALLOT_RECOUNT_OUTLINE),
        "folder" => Some(icons::FOLDER),
        "format-pilcrow-arrow-right" => Some(icons::FORMAT_PILCROW_ARROW_RIGHT),
        "dock-top" => Some(icons::DOCK_TOP),
        "fast-forward-60" => Some(icons::FAST_FORWARD_60),
        "step-backward" => Some(icons::STEP_BACKWARD),
        "format-columns" => Some(icons::FORMAT_COLUMNS),
        "menu-swap-outline" => Some(icons::MENU_SWAP_OUTLINE),
        "note-off-outline" => Some(icons::NOTE_OFF_OUTLINE),
        "exponent" => Some(icons::EXPONENT),
        "label-off-outline" => Some(icons::LABEL_OFF_OUTLINE),
        "abacus" => Some(icons::ABACUS),
        "format-float-left" => Some(icons::FORMAT_FLOAT_LEFT),
        "source-branch-sync" => Some(icons::SOURCE_BRANCH_SYNC),
        "chart-multiple" => Some(icons::CHART_MULTIPLE),
        "contactless-payment-circle-outline" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE_OUTLINE),
        "escalator" => Some(icons::ESCALATOR),
        "hvac-off" => Some(icons::HVAC_OFF),
        "turnstile" => Some(icons::TURNSTILE),
        "content-save-settings" => Some(icons::CONTENT_SAVE_SETTINGS),
        "molecule" => Some(icons::MOLECULE),
        "kettle-steam" => Some(icons::KETTLE_STEAM),
        "robot-dead-outline" => Some(icons::ROBOT_DEAD_OUTLINE),
        "cursor-pointer" => Some(icons::CURSOR_POINTER),
        "human-queue" => Some(icons::HUMAN_QUEUE),
        "bell-remove-outline" => Some(icons::BELL_REMOVE_OUTLINE),
        "sim" => Some(icons::SIM),
        "picture-in-picture-top-right" => Some(icons::PICTURE_IN_PICTURE_TOP_RIGHT),
        "octagon-outline" => Some(icons::OCTAGON_OUTLINE),
        "bell-circle-outline" => Some(icons::BELL_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "tailwind" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'tailwind' is deprecated.").print(py);
            }
            Some(icons::TAILWIND)
        }
        "peanut" => Some(icons::PEANUT),
        "closed-caption" => Some(icons::CLOSED_CAPTION),
        "message-text-lock-outline" => Some(icons::MESSAGE_TEXT_LOCK_OUTLINE),
        "calendar-month-outline" => Some(icons::CALENDAR_MONTH_OUTLINE),
        "safe-square-outline" => Some(icons::SAFE_SQUARE_OUTLINE),
        "floppy" => Some(icons::FLOPPY),
        "table-edit" => Some(icons::TABLE_EDIT),
        "ray-end-arrow" => Some(icons::RAY_END_ARROW),
        "clipboard-outline" => Some(icons::CLIPBOARD_OUTLINE),
        "calendar-heart-outline" => Some(icons::CALENDAR_HEART_OUTLINE),
        "reply-all-outline" => Some(icons::REPLY_ALL_OUTLINE),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "sun-clock-outline" => Some(icons::SUN_CLOCK_OUTLINE),
        "book-heart-outline" => Some(icons::BOOK_HEART_OUTLINE),
        "alphabet-aurebesh" => Some(icons::ALPHABET_AUREBESH),
        "feature-search-outline" => Some(icons::FEATURE_SEARCH_OUTLINE),
        "flashlight" => Some(icons::FLASHLIGHT),
        "book-arrow-right-outline" => Some(icons::BOOK_ARROW_RIGHT_OUTLINE),
        "triangle-down" => Some(icons::TRIANGLE_DOWN),
        "tag-plus-outline" => Some(icons::TAG_PLUS_OUTLINE),
        "face-woman-outline" => Some(icons::FACE_WOMAN_OUTLINE),
        "archive-clock" => Some(icons::ARCHIVE_CLOCK),
        "arrow-down-left-bold" => Some(icons::ARROW_DOWN_LEFT_BOLD),
        "rowing" => Some(icons::ROWING),
        "invoice-multiple-outline" => Some(icons::INVOICE_MULTIPLE_OUTLINE),
        "card-bulleted" => Some(icons::CARD_BULLETED),
        "chair-school" => Some(icons::CHAIR_SCHOOL),
        "arrow-left-right-bold-outline" => Some(icons::ARROW_LEFT_RIGHT_BOLD_OUTLINE),
        "alpha-l-box-outline" => Some(icons::ALPHA_L_BOX_OUTLINE),
        "car-light-fog" => Some(icons::CAR_LIGHT_FOG),
        "stamper" => Some(icons::STAMPER),
        "server-plus" => Some(icons::SERVER_PLUS),
        "file-multiple-outline" => Some(icons::FILE_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "google-ads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-ads' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_ADS)
        }
        "format-horizontal-align-right" => Some(icons::FORMAT_HORIZONTAL_ALIGN_RIGHT),
        "brightness-3" => Some(icons::BRIGHTNESS_3),
        "timer-marker-outline" => Some(icons::TIMER_MARKER_OUTLINE),
        "arrow-left-bold-outline" => Some(icons::ARROW_LEFT_BOLD_OUTLINE),
        "slot-machine" => Some(icons::SLOT_MACHINE),
        "layers-triple" => Some(icons::LAYERS_TRIPLE),
        "lightbulb-on-60" => Some(icons::LIGHTBULB_ON_60),
        "table-row-height" => Some(icons::TABLE_ROW_HEIGHT),
        "chat-remove-outline" => Some(icons::CHAT_REMOVE_OUTLINE),
        "menu-close" => Some(icons::MENU_CLOSE),
        "pier" => Some(icons::PIER),
        "battery-charging-wireless-50" => Some(icons::BATTERY_CHARGING_WIRELESS_50),
        "land-plots" => Some(icons::LAND_PLOTS),
        "shield-car" => Some(icons::SHIELD_CAR),
        "account-clock" => Some(icons::ACCOUNT_CLOCK),
        #[allow(deprecated)]
        "qqchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qqchat' is deprecated.").print(py);
            }
            Some(icons::QQCHAT)
        }
        "chart-gantt" => Some(icons::CHART_GANTT),
        "contacts-outline" => Some(icons::CONTACTS_OUTLINE),
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "ev-plug-ccs2" => Some(icons::EV_PLUG_CCS2),
        "dock-window" => Some(icons::DOCK_WINDOW),
        "play-network-outline" => Some(icons::PLAY_NETWORK_OUTLINE),
        "send-variant-outline" => Some(icons::SEND_VARIANT_OUTLINE),
        "tag-arrow-up-outline" => Some(icons::TAG_ARROW_UP_OUTLINE),
        "bag-suitcase-off" => Some(icons::BAG_SUITCASE_OFF),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "speedometer-medium" => Some(icons::SPEEDOMETER_MEDIUM),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "pentagram" => Some(icons::PENTAGRAM),
        "alpha-x-circle" => Some(icons::ALPHA_X_CIRCLE),
        "translate" => Some(icons::TRANSLATE),
        "database-alert-outline" => Some(icons::DATABASE_ALERT_OUTLINE),
        "play-outline" => Some(icons::PLAY_OUTLINE),
        "set-square" => Some(icons::SET_SQUARE),
        "owl" => Some(icons::OWL),
        "image-sync" => Some(icons::IMAGE_SYNC),
        "folder-wrench-outline" => Some(icons::FOLDER_WRENCH_OUTLINE),
        _ => None,
    }
}
