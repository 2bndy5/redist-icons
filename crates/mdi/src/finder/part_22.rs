// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_22(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "triangle-down-outline" => Some(icons::TRIANGLE_DOWN_OUTLINE),
        "account-key-outline" => Some(icons::ACCOUNT_KEY_OUTLINE),
        "diving-scuba-tank" => Some(icons::DIVING_SCUBA_TANK),
        "clock-time-eight-outline" => Some(icons::CLOCK_TIME_EIGHT_OUTLINE),
        "lock-open-alert-outline" => Some(icons::LOCK_OPEN_ALERT_OUTLINE),
        "lightbulb-group-off" => Some(icons::LIGHTBULB_GROUP_OFF),
        "lan-disconnect" => Some(icons::LAN_DISCONNECT),
        "earth-box-plus" => Some(icons::EARTH_BOX_PLUS),
        "cookie-refresh" => Some(icons::COOKIE_REFRESH),
        "label-off" => Some(icons::LABEL_OFF),
        "move-resize-variant" => Some(icons::MOVE_RESIZE_VARIANT),
        "food-apple-outline" => Some(icons::FOOD_APPLE_OUTLINE),
        "alpha-g-circle-outline" => Some(icons::ALPHA_G_CIRCLE_OUTLINE),
        "car-battery" => Some(icons::CAR_BATTERY),
        "steering" => Some(icons::STEERING),
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        "vacuum-outline" => Some(icons::VACUUM_OUTLINE),
        "skip-forward-outline" => Some(icons::SKIP_FORWARD_OUTLINE),
        "loupe" => Some(icons::LOUPE),
        "flag-triangle" => Some(icons::FLAG_TRIANGLE),
        "credit-card-scan" => Some(icons::CREDIT_CARD_SCAN),
        "view-list-outline" => Some(icons::VIEW_LIST_OUTLINE),
        "circle-box" => Some(icons::CIRCLE_BOX),
        "lightbulb-group-off-outline" => Some(icons::LIGHTBULB_GROUP_OFF_OUTLINE),
        "terrain" => Some(icons::TERRAIN),
        "format-align-bottom" => Some(icons::FORMAT_ALIGN_BOTTOM),
        "filmstrip" => Some(icons::FILMSTRIP),
        "database-lock" => Some(icons::DATABASE_LOCK),
        "application-braces-outline" => Some(icons::APPLICATION_BRACES_OUTLINE),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "dice-6-outline" => Some(icons::DICE_6_OUTLINE),
        "clipboard-text-off-outline" => Some(icons::CLIPBOARD_TEXT_OFF_OUTLINE),
        "alarm" => Some(icons::ALARM),
        "calendar-sync-outline" => Some(icons::CALENDAR_SYNC_OUTLINE),
        "passport-minus" => Some(icons::PASSPORT_MINUS),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "bowl-mix" => Some(icons::BOWL_MIX),
        "database-arrow-right" => Some(icons::DATABASE_ARROW_RIGHT),
        #[allow(deprecated)]
        "blender-software" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'blender-software' is deprecated.")
                    .print(py);
            }
            Some(icons::BLENDER_SOFTWARE)
        }
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "briefcase-check-outline" => Some(icons::BRIEFCASE_CHECK_OUTLINE),
        "alarm-light-outline" => Some(icons::ALARM_LIGHT_OUTLINE),
        "human-non-binary" => Some(icons::HUMAN_NON_BINARY),
        "cellphone-marker" => Some(icons::CELLPHONE_MARKER),
        "dolly" => Some(icons::DOLLY),
        "gesture-swipe-up" => Some(icons::GESTURE_SWIPE_UP),
        "playlist-minus" => Some(icons::PLAYLIST_MINUS),
        "source-branch-sync" => Some(icons::SOURCE_BRANCH_SYNC),
        "brightness-auto" => Some(icons::BRIGHTNESS_AUTO),
        "safe-square-outline" => Some(icons::SAFE_SQUARE_OUTLINE),
        "invoice-send" => Some(icons::INVOICE_SEND),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "stadium-variant" => Some(icons::STADIUM_VARIANT),
        "subway-alert-variant" => Some(icons::SUBWAY_ALERT_VARIANT),
        "cast-variant" => Some(icons::CAST_VARIANT),
        "shield" => Some(icons::SHIELD),
        "chat-remove-outline" => Some(icons::CHAT_REMOVE_OUTLINE),
        "phone-outgoing" => Some(icons::PHONE_OUTGOING),
        "hololens" => Some(icons::HOLOLENS),
        "microphone-question-outline" => Some(icons::MICROPHONE_QUESTION_OUTLINE),
        "mixed-martial-arts" => Some(icons::MIXED_MARTIAL_ARTS),
        "camera-iris" => Some(icons::CAMERA_IRIS),
        "movie-settings-outline" => Some(icons::MOVIE_SETTINGS_OUTLINE),
        "ring" => Some(icons::RING),
        "monitor-off" => Some(icons::MONITOR_OFF),
        "alpha-u-box-outline" => Some(icons::ALPHA_U_BOX_OUTLINE),
        "television-ambient-light" => Some(icons::TELEVISION_AMBIENT_LIGHT),
        "map-marker-down" => Some(icons::MAP_MARKER_DOWN),
        "glass-tulip" => Some(icons::GLASS_TULIP),
        "truck-delivery-outline" => Some(icons::TRUCK_DELIVERY_OUTLINE),
        "content-save-check" => Some(icons::CONTENT_SAVE_CHECK),
        "ruler" => Some(icons::RULER),
        "account-hard-hat" => Some(icons::ACCOUNT_HARD_HAT),
        "bank" => Some(icons::BANK),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "movie-off-outline" => Some(icons::MOVIE_OFF_OUTLINE),
        "delete-sweep" => Some(icons::DELETE_SWEEP),
        "pound-box-outline" => Some(icons::POUND_BOX_OUTLINE),
        "receipt-text-arrow-right-outline" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT_OUTLINE),
        "coach-lamp-variant" => Some(icons::COACH_LAMP_VARIANT),
        "folder-remove-outline" => Some(icons::FOLDER_REMOVE_OUTLINE),
        "eyedropper-plus" => Some(icons::EYEDROPPER_PLUS),
        "window-shutter-auto" => Some(icons::WINDOW_SHUTTER_AUTO),
        "delete-alert" => Some(icons::DELETE_ALERT),
        "ev-plug-tesla" => Some(icons::EV_PLUG_TESLA),
        "text-recognition" => Some(icons::TEXT_RECOGNITION),
        "gamepad-left" => Some(icons::GAMEPAD_LEFT),
        "phone-message" => Some(icons::PHONE_MESSAGE),
        "email-variant" => Some(icons::EMAIL_VARIANT),
        "relation-many-to-only-one" => Some(icons::RELATION_MANY_TO_ONLY_ONE),
        "emoticon-poop" => Some(icons::EMOTICON_POOP),
        "store-settings" => Some(icons::STORE_SETTINGS),
        "shield-crown-outline" => Some(icons::SHIELD_CROWN_OUTLINE),
        "message-arrow-right" => Some(icons::MESSAGE_ARROW_RIGHT),
        "seed-plus-outline" => Some(icons::SEED_PLUS_OUTLINE),
        "beaker-remove" => Some(icons::BEAKER_REMOVE),
        "comment-question" => Some(icons::COMMENT_QUESTION),
        "file-document-refresh" => Some(icons::FILE_DOCUMENT_REFRESH),
        "human-male-female" => Some(icons::HUMAN_MALE_FEMALE),
        "vector-link" => Some(icons::VECTOR_LINK),
        "calendar-text" => Some(icons::CALENDAR_TEXT),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
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
        "table-question" => Some(icons::TABLE_QUESTION),
        "timeline-plus" => Some(icons::TIMELINE_PLUS),
        "candy" => Some(icons::CANDY),
        "tooltip" => Some(icons::TOOLTIP),
        "head-minus-outline" => Some(icons::HEAD_MINUS_OUTLINE),
        "key-alert" => Some(icons::KEY_ALERT),
        "hand-heart-outline" => Some(icons::HAND_HEART_OUTLINE),
        "syllabary-hangul" => Some(icons::SYLLABARY_HANGUL),
        "heat-pump-outline" => Some(icons::HEAT_PUMP_OUTLINE),
        "arrow-top-left-thin-circle-outline" => Some(icons::ARROW_TOP_LEFT_THIN_CIRCLE_OUTLINE),
        "human-male-female-child" => Some(icons::HUMAN_MALE_FEMALE_CHILD),
        "printer-off-outline" => Some(icons::PRINTER_OFF_OUTLINE),
        "comment-search-outline" => Some(icons::COMMENT_SEARCH_OUTLINE),
        "cupboard" => Some(icons::CUPBOARD),
        "lightbulb-fluorescent-tube-outline" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE_OUTLINE),
        "shield-airplane-outline" => Some(icons::SHIELD_AIRPLANE_OUTLINE),
        "download" => Some(icons::DOWNLOAD),
        "radioactive-circle" => Some(icons::RADIOACTIVE_CIRCLE),
        "clock-minus-outline" => Some(icons::CLOCK_MINUS_OUTLINE),
        #[allow(deprecated)]
        "google-cloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cloud' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CLOUD)
        }
        "invoice-text-minus" => Some(icons::INVOICE_TEXT_MINUS),
        "bag-checked" => Some(icons::BAG_CHECKED),
        "laptop-off" => Some(icons::LAPTOP_OFF),
        "map-check" => Some(icons::MAP_CHECK),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "vector-polyline" => Some(icons::VECTOR_POLYLINE),
        "pig-variant-outline" => Some(icons::PIG_VARIANT_OUTLINE),
        "tab-plus" => Some(icons::TAB_PLUS),
        "flask-plus-outline" => Some(icons::FLASK_PLUS_OUTLINE),
        "sort-alphabetical-ascending" => Some(icons::SORT_ALPHABETICAL_ASCENDING),
        "volume-variant-off" => Some(icons::VOLUME_VARIANT_OFF),
        "wifi-arrow-right" => Some(icons::WIFI_ARROW_RIGHT),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "information-variant-circle" => Some(icons::INFORMATION_VARIANT_CIRCLE),
        "folder-zip" => Some(icons::FOLDER_ZIP),
        "gas-station-off" => Some(icons::GAS_STATION_OFF),
        "dice-multiple-outline" => Some(icons::DICE_MULTIPLE_OUTLINE),
        "wallet-bifold-outline" => Some(icons::WALLET_BIFOLD_OUTLINE),
        "dishwasher-alert" => Some(icons::DISHWASHER_ALERT),
        "chevron-down-circle" => Some(icons::CHEVRON_DOWN_CIRCLE),
        "arrow-up-bold-outline" => Some(icons::ARROW_UP_BOLD_OUTLINE),
        "chevron-right" => Some(icons::CHEVRON_RIGHT),
        "relation-zero-or-one-to-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_MANY),
        "bus-stop" => Some(icons::BUS_STOP),
        "minus-box" => Some(icons::MINUS_BOX),
        "toaster-oven" => Some(icons::TOASTER_OVEN),
        "motion-sensor-off" => Some(icons::MOTION_SENSOR_OFF),
        "cursor-default-gesture-outline" => Some(icons::CURSOR_DEFAULT_GESTURE_OUTLINE),
        #[allow(deprecated)]
        "firebase" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firebase' is deprecated.").print(py);
            }
            Some(icons::FIREBASE)
        }
        "eye-remove-outline" => Some(icons::EYE_REMOVE_OUTLINE),
        "seal" => Some(icons::SEAL),
        "calendar-alert" => Some(icons::CALENDAR_ALERT),
        "book-open-page-variant" => Some(icons::BOOK_OPEN_PAGE_VARIANT),
        "water-circle" => Some(icons::WATER_CIRCLE),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "help-box" => Some(icons::HELP_BOX),
        "archive-plus" => Some(icons::ARCHIVE_PLUS),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "camera-account" => Some(icons::CAMERA_ACCOUNT),
        "pier" => Some(icons::PIER),
        "hockey-sticks" => Some(icons::HOCKEY_STICKS),
        "chat-processing-outline" => Some(icons::CHAT_PROCESSING_OUTLINE),
        "page-layout-footer" => Some(icons::PAGE_LAYOUT_FOOTER),
        #[allow(deprecated)]
        "microsoft-internet-explorer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-internet-explorer' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_INTERNET_EXPLORER)
        }
        "invoice-clock" => Some(icons::INVOICE_CLOCK),
        "faucet-variant" => Some(icons::FAUCET_VARIANT),
        "temple-hindu" => Some(icons::TEMPLE_HINDU),
        "coffee-off" => Some(icons::COFFEE_OFF),
        "download-circle-outline" => Some(icons::DOWNLOAD_CIRCLE_OUTLINE),
        "video-input-scart" => Some(icons::VIDEO_INPUT_SCART),
        "delete-sweep-outline" => Some(icons::DELETE_SWEEP_OUTLINE),
        "home-switch" => Some(icons::HOME_SWITCH),
        "curtains-closed" => Some(icons::CURTAINS_CLOSED),
        "alpha-w" => Some(icons::ALPHA_W),
        "ship-wheel" => Some(icons::SHIP_WHEEL),
        "head-plus" => Some(icons::HEAD_PLUS),
        "diamond-outline" => Some(icons::DIAMOND_OUTLINE),
        "select-remove" => Some(icons::SELECT_REMOVE),
        "source-commit-next-local" => Some(icons::SOURCE_COMMIT_NEXT_LOCAL),
        "filter-remove-outline" => Some(icons::FILTER_REMOVE_OUTLINE),
        "attachment" => Some(icons::ATTACHMENT),
        "train-car-centerbeam" => Some(icons::TRAIN_CAR_CENTERBEAM),
        "cloud-key-outline" => Some(icons::CLOUD_KEY_OUTLINE),
        "floor-lamp-outline" => Some(icons::FLOOR_LAMP_OUTLINE),
        "email-open-outline" => Some(icons::EMAIL_OPEN_OUTLINE),
        "layers-triple" => Some(icons::LAYERS_TRIPLE),
        "decimal-comma" => Some(icons::DECIMAL_COMMA),
        "arrow-left-bold-circle-outline" => Some(icons::ARROW_LEFT_BOLD_CIRCLE_OUTLINE),
        "form-select" => Some(icons::FORM_SELECT),
        "auto-mode" => Some(icons::AUTO_MODE),
        "headphones-off" => Some(icons::HEADPHONES_OFF),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "dishwasher" => Some(icons::DISHWASHER),
        "arrow-up" => Some(icons::ARROW_UP),
        "alpha-n-box" => Some(icons::ALPHA_N_BOX),
        #[allow(deprecated)]
        "facebook-workplace" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-workplace' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_WORKPLACE)
        }
        "pi" => Some(icons::PI),
        _ => None,
    }
}
