#[repr(u32)]
pub enum wl_data_device_error {
    WL_DATA_DEVICE_ERROR_ROLE = 0,
    _not_univariant = 0xffffffff
}

#[repr(u32)]
pub enum wl_display_error {
    WL_DISPLAY_ERROR_INVALID_OBJECT = 0,
    WL_DISPLAY_ERROR_INVALID_METHOD = 1,
    WL_DISPLAY_ERROR_NO_MEMORY = 2,
}

#[repr(u32)]
pub enum wl_keyboard_key_state {
    WL_KEYBOARD_KEY_STATE_RELEASED = 0,
    WL_KEYBOARD_KEY_STATE_PRESSED = 1,
}

#[repr(u32)]
pub enum wl_keyboard_keymap_format {
    WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP = 0,
    WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1 = 1,
}

bitflags!(
    flags wl_output_mode: u32 {
        const WL_OUTPUT_MODE_CURRENT = 0x1,
        const WL_OUTPUT_MODE_PREFERRED = 0x2,
    }
);

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum wl_output_subpixel {
    WL_OUTPUT_SUBPIXEL_UNKNOWN = 0,
    WL_OUTPUT_SUBPIXEL_NONE = 1,
    WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB = 2,
    WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR = 3,
    WL_OUTPUT_SUBPIXEL_VERTICAL_RGB = 4,
    WL_OUTPUT_SUBPIXEL_VERTICAL_BGR = 5,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum wl_output_transform {
    WL_OUTPUT_TRANSFORM_NORMAL = 0,
    WL_OUTPUT_TRANSFORM_90 = 1,
    WL_OUTPUT_TRANSFORM_180 = 2,
    WL_OUTPUT_TRANSFORM_270 = 3,
    WL_OUTPUT_TRANSFORM_FLIPPED = 4,
    WL_OUTPUT_TRANSFORM_FLIPPED_90 = 5,
    WL_OUTPUT_TRANSFORM_FLIPPED_180 = 6,
    WL_OUTPUT_TRANSFORM_FLIPPED_270 = 7,
}

#[repr(u32)]
pub enum wl_pointer_axis {
    WL_POINTER_AXIS_VERTICAL_SCROLL = 0,
    WL_POINTER_AXIS_HORIZONTAL_SCROLL = 1,
}

#[repr(u32)]
pub enum wl_pointer_button_state {
    WL_POINTER_BUTTON_STATE_RELEASED = 0,
    WL_POINTER_BUTTON_STATE_PRESSED = 1,
}

#[repr(u32)]
pub enum wl_pointer_error {
    WL_POINTER_ERROR_ROLE = 0,
    _not_univariant = 0xffffffff
}

bitflags! {
    flags wl_seat_capability: u32 {
        const WL_SEAT_CAPABILITY_POINTER = 1,
        const WL_SEAT_CAPABILITY_KEYBOARD = 2,
        const WL_SEAT_CAPABILITY_TOUCH = 4,
    }
}

#[repr(u32)]
pub enum wl_shell_error {
    WL_SHELL_ERROR_ROLE = 0,
    _not_univariant = 0xffffffff
}

#[repr(u32)]
pub enum wl_shell_surface_fullscreen_method {
    WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT = 0,
    WL_SHELL_SURFACE_FULLSCREEN_METHOD_SCALE = 1,
    WL_SHELL_SURFACE_FULLSCREEN_METHOD_DRIVER = 2,
    WL_SHELL_SURFACE_FULLSCREEN_METHOD_FILL = 3,
}

#[repr(u32)]
pub enum wl_shell_surface_resize {
    WL_SHELL_SURFACE_RESIZE_NONE = 0,
    WL_SHELL_SURFACE_RESIZE_TOP = 1,
    WL_SHELL_SURFACE_RESIZE_BOTTOM = 2,
    WL_SHELL_SURFACE_RESIZE_LEFT = 4,
    WL_SHELL_SURFACE_RESIZE_TOP_LEFT = 5,
    WL_SHELL_SURFACE_RESIZE_BOTTOM_LEFT = 6,
    WL_SHELL_SURFACE_RESIZE_RIGHT = 8,
    WL_SHELL_SURFACE_RESIZE_TOP_RIGHT = 9,
    WL_SHELL_SURFACE_RESIZE_BOTTOM_RIGHT = 10,
}

#[repr(u32)]
pub enum wl_shell_surface_transient {
    WL_SHELL_SURFACE_TRANSIENT_INACTIVE = 0x1,
    _not_univariant = 0xffffffff
}

#[repr(u32)]
pub enum wl_shm_error {
    WL_SHM_ERROR_INVALID_FORMAT = 0,
    WL_SHM_ERROR_INVALID_STRIDE = 1,
    WL_SHM_ERROR_INVALID_FD = 2,
}

#[repr(u32)]
pub enum wl_shm_format {
    WL_SHM_FORMAT_ARGB8888 = 0,
    WL_SHM_FORMAT_XRGB8888 = 1,
    WL_SHM_FORMAT_C8 = 0x20203843,
    WL_SHM_FORMAT_RGB332 = 0x38424752,
    WL_SHM_FORMAT_BGR233 = 0x38524742,
    WL_SHM_FORMAT_XRGB4444 = 0x32315258,
    WL_SHM_FORMAT_XBGR4444 = 0x32314258,
    WL_SHM_FORMAT_RGBX4444 = 0x32315852,
    WL_SHM_FORMAT_BGRX4444 = 0x32315842,
    WL_SHM_FORMAT_ARGB4444 = 0x32315241,
    WL_SHM_FORMAT_ABGR4444 = 0x32314241,
    WL_SHM_FORMAT_RGBA4444 = 0x32314152,
    WL_SHM_FORMAT_BGRA4444 = 0x32314142,
    WL_SHM_FORMAT_XRGB1555 = 0x35315258,
    WL_SHM_FORMAT_XBGR1555 = 0x35314258,
    WL_SHM_FORMAT_RGBX5551 = 0x35315852,
    WL_SHM_FORMAT_BGRX5551 = 0x35315842,
    WL_SHM_FORMAT_ARGB1555 = 0x35315241,
    WL_SHM_FORMAT_ABGR1555 = 0x35314241,
    WL_SHM_FORMAT_RGBA5551 = 0x35314152,
    WL_SHM_FORMAT_BGRA5551 = 0x35314142,
    WL_SHM_FORMAT_RGB565 = 0x36314752,
    WL_SHM_FORMAT_BGR565 = 0x36314742,
    WL_SHM_FORMAT_RGB888 = 0x34324752,
    WL_SHM_FORMAT_BGR888 = 0x34324742,
    WL_SHM_FORMAT_XBGR8888 = 0x34324258,
    WL_SHM_FORMAT_RGBX8888 = 0x34325852,
    WL_SHM_FORMAT_BGRX8888 = 0x34325842,
    WL_SHM_FORMAT_ABGR8888 = 0x34324241,
    WL_SHM_FORMAT_RGBA8888 = 0x34324152,
    WL_SHM_FORMAT_BGRA8888 = 0x34324142,
    WL_SHM_FORMAT_XRGB2101010 = 0x30335258,
    WL_SHM_FORMAT_XBGR2101010 = 0x30334258,
    WL_SHM_FORMAT_RGBX1010102 = 0x30335852,
    WL_SHM_FORMAT_BGRX1010102 = 0x30335842,
    WL_SHM_FORMAT_ARGB2101010 = 0x30335241,
    WL_SHM_FORMAT_ABGR2101010 = 0x30334241,
    WL_SHM_FORMAT_RGBA1010102 = 0x30334152,
    WL_SHM_FORMAT_BGRA1010102 = 0x30334142,
    WL_SHM_FORMAT_YUYV = 0x56595559,
    WL_SHM_FORMAT_YVYU = 0x55595659,
    WL_SHM_FORMAT_UYVY = 0x59565955,
    WL_SHM_FORMAT_VYUY = 0x59555956,
    WL_SHM_FORMAT_AYUV = 0x56555941,
    WL_SHM_FORMAT_NV12 = 0x3231564e,
    WL_SHM_FORMAT_NV21 = 0x3132564e,
    WL_SHM_FORMAT_NV16 = 0x3631564e,
    WL_SHM_FORMAT_NV61 = 0x3136564e,
    WL_SHM_FORMAT_YUV410 = 0x39565559,
    WL_SHM_FORMAT_YVU410 = 0x39555659,
    WL_SHM_FORMAT_YUV411 = 0x31315559,
    WL_SHM_FORMAT_YVU411 = 0x31315659,
    WL_SHM_FORMAT_YUV420 = 0x32315559,
    WL_SHM_FORMAT_YVU420 = 0x32315659,
    WL_SHM_FORMAT_YUV422 = 0x36315559,
    WL_SHM_FORMAT_YVU422 = 0x36315659,
    WL_SHM_FORMAT_YUV444 = 0x34325559,
    WL_SHM_FORMAT_YVU444 = 0x34325659,
}

#[repr(u32)]
pub enum wl_subcompositor_error {
    WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE = 0,
    _not_univariant = 0xffffffff
}

#[repr(u32)]
pub enum wl_subsurface_error {
    WL_SUBSURFACE_ERROR_BAD_SURFACE = 0,
    _not_univariant = 0xffffffff
}

#[repr(u32)]
pub enum wl_surface_error {
    WL_SURFACE_ERROR_INVALID_SCALE = 0,
    WL_SURFACE_ERROR_INVALID_TRANSFORM = 1,
}