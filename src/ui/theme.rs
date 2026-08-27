use hudhook::imgui::{Context, FontSource, StyleColor};

pub fn apply_theme(ctx: &mut Context) {
    let style = ctx.style_mut();

    style.alpha = 1.0;
    style.disabled_alpha = 1.0;
    style.window_padding = [12.0, 12.0];
    style.window_rounding = 11.5;
    style.window_border_size = 0.0;
    style.window_min_size = [20.0, 20.0];
    style.window_title_align = [0.5, 0.5];
    style.child_rounding = 0.0;
    style.child_border_size = 1.0;
    style.popup_rounding = 0.0;
    style.popup_border_size = 1.0;
    style.frame_padding = [4.0, 3.4];
    style.frame_rounding = 11.9;
    style.frame_border_size = 0.0;
    style.item_spacing = [4.3, 5.5];
    style.item_inner_spacing = [7.1, 1.8];
    style.cell_padding = [12.1, 9.2];
    style.indent_spacing = 0.0;
    style.columns_min_spacing = 4.9;
    style.scrollbar_size = 11.6;
    style.scrollbar_rounding = 15.9;
    style.grab_min_size = 3.7;
    style.grab_rounding = 20.0;
    style.tab_rounding = 0.0;
    style.tab_border_size = 0.0;
    style.button_text_align = [0.5, 0.5];
    style.selectable_text_align = [0.0, 0.0];

    let c = &mut style.colors;

    c[StyleColor::Text as usize] = [1.0, 1.0, 1.0, 1.0];
    c[StyleColor::TextDisabled as usize] = [0.27450982, 0.31764707, 0.4509804, 1.0];
    c[StyleColor::WindowBg as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::ChildBg as usize] = [0.09411765, 0.101960786, 0.11764706, 1.0];
    c[StyleColor::PopupBg as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::Border as usize] = [0.15686275, 0.16862746, 0.19215687, 1.0];
    c[StyleColor::BorderShadow as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::FrameBg as usize] = [0.11372549, 0.1254902, 0.15294118, 1.0];
    c[StyleColor::FrameBgHovered as usize] = [0.15686275, 0.16862746, 0.19215687, 1.0];
    c[StyleColor::FrameBgActive as usize] = [0.15686275, 0.16862746, 0.19215687, 1.0];
    c[StyleColor::TitleBg as usize] = [0.047058824, 0.05490196, 0.07058824, 1.0];
    c[StyleColor::TitleBgActive as usize] = [0.047058824, 0.05490196, 0.07058824, 1.0];
    c[StyleColor::TitleBgCollapsed as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::MenuBarBg as usize] = [0.09803922, 0.105882354, 0.12156863, 1.0];
    c[StyleColor::ScrollbarBg as usize] = [0.047058824, 0.05490196, 0.07058824, 1.0];
    c[StyleColor::ScrollbarGrab as usize] = [0.11764706, 0.13333334, 0.14901961, 1.0];
    c[StyleColor::ScrollbarGrabHovered as usize] = [0.15686275, 0.16862746, 0.19215687, 1.0];
    c[StyleColor::ScrollbarGrabActive as usize] = [0.11764706, 0.13333334, 0.14901961, 1.0];
    c[StyleColor::CheckMark as usize] = [0.972549, 1.0, 0.49803922, 1.0];
    c[StyleColor::SliderGrab as usize] = [0.972549, 1.0, 0.49803922, 1.0];
    c[StyleColor::SliderGrabActive as usize] = [1.0, 0.79607844, 0.49803922, 1.0];
    c[StyleColor::Button as usize] = [0.11764706, 0.13333334, 0.14901961, 1.0];
    c[StyleColor::ButtonHovered as usize] = [0.18039216, 0.1882353, 0.19607843, 1.0];
    c[StyleColor::ButtonActive as usize] = [0.15294118, 0.15294118, 0.15294118, 1.0];
    c[StyleColor::Header as usize] = [0.14117648, 0.16470589, 0.20784314, 1.0];
    c[StyleColor::HeaderHovered as usize] = [0.105882354, 0.105882354, 0.105882354, 1.0];
    c[StyleColor::HeaderActive as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::Separator as usize] = [0.12941177, 0.14901961, 0.19215687, 1.0];
    c[StyleColor::SeparatorHovered as usize] = [0.15686275, 0.18431373, 0.2509804, 1.0];
    c[StyleColor::SeparatorActive as usize] = [0.15686275, 0.18431373, 0.2509804, 1.0];
    c[StyleColor::ResizeGrip as usize] = [0.14509805, 0.14509805, 0.14509805, 1.0];
    c[StyleColor::ResizeGripHovered as usize] = [0.972549, 1.0, 0.49803922, 1.0];
    c[StyleColor::ResizeGripActive as usize] = [1.0, 1.0, 1.0, 1.0];
    c[StyleColor::Tab as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::TabHovered as usize] = [0.11764706, 0.13333334, 0.14901961, 1.0];
    c[StyleColor::TabActive as usize] = [0.11764706, 0.13333334, 0.14901961, 1.0];
    c[StyleColor::TabUnfocused as usize] = [0.078431375, 0.08627451, 0.101960786, 1.0];
    c[StyleColor::TabUnfocusedActive as usize] = [0.1254902, 0.27450982, 0.57254905, 1.0];
    c[StyleColor::PlotLines as usize] = [0.52156866, 0.6, 0.7019608, 1.0];
    c[StyleColor::PlotLinesHovered as usize] = [0.039215688, 0.98039216, 0.98039216, 1.0];
    c[StyleColor::PlotHistogram as usize] = [0.88235295, 0.79607844, 0.56078434, 1.0];
    c[StyleColor::PlotHistogramHovered as usize] = [0.95686275, 0.95686275, 0.95686275, 1.0];
    c[StyleColor::TableHeaderBg as usize] = [0.047058824, 0.05490196, 0.07058824, 1.0];
    c[StyleColor::TableBorderStrong as usize] = [0.047058824, 0.05490196, 0.07058824, 1.0];
    c[StyleColor::TableBorderLight as usize] = [0.0, 0.0, 0.0, 1.0];
    c[StyleColor::TableRowBg as usize] = [0.11764706, 0.13333334, 0.14901961, 1.0];
    c[StyleColor::TableRowBgAlt as usize] = [0.09803922, 0.105882354, 0.12156863, 1.0];
    c[StyleColor::TextSelectedBg as usize] = [0.9372549, 0.9372549, 0.9372549, 1.0];
    c[StyleColor::DragDropTarget as usize] = [0.49803922, 0.5137255, 1.0, 1.0];
    c[StyleColor::NavHighlight as usize] = [0.26666668, 0.2901961, 1.0, 1.0];
    c[StyleColor::NavWindowingHighlight as usize] = [0.49803922, 0.5137255, 1.0, 1.0];
    c[StyleColor::NavWindowingDimBg as usize] = [0.19607843, 0.1764706, 0.54509807, 0.5019608];
    c[StyleColor::ModalWindowDimBg as usize] = [0.19607843, 0.1764706, 0.54509807, 0.5019608];

    ctx.fonts().clear();

    let _ = ctx.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../../assets/segoeui.ttf"),
            size_pixels: 20.0,
            config: None,
        },
    ]);

    let _ = ctx.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../../assets/segoeuib.ttf"),
            size_pixels: 20.0,
            config: None,
        },
    ]);
}

#[inline(always)]
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> [f32; 4] {
    [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ]
}