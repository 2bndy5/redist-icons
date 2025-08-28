// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_31(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "solar-power" => Some(icons::SOLAR_POWER),
        "soldering-iron" => Some(icons::SOLDERING_IRON),
        "solid" => Some(icons::SOLID),
        #[allow(deprecated)]
        "sony-playstation" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sony-playstation' is deprecated.")
                    .print(py);
            }
            Some(icons::SONY_PLAYSTATION)
        }
        "sort-alphabetical-ascending-variant" => Some(icons::SORT_ALPHABETICAL_ASCENDING_VARIANT),
        "sort-alphabetical-ascending" => Some(icons::SORT_ALPHABETICAL_ASCENDING),
        "sort-alphabetical-descending-variant" => Some(icons::SORT_ALPHABETICAL_DESCENDING_VARIANT),
        "sort-alphabetical-descending" => Some(icons::SORT_ALPHABETICAL_DESCENDING),
        "sort-alphabetical-variant" => Some(icons::SORT_ALPHABETICAL_VARIANT),
        "sort-ascending" => Some(icons::SORT_ASCENDING),
        "sort-bool-ascending-variant" => Some(icons::SORT_BOOL_ASCENDING_VARIANT),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "sort-bool-descending-variant" => Some(icons::SORT_BOOL_DESCENDING_VARIANT),
        "sort-bool-descending" => Some(icons::SORT_BOOL_DESCENDING),
        "sort-calendar-ascending" => Some(icons::SORT_CALENDAR_ASCENDING),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "sort-clock-ascending-outline" => Some(icons::SORT_CLOCK_ASCENDING_OUTLINE),
        "sort-clock-ascending" => Some(icons::SORT_CLOCK_ASCENDING),
        "sort-clock-descending-outline" => Some(icons::SORT_CLOCK_DESCENDING_OUTLINE),
        "sort-clock-descending" => Some(icons::SORT_CLOCK_DESCENDING),
        "sort-descending" => Some(icons::SORT_DESCENDING),
        "sort-numeric-ascending-variant" => Some(icons::SORT_NUMERIC_ASCENDING_VARIANT),
        "sort-numeric-ascending" => Some(icons::SORT_NUMERIC_ASCENDING),
        "sort-numeric-descending-variant" => Some(icons::SORT_NUMERIC_DESCENDING_VARIANT),
        "sort-numeric-descending" => Some(icons::SORT_NUMERIC_DESCENDING),
        "sort-numeric-variant" => Some(icons::SORT_NUMERIC_VARIANT),
        "sort-reverse-variant" => Some(icons::SORT_REVERSE_VARIANT),
        "sort-variant-lock-open" => Some(icons::SORT_VARIANT_LOCK_OPEN),
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "sort-variant-off" => Some(icons::SORT_VARIANT_OFF),
        "sort-variant-remove" => Some(icons::SORT_VARIANT_REMOVE),
        "sort-variant" => Some(icons::SORT_VARIANT),
        "sort" => Some(icons::SORT),
        "soundbar" => Some(icons::SOUNDBAR),
        #[allow(deprecated)]
        "soundcloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'soundcloud' is deprecated.").print(py);
            }
            Some(icons::SOUNDCLOUD)
        }
        "source-branch-check" => Some(icons::SOURCE_BRANCH_CHECK),
        "source-branch-minus" => Some(icons::SOURCE_BRANCH_MINUS),
        "source-branch-plus" => Some(icons::SOURCE_BRANCH_PLUS),
        "source-branch-refresh" => Some(icons::SOURCE_BRANCH_REFRESH),
        "source-branch-remove" => Some(icons::SOURCE_BRANCH_REMOVE),
        "source-branch-sync" => Some(icons::SOURCE_BRANCH_SYNC),
        "source-branch" => Some(icons::SOURCE_BRANCH),
        "source-commit-end-local" => Some(icons::SOURCE_COMMIT_END_LOCAL),
        "source-commit-end" => Some(icons::SOURCE_COMMIT_END),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "source-commit-next-local" => Some(icons::SOURCE_COMMIT_NEXT_LOCAL),
        "source-commit-start-next-local" => Some(icons::SOURCE_COMMIT_START_NEXT_LOCAL),
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        "source-commit" => Some(icons::SOURCE_COMMIT),
        "source-fork" => Some(icons::SOURCE_FORK),
        "source-merge" => Some(icons::SOURCE_MERGE),
        "source-pull" => Some(icons::SOURCE_PULL),
        "source-repository-multiple" => Some(icons::SOURCE_REPOSITORY_MULTIPLE),
        "source-repository" => Some(icons::SOURCE_REPOSITORY),
        "soy-sauce-off" => Some(icons::SOY_SAUCE_OFF),
        "soy-sauce" => Some(icons::SOY_SAUCE),
        "spa-outline" => Some(icons::SPA_OUTLINE),
        "spa" => Some(icons::SPA),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "space-station" => Some(icons::SPACE_STATION),
        "spade" => Some(icons::SPADE),
        "speaker-bluetooth" => Some(icons::SPEAKER_BLUETOOTH),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "speaker-multiple" => Some(icons::SPEAKER_MULTIPLE),
        "speaker-off" => Some(icons::SPEAKER_OFF),
        "speaker-pause" => Some(icons::SPEAKER_PAUSE),
        "speaker-play" => Some(icons::SPEAKER_PLAY),
        "speaker-stop" => Some(icons::SPEAKER_STOP),
        "speaker-wireless" => Some(icons::SPEAKER_WIRELESS),
        "speaker" => Some(icons::SPEAKER),
        "spear" => Some(icons::SPEAR),
        "speedometer-medium" => Some(icons::SPEEDOMETER_MEDIUM),
        "speedometer-slow" => Some(icons::SPEEDOMETER_SLOW),
        "speedometer" => Some(icons::SPEEDOMETER),
        "spellcheck" => Some(icons::SPELLCHECK),
        "sphere-off" => Some(icons::SPHERE_OFF),
        "sphere" => Some(icons::SPHERE),
        "spider-outline" => Some(icons::SPIDER_OUTLINE),
        "spider-thread" => Some(icons::SPIDER_THREAD),
        "spider-web" => Some(icons::SPIDER_WEB),
        "spider" => Some(icons::SPIDER),
        "spirit-level" => Some(icons::SPIRIT_LEVEL),
        "spoon-sugar" => Some(icons::SPOON_SUGAR),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        "spotlight-beam" => Some(icons::SPOTLIGHT_BEAM),
        "spotlight" => Some(icons::SPOTLIGHT),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "spray" => Some(icons::SPRAY),
        "sprinkler-fire" => Some(icons::SPRINKLER_FIRE),
        "sprinkler-variant" => Some(icons::SPRINKLER_VARIANT),
        "sprinkler" => Some(icons::SPRINKLER),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        "sprout" => Some(icons::SPROUT),
        "square-circle-outline" => Some(icons::SQUARE_CIRCLE_OUTLINE),
        "square-circle" => Some(icons::SQUARE_CIRCLE),
        "square-edit-outline" => Some(icons::SQUARE_EDIT_OUTLINE),
        "square-medium-outline" => Some(icons::SQUARE_MEDIUM_OUTLINE),
        "square-medium" => Some(icons::SQUARE_MEDIUM),
        "square-off-outline" => Some(icons::SQUARE_OFF_OUTLINE),
        "square-off" => Some(icons::SQUARE_OFF),
        "square-opacity" => Some(icons::SQUARE_OPACITY),
        "square-outline" => Some(icons::SQUARE_OUTLINE),
        "square-root-box" => Some(icons::SQUARE_ROOT_BOX),
        "square-root" => Some(icons::SQUARE_ROOT),
        "square-rounded-badge-outline" => Some(icons::SQUARE_ROUNDED_BADGE_OUTLINE),
        "square-rounded-badge" => Some(icons::SQUARE_ROUNDED_BADGE),
        "square-rounded-outline" => Some(icons::SQUARE_ROUNDED_OUTLINE),
        "square-rounded" => Some(icons::SQUARE_ROUNDED),
        "square-small" => Some(icons::SQUARE_SMALL),
        "square-wave" => Some(icons::SQUARE_WAVE),
        "square" => Some(icons::SQUARE),
        "squeegee" => Some(icons::SQUEEGEE),
        "ssh" => Some(icons::SSH),
        #[allow(deprecated)]
        "stack-exchange" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-exchange' is deprecated.").print(py);
            }
            Some(icons::STACK_EXCHANGE)
        }
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        #[allow(deprecated)]
        "stackpath" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stackpath' is deprecated.").print(py);
            }
            Some(icons::STACKPATH)
        }
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "stadium-variant" => Some(icons::STADIUM_VARIANT),
        "stadium" => Some(icons::STADIUM),
        "stairs-box" => Some(icons::STAIRS_BOX),
        "stairs-down" => Some(icons::STAIRS_DOWN),
        "stairs-up" => Some(icons::STAIRS_UP),
        "stairs" => Some(icons::STAIRS),
        "stamper" => Some(icons::STAMPER),
        "standard-definition" => Some(icons::STANDARD_DEFINITION),
        "star-box-multiple-outline" => Some(icons::STAR_BOX_MULTIPLE_OUTLINE),
        "star-box-multiple" => Some(icons::STAR_BOX_MULTIPLE),
        "star-box-outline" => Some(icons::STAR_BOX_OUTLINE),
        "star-box" => Some(icons::STAR_BOX),
        "star-check-outline" => Some(icons::STAR_CHECK_OUTLINE),
        "star-check" => Some(icons::STAR_CHECK),
        "star-circle-outline" => Some(icons::STAR_CIRCLE_OUTLINE),
        "star-circle" => Some(icons::STAR_CIRCLE),
        "star-cog-outline" => Some(icons::STAR_COG_OUTLINE),
        "star-cog" => Some(icons::STAR_COG),
        "star-crescent" => Some(icons::STAR_CRESCENT),
        "star-david" => Some(icons::STAR_DAVID),
        "star-face" => Some(icons::STAR_FACE),
        "star-four-points-box-outline" => Some(icons::STAR_FOUR_POINTS_BOX_OUTLINE),
        "star-four-points-box" => Some(icons::STAR_FOUR_POINTS_BOX),
        "star-four-points-circle-outline" => Some(icons::STAR_FOUR_POINTS_CIRCLE_OUTLINE),
        "star-four-points-circle" => Some(icons::STAR_FOUR_POINTS_CIRCLE),
        "star-four-points-outline" => Some(icons::STAR_FOUR_POINTS_OUTLINE),
        "star-four-points-small" => Some(icons::STAR_FOUR_POINTS_SMALL),
        "star-four-points" => Some(icons::STAR_FOUR_POINTS),
        "star-half-full" => Some(icons::STAR_HALF_FULL),
        "star-half" => Some(icons::STAR_HALF),
        "star-minus-outline" => Some(icons::STAR_MINUS_OUTLINE),
        "star-minus" => Some(icons::STAR_MINUS),
        "star-off-outline" => Some(icons::STAR_OFF_OUTLINE),
        "star-off" => Some(icons::STAR_OFF),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "star-plus-outline" => Some(icons::STAR_PLUS_OUTLINE),
        "star-plus" => Some(icons::STAR_PLUS),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "star-remove" => Some(icons::STAR_REMOVE),
        "star-settings-outline" => Some(icons::STAR_SETTINGS_OUTLINE),
        "star-settings" => Some(icons::STAR_SETTINGS),
        "star-shooting-outline" => Some(icons::STAR_SHOOTING_OUTLINE),
        "star-shooting" => Some(icons::STAR_SHOOTING),
        "star-three-points-outline" => Some(icons::STAR_THREE_POINTS_OUTLINE),
        "star-three-points" => Some(icons::STAR_THREE_POINTS),
        "star" => Some(icons::STAR),
        "state-machine" => Some(icons::STATE_MACHINE),
        #[allow(deprecated)]
        "steam" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'steam' is deprecated.").print(py);
            }
            Some(icons::STEAM)
        }
        "steering-off" => Some(icons::STEERING_OFF),
        "steering" => Some(icons::STEERING),
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "step-backward" => Some(icons::STEP_BACKWARD),
        "step-forward-2" => Some(icons::STEP_FORWARD_2),
        "step-forward" => Some(icons::STEP_FORWARD),
        "stethoscope" => Some(icons::STETHOSCOPE),
        "sticker-alert-outline" => Some(icons::STICKER_ALERT_OUTLINE),
        "sticker-alert" => Some(icons::STICKER_ALERT),
        "sticker-check-outline" => Some(icons::STICKER_CHECK_OUTLINE),
        "sticker-check" => Some(icons::STICKER_CHECK),
        "sticker-circle-outline" => Some(icons::STICKER_CIRCLE_OUTLINE),
        "sticker-emoji" => Some(icons::STICKER_EMOJI),
        "sticker-minus-outline" => Some(icons::STICKER_MINUS_OUTLINE),
        "sticker-minus" => Some(icons::STICKER_MINUS),
        "sticker-outline" => Some(icons::STICKER_OUTLINE),
        "sticker-plus-outline" => Some(icons::STICKER_PLUS_OUTLINE),
        "sticker-plus" => Some(icons::STICKER_PLUS),
        "sticker-remove-outline" => Some(icons::STICKER_REMOVE_OUTLINE),
        "sticker-remove" => Some(icons::STICKER_REMOVE),
        "sticker-text-outline" => Some(icons::STICKER_TEXT_OUTLINE),
        "sticker-text" => Some(icons::STICKER_TEXT),
        "sticker" => Some(icons::STICKER),
        "stocking" => Some(icons::STOCKING),
        "stomach" => Some(icons::STOMACH),
        "stool-outline" => Some(icons::STOOL_OUTLINE),
        "stool" => Some(icons::STOOL),
        "stop-circle-outline" => Some(icons::STOP_CIRCLE_OUTLINE),
        "stop-circle" => Some(icons::STOP_CIRCLE),
        "stop" => Some(icons::STOP),
        "storage-tank-outline" => Some(icons::STORAGE_TANK_OUTLINE),
        "storage-tank" => Some(icons::STORAGE_TANK),
        "store-24-hour" => Some(icons::STORE_24_HOUR),
        "store-alert-outline" => Some(icons::STORE_ALERT_OUTLINE),
        "store-alert" => Some(icons::STORE_ALERT),
        _ => None,
    }
}
