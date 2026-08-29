// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_6(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "archive-remove" => Some(icons::ARCHIVE_REMOVE),
        "battery-clock-outline" => Some(icons::BATTERY_CLOCK_OUTLINE),
        "monitor-share" => Some(icons::MONITOR_SHARE),
        "gradient-vertical" => Some(icons::GRADIENT_VERTICAL),
        "grill" => Some(icons::GRILL),
        "widgets" => Some(icons::WIDGETS),
        "fruit-pineapple" => Some(icons::FRUIT_PINEAPPLE),
        "radio" => Some(icons::RADIO),
        "filter-variant" => Some(icons::FILTER_VARIANT),
        "pitchfork" => Some(icons::PITCHFORK),
        "earbuds-off-outline" => Some(icons::EARBUDS_OFF_OUTLINE),
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        "liquor" => Some(icons::LIQUOR),
        "script-text-key-outline" => Some(icons::SCRIPT_TEXT_KEY_OUTLINE),
        "calendar-remove-outline" => Some(icons::CALENDAR_REMOVE_OUTLINE),
        "flask-off-outline" => Some(icons::FLASK_OFF_OUTLINE),
        #[allow(deprecated)]
        "microsoft-visual-studio-code" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-visual-studio-code' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO_CODE)
        }
        "vote-outline" => Some(icons::VOTE_OUTLINE),
        "numeric-3-box-multiple-outline" => Some(icons::NUMERIC_3_BOX_MULTIPLE_OUTLINE),
        "account-star-outline" => Some(icons::ACCOUNT_STAR_OUTLINE),
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        "lock-percent-open-outline" => Some(icons::LOCK_PERCENT_OPEN_OUTLINE),
        "cloud-upload" => Some(icons::CLOUD_UPLOAD),
        "image-filter-drama" => Some(icons::IMAGE_FILTER_DRAMA),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-medium" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-medium' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_MEDIUM)
        }
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "timeline" => Some(icons::TIMELINE),
        "floor-lamp-torchiere-outline" => Some(icons::FLOOR_LAMP_TORCHIERE_OUTLINE),
        "calendar-today-outline" => Some(icons::CALENDAR_TODAY_OUTLINE),
        "emoticon" => Some(icons::EMOTICON),
        "keyboard-off" => Some(icons::KEYBOARD_OFF),
        "television-speaker" => Some(icons::TELEVISION_SPEAKER),
        "account-clock-outline" => Some(icons::ACCOUNT_CLOCK_OUTLINE),
        "apps-box" => Some(icons::APPS_BOX),
        "timer-3" => Some(icons::TIMER_3),
        "format-text-rotation-none" => Some(icons::FORMAT_TEXT_ROTATION_NONE),
        #[allow(deprecated)]
        "facebook-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-gaming' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_GAMING)
        }
        "bookmark-music-outline" => Some(icons::BOOKMARK_MUSIC_OUTLINE),
        "white-balance-incandescent" => Some(icons::WHITE_BALANCE_INCANDESCENT),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "moon-full" => Some(icons::MOON_FULL),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "human-female-female" => Some(icons::HUMAN_FEMALE_FEMALE),
        "sign-direction-minus" => Some(icons::SIGN_DIRECTION_MINUS),
        "transition" => Some(icons::TRANSITION),
        "cloud-clock-outline" => Some(icons::CLOUD_CLOCK_OUTLINE),
        "train-car-tank" => Some(icons::TRAIN_CAR_TANK),
        "minus-circle-off-outline" => Some(icons::MINUS_CIRCLE_OFF_OUTLINE),
        "chart-line" => Some(icons::CHART_LINE),
        "rocket-launch-outline" => Some(icons::ROCKET_LAUNCH_OUTLINE),
        "heart" => Some(icons::HEART),
        "filter-multiple-outline" => Some(icons::FILTER_MULTIPLE_OUTLINE),
        "rhombus-split" => Some(icons::RHOMBUS_SPLIT),
        "fire-off" => Some(icons::FIRE_OFF),
        "shuffle-variant" => Some(icons::SHUFFLE_VARIANT),
        "lightbulb-on-20" => Some(icons::LIGHTBULB_ON_20),
        "numeric-2-box" => Some(icons::NUMERIC_2_BOX),
        "align-vertical-bottom" => Some(icons::ALIGN_VERTICAL_BOTTOM),
        "content-save-alert" => Some(icons::CONTENT_SAVE_ALERT),
        "clipboard-arrow-down" => Some(icons::CLIPBOARD_ARROW_DOWN),
        "home-city-outline" => Some(icons::HOME_CITY_OUTLINE),
        "calendar-blank-multiple" => Some(icons::CALENDAR_BLANK_MULTIPLE),
        "flask-round-bottom-outline" => Some(icons::FLASK_ROUND_BOTTOM_OUTLINE),
        "message-text-lock" => Some(icons::MESSAGE_TEXT_LOCK),
        "store-off" => Some(icons::STORE_OFF),
        "microphone-question-outline" => Some(icons::MICROPHONE_QUESTION_OUTLINE),
        "shield-alert" => Some(icons::SHIELD_ALERT),
        "chevron-right-circle-outline" => Some(icons::CHEVRON_RIGHT_CIRCLE_OUTLINE),
        "arrow-left-box" => Some(icons::ARROW_LEFT_BOX),
        "robot-vacuum" => Some(icons::ROBOT_VACUUM),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-charging" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-charging' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_CHARGING)
        }
        "wifi-cancel" => Some(icons::WIFI_CANCEL),
        "alert-minus-outline" => Some(icons::ALERT_MINUS_OUTLINE),
        "gamepad-round-right" => Some(icons::GAMEPAD_ROUND_RIGHT),
        "puzzle-star" => Some(icons::PUZZLE_STAR),
        "cash-marker" => Some(icons::CASH_MARKER),
        "link-box-outline" => Some(icons::LINK_BOX_OUTLINE),
        "code-block-parentheses" => Some(icons::CODE_BLOCK_PARENTHESES),
        "movie-off" => Some(icons::MOVIE_OFF),
        "room-service-outline" => Some(icons::ROOM_SERVICE_OUTLINE),
        "white-balance-sunny" => Some(icons::WHITE_BALANCE_SUNNY),
        "store-off-outline" => Some(icons::STORE_OFF_OUTLINE),
        "storefront-edit" => Some(icons::STOREFRONT_EDIT),
        "forum" => Some(icons::FORUM),
        "pipe-disconnected" => Some(icons::PIPE_DISCONNECTED),
        "account-alert-outline" => Some(icons::ACCOUNT_ALERT_OUTLINE),
        "tray-remove" => Some(icons::TRAY_REMOVE),
        "emoticon-dead-outline" => Some(icons::EMOTICON_DEAD_OUTLINE),
        "van-utility" => Some(icons::VAN_UTILITY),
        "calendar-start" => Some(icons::CALENDAR_START),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "gate-buffer" => Some(icons::GATE_BUFFER),
        "battery-sync" => Some(icons::BATTERY_SYNC),
        "alpha-g-box" => Some(icons::ALPHA_G_BOX),
        "arrow-u-down-left" => Some(icons::ARROW_U_DOWN_LEFT),
        "key-wireless" => Some(icons::KEY_WIRELESS),
        "octahedron" => Some(icons::OCTAHEDRON),
        "track-light" => Some(icons::TRACK_LIGHT),
        "clock-star-four-points" => Some(icons::CLOCK_STAR_FOUR_POINTS),
        "engine-off-outline" => Some(icons::ENGINE_OFF_OUTLINE),
        "watch-vibrate" => Some(icons::WATCH_VIBRATE),
        "leak" => Some(icons::LEAK),
        "power-plug-battery-outline" => Some(icons::POWER_PLUG_BATTERY_OUTLINE),
        "printer-search" => Some(icons::PRINTER_SEARCH),
        "arrow-up-right" => Some(icons::ARROW_UP_RIGHT),
        "scanner" => Some(icons::SCANNER),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "access-point-off" => Some(icons::ACCESS_POINT_OFF),
        "calculator" => Some(icons::CALCULATOR),
        "bell" => Some(icons::BELL),
        "relation-one-or-many-to-only-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONLY_ONE),
        "diving-scuba-tank-multiple" => Some(icons::DIVING_SCUBA_TANK_MULTIPLE),
        "numeric-10" => Some(icons::NUMERIC_10),
        "folder-wrench" => Some(icons::FOLDER_WRENCH),
        "alpha-a" => Some(icons::ALPHA_A),
        "keyboard-f6" => Some(icons::KEYBOARD_F6),
        "gift-off" => Some(icons::GIFT_OFF),
        "table-multiple" => Some(icons::TABLE_MULTIPLE),
        #[allow(deprecated)]
        "pi-hole" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pi-hole' is deprecated.").print(py);
            }
            Some(icons::PI_HOLE)
        }
        "texture" => Some(icons::TEXTURE),
        "white-balance-iridescent" => Some(icons::WHITE_BALANCE_IRIDESCENT),
        "cabin-a-frame" => Some(icons::CABIN_A_FRAME),
        "circle-slice-8" => Some(icons::CIRCLE_SLICE_8),
        "package-variant-closed" => Some(icons::PACKAGE_VARIANT_CLOSED),
        "table-key" => Some(icons::TABLE_KEY),
        "battery-charging-wireless-30" => Some(icons::BATTERY_CHARGING_WIRELESS_30),
        "sledding" => Some(icons::SLEDDING),
        "bucket" => Some(icons::BUCKET),
        "checkbox-marked-circle" => Some(icons::CHECKBOX_MARKED_CIRCLE),
        #[allow(deprecated)]
        "oci" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'oci' is deprecated.").print(py);
            }
            Some(icons::OCI)
        }
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "seat-flat" => Some(icons::SEAT_FLAT),
        "cellphone-remove" => Some(icons::CELLPHONE_REMOVE),
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "printer-pos-edit" => Some(icons::PRINTER_POS_EDIT),
        "share-circle" => Some(icons::SHARE_CIRCLE),
        "math-tan" => Some(icons::MATH_TAN),
        "weather-moonset" => Some(icons::WEATHER_MOONSET),
        "clock-time-four-outline" => Some(icons::CLOCK_TIME_FOUR_OUTLINE),
        "sim-off-outline" => Some(icons::SIM_OFF_OUTLINE),
        "arrow-bottom-right" => Some(icons::ARROW_BOTTOM_RIGHT),
        "book-cog" => Some(icons::BOOK_COG),
        "window-maximize" => Some(icons::WINDOW_MAXIMIZE),
        "car-shift-pattern" => Some(icons::CAR_SHIFT_PATTERN),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "screw-flat-top" => Some(icons::SCREW_FLAT_TOP),
        "credit-card-plus-outline" => Some(icons::CREDIT_CARD_PLUS_OUTLINE),
        "home-heart" => Some(icons::HOME_HEART),
        "cellphone-basic" => Some(icons::CELLPHONE_BASIC),
        "file-certificate-outline" => Some(icons::FILE_CERTIFICATE_OUTLINE),
        "microphone-message" => Some(icons::MICROPHONE_MESSAGE),
        "pi" => Some(icons::PI),
        "file-multiple" => Some(icons::FILE_MULTIPLE),
        "flag-variant-off-outline" => Some(icons::FLAG_VARIANT_OFF_OUTLINE),
        "liquid-spot" => Some(icons::LIQUID_SPOT),
        #[allow(deprecated)]
        "xmpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xmpp' is deprecated.").print(py);
            }
            Some(icons::XMPP)
        }
        "printer-pos-play-outline" => Some(icons::PRINTER_POS_PLAY_OUTLINE),
        "image-edit" => Some(icons::IMAGE_EDIT),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "tooltip-plus" => Some(icons::TOOLTIP_PLUS),
        "gas-burner" => Some(icons::GAS_BURNER),
        "delete-sweep-outline" => Some(icons::DELETE_SWEEP_OUTLINE),
        "video-box" => Some(icons::VIDEO_BOX),
        "alpha-i-circle-outline" => Some(icons::ALPHA_I_CIRCLE_OUTLINE),
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "volume-equal" => Some(icons::VOLUME_EQUAL),
        "web-sync" => Some(icons::WEB_SYNC),
        "barrel" => Some(icons::BARREL),
        "timer-marker-outline" => Some(icons::TIMER_MARKER_OUTLINE),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "skateboarding" => Some(icons::SKATEBOARDING),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "sign-yield" => Some(icons::SIGN_YIELD),
        "yurt" => Some(icons::YURT),
        "wifi-strength-3-alert" => Some(icons::WIFI_STRENGTH_3_ALERT),
        "lightbulb-on-outline" => Some(icons::LIGHTBULB_ON_OUTLINE),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "angle-right" => Some(icons::ANGLE_RIGHT),
        "book-open-page-variant" => Some(icons::BOOK_OPEN_PAGE_VARIANT),
        "bank-off" => Some(icons::BANK_OFF),
        "emoticon-angry-outline" => Some(icons::EMOTICON_ANGRY_OUTLINE),
        "folder-clock" => Some(icons::FOLDER_CLOCK),
        "image-area" => Some(icons::IMAGE_AREA),
        "star-four-points-outline" => Some(icons::STAR_FOUR_POINTS_OUTLINE),
        "sort-variant-off" => Some(icons::SORT_VARIANT_OFF),
        "yeast" => Some(icons::YEAST),
        "waves-arrow-up" => Some(icons::WAVES_ARROW_UP),
        "baby" => Some(icons::BABY),
        "clock-time-twelve" => Some(icons::CLOCK_TIME_TWELVE),
        "dna" => Some(icons::DNA),
        "receipt-clock-outline" => Some(icons::RECEIPT_CLOCK_OUTLINE),
        "battery-90-bluetooth" => Some(icons::BATTERY_90_BLUETOOTH),
        "clock-time-ten-outline" => Some(icons::CLOCK_TIME_TEN_OUTLINE),
        "food-drumstick-off-outline" => Some(icons::FOOD_DRUMSTICK_OFF_OUTLINE),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "hexagon-slice-4" => Some(icons::HEXAGON_SLICE_4),
        "wifi-strength-1-alert" => Some(icons::WIFI_STRENGTH_1_ALERT),
        _ => None,
    }
}
