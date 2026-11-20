// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_9(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "google-cloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cloud' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CLOUD)
        }
        "cloud-cancel" => Some(icons::CLOUD_CANCEL),
        "robot-outline" => Some(icons::ROBOT_OUTLINE),
        "folder-arrow-left-outline" => Some(icons::FOLDER_ARROW_LEFT_OUTLINE),
        "link-box" => Some(icons::LINK_BOX),
        "align-vertical-bottom" => Some(icons::ALIGN_VERTICAL_BOTTOM),
        "cloud-arrow-left" => Some(icons::CLOUD_ARROW_LEFT),
        "gradient-vertical" => Some(icons::GRADIENT_VERTICAL),
        "satellite-uplink" => Some(icons::SATELLITE_UPLINK),
        "flask-minus" => Some(icons::FLASK_MINUS),
        "kettle-off" => Some(icons::KETTLE_OFF),
        "head-outline" => Some(icons::HEAD_OUTLINE),
        "tooltip-image-outline" => Some(icons::TOOLTIP_IMAGE_OUTLINE),
        "calculator-variant-outline" => Some(icons::CALCULATOR_VARIANT_OUTLINE),
        "calendar-clock" => Some(icons::CALENDAR_CLOCK),
        #[allow(deprecated)]
        "ansible" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ansible' is deprecated.").print(py);
            }
            Some(icons::ANSIBLE)
        }
        "video-2d" => Some(icons::VIDEO_2D),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "alpha-m-box" => Some(icons::ALPHA_M_BOX),
        "fruit-citrus" => Some(icons::FRUIT_CITRUS),
        "vanity-light" => Some(icons::VANITY_LIGHT),
        "account-music" => Some(icons::ACCOUNT_MUSIC),
        "arrow-down-bold-hexagon-outline" => Some(icons::ARROW_DOWN_BOLD_HEXAGON_OUTLINE),
        "variable" => Some(icons::VARIABLE),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "pitchfork" => Some(icons::PITCHFORK),
        "emoticon-neutral" => Some(icons::EMOTICON_NEUTRAL),
        "dice-5-outline" => Some(icons::DICE_5_OUTLINE),
        "relation-zero-or-one-to-zero-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ZERO_OR_MANY),
        #[allow(deprecated)]
        "folder-google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'folder-google-drive' is deprecated.")
                    .print(py);
            }
            Some(icons::FOLDER_GOOGLE_DRIVE)
        }
        "curtains" => Some(icons::CURTAINS),
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        "kettle-alert" => Some(icons::KETTLE_ALERT),
        "battery-30" => Some(icons::BATTERY_30),
        "numeric-0-box-outline" => Some(icons::NUMERIC_0_BOX_OUTLINE),
        "folder-search-outline" => Some(icons::FOLDER_SEARCH_OUTLINE),
        "nature-people-outline" => Some(icons::NATURE_PEOPLE_OUTLINE),
        "hand-pointing-right" => Some(icons::HAND_POINTING_RIGHT),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
        "comment-flash" => Some(icons::COMMENT_FLASH),
        "cards-playing-diamond-outline" => Some(icons::CARDS_PLAYING_DIAMOND_OUTLINE),
        "signature-text" => Some(icons::SIGNATURE_TEXT),
        "filter-plus" => Some(icons::FILTER_PLUS),
        "email-arrow-right-outline" => Some(icons::EMAIL_ARROW_RIGHT_OUTLINE),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "ladybug" => Some(icons::LADYBUG),
        #[allow(deprecated)]
        "google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-drive' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_DRIVE)
        }
        "projector-screen-variant-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OUTLINE),
        "folder-file" => Some(icons::FOLDER_FILE),
        "movie-open-settings" => Some(icons::MOVIE_OPEN_SETTINGS),
        "database-edit" => Some(icons::DATABASE_EDIT),
        "book-open-variant" => Some(icons::BOOK_OPEN_VARIANT),
        "circle-slice-7" => Some(icons::CIRCLE_SLICE_7),
        "snowflake-thermometer" => Some(icons::SNOWFLAKE_THERMOMETER),
        "dna" => Some(icons::DNA),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "folder-play-outline" => Some(icons::FOLDER_PLAY_OUTLINE),
        "text-box-check" => Some(icons::TEXT_BOX_CHECK),
        "magazine-rifle" => Some(icons::MAGAZINE_RIFLE),
        "tennis" => Some(icons::TENNIS),
        #[allow(deprecated)]
        "wechat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wechat' is deprecated.").print(py);
            }
            Some(icons::WECHAT)
        }
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "file-alert-outline" => Some(icons::FILE_ALERT_OUTLINE),
        "play-network" => Some(icons::PLAY_NETWORK),
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "fit-to-screen-outline" => Some(icons::FIT_TO_SCREEN_OUTLINE),
        "database-clock-outline" => Some(icons::DATABASE_CLOCK_OUTLINE),
        "code-braces-box" => Some(icons::CODE_BRACES_BOX),
        "focus-field-horizontal" => Some(icons::FOCUS_FIELD_HORIZONTAL),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "smart-card-off" => Some(icons::SMART_CARD_OFF),
        "virus" => Some(icons::VIRUS),
        "close-circle-multiple" => Some(icons::CLOSE_CIRCLE_MULTIPLE),
        "keyboard-f3" => Some(icons::KEYBOARD_F3),
        "printer-pos-outline" => Some(icons::PRINTER_POS_OUTLINE),
        "bookmark-plus" => Some(icons::BOOKMARK_PLUS),
        "book-alphabet" => Some(icons::BOOK_ALPHABET),
        "filmstrip-box" => Some(icons::FILMSTRIP_BOX),
        "axis-y-rotate-clockwise" => Some(icons::AXIS_Y_ROTATE_CLOCKWISE),
        "elephant" => Some(icons::ELEPHANT),
        #[allow(deprecated)]
        "goodreads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'goodreads' is deprecated.").print(py);
            }
            Some(icons::GOODREADS)
        }
        "vote" => Some(icons::VOTE),
        "bell" => Some(icons::BELL),
        "format-letter-case-upper" => Some(icons::FORMAT_LETTER_CASE_UPPER),
        "target" => Some(icons::TARGET),
        "head-cog" => Some(icons::HEAD_COG),
        "arrow-down-drop-circle" => Some(icons::ARROW_DOWN_DROP_CIRCLE),
        "battery-charging-low" => Some(icons::BATTERY_CHARGING_LOW),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "snail" => Some(icons::SNAIL),
        "firewire" => Some(icons::FIREWIRE),
        "link-variant" => Some(icons::LINK_VARIANT),
        "table-filter" => Some(icons::TABLE_FILTER),
        "flag" => Some(icons::FLAG),
        "rug" => Some(icons::RUG),
        "movie-open-off" => Some(icons::MOVIE_OPEN_OFF),
        "email-plus-outline" => Some(icons::EMAIL_PLUS_OUTLINE),
        "wifi-strength-lock-open-outline" => Some(icons::WIFI_STRENGTH_LOCK_OPEN_OUTLINE),
        "emoticon-sick-outline" => Some(icons::EMOTICON_SICK_OUTLINE),
        "nfc-variant" => Some(icons::NFC_VARIANT),
        "text-box-remove" => Some(icons::TEXT_BOX_REMOVE),
        "umbrella-closed-outline" => Some(icons::UMBRELLA_CLOSED_OUTLINE),
        "format-letter-case-lower" => Some(icons::FORMAT_LETTER_CASE_LOWER),
        "head-flash" => Some(icons::HEAD_FLASH),
        "paw" => Some(icons::PAW),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        "stethoscope" => Some(icons::STETHOSCOPE),
        "note-minus-outline" => Some(icons::NOTE_MINUS_OUTLINE),
        "routes" => Some(icons::ROUTES),
        "slash-forward-box" => Some(icons::SLASH_FORWARD_BOX),
        "chat-remove-outline" => Some(icons::CHAT_REMOVE_OUTLINE),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "home-minus-outline" => Some(icons::HOME_MINUS_OUTLINE),
        "transit-connection" => Some(icons::TRANSIT_CONNECTION),
        "ear-hearing-off" => Some(icons::EAR_HEARING_OFF),
        "share-variant" => Some(icons::SHARE_VARIANT),
        "arrow-top-right-thin-circle-outline" => Some(icons::ARROW_TOP_RIGHT_THIN_CIRCLE_OUTLINE),
        "credit-card-fast" => Some(icons::CREDIT_CARD_FAST),
        "dock-left" => Some(icons::DOCK_LEFT),
        "engine-off" => Some(icons::ENGINE_OFF),
        "clock-star-four-points-outline" => Some(icons::CLOCK_STAR_FOUR_POINTS_OUTLINE),
        "shield-refresh" => Some(icons::SHIELD_REFRESH),
        "tooltip-image" => Some(icons::TOOLTIP_IMAGE),
        "email-minus" => Some(icons::EMAIL_MINUS),
        "bug-check" => Some(icons::BUG_CHECK),
        "receipt-text-check" => Some(icons::RECEIPT_TEXT_CHECK),
        "animation-play" => Some(icons::ANIMATION_PLAY),
        "weather-sunset-up" => Some(icons::WEATHER_SUNSET_UP),
        "currency-btc" => Some(icons::CURRENCY_BTC),
        "account-minus-outline" => Some(icons::ACCOUNT_MINUS_OUTLINE),
        "star-off" => Some(icons::STAR_OFF),
        #[allow(deprecated)]
        "trello" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'trello' is deprecated.").print(py);
            }
            Some(icons::TRELLO)
        }
        "numeric-9-circle-outline" => Some(icons::NUMERIC_9_CIRCLE_OUTLINE),
        "package-up" => Some(icons::PACKAGE_UP),
        "movie-filter-outline" => Some(icons::MOVIE_FILTER_OUTLINE),
        "folder-star-multiple" => Some(icons::FOLDER_STAR_MULTIPLE),
        "book-search-outline" => Some(icons::BOOK_SEARCH_OUTLINE),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "faucet" => Some(icons::FAUCET),
        "laptop-account" => Some(icons::LAPTOP_ACCOUNT),
        "briefcase-upload-outline" => Some(icons::BRIEFCASE_UPLOAD_OUTLINE),
        "restart" => Some(icons::RESTART),
        "email-box" => Some(icons::EMAIL_BOX),
        "medical-cotton-swab" => Some(icons::MEDICAL_COTTON_SWAB),
        "leak-off" => Some(icons::LEAK_OFF),
        "phone-return" => Some(icons::PHONE_RETURN),
        "waves" => Some(icons::WAVES),
        "briefcase-remove-outline" => Some(icons::BRIEFCASE_REMOVE_OUTLINE),
        "wifi-arrow-up" => Some(icons::WIFI_ARROW_UP),
        "account-tie-voice-off-outline" => Some(icons::ACCOUNT_TIE_VOICE_OFF_OUTLINE),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "arrow-u-down-left" => Some(icons::ARROW_U_DOWN_LEFT),
        "cellphone-cog" => Some(icons::CELLPHONE_COG),
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "sync-circle" => Some(icons::SYNC_CIRCLE),
        "eject-outline" => Some(icons::EJECT_OUTLINE),
        "plus-circle-outline" => Some(icons::PLUS_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "pinterest" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pinterest' is deprecated.").print(py);
            }
            Some(icons::PINTEREST)
        }
        "switch" => Some(icons::SWITCH),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        "tooltip" => Some(icons::TOOLTIP),
        "numeric-8-box" => Some(icons::NUMERIC_8_BOX),
        "subdirectory-arrow-left" => Some(icons::SUBDIRECTORY_ARROW_LEFT),
        "gas-station-off-outline" => Some(icons::GAS_STATION_OFF_OUTLINE),
        #[allow(deprecated)]
        "microsoft-azure-devops" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure-devops' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE_DEVOPS)
        }
        "download-lock-outline" => Some(icons::DOWNLOAD_LOCK_OUTLINE),
        "tag-search" => Some(icons::TAG_SEARCH),
        "message-arrow-left" => Some(icons::MESSAGE_ARROW_LEFT),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "shield-moon" => Some(icons::SHIELD_MOON),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "email-alert-outline" => Some(icons::EMAIL_ALERT_OUTLINE),
        "basket-check" => Some(icons::BASKET_CHECK),
        "image-minus" => Some(icons::IMAGE_MINUS),
        "star-face" => Some(icons::STAR_FACE),
        "camera-metering-matrix" => Some(icons::CAMERA_METERING_MATRIX),
        "heart-cog" => Some(icons::HEART_COG),
        "blinds-horizontal" => Some(icons::BLINDS_HORIZONTAL),
        "numeric-negative-1" => Some(icons::NUMERIC_NEGATIVE_1),
        "recycle" => Some(icons::RECYCLE),
        "crop-square" => Some(icons::CROP_SQUARE),
        "nail" => Some(icons::NAIL),
        "cheese-off" => Some(icons::CHEESE_OFF),
        "temple-buddhist" => Some(icons::TEMPLE_BUDDHIST),
        "timer-sand-complete" => Some(icons::TIMER_SAND_COMPLETE),
        "glass-mug-variant-off" => Some(icons::GLASS_MUG_VARIANT_OFF),
        "map-marker-account-outline" => Some(icons::MAP_MARKER_ACCOUNT_OUTLINE),
        #[allow(deprecated)]
        "google-play" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-play' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLAY)
        }
        "web-refresh" => Some(icons::WEB_REFRESH),
        "cupcake" => Some(icons::CUPCAKE),
        "toy-brick-minus" => Some(icons::TOY_BRICK_MINUS),
        "battery-clock-outline" => Some(icons::BATTERY_CLOCK_OUTLINE),
        "table-account" => Some(icons::TABLE_ACCOUNT),
        "file-code" => Some(icons::FILE_CODE),
        "clock-time-seven-outline" => Some(icons::CLOCK_TIME_SEVEN_OUTLINE),
        "movie-search" => Some(icons::MOVIE_SEARCH),
        "printer-pos-pause-outline" => Some(icons::PRINTER_POS_PAUSE_OUTLINE),
        _ => None,
    }
}
