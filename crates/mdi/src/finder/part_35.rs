// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_35(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "unicorn-variant" => Some(icons::UNICORN_VARIANT),
        "unicorn" => Some(icons::UNICORN),
        "unicycle" => Some(icons::UNICYCLE),
        #[allow(deprecated)]
        "unity" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unity' is deprecated.").print(py);
            }
            Some(icons::UNITY)
        }
        #[allow(deprecated)]
        "unreal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unreal' is deprecated.").print(py);
            }
            Some(icons::UNREAL)
        }
        "update" => Some(icons::UPDATE),
        "upload-box-outline" => Some(icons::UPLOAD_BOX_OUTLINE),
        "upload-box" => Some(icons::UPLOAD_BOX),
        "upload-circle-outline" => Some(icons::UPLOAD_CIRCLE_OUTLINE),
        "upload-circle" => Some(icons::UPLOAD_CIRCLE),
        "upload-lock-outline" => Some(icons::UPLOAD_LOCK_OUTLINE),
        "upload-lock" => Some(icons::UPLOAD_LOCK),
        "upload-multiple-outline" => Some(icons::UPLOAD_MULTIPLE_OUTLINE),
        "upload-multiple" => Some(icons::UPLOAD_MULTIPLE),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "upload-off-outline" => Some(icons::UPLOAD_OFF_OUTLINE),
        "upload-off" => Some(icons::UPLOAD_OFF),
        "upload-outline" => Some(icons::UPLOAD_OUTLINE),
        "upload" => Some(icons::UPLOAD),
        "usb-c-port" => Some(icons::USB_C_PORT),
        "usb-flash-drive-outline" => Some(icons::USB_FLASH_DRIVE_OUTLINE),
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "usb-port" => Some(icons::USB_PORT),
        "usb" => Some(icons::USB),
        "vacuum-outline" => Some(icons::VACUUM_OUTLINE),
        "vacuum" => Some(icons::VACUUM),
        "valve-closed" => Some(icons::VALVE_CLOSED),
        "valve-open" => Some(icons::VALVE_OPEN),
        "valve" => Some(icons::VALVE),
        "van-passenger" => Some(icons::VAN_PASSENGER),
        "van-utility" => Some(icons::VAN_UTILITY),
        "vanish-quarter" => Some(icons::VANISH_QUARTER),
        "vanish" => Some(icons::VANISH),
        "vanity-light" => Some(icons::VANITY_LIGHT),
        "variable-box" => Some(icons::VARIABLE_BOX),
        "variable" => Some(icons::VARIABLE),
        "vector-arrange-above" => Some(icons::VECTOR_ARRANGE_ABOVE),
        "vector-arrange-below" => Some(icons::VECTOR_ARRANGE_BELOW),
        "vector-bezier" => Some(icons::VECTOR_BEZIER),
        "vector-circle-variant" => Some(icons::VECTOR_CIRCLE_VARIANT),
        "vector-circle" => Some(icons::VECTOR_CIRCLE),
        "vector-combine" => Some(icons::VECTOR_COMBINE),
        "vector-curve" => Some(icons::VECTOR_CURVE),
        "vector-difference-ab" => Some(icons::VECTOR_DIFFERENCE_AB),
        "vector-difference-ba" => Some(icons::VECTOR_DIFFERENCE_BA),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "vector-ellipse" => Some(icons::VECTOR_ELLIPSE),
        "vector-intersection" => Some(icons::VECTOR_INTERSECTION),
        "vector-line" => Some(icons::VECTOR_LINE),
        "vector-link" => Some(icons::VECTOR_LINK),
        "vector-point-edit" => Some(icons::VECTOR_POINT_EDIT),
        "vector-point-minus" => Some(icons::VECTOR_POINT_MINUS),
        "vector-point-plus" => Some(icons::VECTOR_POINT_PLUS),
        "vector-point-select" => Some(icons::VECTOR_POINT_SELECT),
        "vector-point" => Some(icons::VECTOR_POINT),
        "vector-polygon-variant" => Some(icons::VECTOR_POLYGON_VARIANT),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "vector-polyline-edit" => Some(icons::VECTOR_POLYLINE_EDIT),
        "vector-polyline-minus" => Some(icons::VECTOR_POLYLINE_MINUS),
        "vector-polyline-plus" => Some(icons::VECTOR_POLYLINE_PLUS),
        "vector-polyline-remove" => Some(icons::VECTOR_POLYLINE_REMOVE),
        "vector-polyline" => Some(icons::VECTOR_POLYLINE),
        "vector-radius" => Some(icons::VECTOR_RADIUS),
        "vector-rectangle" => Some(icons::VECTOR_RECTANGLE),
        "vector-selection" => Some(icons::VECTOR_SELECTION),
        "vector-square-close" => Some(icons::VECTOR_SQUARE_CLOSE),
        "vector-square-edit" => Some(icons::VECTOR_SQUARE_EDIT),
        "vector-square-minus" => Some(icons::VECTOR_SQUARE_MINUS),
        "vector-square-open" => Some(icons::VECTOR_SQUARE_OPEN),
        "vector-square-plus" => Some(icons::VECTOR_SQUARE_PLUS),
        "vector-square-remove" => Some(icons::VECTOR_SQUARE_REMOVE),
        "vector-square" => Some(icons::VECTOR_SQUARE),
        "vector-triangle" => Some(icons::VECTOR_TRIANGLE),
        "vector-union" => Some(icons::VECTOR_UNION),
        "vhs" => Some(icons::VHS),
        "vibrate-off" => Some(icons::VIBRATE_OFF),
        "vibrate" => Some(icons::VIBRATE),
        "video-2d" => Some(icons::VIDEO_2D),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "video-3d-variant" => Some(icons::VIDEO_3D_VARIANT),
        "video-3d" => Some(icons::VIDEO_3D),
        "video-4k-box" => Some(icons::VIDEO_4K_BOX),
        "video-account" => Some(icons::VIDEO_ACCOUNT),
        "video-box-off" => Some(icons::VIDEO_BOX_OFF),
        "video-box" => Some(icons::VIDEO_BOX),
        "video-check-outline" => Some(icons::VIDEO_CHECK_OUTLINE),
        "video-check" => Some(icons::VIDEO_CHECK),
        "video-high-definition" => Some(icons::VIDEO_HIGH_DEFINITION),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "video-input-antenna" => Some(icons::VIDEO_INPUT_ANTENNA),
        "video-input-component" => Some(icons::VIDEO_INPUT_COMPONENT),
        "video-input-hdmi" => Some(icons::VIDEO_INPUT_HDMI),
        "video-input-scart" => Some(icons::VIDEO_INPUT_SCART),
        "video-input-svideo" => Some(icons::VIDEO_INPUT_SVIDEO),
        "video-marker-outline" => Some(icons::VIDEO_MARKER_OUTLINE),
        "video-marker" => Some(icons::VIDEO_MARKER),
        "video-minus-outline" => Some(icons::VIDEO_MINUS_OUTLINE),
        "video-minus" => Some(icons::VIDEO_MINUS),
        "video-off-outline" => Some(icons::VIDEO_OFF_OUTLINE),
        "video-off" => Some(icons::VIDEO_OFF),
        "video-outline" => Some(icons::VIDEO_OUTLINE),
        "video-plus-outline" => Some(icons::VIDEO_PLUS_OUTLINE),
        "video-plus" => Some(icons::VIDEO_PLUS),
        "video-stabilization" => Some(icons::VIDEO_STABILIZATION),
        "video-standard-definition" => Some(icons::VIDEO_STANDARD_DEFINITION),
        "video-switch-outline" => Some(icons::VIDEO_SWITCH_OUTLINE),
        "video-switch" => Some(icons::VIDEO_SWITCH),
        "video-vintage" => Some(icons::VIDEO_VINTAGE),
        "video-wireless-outline" => Some(icons::VIDEO_WIRELESS_OUTLINE),
        "video-wireless" => Some(icons::VIDEO_WIRELESS),
        "video" => Some(icons::VIDEO),
        "view-agenda-outline" => Some(icons::VIEW_AGENDA_OUTLINE),
        "view-agenda" => Some(icons::VIEW_AGENDA),
        "view-array-outline" => Some(icons::VIEW_ARRAY_OUTLINE),
        "view-array" => Some(icons::VIEW_ARRAY),
        "view-carousel-outline" => Some(icons::VIEW_CAROUSEL_OUTLINE),
        "view-carousel" => Some(icons::VIEW_CAROUSEL),
        "view-column-outline" => Some(icons::VIEW_COLUMN_OUTLINE),
        "view-column" => Some(icons::VIEW_COLUMN),
        "view-comfy-outline" => Some(icons::VIEW_COMFY_OUTLINE),
        "view-comfy" => Some(icons::VIEW_COMFY),
        "view-compact-outline" => Some(icons::VIEW_COMPACT_OUTLINE),
        "view-compact" => Some(icons::VIEW_COMPACT),
        "view-dashboard-edit-outline" => Some(icons::VIEW_DASHBOARD_EDIT_OUTLINE),
        "view-dashboard-edit" => Some(icons::VIEW_DASHBOARD_EDIT),
        "view-dashboard-outline" => Some(icons::VIEW_DASHBOARD_OUTLINE),
        "view-dashboard-variant-outline" => Some(icons::VIEW_DASHBOARD_VARIANT_OUTLINE),
        "view-dashboard-variant" => Some(icons::VIEW_DASHBOARD_VARIANT),
        "view-dashboard" => Some(icons::VIEW_DASHBOARD),
        "view-day-outline" => Some(icons::VIEW_DAY_OUTLINE),
        "view-day" => Some(icons::VIEW_DAY),
        "view-gallery-outline" => Some(icons::VIEW_GALLERY_OUTLINE),
        "view-gallery" => Some(icons::VIEW_GALLERY),
        "view-grid-compact" => Some(icons::VIEW_GRID_COMPACT),
        "view-grid-outline" => Some(icons::VIEW_GRID_OUTLINE),
        "view-grid-plus-outline" => Some(icons::VIEW_GRID_PLUS_OUTLINE),
        "view-grid-plus" => Some(icons::VIEW_GRID_PLUS),
        "view-grid" => Some(icons::VIEW_GRID),
        "view-headline" => Some(icons::VIEW_HEADLINE),
        "view-list-outline" => Some(icons::VIEW_LIST_OUTLINE),
        "view-list" => Some(icons::VIEW_LIST),
        "view-module-outline" => Some(icons::VIEW_MODULE_OUTLINE),
        "view-module" => Some(icons::VIEW_MODULE),
        "view-parallel-outline" => Some(icons::VIEW_PARALLEL_OUTLINE),
        "view-parallel" => Some(icons::VIEW_PARALLEL),
        "view-quilt-outline" => Some(icons::VIEW_QUILT_OUTLINE),
        "view-quilt" => Some(icons::VIEW_QUILT),
        "view-sequential-outline" => Some(icons::VIEW_SEQUENTIAL_OUTLINE),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "view-split-horizontal" => Some(icons::VIEW_SPLIT_HORIZONTAL),
        "view-split-vertical" => Some(icons::VIEW_SPLIT_VERTICAL),
        "view-stream-outline" => Some(icons::VIEW_STREAM_OUTLINE),
        "view-stream" => Some(icons::VIEW_STREAM),
        "view-week-outline" => Some(icons::VIEW_WEEK_OUTLINE),
        "view-week" => Some(icons::VIEW_WEEK),
        #[allow(deprecated)]
        "vimeo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vimeo' is deprecated.").print(py);
            }
            Some(icons::VIMEO)
        }
        "violin" => Some(icons::VIOLIN),
        "virtual-reality" => Some(icons::VIRTUAL_REALITY),
        "virus-off-outline" => Some(icons::VIRUS_OFF_OUTLINE),
        "virus-off" => Some(icons::VIRUS_OFF),
        "virus-outline" => Some(icons::VIRUS_OUTLINE),
        "virus" => Some(icons::VIRUS),
        #[allow(deprecated)]
        "vlc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vlc' is deprecated.").print(py);
            }
            Some(icons::VLC)
        }
        "voicemail" => Some(icons::VOICEMAIL),
        "volcano-outline" => Some(icons::VOLCANO_OUTLINE),
        "volcano" => Some(icons::VOLCANO),
        "volleyball" => Some(icons::VOLLEYBALL),
        "volume-equal" => Some(icons::VOLUME_EQUAL),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "volume-low" => Some(icons::VOLUME_LOW),
        "volume-medium" => Some(icons::VOLUME_MEDIUM),
        "volume-minus" => Some(icons::VOLUME_MINUS),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        "volume-off" => Some(icons::VOLUME_OFF),
        "volume-plus" => Some(icons::VOLUME_PLUS),
        "volume-source" => Some(icons::VOLUME_SOURCE),
        "volume-variant-off" => Some(icons::VOLUME_VARIANT_OFF),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "vote-outline" => Some(icons::VOTE_OUTLINE),
        "vote" => Some(icons::VOTE),
        "vpn" => Some(icons::VPN),
        #[allow(deprecated)]
        "vuejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuejs' is deprecated.").print(py);
            }
            Some(icons::VUEJS)
        }
        #[allow(deprecated)]
        "vuetify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuetify' is deprecated.").print(py);
            }
            Some(icons::VUETIFY)
        }
        "walk" => Some(icons::WALK),
        "wall-fire" => Some(icons::WALL_FIRE),
        "wall-sconce-flat-outline" => Some(icons::WALL_SCONCE_FLAT_OUTLINE),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "wall-sconce-flat-variant" => Some(icons::WALL_SCONCE_FLAT_VARIANT),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "wall-sconce-outline" => Some(icons::WALL_SCONCE_OUTLINE),
        "wall-sconce-round-outline" => Some(icons::WALL_SCONCE_ROUND_OUTLINE),
        "wall-sconce-round-variant-outline" => Some(icons::WALL_SCONCE_ROUND_VARIANT_OUTLINE),
        "wall-sconce-round-variant" => Some(icons::WALL_SCONCE_ROUND_VARIANT),
        "wall-sconce-round" => Some(icons::WALL_SCONCE_ROUND),
        "wall-sconce" => Some(icons::WALL_SCONCE),
        "wall" => Some(icons::WALL),
        "wallet-bifold-outline" => Some(icons::WALLET_BIFOLD_OUTLINE),
        "wallet-bifold" => Some(icons::WALLET_BIFOLD),
        "wallet-giftcard" => Some(icons::WALLET_GIFTCARD),
        _ => None,
    }
}
