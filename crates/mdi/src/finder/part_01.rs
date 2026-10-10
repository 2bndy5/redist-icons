// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_1(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "battery-10" => Some(icons::BATTERY_10),
        "cloud-off-outline" => Some(icons::CLOUD_OFF_OUTLINE),
        "wifi-cancel" => Some(icons::WIFI_CANCEL),
        "web-remove" => Some(icons::WEB_REMOVE),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "folder-multiple-plus" => Some(icons::FOLDER_MULTIPLE_PLUS),
        "card-remove-outline" => Some(icons::CARD_REMOVE_OUTLINE),
        "card" => Some(icons::CARD),
        "target-account" => Some(icons::TARGET_ACCOUNT),
        "book-arrow-left-outline" => Some(icons::BOOK_ARROW_LEFT_OUTLINE),
        "inbox-full" => Some(icons::INBOX_FULL),
        "apple-keyboard-shift" => Some(icons::APPLE_KEYBOARD_SHIFT),
        "movie-cog" => Some(icons::MOVIE_COG),
        "close-circle-multiple" => Some(icons::CLOSE_CIRCLE_MULTIPLE),
        "gamepad-round-down" => Some(icons::GAMEPAD_ROUND_DOWN),
        "link-variant" => Some(icons::LINK_VARIANT),
        "gas-cylinder" => Some(icons::GAS_CYLINDER),
        "key-minus" => Some(icons::KEY_MINUS),
        "car-wash" => Some(icons::CAR_WASH),
        "vector-bezier" => Some(icons::VECTOR_BEZIER),
        "reply-all" => Some(icons::REPLY_ALL),
        "upload-lock" => Some(icons::UPLOAD_LOCK),
        "tag-check-outline" => Some(icons::TAG_CHECK_OUTLINE),
        "circle-slice-6" => Some(icons::CIRCLE_SLICE_6),
        "ellipse" => Some(icons::ELLIPSE),
        "hexagon" => Some(icons::HEXAGON),
        "arrow-left-thin-circle-outline" => Some(icons::ARROW_LEFT_THIN_CIRCLE_OUTLINE),
        "cup-off-outline" => Some(icons::CUP_OFF_OUTLINE),
        "send-lock" => Some(icons::SEND_LOCK),
        "vector-curve" => Some(icons::VECTOR_CURVE),
        "cloud-question" => Some(icons::CLOUD_QUESTION),
        "pan-left" => Some(icons::PAN_LEFT),
        "home-group-minus" => Some(icons::HOME_GROUP_MINUS),
        "vector-square-minus" => Some(icons::VECTOR_SQUARE_MINUS),
        "music-rest-whole" => Some(icons::MUSIC_REST_WHOLE),
        "head-check-outline" => Some(icons::HEAD_CHECK_OUTLINE),
        "seat-flat-angled" => Some(icons::SEAT_FLAT_ANGLED),
        "border-top-variant" => Some(icons::BORDER_TOP_VARIANT),
        "fridge-outline" => Some(icons::FRIDGE_OUTLINE),
        "earth-arrow-left" => Some(icons::EARTH_ARROW_LEFT),
        "transmission-tower-off" => Some(icons::TRANSMISSION_TOWER_OFF),
        "network-strength-off" => Some(icons::NETWORK_STRENGTH_OFF),
        "cloud-upload-outline" => Some(icons::CLOUD_UPLOAD_OUTLINE),
        "minus-circle" => Some(icons::MINUS_CIRCLE),
        "menu-swap-outline" => Some(icons::MENU_SWAP_OUTLINE),
        "cart-arrow-right" => Some(icons::CART_ARROW_RIGHT),
        "arrow-u-down-right" => Some(icons::ARROW_U_DOWN_RIGHT),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "emoticon-excited-outline" => Some(icons::EMOTICON_EXCITED_OUTLINE),
        "tools" => Some(icons::TOOLS),
        "image-filter-center-focus-weak" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_WEAK),
        "format-paint" => Some(icons::FORMAT_PAINT),
        "tag-minus-outline" => Some(icons::TAG_MINUS_OUTLINE),
        "numeric" => Some(icons::NUMERIC),
        "av-timer" => Some(icons::AV_TIMER),
        "trophy-variant" => Some(icons::TROPHY_VARIANT),
        "file-settings-outline" => Some(icons::FILE_SETTINGS_OUTLINE),
        "briefcase-arrow-up-down" => Some(icons::BRIEFCASE_ARROW_UP_DOWN),
        "alpha-w-circle-outline" => Some(icons::ALPHA_W_CIRCLE_OUTLINE),
        "alpha-x-box" => Some(icons::ALPHA_X_BOX),
        "credit-card-plus-outline" => Some(icons::CREDIT_CARD_PLUS_OUTLINE),
        "grid-large" => Some(icons::GRID_LARGE),
        "comment-quote" => Some(icons::COMMENT_QUOTE),
        "headphones-bluetooth" => Some(icons::HEADPHONES_BLUETOOTH),
        "leaf-maple-off" => Some(icons::LEAF_MAPLE_OFF),
        "glass-mug" => Some(icons::GLASS_MUG),
        "receipt-text-edit-outline" => Some(icons::RECEIPT_TEXT_EDIT_OUTLINE),
        "format-header-3" => Some(icons::FORMAT_HEADER_3),
        "invoice-check" => Some(icons::INVOICE_CHECK),
        "heating-coil" => Some(icons::HEATING_COIL),
        #[allow(deprecated)]
        "box" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'box' is deprecated.").print(py);
            }
            Some(icons::BOX)
        }
        "math-integral" => Some(icons::MATH_INTEGRAL),
        "gamepad" => Some(icons::GAMEPAD),
        "transfer-left" => Some(icons::TRANSFER_LEFT),
        "traffic-light-outline" => Some(icons::TRAFFIC_LIGHT_OUTLINE),
        "view-week" => Some(icons::VIEW_WEEK),
        "stocking" => Some(icons::STOCKING),
        "settings-helper" => Some(icons::SETTINGS_HELPER),
        "currency-eth" => Some(icons::CURRENCY_ETH),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        "home-floor-l" => Some(icons::HOME_FLOOR_L),
        "cards-playing-diamond" => Some(icons::CARDS_PLAYING_DIAMOND),
        "golf-cart" => Some(icons::GOLF_CART),
        "flask-plus" => Some(icons::FLASK_PLUS),
        #[allow(deprecated)]
        "creative-commons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'creative-commons' is deprecated.")
                    .print(py);
            }
            Some(icons::CREATIVE_COMMONS)
        }
        "fast-forward-60" => Some(icons::FAST_FORWARD_60),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        "volume-source" => Some(icons::VOLUME_SOURCE),
        "star-cog" => Some(icons::STAR_COG),
        "image-edit" => Some(icons::IMAGE_EDIT),
        "numeric-9-circle-outline" => Some(icons::NUMERIC_9_CIRCLE_OUTLINE),
        "calendar-outline" => Some(icons::CALENDAR_OUTLINE),
        "hexagram-outline" => Some(icons::HEXAGRAM_OUTLINE),
        "phone-paused-outline" => Some(icons::PHONE_PAUSED_OUTLINE),
        "thermostat-cog" => Some(icons::THERMOSTAT_COG),
        "code-tags" => Some(icons::CODE_TAGS),
        "link-box-variant-outline" => Some(icons::LINK_BOX_VARIANT_OUTLINE),
        "chart-donut" => Some(icons::CHART_DONUT),
        #[allow(deprecated)]
        "google-translate" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-translate' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_TRANSLATE)
        }
        "hamburger-minus" => Some(icons::HAMBURGER_MINUS),
        "invoice-minus" => Some(icons::INVOICE_MINUS),
        "clipboard-arrow-left-outline" => Some(icons::CLIPBOARD_ARROW_LEFT_OUTLINE),
        "backspace" => Some(icons::BACKSPACE),
        "sort-calendar-ascending" => Some(icons::SORT_CALENDAR_ASCENDING),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "close-octagon" => Some(icons::CLOSE_OCTAGON),
        "feature-search" => Some(icons::FEATURE_SEARCH),
        "satellite" => Some(icons::SATELLITE),
        #[allow(deprecated)]
        "dev-to" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dev-to' is deprecated.").print(py);
            }
            Some(icons::DEV_TO)
        }
        "water-well-outline" => Some(icons::WATER_WELL_OUTLINE),
        "keyboard-f5" => Some(icons::KEYBOARD_F5),
        "truck-alert-outline" => Some(icons::TRUCK_ALERT_OUTLINE),
        "remote-desktop" => Some(icons::REMOTE_DESKTOP),
        "music-clef-treble" => Some(icons::MUSIC_CLEF_TREBLE),
        "glass-fragile" => Some(icons::GLASS_FRAGILE),
        "social-distance-2-meters" => Some(icons::SOCIAL_DISTANCE_2_METERS),
        "volume-plus" => Some(icons::VOLUME_PLUS),
        "star-circle" => Some(icons::STAR_CIRCLE),
        "alpha-i-box" => Some(icons::ALPHA_I_BOX),
        "safety-goggles" => Some(icons::SAFETY_GOGGLES),
        "audio-video" => Some(icons::AUDIO_VIDEO),
        "phone-outline" => Some(icons::PHONE_OUTLINE),
        "weight-pound" => Some(icons::WEIGHT_POUND),
        "format-text-rotation-up" => Some(icons::FORMAT_TEXT_ROTATION_UP),
        "water-off-outline" => Some(icons::WATER_OFF_OUTLINE),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "cloud-check" => Some(icons::CLOUD_CHECK),
        #[allow(deprecated)]
        "language-cpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-cpp' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CPP)
        }
        "account-multiple-check-outline" => Some(icons::ACCOUNT_MULTIPLE_CHECK_OUTLINE),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "boom-gate-alert-outline" => Some(icons::BOOM_GATE_ALERT_OUTLINE),
        "spade" => Some(icons::SPADE),
        "image-album" => Some(icons::IMAGE_ALBUM),
        "account-school-outline" => Some(icons::ACCOUNT_SCHOOL_OUTLINE),
        "swap-horizontal-hidden" => Some(icons::SWAP_HORIZONTAL_HIDDEN),
        "rotate-3d" => Some(icons::ROTATE_3D),
        "numeric-0-circle" => Some(icons::NUMERIC_0_CIRCLE),
        "passport-biometric" => Some(icons::PASSPORT_BIOMETRIC),
        "cash-sync" => Some(icons::CASH_SYNC),
        "cloud-question-outline" => Some(icons::CLOUD_QUESTION_OUTLINE),
        "credit-card-refund" => Some(icons::CREDIT_CARD_REFUND),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "view-comfy" => Some(icons::VIEW_COMFY),
        "music-note-whole-dotted" => Some(icons::MUSIC_NOTE_WHOLE_DOTTED),
        "file-document" => Some(icons::FILE_DOCUMENT),
        "credit-card-search" => Some(icons::CREDIT_CARD_SEARCH),
        "server-network-off" => Some(icons::SERVER_NETWORK_OFF),
        "sleep" => Some(icons::SLEEP),
        "timeline-question-outline" => Some(icons::TIMELINE_QUESTION_OUTLINE),
        "mouse-right-click-outline" => Some(icons::MOUSE_RIGHT_CLICK_OUTLINE),
        "elevator-passenger-off-outline" => Some(icons::ELEVATOR_PASSENGER_OFF_OUTLINE),
        "arrow-left-right-bold-outline" => Some(icons::ARROW_LEFT_RIGHT_BOLD_OUTLINE),
        "chart-pie" => Some(icons::CHART_PIE),
        "panorama-wide-angle-outline" => Some(icons::PANORAMA_WIDE_ANGLE_OUTLINE),
        "account-plus-outline" => Some(icons::ACCOUNT_PLUS_OUTLINE),
        "train-car-box-full" => Some(icons::TRAIN_CAR_BOX_FULL),
        "cards-variant" => Some(icons::CARDS_VARIANT),
        "cloud-lock-open-outline" => Some(icons::CLOUD_LOCK_OPEN_OUTLINE),
        "video-plus" => Some(icons::VIDEO_PLUS),
        "human-female-girl" => Some(icons::HUMAN_FEMALE_GIRL),
        "bag-suitcase-off" => Some(icons::BAG_SUITCASE_OFF),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "town-hall" => Some(icons::TOWN_HALL),
        "hand-pointing-up" => Some(icons::HAND_POINTING_UP),
        "audio-input-xlr" => Some(icons::AUDIO_INPUT_XLR),
        "sofa-single-outline" => Some(icons::SOFA_SINGLE_OUTLINE),
        "card-bulleted-off" => Some(icons::CARD_BULLETED_OFF),
        #[allow(deprecated)]
        "google-spreadsheet" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-spreadsheet' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_SPREADSHEET)
        }
        "hospital" => Some(icons::HOSPITAL),
        "presentation" => Some(icons::PRESENTATION),
        "moped-outline" => Some(icons::MOPED_OUTLINE),
        "account-multiple-plus" => Some(icons::ACCOUNT_MULTIPLE_PLUS),
        "air-humidifier-off" => Some(icons::AIR_HUMIDIFIER_OFF),
        "phone-missed-outline" => Some(icons::PHONE_MISSED_OUTLINE),
        "pokeball" => Some(icons::POKEBALL),
        "align-horizontal-distribute" => Some(icons::ALIGN_HORIZONTAL_DISTRIBUTE),
        "plus-lock-open" => Some(icons::PLUS_LOCK_OPEN),
        "format-line-weight" => Some(icons::FORMAT_LINE_WEIGHT),
        "currency-twd" => Some(icons::CURRENCY_TWD),
        "home-minus" => Some(icons::HOME_MINUS),
        "cog-play" => Some(icons::COG_PLAY),
        "clock-remove-outline" => Some(icons::CLOCK_REMOVE_OUTLINE),
        "cloud-download-outline" => Some(icons::CLOUD_DOWNLOAD_OUTLINE),
        "ticket-outline" => Some(icons::TICKET_OUTLINE),
        "temple-buddhist" => Some(icons::TEMPLE_BUDDHIST),
        "credit-card-refund-outline" => Some(icons::CREDIT_CARD_REFUND_OUTLINE),
        "newspaper-variant-multiple" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE),
        "comment-plus-outline" => Some(icons::COMMENT_PLUS_OUTLINE),
        "code-equal" => Some(icons::CODE_EQUAL),
        "panorama-horizontal-outline" => Some(icons::PANORAMA_HORIZONTAL_OUTLINE),
        "format-underline-wavy" => Some(icons::FORMAT_UNDERLINE_WAVY),
        "arrow-expand-left" => Some(icons::ARROW_EXPAND_LEFT),
        "printer-pos-remove-outline" => Some(icons::PRINTER_POS_REMOVE_OUTLINE),
        "phone-rotate-portrait" => Some(icons::PHONE_ROTATE_PORTRAIT),
        "smoking-pipe" => Some(icons::SMOKING_PIPE),
        "battery-clock" => Some(icons::BATTERY_CLOCK),
        "view-quilt" => Some(icons::VIEW_QUILT),
        "battery-90-bluetooth" => Some(icons::BATTERY_90_BLUETOOTH),
        "robber" => Some(icons::ROBBER),
        "alpha-m-circle-outline" => Some(icons::ALPHA_M_CIRCLE_OUTLINE),
        _ => None,
    }
}
